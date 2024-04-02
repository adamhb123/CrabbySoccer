mod queries;

use std::{fs::read_to_string, path::Path};
use sqlite;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn parse_csv() -> (String, Vec<String>){
    let mut lines = read_lines("soccer.csv");
    let csv_header = lines.remove(0);
    (csv_header, lines)
}

pub fn csv_to_sqlite(){
    let (header, data) = parse_csv();
    let _path = Path::new("soccer.db");
    
    match std::fs::remove_file(_path) {
        Ok(_) => { println!("Successfully deleted file: {}", _path.display()) },
        Err(err) => println!("[ERROR] {}", err),
    }
    let connection = sqlite::open("soccer.db").unwrap();
    connection.execute("CREATE TABLE ");
}