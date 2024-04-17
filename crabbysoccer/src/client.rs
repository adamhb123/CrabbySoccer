use crate::requests;
use queue::Queue;
use std::collections::HashMap;
use std::io::{self, Write};
use std::net::TcpStream;
use std::str::Bytes;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
    let mut argsplit: Vec<String> = buf.split(" ").map(|e| e.to_owned()).collect();
    // Merge double-quote strings into single args
    println!("ARGSPLIT before double quote parse");
    println!("{:?}", argsplit);
    let double_quote_args: Vec<(usize, String)> = argsplit
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_, a)| a.contains("\"") && !a.ends_with("\""))
        .collect();
    println!("DQA {:?}", double_quote_args);
    double_quote_args.iter().for_each(|(i, _)| {
        let idx = *i;
        let mut join_vec = vec![];
        loop {
            let val = argsplit.remove(idx);
            join_vec.push(val.clone());
            if val.ends_with("\"") {
                break;
            }
        }
        println!("join_vec {:?}", join_vec);
        argsplit.insert(idx, join_vec.join(" "));
    });
    println!("ARGSPLIT after double quote parse");
    println!("{:?}", argsplit);

    // Check if quitting
    if argsplit[0].contains("quit") || argsplit[0] == "q" {
        return Err("Quitting application...");
    }
    // Parse and verify endpoint
    let mut endpoint = if let Some(e) = requests::clone_authoritative_endpoint_by_uri(argsplit.remove(0).as_str()) {
        e
    } else {
        return Err("No such Endpoint exists");
    };
    let mut query_pv_map: HashMap<String, Vec<String>> = HashMap::new();
    // Parse and verify query parameters and associated values
    while !argsplit.is_empty() {
        let mut query_kv_split: Vec<String> = argsplit.pop().unwrap().split("=").map(|e| e.to_owned()).collect();
        println!("Query_kv_split: {:?}", query_kv_split);
        if query_kv_split.len() != 2 {
            return Err("Malformed input (couldn't parse query parameter-value pair)");
        }
        let vals: Vec<String> = query_kv_split.pop().unwrap().split(",").map(|e| e.to_owned()).collect();
        let param = query_kv_split.pop().unwrap();
        query_pv_map.insert(param, vals);
    }
    println!("Query PV Map parse_args: {:?}", query_pv_map);
    endpoint.query_pv_map = query_pv_map;
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
                    Err(_) => "[ERROR] UNABLE TO RETREIVE SERVER ADDRESS".to_owned(),
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
                if error_timeout.as_millis() > CONNECT_MAX_ERROR_TIMEOUT_MS {
                    error_timeout = std::time::Duration::from_millis(5000);
                }
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

fn handle_stream(mut stream: TcpStream, send_queue: Arc<Mutex<Queue<String>>>, receive_queue: Arc<Mutex<Queue<String>>>) {
    stream.set_nonblocking(true).expect("Failed to set stream nonblocking");
    loop {
        // if shutdown_trigger.load(Ordering::Relaxed) {
        //     break;
        // }
        // Handle 
        // Note: Mutex unlocks when MutexGuard (iq_locked, here) goes out of scope or gets manually drop()-ed
        let mut send_q_locked = send_queue.lock().unwrap();
        while !send_q_locked.is_empty() {
            if let Some(request_bytes) = send_q_locked.dequeue() {
                stream.write_all(request_bytes.as_bytes()).unwrap();
            }
        }
        // Unlock send_queue by dropping MutexGuard iq_locked
        // drop(iq_locked);
        // Allow other parties to modify send_queue
        // thread::sleep(Duration::from_millis(50));
    }
}

pub fn run() {
    _assertion_checks();
    let exit_trigger = Arc::new(AtomicBool::new(false));
    let ctrl_c_exit_trigger: Arc<AtomicBool> = exit_trigger.clone();
    ctrlc::set_handler(move || {
        println!("Ctrl-C event triggered");
        ctrl_c_exit_trigger.store(true, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    let send_queue: Arc<Mutex<Queue<String>>> = Arc::new(Mutex::new(Queue::new()));
    let send_queue_stream_handler: Arc<Mutex<Queue<String>>> = send_queue.clone();
    let receive_queue: Arc<Mutex<Queue<String>>> = Arc::new(Mutex::new(Queue::new()));
    let receive_queue_stream_handler: Arc<Mutex<Queue<String>>> = receive_queue.clone();

    let stream_handler = thread::spawn(|| handle_stream(try_connect(), send_queue_stream_handler, receive_queue_stream_handler));
    print_help();
    let mut buf: String = String::new();
    loop {
        if exit_trigger.load(Ordering::SeqCst) {
            break;
        }
        buf.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();
        let buf = buf.trim();
        
        if let Ok(mut iq_locked) = send_queue.lock() {
            let request_string = match parse_input(&buf) {
                Ok(s) => s,
                Err(err) => {
                    println!("[ERROR] {}", err);
                    continue;
                }
            };
            iq_locked.queue(request_string).unwrap();
        }
    }
    println!("Loop exited");
    stream_handler.join().unwrap();
}
