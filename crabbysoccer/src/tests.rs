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
    /* common::format_vec */
    println!("FUFUFUFU {}", common::format_vec("Plain placeholders: One: {} Two: {} Three: {} and that's it!", &vec!["1", "2", "3"]));
    assert!(common::format_vec("Plain placeholders: One: {} Two: {} Three: {} and that's it!", &vec!["1", "2", "3"]) == "Plain placeholders: One: 1 Two: 2 Three: 3 and that's it!");
    println!("FUFUFUFU {}", common::format_vec("Index-based placeholders: One: {0} Two: {1} Three: {2} and that's it!", &vec!["1", "2", "3"]));
    //assert!(common::format_vec("Index-based placeholders: One: {0} Two: {1} Three: {2} and that's it!", &vec!["1", "2", "3"]) == "Index-based placeholders: One: 1 Two: 2 Three: 3 and that's it!");
}