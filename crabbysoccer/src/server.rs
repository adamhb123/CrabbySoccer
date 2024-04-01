use std::{io::{BufRead, BufReader}, net::{TcpListener, TcpStream}, thread};
use crate::requests;


fn handle_connection(mut stream: TcpStream) {
    loop {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {:#?}", http_request);
    }
}

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Incoming connection from: {}", stream.peer_addr().unwrap().to_string());
        handle_connection(stream);
    }
}