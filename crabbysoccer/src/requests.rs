use std::{collections::HashMap, io::Read};
use phf;
use reqwest;
use std::cmp;
// ‘/get-player?player_id={player_id}&statistics={goals, assists, etc…}’:
// ‘/get-all-players?name={name}’

pub type QueryPVMap<'a> = HashMap<&'a str, Vec<&'a str>>;

#[derive(std::fmt::Debug, Clone, Copy)]
pub struct Endpoint<'a> {
    pub uri: &'a str,
    pub query_parameters: &'a [&'a str]
}
impl <'a>Endpoint<'a> {
    const fn new(uri: &'a str, query_parameters: &'a [&'a str]) -> Self {
        Self { uri, query_parameters }
    }
    pub fn get_valued_uri(&self, query_pv_map: &QueryPVMap) -> Result<String, &str> {
        let mut formatted = format!("/{}?", &self.uri);
        for (p, vset) in query_pv_map {
            let vals = vset.join(",");
            formatted.push_str(format!("{p}={vals}&").as_str())
        }
        formatted.pop().unwrap();
        Ok(formatted)
    }
    pub fn as_bytes(&self, query_pv_map: &QueryPVMap) -> Vec<u8> {
        self.get_valued_uri(query_pv_map).unwrap().as_bytes().to_owned()
    }
    pub fn get_request_string(&self, query_pv_map: &QueryPVMap) -> String {
        let uri = self.get_valued_uri(&query_pv_map).unwrap();
        format!("GET {} HTTP/1.1\nUser-Agent: crabbysoccer/1.0.0\nHost: temporarily-not-included\nAccept-Language: en\n\n", uri)
    }
}
impl From<Vec<String>> for Endpoint<'_> {
    fn from(value: Vec<String>) -> Self {
        for 
    }
}

pub const ENDPOINTS: [Endpoint; 2] = [
    Endpoint::new("get-player", &["player_id", "statistics"]),
    Endpoint::new("get-all-players", &["name"])
];

pub fn find_endpoint_by_uri(uri: &str) -> Option<Endpoint> {
    let ep = ENDPOINTS.into_iter().filter(|e| e.uri == uri).collect::<Vec<_>>();
    if ep.len() > 1 { panic!("Duplicate ENDPOINTS entry: {}", uri); }
    else if ep.len() == 0 { None }
    else { Some(ep[0]) }
}

pub fn endpoint_uri_exists(uri: &str) -> bool {
    ENDPOINTS.map(|e| e.uri).contains(&uri)
}
