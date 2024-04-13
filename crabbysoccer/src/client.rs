use crate::requests;
use std::collections::HashMap;
use std::env::args;
use std::io::{self, Write};
use std::net::TcpStream;

const SERVER_ADDR: &str = "127.0.0.1:7878";
const CONNECT_INIT_ERROR_TIMEOUT_MS: u64 = 1000;
const CONNECT_MAX_ERROR_TIMEOUT_MS: u128 = 5000;
const CONNECT_MAX_TRIES: u8 = 10;

const _HELP_MSG: &'static str = "
 ----------------------------------------------------------------------
| CrabbySoccer Client CLI                                              |
 ----------------------------------------------------------------------
[USAGE]
    $ <COMMAND> <PARAMETER_1>=<ARG_1>,<ARG_2>,...,<ARG_N> <PARAMETER_2>=<VALUE_1>,<VALUE_2>,...,<VALUE_N>
    e.g.:
        $ get-all-players name=Joe\\ Smith
        $ get-player player_id=12345 statistics=goals,assists,shots,saves
";

fn print_help() {
    println!("{_HELP_MSG}");
}

fn parse_input(buf: &str) -> Result<String, &str> {
    let mut argsplit: Vec<&str> = buf.split(" ").collect();
    // Check if quitting
    if argsplit[0].contains("quit") || argsplit[0] == "q" { return Err("Quitting application..."); }
    // Parse and verify endpoint
    let endpoint =
        if let Some(e) = requests::clone_authoritative_endpoint_by_uri(argsplit.remove(0)) {
            e
        } else {
            return Err("No such Endpoint exists");
        };
    let mut query_pv_map: HashMap<&str, Vec<&str>> = HashMap::new();
    // Parse and verify query parameters and associated values
    while !argsplit.is_empty() {
        let mut query_kv_split: Vec<&str> = argsplit.pop().unwrap().split("=").collect();
        if query_kv_split.len() != 2 {
            return Err("Malformed input (couldn't parse query parameter-value pair)");
        }
        let vals: Vec<&str> = query_kv_split.pop().unwrap().split(",").collect();
        let param = query_kv_split.pop().unwrap();
        query_pv_map.insert(param, vals);
    }
    Ok(endpoint.get_request_string())
}

fn try_connect() -> TcpStream {
    let sock: TcpStream;
    let mut error_timeout: std::time::Duration = std::time::Duration::from_millis(CONNECT_INIT_ERROR_TIMEOUT_MS);
    let mut attempts: u8 = 0;
    loop {
        sock = match TcpStream::connect(SERVER_ADDR) {
            Ok(s) => {
                let peer_addr = match s.peer_addr() {
                    Ok(addr) => addr.to_string(),
                    Err(_) => "[ERROR] UNABLE TO RETREIVE SERVER ADDRESS".to_owned()
                };
                println!("Connection established with {}", peer_addr);
                s
            }
            Err(e) => {
                attempts += 1;
                println!("[ERROR] {}", e);
                if error_timeout.as_millis() < CONNECT_MAX_ERROR_TIMEOUT_MS {
                    error_timeout = error_timeout.mul_f64(1.1);
                }
                if error_timeout.as_millis() > CONNECT_MAX_ERROR_TIMEOUT_MS { error_timeout = std::time::Duration::from_millis(5000); }
                println!("\t! Retrying in {} ms...", error_timeout.as_millis());
                std::thread::sleep(error_timeout);
                if attempts > CONNECT_MAX_TRIES {
                    panic!("Exceeded max connection attempts ({})", CONNECT_MAX_TRIES);
                }
                continue;
            }
        };
        break;
    }
    sock
}

fn _assertion_checks() {
    assert!((CONNECT_INIT_ERROR_TIMEOUT_MS as u128) < CONNECT_MAX_ERROR_TIMEOUT_MS);
}

pub fn run() {
    _assertion_checks();
    let mut sock = try_connect();
    print_help();
    let mut buf: String = String::new();
    
    loop {
        buf.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();
        let buf = buf.trim();
        let request_string = match parse_input(&buf) {
            Ok(s) => s,
            Err(err) => {
                println!("{}", err);
                break;
            }
        };
        sock.write_all(request_string.as_bytes()).unwrap();
    }
}
