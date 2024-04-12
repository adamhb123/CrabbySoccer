use crate::{
    database,
    requests::{Endpoint, QueryPVMap},
};
use std::{
    collections::HashMap, io::{BufRead, BufReader, ErrorKind, self, Write}, net::{TcpListener, TcpStream}, thread::{self, JoinHandle}, sync::{Arc, atomic::{AtomicBool, Ordering}}
};

fn parse_request(request: Vec<String>) -> Endpoint {
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
    endpoint
}

fn respond(request: &Endpoint, db: &database::DB) {
    if request.uri == "get-all-players" { // expected params: name
    } else if request.uri == "get-player" {
        // expected params: player_id, statistics
        let (player_id, statistics) = (
            request.query_pv_map.get("player_id"),
            request.query_pv_map.get("statistics")
        );
        let (player_id_arg, statistics_arg) = (
            if player_id.is_some() { Some(player_id.unwrap()[0].clone()) } else { None },
            if statistics.is_some() { Some(statistics.unwrap().clone()) } else { None }
        );

        db.get_player(player_id_arg, statistics_arg).unwrap();
    }
}

fn handle_connection(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    let db = database::DB::new();
    loop {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(core::result::Result::unwrap)
            .take_while(|line| !line.is_empty())
            .collect();

        println!("[{}]: {:#?}", peer_addr, http_request);
        let parsed = parse_request(http_request);
        respond(&parsed, &db);
    }
}

fn cleanup(thread_handles: &mut Vec<JoinHandle<()>>) {
    while !thread_handles.is_empty() {
        thread_handles.pop().unwrap().join().unwrap();
    }
}

enum InputAction {
    Quit
}

fn parse_input(buf: &str) -> Option<InputAction> {
    let argsplit: Vec<String> = buf.split(" ").map(|e| e.trim().to_lowercase()).collect();
    if argsplit[0].contains("quit") { Some(InputAction::Quit) }
    else { None }
}


fn run_cli(shutdown_trigger: Arc<AtomicBool>) {
    let mut buf: String = String::new();
    
    loop {
        buf.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();
        let buf = buf.trim();
        match parse_input(&buf) {
            Some(action) => {
                match action {
                    InputAction::Quit => shutdown_trigger.store(true, Ordering::Relaxed),
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
                println!(
                    "Incoming connection from: {}",
                    _stream.peer_addr().unwrap().to_string()
                );
                stream_thread_handles.push(thread::spawn(|| handle_connection(_stream)));
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                println!("WOULD BLOCK: {}", e);
                // Decide if we should exit
                if shutdown_trigger.load(Ordering::Relaxed) { break; }
                // break;
                // Decide if we should try to accept a connection again
                thread::sleep(Duration::from())
                continue;
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }
    // Clean up threads (probably never actually runs, but it looks cute)
    stream_thread_handles.push(cli_thread_handle);
    cleanup(&mut stream_thread_handles);
}
