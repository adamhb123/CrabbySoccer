use crate::requests::{self, Endpoint};
use std::collections::HashMap;
use std::io::{self, Write};
use std::net::{TcpStream};
use std::vec;

const HELP_MSG: &'static str = "
 ----------------------------------------------------------------------
| CrabbySoccer Client CLI                                              |
 ----------------------------------------------------------------------
[USAGE]
    $ <COMMAND> <PARAMETER_1>=<ARG_1>,<ARG_2>,...,<ARG_N> <PARAMETER_2>=<VALUE_1>,<VALUE_2>,...,<VALUE_N>
    e.g.:
        $ get-all-players name=Joe\\ Smith
        $ get-player player_id=12345 statistics=goals,assists,shots,saves
"; 

fn print_help(){
    println!("{HELP_MSG}");
}

fn parse_input(buf: &str) -> Result<String, &str> {
    let mut argsplit: Vec<&str> = buf.split(" ").collect();
    // Parse and verify endpoint
    let endpoint = if let Some(e) = requests::clone_authoritative_endpoint_by_uri(argsplit.remove(0)) { e } else { return Err("No such Endpoint exists") };
    let mut query_pv_map: HashMap<&str, Vec<&str>> = HashMap::new();
    // Parse and verify query parameters and associated values
    while !argsplit.is_empty() {
        let mut query_kv_split: Vec<&str> = argsplit.pop().unwrap().split("=").collect();
        if query_kv_split.len() != 2 { return Err("Malformed input (couldn't parse query parameter-value pair)") }
        let vals: Vec<&str> = query_kv_split.pop().unwrap().split(",").collect();
        let param = query_kv_split.pop().unwrap();
        query_pv_map.insert(param, vals);
    }
    Ok(endpoint.get_request_string())
}

pub fn run() {
    print_help();
    let mut sock = TcpStream::connect("127.0.0.1:7878").unwrap();
    loop {
        let mut buf: String = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let buf = buf.trim();
        let request_string = match parse_input(&buf) {
            Ok(s) => s,
            Err(err) => {
                println!("ERROR: {}", err);
                continue;
            }
        };
        sock.write_all(request_string.as_bytes()).unwrap();
    }
}