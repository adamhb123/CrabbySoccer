use std::fmt::Arguments;

pub trait QueryEnum {
    fn get_values() -> Vec<&'static str>;
    fn as_str(&self) -> &'static str;
    fn 
}

pub enum TablePlayerAttributes {
    ID,
    Name,
    JerseyNumber,
    ClubName
}
type _TPA = TablePlayerAttributes; // alias

impl QueryEnum for TablePlayerAttributes {
    fn get_values() -> Vec<&'static str> {
        vec![_TPA::ID, _TPA::Name, _TPA::JerseyNumber, _TPA::ClubName].iter().map(_TPA::as_str).collect()
    }
    fn as_str(&self) -> &'static str {
        match &self {
            _TPA::ID => "id",
            _TPA::Name => "name",
            _TPA::JerseyNumber => "jersey_number",
            _TPA::ClubName => "club_name",
        }
    }
}

pub const CREATE_TABLE_PLAYER: String = format!("CREATE TABLE player (
{} INTEGER PRIMARY KEY AUTO_INCREMENT, {} VARCHAR(128) NOT NULL, {} INTEGER NOT NULL, {})", _TPA::get_values());