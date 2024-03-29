use std::collections::HashMap;
use phf;
use reqwest;
use std::cmp;
// ‘/get-player?player_id={player_id}&statistics={goals, assists, etc…}’:
// ‘/get-all-players?name={name}’

#[derive(std::fmt::Debug)]
pub struct Endpoint {
    pub uri: &'static str,
    pub query_parameters: &'static [&'static str]
}
impl Endpoint {
    const fn new(uri: &'static str, query_parameters: &'static [&'static str]) -> Self {
        Self { uri, query_parameters }
    }
    pub fn get_valued_uri(&self, query_parameter_values: &'static [&'static str]) -> Result<String, &str> {
        match query_parameter_values.len().cmp(&self.query_parameters.len()) {
            cmp::Ordering::Less => Err("Too few query parameter values provided!"),
            cmp::Ordering::Greater => Err("Too many query parameter values provided!"),
            cmp::Ordering::Equal => Ok(format!("/{}?",&self.uri))
        }
    }
}

pub const ENDPOINTS: [Endpoint; 2] = [
    Endpoint::new("get-player", &["player_id", "statistics"]),
    Endpoint::new("get-all-players", &["name"])
];
