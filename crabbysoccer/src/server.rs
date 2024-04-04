use crate::{
    database,
    requests::{self, Endpoint, QueryPVMap},
};
use std::{
    collections::HashMap, io::{BufRead, BufReader}, net::{TcpListener, TcpStream}, thread::{self, JoinHandle}
};

fn parse_request(request: Vec<String>) -> Endpoint {
    let uri = (request[0].split(" ").collect::<Vec<&str>>()[1])[1..].to_string();
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

        db.get_player(player_id_arg, statistics_arg);
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

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut thread_handles: Vec<JoinHandle<()>> = vec![];
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!(
            "Incoming connection from: {}",
            stream.peer_addr().unwrap().to_string()
        );
        thread_handles.push(thread::spawn(|| handle_connection(stream)));
    }
    // Clean up threads (probably never actually runs, but it looks cute)
    cleanup(&mut thread_handles);
}
