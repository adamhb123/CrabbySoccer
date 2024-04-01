use crate::requests;
use std::io;
use std::net::{TcpStream};

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

fn parse_input(buf: &String) -> Result<(),()>{
    let mut argsplit: Vec<&str> = buf.split(" ").collect();
    // Parse and verify endpoint
    let endpoint = if requests::endpoint_uri_exists(argsplit[0]) { argsplit.swap_remove(0) } else { return Err(()) };
    // Parse and verify query parameters and associated values
    while !argsplit.is_empty() {
        let query_kv_split: Vec<&str> = argsplit.pop().unwrap().split("=").collect();
        if query_kv_split.len() != 2 { return Err(()) }
        
    }
    
    Ok(())
}

pub fn run() {
    print_help();
    loop {
        let mut buf: String = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        parse_input(&buf);
    }
}