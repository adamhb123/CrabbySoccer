#[cfg(test)]
use crate::*;

#[test]
fn test_endpoints(){
    let auth_endpoints = requests::AUTHORITATIVE_ENDPOINTS();
    println!("Authoritative Endpoints: {:#?}", auth_endpoints);
}

#[test]
fn test_database() {
    // database::csv_to_sqlite
    println!("{:?}", database::csv_to_sqlite());
}

#[test]
fn test_common() {
    // common::format_vec
    assert!(common::format_vec("One: {} Two: {} Three: {} and that's it!", &vec!["1", "2", "3"]) == "One: 1 Two: 2 Three: 3 and that's it!");
}