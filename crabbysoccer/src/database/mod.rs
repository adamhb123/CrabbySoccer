use itertools::Itertools;
use sqlite::{self, Connection};
use std::{collections::HashMap, fmt::Display, fs::read_to_string, path::Path};

type AttributeMap<'a> = HashMap<&'a str, &'a str>;

const CREATE_TABLE_QUERIES: [&'static str; 3] = [
    "CREATE TABLE player (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(128) NOT NULL,
        jersey_number INTEGER NOT NULL,
        club_name VARCHAR(128),
        nationality VARCHAR(64) NOT NULL,
        age INTEGER NOT NULL);",
    "CREATE TABLE statistics (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        player_id INTEGER,
        appearances INTEGER NOT NULL,
        wins INTEGER NOT NULL,
        losses INTEGER NOT NULL,
        goals INTEGER NOT NULL,
        goals_per_match INTEGER NOT NULL,
        headed_goals INTEGER NOT NULL,
        goals_right_foot INTEGER NOT NULL,
        goals_left_foot INTEGER NOT NULL,
        goals_from_penalties INTEGER NOT NULL,
        goals_from_freekicks INTEGER NOT NULL,
        shots INTEGER NOT NULL,
        shots_on_target INTEGER NOT NULL,
        shooting_accuracy_pct DECIMAL(5,4) NOT NULL,
        hit_woodwork INTEGER NOT NULL,
        clean_sheets INTEGER NOT NULL,
        goals_conceded INTEGER NOT NULL,
        tackles INTEGER NOT NULL,
        tackle_success_pct DECIMAL(5,4) NOT NULL,
        shots_blocked INTEGER NOT NULL,
        interceptions INTEGER NOT NULL,
        clearances INTEGER NOT NULL,
        headed_clearances INTEGER NOT NULL,
        own_goals INTEGER NOT NULL,
        assists INTEGER NOT NULL,
        passes INTEGER NOT NULL,
        crosses INTEGER NOT NULL,
        cross_accuracy_pct DECIMAL(5,4) NOT NULL,
        passes_per_match INTEGER NOT NULL,
        saves INTEGER NOT NULL,
        penalties_saved INTEGER NOT NULL,
        punches INTEGER NOT NULL,
        high_claims INTEGER NOT NULL,
        catches INTEGER NOT NULL,
        throw_outs INTEGER NOT NULL,
        goal_kicks INTEGER NOT NULL,
        cards_yellow INTEGER NOT NULL,
        cards_red INTEGER NOT NULL,
        fouls INTEGER NOT NULL,
        offsides INTEGER NOT NULL
    );",
    "CREATE TABLE position (
        player_id INTEGER,
        name VARCHAR(10),
        PRIMARY KEY(player_id, name),
        FOREIGN KEY (player_id) REFERENCES player(id),
        CONSTRAINT chk_position_name CHECK (name IN ('Forward', 'Midfielder', 'Goalkeeper', 'Defender'))
    );",
];

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

fn insert_all_into<T: Copy + Display> (
    connection: &Connection,
    table_name: &'static str,
    attributes: &Vec<T>,
    data: &Vec<Vec<String>>,
) -> Result<(), sqlite::Error> {
    let values_string: String = data.iter().map(|row: &Vec<String>| {
        let formatted: Vec<String> = row.iter().map(|e| {
            if e.trim().parse::<f64>().is_ok() { e.clone() }
            else {
                format!("\"{}\"", e) // Add quotations for Stringy data 
            }
        }).collect();
        format!("({}),", formatted.join(","))
    }).collect::<Vec<String>>().join("\n");
    let query = format!("INSERT INTO {}({}) VALUES {}", table_name, attributes.iter().copied().join(","), values_string);
    println!("QUERY: {}", query);
    connection.execute(query)
}

pub fn csv_to_sqlite() {
    let mut csv_to_db_attribute_map: AttributeMap = HashMap::new();
    csv_to_db_attribute_map.extend([
        ("Name", "name"),
        ("Jersey Number", "jersey_number"),
        ("Club", "club"),
        ("Position", "position"),
        ("Nationality", "nationality"),
        ("Age", "age"),
        ("Appearances", "appearances"),
        ("Wins", "wins"),
        ("Losses", "losses"),
        ("Goals", "goals"),
        ("Goals per match", "goals_per_match"),
        ("Headed goals", "headed_goals"),
        ("Goals with right foot", "goals_left_foot"),
        ("Goals with left foot", "goals_right_foot"),
        ("Penalties scored", "goals_from_penalties"),
        ("Freekicks scored", "goals_from_freekicks"),
        ("Shots", "shots"),
        ("Shots on target", "shots_on_target"),
        ("Shooting accuracy %", "shooting_accuracy_pct"),
        ("Hit woodwork", "hit_woodwork"),
        ("Clean sheets", "clean_sheets"),
        ("Goals conceded", "goals_conceded"),
        ("Tackles", "tackles"),
        ("Tackle success %", "tackle_success_pct"),
        ("Blocked shots", "shots_blocked"),
        ("Interceptions", "interceptions"),
        ("Clearances", "clearances"),
        ("Headed Clearance", "headed_clearances"),
        ("Own goals", "own_goals"),
        ("Assists", "assists"),
        ("Passes", "passes"),
        ("Passes per match", "passes_per_match"),
        ("Crosses", "crosses"),
        ("Cross accuracy %", "cross_accuracy_pct"),
        ("Saves", "saves"),
        ("Penalties saved", "penalties_saved"),
        ("Punches", "punches"),
        ("High Claims", "high_claims"),
        ("Catches", "catches"),
        ("Throw outs", "throw_outs"),
        ("Goal Kicks", "goal_kicks"),
        ("Yellow cards", "cards_yellow"),
        ("Red cards", "cards_red"),
        ("Fouls", "fouls"),
        ("Offsides", "offsides"),
    ]);
    println!("Retrieving data from csv...");
    let (header, data) = parse_csv();
    println!("HEADER: {:#?}\nDATA HEAD: {:#?}", header, data[0]);
    let _path = Path::new("soccer.db");
    println!("Deleting old db file if exists...");
    match std::fs::remove_file(_path) {
        Ok(_) => {
            println!("Successfully deleted file: {}", _path.display())
        }
        Err(_) => (),
    }
    let connection = sqlite::open("soccer.db").unwrap();
    println!("Creating database tables...");
    CREATE_TABLE_QUERIES.iter().for_each(|e| connection.execute(e).unwrap());
    println!("Inserting data from csv into db tables...");
    let attributes: Vec<&&str> = header.iter().map(|a| csv_to_db_attribute_map.get(a.as_str()).unwrap()).collect();
    insert_all_into(&connection, "player", &attributes, &data).unwrap();
}
