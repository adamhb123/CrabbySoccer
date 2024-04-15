#[cfg(test)]
use crate::*;

#[test]
fn test_endpoints() {
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
    /* common::format_vec
       The following use cases should be covered:
            1. Plain placeholders only:
                1. "{}"
                2. "a{}"
                3. "{}a"
                4. "{}{}"
                5. "a{}{}"
                6. "{}a{}"
                7. "{}{}a"
                8. "a{}a{}a"
    */
    // 1. Plain placeholders only
    assert_eq!(common::format_vec("{}", &vec![0]), "0");
    assert_eq!(common::format_vec("a{}", &vec![0]), "a0");
    assert_eq!(common::format_vec("{}a", &vec![0]), "0a");
    assert_eq!(common::format_vec("{}{}", &vec![0, 1]), "01");
    assert_eq!(common::format_vec("a{}{}", &vec![0, 1]), "a01");
    assert_eq!(common::format_vec("{}a{}", &vec![0, 1]), "0a1");
    assert_eq!(common::format_vec("{}{}a", &vec![0, 1]), "01a");
    assert_eq!(common::format_vec("a{}a{}", &vec![0, 1]), "a0a1");
    assert_eq!(common::format_vec("{}a{}a", &vec![0, 1]), "0a1a");
    assert_eq!(common::format_vec("a{}a{}a", &vec![0, 1]), "a0a1a");
    // Index-based placeholders only
    assert_eq!(common::format_vec("{0}", &vec![0]), "0");
    assert_eq!(common::format_vec("a{0}", &vec![0]), "a0");
    assert_eq!(common::format_vec("{0}a", &vec![0]), "0a");
    assert_eq!(common::format_vec("{0}{1}", &vec![0, 1]), "01");
    assert_eq!(common::format_vec("a{0}{1}", &vec![0, 1]), "a01");
    assert_eq!(common::format_vec("{0}a{1}", &vec![0, 1]), "0a1");
    assert_eq!(common::format_vec("{0}{1}a", &vec![0, 1]), "01a");
    assert_eq!(common::format_vec("a{0}a{1}", &vec![0, 1]), "a0a1");
    assert_eq!(common::format_vec("{0}a{1}a", &vec![0, 1]), "0a1a");
    assert_eq!(common::format_vec("a{0}a{1}a", &vec![0, 1]), "a0a1a");
    /* Incorrect usage */
}
