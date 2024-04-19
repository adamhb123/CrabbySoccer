use crabbylib::println_around_input;
use queue::Queue;

use crate::{
    common::{self, InputAction},
    database,
    requests::{self, Endpoint, QueryPVMap},
};
use std::{
    any::Any,
    collections::HashMap,
    io::{self, BufRead, BufReader, ErrorKind, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, RwLock,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

#[derive(Debug)]
struct Connection {
    stream: TcpStream,
    _shutdown_trigger: Arc<AtomicBool>,
    _handle: Option<JoinHandle<()>>,
}
impl Connection {
    fn new(stream: TcpStream, shutdown_trigger: Option<Arc<AtomicBool>>) -> Self {
        Connection {
            stream,
            _shutdown_trigger: shutdown_trigger.unwrap_or(Arc::new(AtomicBool::new(false))),
            _handle: None,
        }
    }

    fn get_name(&self) -> String {
        self.stream.peer_addr().unwrap().to_string().to_owned()
    }

    fn start_thread(&mut self) {
        let sd = &self._shutdown_trigger;
        self._handle = Some(thread::spawn(|| loop {
            if sd.load(Ordering::Relaxed) {
                break;
            }
        }));
    }
    fn shutdown(&mut self) {
        self._shutdown_trigger.store(true, Ordering::Relaxed)
    }

    fn join(&self) -> Result<(), Box<dyn Any + Send>> {
        self._handle.unwrap().join()
    }
}

fn parse_request(request: Vec<String>) -> Option<Endpoint> {
    if request.is_empty() {
        return None;
    }
    let uri = (request[0].split(' ').collect::<Vec<&str>>()[1])[1..].to_owned();
    let (uri, query_param_str) = match uri.split_once('?') {
        Some(split) => split,
        None => (uri.as_str(), ""),
    };
    println!("Parsed uri: {}", uri);
    let mut query_pv_map: QueryPVMap = HashMap::new();
    for qp_str in query_param_str.split('&').collect::<Vec<&str>>() {
        if qp_str.is_empty() {
            break;
        };
        let (qp, qvals) = qp_str.split_once('=').unwrap();
        let qvals = qvals.split(',').map(String::from).collect();
        query_pv_map.insert(qp.to_owned(), qvals);
    }
    let endpoint = Endpoint::new(uri, query_pv_map);
    println!("Parsed endpoint: {:#?}", endpoint);
    Some(endpoint)
}

#[allow(clippy::manual_map)]
fn get_response_string(request: &Endpoint, db: &database::DB) -> Option<String> {
    if request.uri == "get-all-players" {
        // optional params: name
        return Some(if let Some(name) = request.query_pv_map.get("name") {
            let name = if name.len() == 1 { Some(name[0].clone()) } else { None };
            db.get_all_players(name).unwrap()
        } else {
            db.get_all_players(None).unwrap()
        });
    } else if request.uri == "get-player" {
        // optional params: player_id, statistics
        let (player_id, statistics) = (
            request.query_pv_map.get("player_id"),
            request.query_pv_map.get("statistics"),
        );
        let (player_id_arg, statistics_arg) = (
            match player_id {
                Some(player_id) => Some(player_id[0].clone()),
                None => None,
            },
            match statistics {
                Some(statistics) => Some(statistics.clone()),
                None => None,
            },
        );
        let player = db.get_player(player_id_arg, statistics_arg).unwrap();
        println!("get_player result: \n{}", player);
        return Some(player);
    }
    None
}

fn handle_connection(stream: TcpStream, shutdown_trigger: Arc<AtomicBool>, connections: Arc<RwLock<Vec<Connection>>>) {
    stream.set_nonblocking(false).expect("set_nonblocking call failed");
    let peer_addr = stream.peer_addr().unwrap();
    let db = database::DB::new();
    let mut buf_reader = BufReader::new(&stream);
    let mut buf: Vec<u8> = vec![];
    loop {
        if shutdown_trigger.load(Ordering::Relaxed) {
            println!("Dropping connection: {}", peer_addr);
            break;
        }
        buf.clear();
        // Below should be converted to a non-blocking read in order to avoid hanging on server quit
        match buf_reader.read_until(requests::REQUEST_TERMINATOR, &mut buf) {
            Ok(len) => {
                println!("BUFREAD LEN: {}", len);
                if len == 0 {
                    println!("Connection {} dropped!", stream.peer_addr().unwrap());
                    break;
                }
            }
            Err(_) => todo!(),
        }
        let http_request: Vec<String> = buf
            .lines()
            .take_while(|line| {
                let line = line.as_ref().unwrap();
                println!("{} is empty? {}", line, line.is_empty());
                !line.is_empty()
            })
            .map(Result::unwrap)
            .collect();
        println!("[{}]: {:#?}", peer_addr, http_request);
        let parsed = match parse_request(http_request) {
            Some(ep) => ep,
            None => break,
        };
        let response_string = match get_response_string(&parsed, &db) {
            Some(rs) => rs,
            None => "Failed to parse OR no response required".to_owned(),
        };
        println!("RESPONSE:\n{}", response_string);
        (&stream).write_all(response_string.as_bytes()).unwrap();
        /*Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            if shutdown_trigger.load(Ordering::Relaxed) {
                println!("Dropping connection: {}", peer_addr);
                break;
            }
            thread::sleep(Duration::from_millis(50))
        }
        Err(e) => panic!("encountered IO error: {e}"),*/
    }
}

fn cleanup(connections: Arc<RwLock<Vec<Connection>>>) {
    println!("Cleaning up thread handles...");
    while let Some(mut conn) = connections.write().unwrap().pop() {
        let name = conn.get_name();
        println!("Cleaning up connection: {}", name);
        conn.shutdown();
        conn.join().unwrap();
    }
    println!("Finished cleaning up thread handles...goodbye!");
}

fn parse_input(buf: &str) -> Option<InputAction> {
    let argsplit: Vec<String> = buf.split(' ').map(|e| e.trim().to_lowercase().to_owned()).collect();
    common::parse_input_action(&argsplit)
}

fn run_cli(shutdown_trigger: Arc<AtomicBool>, stream_handles: Arc<RwLock<Vec<Connection>>>) {
    let mut buf: String = String::new();
    loop {
        if shutdown_trigger.load(Ordering::Relaxed) {
            break;
        }
        buf.clear();
        io::stdout().flush().unwrap();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();
        let buf = buf.trim();
        if let Some(action) = parse_input(buf) {
            match action {
                InputAction::Quit => shutdown_trigger.store(true, Ordering::Relaxed),
                InputAction::ListConnections => {
                    println!("Connections: {:#?}", stream_handles.read().unwrap());
                }
            }
        }
    }
}

pub fn run(init_db: Option<bool>) {
    // Define events
    let shutdown_trigger = Arc::new(AtomicBool::new(false));
    let cli_shutdown_trigger: Arc<AtomicBool> = shutdown_trigger.clone();
    let ctrlc_shutdown_trigger: Arc<AtomicBool> = shutdown_trigger.clone();
    ctrlc::set_handler(move || {
        if !ctrlc_shutdown_trigger.load(Ordering::SeqCst) {
            println!("Ctrl-C detected...press ENTER to exit");
            ctrlc_shutdown_trigger.store(true, Ordering::SeqCst);
        }
    })
    .unwrap();
    let _path = std::path::Path::new("soccer.db");
    if init_db.is_some_and(|b| b) || !_path.exists() {
        println!("Database initializating...");
        println!("Running initialization (conversion of 'soccer.csv' -> 'soccer.db'...");
        database::csv_to_sqlite();
    }
    println!("Starting server...");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");
    let stream_thread_handles: Arc<RwLock<Vec<Connection>>> = Arc::new(RwLock::new(vec![]));
    let cli_stream_thread_handles: Arc<RwLock<Vec<Connection>>> = stream_thread_handles.clone();
    println!("Server started successfully!");

    // Initialize Server CLI IO
    let cli_thread_handle = thread::spawn(|| run_cli(cli_shutdown_trigger, cli_stream_thread_handles));
    // Connection listener loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let connection = Connection::new(stream, Some(shutdown_trigger.clone()));
                println!("Incoming connection from: {}", connection.get_name());
                stream_thread_handles.write().unwrap().push(connection);
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // println!("WOULD BLOCK: {}", e);
                // Decide if we should exit
                if shutdown_trigger.load(Ordering::Relaxed) {
                    break;
                }
                thread::sleep(Duration::from_millis(50));
                continue;
            }
            Err(e) => panic!("Encountered IO error: {}", e),
        }
    }
    shutdown_trigger.store(true, Ordering::Relaxed);
    //stream_thread_handles.push((Some("CLI".to_owned()), cli_thread_handle));
    cleanup(stream_thread_handles);
}
