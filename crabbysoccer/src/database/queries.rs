use itertools::Itertools;

use crate::common::format_vec;

pub trait AsStr {
    fn as_str(&self) -> &'static str;
}
pub enum TablePlayerAttributes {
    ID,
    Name,
    JerseyNumber,
    ClubName,
    Nationality,
    Age,
}
impl AsStr for TablePlayerAttributes {
    fn as_str(&self) -> &'static str {
        match &self {
            TablePlayerAttributes::ID => "id",
            TablePlayerAttributes::Name => "name",
            TablePlayerAttributes::JerseyNumber => "jersey_number",
            TablePlayerAttributes::ClubName => "club_name",
            TablePlayerAttributes::Nationality => "nationality",
            TablePlayerAttributes::Age => "age",
        }
    }
}
type _TPA = TablePlayerAttributes; // alias
const _TPA_VALUES: [_TPA; 6] = [
    _TPA::ID,
    _TPA::Name,
    _TPA::JerseyNumber,
    _TPA::ClubName,
    _TPA::Nationality,
    _TPA::Age,
];

pub enum PredefinedQuery {
    CreateTablePlayer,
    CreateTableStatistics,
    CreateTablePosition,
}

pub fn get_predefined_query(query: PredefinedQuery) -> String {
    match query {
        PredefinedQuery::CreateTablePlayer => format_vec(
            "CREATE TABLE player (
            {} INTEGER PRIMARY KEY AUTO_INCREMENT, {} VARCHAR(128) NOT NULL, {} INTEGER NOT NULL, {} VARCHAR(128), {} VARCHAR(64) NOT NULL, {} INTEGER NOT NULL)",
            &_TPA_VALUES.iter().map(|e| e.as_str()).collect(),
        ),
        PredefinedQuery::CreateTableStatistics => todo!(),
        PredefinedQuery::CreateTablePosition => todo!(),
    }
}
