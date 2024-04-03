#[cfg(test)]
use crate::*;

#[test]
fn test_endpoints(){
    let auth_endpoints = requests::AUTHORITATIVE_ENDPOINTS();
    println!("Authoritative Endpoints: {:#?}", auth_endpoints);
}

#[test]
fn test_database() {
    println!("{:?}", database::csv_to_sqlite());
}

#[test]
fn test_macros() {
    let mut stft = macros::StatefulFormat::new("{} doggy {} bigdawg", None);
    stft.assign(vec!["Yo", "Bro"]);
    println!("{}", stft);
}