use reqwest;
// ‘/get-player?player_id={player_id}&statistics={goals, assists, etc…}’:
// ‘/get-all-players?name={name}’


struct Endpoint<'a> {
    uri: &'a str,
    query_parameters: [&'a str]
}

/*async fn send_request(ip: &str, port: u8) -> Result<String, _>{
    let body: String = reqwest::get(format!("{}:{}",ip, port)).await?
    .text().await?;
    return Ok("");
}*/