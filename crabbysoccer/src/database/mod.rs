mod queries;

use sqlite;
use std::{fmt::Display, fs::read_to_string, path::Path, str::FromStr};

const JOIN_ALL: &str =
    "player JOIN statistics ON player.id = statistics.player_id JOIN position ON player.id = position.player_id";
pub struct DB {
    pub connection: sqlite::ConnectionThreadSafe,
}
impl DB {
    pub fn new() -> Self {
        Self {
            connection: sqlite::Connection::open_thread_safe("soccer.db").unwrap(),
        }
    }
    pub fn get_player(&self, player_id: Option<String>, statistics: Option<Vec<String>>) -> Result<(), sqlite::Error> {
        let statistics_string = match statistics {
            Some(s) => {
                let mut statistics = s;
                if !statistics.is_empty() {
                    statistics
                        .iter_mut()
                        .for_each(|e: &mut String| e.insert_str(0, "statistics."));
                    let mut statistics = statistics.join(",");
                    statistics.insert(0, ',');
                    statistics
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        };

        let where_pid_clause = match player_id {
            Some(id) => format!("WHERE player.id = {id}"),
            None => "".to_owned(),
        };
        let query = format!("SELECT player.id, position.name {statistics_string} FROM {JOIN_ALL} {where_pid_clause};");
        println!("Querying DB: {}", query);
        self.connection.execute(query)
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(|e| e.trim().to_string())
        .collect() // gather them together into a vector
}

fn _get_ignored_columns(csv_header: &Vec<String>) -> Vec<(usize, String)> {
    const IGNORE_COLUMNS: [&'static str; 14] = [
        "Big chances missed",
        "Last man tackles",
        "Clearances off line",
        "Recoveries",
        "Duels won",
        "Duels lost",
        "Successful 50/50s",
        "Aerial battles won",
        "Aerial battles lost",
        "Errors leading to goal",
        "Big chances created",
        "Through balls",
        "Accurate long balls",
        "Sweeper clearances",
    ];
    csv_header
        .iter()
        .enumerate()
        .filter(|e| IGNORE_COLUMNS.contains(&e.1.as_str()))
        .map(|e| (e.0, e.1.to_owned()))
        .collect()
}

fn parse_csv() -> (Vec<String>, Vec<Vec<String>>) {
    let mut lines: Vec<Vec<String>> = read_lines("soccer.csv")
        .iter()
        .map(|e| e.split(",").map(|e| String::from(e.trim())).collect())
        .collect();

    let mut csv_header = lines.remove(0);
    let ignore_columns = _get_ignored_columns(&csv_header);
    println!("IGNORE COLUMNS INDICES: {:?}", ignore_columns);
    let mut csvh_offset = 0;
    for (idx, _) in &ignore_columns {
        csv_header.remove(*idx - csvh_offset);
        csvh_offset += 1;
    }
    //csv_header.iter_mut().for_each(|line| {  ignore_columns_indices.iter().for_each(|idx| {line.remove((*idx) - csvh_offset); csvh_offset += 1;})});
    lines.iter_mut().for_each(|line| {
        let mut offset = 0;
        ignore_columns.iter().for_each(|(idx, _)| {
            line.remove((*idx) - offset);
            offset += 1;
        })
    });
    (csv_header, lines)
}

pub fn csv_to_sqlite() {
    let (header, data) = parse_csv();
    println!("HEADER: {:#?}\nDATA HEAD: {:#?}", header, data[0]);
    let _path = Path::new("soccer.db");

    match std::fs::remove_file(_path) {
        Ok(_) => {
            println!("Successfully deleted file: {}", _path.display())
        }
        Err(_) => (),
    }
    println!("Building database...");

    let connection = sqlite::open("soccer.db").unwrap();
    connection.execute("CREATE TABLE player "); 
}
