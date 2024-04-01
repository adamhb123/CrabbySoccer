mod client;
mod server;
mod requests;
mod database;
mod tests;

use colored::*;

#[derive(PartialEq)]
enum ApplicationType {
    Server,
    Client
}
impl std::fmt::Display for ApplicationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let utype = if let ApplicationType::Server = &self { ("Server", ApplicationType::Server as u8) } 
            else { ("Client", ApplicationType::Server as u8) };
        write!(f, "ApplicationType {} = {}", utype.0, utype.1)
    }
}

fn parse_args(args: std::env::Args) -> ApplicationType {
    match args.collect::<Vec<String>>().get(1) {
        Some(s) => match s.to_lowercase() { 
            s if s.contains("server") => ApplicationType::Server,
            s if s.contains("client") => ApplicationType::Client,
            _ => { println!("Invalid ApplicationType argument provided, assuming Client..."); ApplicationType::Client}
        },
        None => {
            println!("ApplicationType argument not provided, assuming Client...");
            ApplicationType::Client
        }
    }
}


fn main() {
    let utype = parse_args(std::env::args());
    println!("Running as: {}", utype);
    if utype == ApplicationType::Server {
        server::run()
    }
}