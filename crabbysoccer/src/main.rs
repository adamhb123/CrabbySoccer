mod client;
mod server;
mod requests;
mod database;
mod common;
mod tests;


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

fn parse_args(args: std::env::Args) -> (ApplicationType, Option<bool>) {
    let args: Vec<String> = args.collect();
    // init_db is only relevant to the server
    let init_db: Option<bool> = if let Some(utype) = args.get(2) {
        if utype.contains("init-db") { Some(true) }
        else { Some(false) }
    } else { None };
    match args.get(1) {
        Some(s) => match s.to_lowercase() { 
            s if s.contains("server") => (ApplicationType::Server, init_db),
            s if s.contains("client") => (ApplicationType::Client, None),
            _ => { println!("Invalid ApplicationType argument provided, assuming Client..."); (ApplicationType::Client, None)}
        },
        None => {
            println!("ApplicationType argument not provided, assuming Client...");
            (ApplicationType::Client, None)
        }
    }
}


fn main() {
    let (utype, init_db)= parse_args(std::env::args());
    println!("Running as: {}", utype);
    if utype == ApplicationType::Server {
        server::run(init_db);
    } else if utype == ApplicationType::Client {
        client::run();
    }
}