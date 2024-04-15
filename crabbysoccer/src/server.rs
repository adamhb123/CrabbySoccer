use crate::{
    database,
    requests::{self, Endpoint, QueryPVMap},
};
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, ErrorKind, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

fn parse_request(request: Vec<String>) -> Option<Endpoint> {
    if request.is_empty() {
        return None;
    }
    let uri = (request[0].split(" ").collect::<Vec<&str>>()[1])[1..].to_owned();
    let (uri, query_param_str) = match uri.split_once("?") {
        Some(split) => split,
        None => (uri.as_str(), ""),
    };
    println!("Parsed uri: {}", uri);
    let mut query_pv_map: QueryPVMap = HashMap::new();
    for qp_str in query_param_str.split("&").collect::<Vec<&str>>() {
        if qp_str.is_empty() {
            break;
        };
        let (qp, qvals) = qp_str.split_once("=").unwrap();
        let qvals = qvals.split(",").map(String::from).collect();
        query_pv_map.insert(qp.to_owned(), qvals);
    }
    let endpoint = Endpoint::new(uri, query_pv_map);
    println!("Parsed endpoint: {:#?}", endpoint);
    Some(endpoint)
}

fn respond(request: &Endpoint, db: &database::DB) {
    if request.uri == "get-all-players" { // optional params: name
    } else if request.uri == "get-player" {
        // optional params: player_id, statistics
        let (player_id, statistics) = (
            request.query_pv_map.get("player_id"),
            request.query_pv_map.get("statistics"),
        );
        let (player_id_arg, statistics_arg) = (
            if player_id.is_some() {
                Some(player_id.unwrap()[0].clone())
            } else {
                None
            },
            if statistics.is_some() {
                Some(statistics.unwrap().clone())
            } else {
                None
            },
        );

        let player = db.get_player(player_id_arg, statistics_arg).unwrap();
        println!("get_player result: {}", player);
    }
}

fn handle_connection(stream: TcpStream, shutdown_trigger: Arc<AtomicBool>) {
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
        buf_reader.read_until(requests::LF, &mut buf).unwrap();
        let http_request: Vec<String> =  buf.lines()
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
        respond(&parsed, &db);
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

fn cleanup(thread_handles: &mut Vec<JoinHandle<()>>) {
    println!("Cleaning up thread handles...");
    while !thread_handles.is_empty() {
        thread_handles.pop().unwrap().join().unwrap();
    }
    println!("Finished cleaning up thread handles...goodbye!");
}

enum InputAction {
    Quit,
}

fn parse_input(buf: &str) -> Option<InputAction> {
    let argsplit: Vec<String> = buf.split(" ").map(|e| e.trim().to_lowercase()).collect();
    if argsplit[0].contains("quit") || argsplit[0] == "q" {
        Some(InputAction::Quit)
    } else {
        None
    }
}

fn run_cli(shutdown_trigger: Arc<AtomicBool>) {
    let mut buf: String = String::new();
    loop {
        buf.clear();
        io::stdout().flush().unwrap();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();
        let buf = buf.trim();
        match parse_input(&buf) {
            Some(action) => match action {
                InputAction::Quit => {
                    shutdown_trigger.store(true, Ordering::Relaxed);
                    break;
                }
            },
            None => (),
        };
    }
}

pub fn run(init_db: Option<bool>) {
    // Define events
    let shutdown_trigger = Arc::new(AtomicBool::new(false));
    let cli_shutdown_trigger = shutdown_trigger.clone();

    if init_db.is_some_and(|b| b) {
        println!("Database initialization requested");
        println!("Running initialization (conversion of 'soccer.csv' -> 'soccer.db'");
        database::csv_to_sqlite();
    }
    println!("Starting server...");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");
    let mut stream_thread_handles: Vec<JoinHandle<()>> = vec![];
    println!("Server started successfully!");
    // Initialize Server CLI IO
    let cli_thread_handle = thread::spawn(|| run_cli(cli_shutdown_trigger));
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("Incoming connection from: {}", _stream.peer_addr().unwrap().to_string());
                let stream_shutdown = shutdown_trigger.clone();
                stream_thread_handles.push(thread::spawn(|| handle_connection(_stream, stream_shutdown)));
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
    stream_thread_handles.push(cli_thread_handle);
    cleanup(&mut stream_thread_handles);
}
