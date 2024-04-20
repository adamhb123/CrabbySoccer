use crate::{
    common::{self, println_then_show_input_indicator, InputAction},
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
        Arc, RwLock,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

#[derive(Debug)]
struct Connection {
    name: String,
    stream: Option<TcpStream>,
    _shutdown_trigger: Arc<AtomicBool>,
    _handle: Option<JoinHandle<()>>,
}
impl Connection {
    fn new(stream: TcpStream, shutdown_trigger: Option<Arc<AtomicBool>>) -> Self {
        Connection {
            name: stream.peer_addr().unwrap().to_string().to_owned(),
            stream: Some(stream),
            _shutdown_trigger: shutdown_trigger.unwrap_or(Arc::new(AtomicBool::new(false))),
            _handle: None,
        }
    }

    fn start_thread(self) -> Self {
        // Consumes self and returns corpse with stream inaccessible
        let sd = self._shutdown_trigger.clone();
        let name = self.name.clone();
        let handle = thread::spawn(move || {
            self.stream
                .as_ref()
                .unwrap()
                .set_nonblocking(false)
                .expect("set_nonblocking call failed");
            let db = database::DB::new();
            let mut buf_reader = BufReader::new(self.stream.as_ref().unwrap());
            let mut buf: Vec<u8> = vec![];
            loop {
                if self._shutdown_trigger.load(Ordering::Relaxed) {
                    println!("Dropping connection: {}", self.name);
                    self.stream.as_ref().unwrap().write_all(&[]).unwrap(); // Send 0-len to notify other end of drop
                    break;
                }
                buf.clear();
                // Below should be converted to a non-blocking read in order to avoid hanging on server quit
                match buf_reader.read_until(requests::REQUEST_TERMINATOR, &mut buf) {
                    Ok(len) => {
                        println!("BUFREAD LEN: {}", len);
                        if len == 0 {
                            println!("Connection {} dropped!", self.name);
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
                println!("[{}]: {:#?}", self.name, http_request);
                let parsed = match parse_request(http_request) {
                    Some(ep) => ep,
                    None => break,
                };
                let response_string = match get_response_string(&parsed, &db) {
                    Some(rs) => rs,
                    None => "Failed to parse OR no response required".to_owned(),
                };
                println_then_show_input_indicator(format!("RESPONSE:\n{}", response_string));
                self.stream
                    .as_ref()
                    .unwrap()
                    .write_all(response_string.as_bytes())
                    .unwrap();

                /*Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    if shutdown_trigger.load(Ordering::Relaxed) {
                        println!("Dropping connection: {}", peer_addr);
                        break;
                    }
                    thread::sleep(Duration::from_millis(50))
                }
                Err(e) => panic!("encountered IO error: {e}"),*/
            }
        });
        Self {
            name,
            stream: None, // Stream is consumed by thread
            _shutdown_trigger: sd,
            _handle: Some(handle),
        }
    }
    fn shutdown(&mut self) {
        self._shutdown_trigger.store(true, Ordering::Relaxed)
    }

    fn join(self) -> Result<(), Box<dyn Any + Send>> {
        if let Some(h) = self._handle {
            h.join()
        } else {
            Err(Box::new("Cannot join: Connection stream handler thread not started!"))
        }
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
    let mut response_string: Option<String> = None;
    if request.uri == "get-all-players" {
        // optional params: name
        response_string = Some(if let Some(name) = request.query_pv_map.get("name") {
            let name = if name.len() == 1 {
                Some(name[0].clone().replace('+', " "))
            } else {
                None
            };
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
        response_string = Some(player);
    }
    if response_string.is_some() {
        response_string.map(|mut rs| {
            rs.push(char::from(requests::REQUEST_TERMINATOR));
            rs
        })
    } else {
        None
    }
}

fn cleanup(cli_thread_handle: JoinHandle<()>, connections: Arc<RwLock<Vec<Connection>>>) {
    println!("Cleaning up thread handles...");
    while let Some(mut conn) = connections.write().unwrap().pop() {
        println!("Cleaning up connection: {}", conn.name);
        conn.shutdown();
        conn.join().unwrap();
    }
    println!("Finished cleaning up thread handles...goodbye!");
    cli_thread_handle.join().unwrap();
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
    let connections: Arc<RwLock<Vec<Connection>>> = Arc::new(RwLock::new(vec![]));
    let cli_connections: Arc<RwLock<Vec<Connection>>> = connections.clone();
    println!("Server started successfully!");

    // Initialize Server CLI IO
    let cli_thread_handle = thread::spawn(|| run_cli(cli_shutdown_trigger, cli_connections));
    // Connection listener loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let conn = Connection::new(stream, Some(shutdown_trigger.clone()));
                println_then_show_input_indicator(format!("Incoming connection from: {}", conn.name));
                connections.write().unwrap().push(conn.start_thread());
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
    cleanup(cli_thread_handle, connections);
}
