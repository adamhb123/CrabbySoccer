use std::collections::HashMap;

// ‘/get-player?player_id={player_id}&statistics={goals, assists, etc…}’:
// ‘/get-all-players?name={name}’

pub type QueryPVMap = HashMap<String, Vec<String>>;

#[derive(std::fmt::Debug, Clone)]
pub struct Endpoint {
    pub uri: String,
    pub query_pv_map: QueryPVMap
}
impl Endpoint {
    pub fn new<T: ToString>(uri: T, query_pv_map: QueryPVMap) -> Self {
        Self { uri: uri.to_string(), query_pv_map }
    }
    fn new_authority<T: ToString>(uri: T, query_parameters: &[T]) -> Self {
        // Creates a new AUTHORITATIVE ENDPOINT:
        //      * All Endpoints created with Endpoint::new() MUST ONLY include endpoint uri's and their respective query parameters as defined
        //          by AUTHORITATIVE ENDPOINTS
        //
        // Note that for an Endpoint to be considered an AUTHORITY, it MUST exist in the ENDPOINTS const, otherwise it is just another Endpoint...
        let mut fauxmap: QueryPVMap = HashMap::new();
        for p in query_parameters { fauxmap.insert(p.to_string(), vec![]); }
        Self { uri: uri.to_string(), query_pv_map: fauxmap}
    }
    pub fn get_valued_uri(&self) -> Result<String, &str> {
        // If no query parameters have associated values, then they are useless, so return base uri
        if self.query_pv_map.values().len() == 0 {
            return Ok(format!("/{}", self.uri));
        }

        let mut formatted = format!("/{}?", &self.uri);
        for (p, vset) in self.query_pv_map.iter() {
            if !vset.is_empty() {
                let vals = vset.join(",");
                formatted.push_str(format!("{p}={vals}&").as_str());
            }
        }
        formatted.pop().unwrap();
        Ok(formatted)
    }
    pub fn get_request_string(&self) -> String {
        let uri = self.get_valued_uri().unwrap();
        format!("GET {} HTTP/1.1\nUser-Agent: crabbysoccer/1.0.0\nHost: temporarily-not-included\nAccept-Language: en\n\n", uri)
    }
}
impl From<Vec<String>> for Endpoint {
    fn from(value: Vec<String>) -> Self {
        // value is expected to be an HTTP GET request
        let uri = value[0].split(" ").collect::<Vec<&str>>()[1].to_owned();
        let user_agent = value[1].split(" ").collect::<Vec<&str>>()[1].to_owned();
        println!("Parsed:\n\turi={}\n\tuser-agent={}", uri, user_agent);
        let uri_split: (&str, &str) = uri.split_once("?").unwrap();
        let query_params: Vec<&str> = uri_split.1.split("&").collect();
        let endpoint: String = String::from(&uri_split.0[1..]);
        let mut query_pv_map: QueryPVMap = HashMap::new();
        for qp_entry in query_params {
            let (p, vals) = qp_entry.split_once("=").unwrap();
            let vals: Vec<String> = vals.split(",").map(String::from).collect();
            query_pv_map.insert(p.to_owned(), vals);
        }
        Endpoint::new(endpoint, query_pv_map)
    }
}

#[allow(non_snake_case)]
pub fn AUTHORITATIVE_ENDPOINTS() -> [Endpoint; 2] {
    [
        Endpoint::new_authority("get-player", &["player_id", "statistics"]),
        Endpoint::new_authority("get-all-players", &["name"])
    ]
}

pub fn clone_authoritative_endpoint_by_uri(uri: &str) -> Option<Endpoint> {
    let ep = AUTHORITATIVE_ENDPOINTS().into_iter().filter(|e| e.uri == uri).collect::<Vec<_>>();
    if ep.len() > 1 { panic!("Duplicate ENDPOINTS entry: {}", uri); }
    else if ep.len() == 0 { None }
    else { Some(ep[0].clone()) }
}
