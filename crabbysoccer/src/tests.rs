#[cfg(test)]
use crate::*;

#[test]
fn test(){
    println!("Endpoints: {:#?}", requests::ENDPOINTS);
    println!("{:?}", requests::ENDPOINTS[0].get_query);
}
