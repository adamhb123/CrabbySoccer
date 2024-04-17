use rusqlite::{self, types::ValueRef, Connection, Row, Statement};
use std::{collections::HashMap, fmt::Display, fs::read_to_string, path::Path};
use strum::EnumIter;

trait TableNameTrait {
    fn as_str(&self) -> &str;
}
#[derive(PartialEq, Eq, EnumIter)]
enum TableName {
    Player,
    Statistics,
    Position,
}
impl TableNameTrait for TableName {
    fn as_str(&self) -> &str {
        match &self {
            TableName::Player => "player",
            TableName::Statistics => "statistics",
            TableName::Position => "position",
        }
    }
}
impl Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_str())
    }
}
const CREATE_TABLE_QUERIES: [&'static str; 3] = [
    // player
    "CREATE TABLE player (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(128) NOT NULL,
        jersey_number INTEGER NOT NULL,
        club_name VARCHAR(128),
        nationality VARCHAR(64) NOT NULL,
        age INTEGER NOT NULL);",
    // statistics
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
        offsides INTEGER NOT NULL,
        FOREIGN KEY (player_id) REFERENCES player(id)
    );",
    // position
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
    pub connection: Connection,
}
impl DB {
    pub fn new() -> Self {
        Self {
            connection: Connection::open("soccer.db").unwrap(),
        }
    }
    fn parsed_rows_to_string<T: ToString, U: ToString>(column_names: &Vec<T>, values: &Vec<Vec<U>>) -> String {
        let column_names = column_names
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("\t|\t");
        let values = values
            .iter()
            .map(|e| e.iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\t|\t"))
            .collect::<Vec<String>>()
            .join("\n");
        format!("{}\n{}", column_names, values)
    }

    fn row_to_vec_string(n_columns: &usize, row: &Row) -> Vec<String> {
        (0..*n_columns)
            .map(|i: usize| match row.get_ref_unwrap(i) {
                ValueRef::Null => "".to_owned(),
                ValueRef::Integer(v) => v.to_string(),
                ValueRef::Real(v) => v.to_string(),
                ValueRef::Text(v) | ValueRef::Blob(v) => String::from_utf8(v.to_vec()).unwrap(),
            })
            .collect::<Vec<String>>()
    }

    fn rows_as_2d_vec_string(statement: &mut Statement) -> Vec<Vec<String>>{
        let n_columns = statement.column_count();
        statement.query_map([], |row| Ok(DB::row_to_vec_string(&n_columns, row))).unwrap().map(Result::unwrap).collect()
    }

    fn rows_to_string(statement: &mut Statement) -> String {
        let rows: Vec<Vec<String>> = DB::rows_as_2d_vec_string(statement);
        let col_names: Vec<&str> = statement.column_names();
        DB::parsed_rows_to_string(&col_names, &rows)
    }

    pub fn get_all_players(&self, name: Option<String>) -> Result<String, rusqlite::Error> {
        // -> Result<String, rusqlite::Error> {
        let mut statement: Statement;
        if let Some(_name) = name {
            let sql = format!("SELECT * FROM player WHERE player.name LIKE \"%{}%\"", _name);
            statement = self.connection.prepare(&sql).unwrap();
        } else {
            statement = self.connection.prepare("SELECT * FROM player").unwrap();
        }
        Ok(DB::rows_to_string(&mut statement))
    }

    pub fn get_player(
        &self,
        player_id: Option<String>,
        statistics: Option<Vec<String>>,
    ) -> Result<String, rusqlite::Error> {
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
                    "".into()
                }
            }
            None => "".into(),
        };

        let where_pid_clause = match player_id {
            Some(id) => format!("WHERE player.id = {id}"),
            None => "".to_owned(),
        };
        let sql =
            format!("SELECT player.id, player.name, position.name as position {statistics_string} FROM {JOIN_ALL} {where_pid_clause};");
        println!("Querying DB: {}", sql);
        let mut statement = self.connection.prepare(&sql).unwrap();
        Ok(DB::rows_to_string(&mut statement))
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(|e| e.trim().into())
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

fn insert_all_into(
    connection: &Connection,
    table_name: TableName,
    attributes: &Vec<&(TableName, &str)>,
    data: &Vec<Vec<String>>,
) -> Result<(), rusqlite::Error> {
    // Get data_indices and respective attributes for table_name
    let (data_indices, attributes): (Vec<usize>, Vec<&(TableName, &str)>) =
        attributes.iter().enumerate().filter(|(_, e)| e.0 == table_name).unzip();
    println!("INDICES: {:?}", data_indices);
    // Map attributes (TableName, attribute_name pair) to just the attribute name (i.e., the column name)
    let mut attributes: Vec<&str> = attributes.iter().map(|e| e.1).collect();
    // Retrieve appropriate data relevant to the table using data_indices
    let mut data: Vec<Vec<String>> = data
        .iter()
        .map(|r| {
            r.iter()
                .enumerate()
                .filter(|(i, _)| data_indices.contains(i))
                .map(|e| e.1.to_owned())
                .collect()
        })
        .collect();
    println!("DATA[0]: {:?}", data[0]);

    // Handle foreign key inserts
    let player_ids = (1..=data.len()).map(|e| e.to_string()).collect::<Vec<String>>();
    match table_name {
        TableName::Player => (),
        TableName::Statistics | TableName::Position => {
            attributes.push("player_id");
            data.iter_mut()
                .enumerate()
                .for_each(|(i, e)| e.push(player_ids[i].to_owned()))
        }
    }

    let values_string: String = data // Parse VALUES entries; Prepare for SQL statement
        .iter()
        .map(|row| {
            let formatted: Vec<String> = row // Format each value in data row
                .iter()
                .map(|e| {
                    let e = e.trim().to_owned();
                    if e.is_empty() {
                        "0".to_owned()
                    } else {
                        if e.parse::<f64>().is_ok() {
                            e.clone()
                        } else {
                            if e.ends_with("%") {
                                // Percentage into decimal - does not enforce the total number of digits denoted by n in SQL's Decimal(n, p),
                                // but does enforce p (# of digits after decimal)
                                format!("{:.2}", e[..e.len() - 1].parse::<f64>().unwrap() / 100.0)
                            } else {
                                // Stringy data
                                format!("\"{}\"", e) // Add quotations for Stringy data
                            }
                        }
                    }
                })
                .collect();
            format!("({})", formatted.join(",")) // Join each value in row with comma
        })
        .collect::<Vec<String>>()
        .join(",\n"); // Join each VALUES entry with comma-separator and newline

    // Prepare final statement for execution
    let statement = format!(
        "INSERT INTO {}({}) VALUES {};",
        table_name,
        attributes.join(","),
        values_string
    );
    println!("statement: {}", statement);
    connection.execute(&statement, ()).unwrap();
    Ok(())
}

pub fn csv_to_sqlite<'a>() {
    let mut csv_to_db_attribute_map: HashMap<&'a str, (TableName, &'static str)> = HashMap::new();
    csv_to_db_attribute_map.extend([
        ("Name", (TableName::Player, "name")),
        ("Jersey Number", (TableName::Player, "jersey_number")),
        ("Club", (TableName::Player, "club_name")),
        ("Position", (TableName::Position, "name")),
        ("Nationality", (TableName::Player, "nationality")),
        ("Age", (TableName::Player, "age")),
        ("Appearances", (TableName::Statistics, "appearances")),
        ("Wins", (TableName::Statistics, "wins")),
        ("Losses", (TableName::Statistics, "losses")),
        ("Goals", (TableName::Statistics, "goals")),
        ("Goals per match", (TableName::Statistics, "goals_per_match")),
        ("Headed goals", (TableName::Statistics, "headed_goals")),
        ("Goals with right foot", (TableName::Statistics, "goals_left_foot")),
        ("Goals with left foot", (TableName::Statistics, "goals_right_foot")),
        ("Penalties scored", (TableName::Statistics, "goals_from_penalties")),
        ("Freekicks scored", (TableName::Statistics, "goals_from_freekicks")),
        ("Shots", (TableName::Statistics, "shots")),
        ("Shots on target", (TableName::Statistics, "shots_on_target")),
        ("Shooting accuracy %", (TableName::Statistics, "shooting_accuracy_pct")),
        ("Hit woodwork", (TableName::Statistics, "hit_woodwork")),
        ("Clean sheets", (TableName::Statistics, "clean_sheets")),
        ("Goals conceded", (TableName::Statistics, "goals_conceded")),
        ("Tackles", (TableName::Statistics, "tackles")),
        ("Tackle success %", (TableName::Statistics, "tackle_success_pct")),
        ("Blocked shots", (TableName::Statistics, "shots_blocked")),
        ("Interceptions", (TableName::Statistics, "interceptions")),
        ("Clearances", (TableName::Statistics, "clearances")),
        ("Headed Clearance", (TableName::Statistics, "headed_clearances")),
        ("Own goals", (TableName::Statistics, "own_goals")),
        ("Assists", (TableName::Statistics, "assists")),
        ("Passes", (TableName::Statistics, "passes")),
        ("Passes per match", (TableName::Statistics, "passes_per_match")),
        ("Crosses", (TableName::Statistics, "crosses")),
        ("Cross accuracy %", (TableName::Statistics, "cross_accuracy_pct")),
        ("Saves", (TableName::Statistics, "saves")),
        ("Penalties saved", (TableName::Statistics, "penalties_saved")),
        ("Punches", (TableName::Statistics, "punches")),
        ("High Claims", (TableName::Statistics, "high_claims")),
        ("Catches", (TableName::Statistics, "catches")),
        ("Throw outs", (TableName::Statistics, "throw_outs")),
        ("Goal Kicks", (TableName::Statistics, "goal_kicks")),
        ("Yellow cards", (TableName::Statistics, "cards_yellow")),
        ("Red cards", (TableName::Statistics, "cards_red")),
        ("Fouls", (TableName::Statistics, "fouls")),
        ("Offsides", (TableName::Statistics, "offsides")),
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
    let connection = Connection::open("soccer.db").unwrap();
    println!("Creating database tables...");
    CREATE_TABLE_QUERIES.iter().for_each(|e| {
        connection.execute(e, ()).unwrap();
    });
    println!("Inserting data from csv into db tables...");
    let attributes: Vec<&(TableName, &str)> = header
        .iter()
        .map(|a| csv_to_db_attribute_map.get(a.as_str()).unwrap())
        .collect();

    // TODO: Need to add foreign key constraints:
    // statistics.player_id -> player.id
    // position.player_id -> player.id
    // In order to do this, perhaps add a "universal_data" Option arg to insert_all_into that applies regardless of table_name restriction
    insert_all_into(&connection, TableName::Player, &attributes, &data).unwrap();
    insert_all_into(&connection, TableName::Statistics, &attributes, &data).unwrap();
    insert_all_into(&connection, TableName::Position, &attributes, &data).unwrap();
}
