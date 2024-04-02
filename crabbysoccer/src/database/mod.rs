mod queries;

use sqlite;
use std::{fmt::Display, fs::read_to_string, path::Path, str::FromStr};

const JOIN_ALL: &str = "player JOIN statistics ON player.id = statistics.player_id JOIN position ON player.id = position.player_id";
pub struct DB {
    pub connection: sqlite::ConnectionThreadSafe,
}
impl DB {
    pub fn new() -> Self {
        Self {
            connection: sqlite::Connection::open_thread_safe("soccer.db").unwrap()
        }
    }
    pub fn get_player(
        &self,
        player_id: Option<String>,
        statistics: Option<Vec<String>>,
    ) -> Result<(), sqlite::Error> {
        let mut statistics = statistics.unwrap_or(vec![]);
        statistics
            .iter_mut()
            .for_each(|e: &mut String| e.insert_str(0, "statistics."));
        let statistics_string = if !statistics.is_empty() {
            let mut statistics = statistics.join(",");
            statistics.insert(0, ',');
            statistics
        } else {
            "".to_string()
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

fn parse_csv() -> (Vec<String>, Vec<Vec<String>>) {
    let mut lines: Vec<Vec<String>> = read_lines("soccer.csv")
        .iter()
        .map(|e| e.split(",").map(|e| String::from(e.trim())).collect())
        .collect();
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
    let csv_header = lines.remove(0);
    let ignore_columns_indices: Vec<usize> = csv_header
            .iter()
            .enumerate()
            .filter(|e| IGNORE_COLUMNS.contains(&e.1.as_str()))
            .map(|e| e.0)
            .collect();
    println!("IGNORE COLUMNS INDICES: {:?}", ignore_columns_indices);
    // lines.iter_mut().for_each(|line| ignore_columns_indices.iter().for_each(|idx| {line.remove(*idx);}));
    for mut line in lines {
        for idx in ignore_columns_indices {
            line.remove(idx);
        }
    }

    (csv_header, lines)
}

pub fn csv_to_sqlite() {
    let (header, data) = parse_csv();
    let _path = Path::new("soccer.db");

    match std::fs::remove_file(_path) {
        Ok(_) => {
            println!("Successfully deleted file: {}", _path.display())
        }
        Err(err) => println!("[ERROR] {}", err),
    }
    let connection = sqlite::open("soccer.db").unwrap();
    connection.execute("CREATE TABLE ");
}
