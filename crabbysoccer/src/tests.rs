#[cfg(test)]
use crate::*;

#[test]
fn test_endpoints(){
    println!("Endpoints: {:#?}", requests::ENDPOINTS);
    println!("{:?}", requests::ENDPOINTS[0].query_parameters);
}

#[test]
fn test_database() {
    println!("{:?}", database::csv_to_sqlite());
}
