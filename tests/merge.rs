extern crate gtmpl;
extern crate qdht;

use gtmpl::Value;
use qdht::loader;

#[test]
fn test_flat_merge() {
    let merged = loader::load_values_yaml(&vec![
        "tests/values/values1.yaml".to_owned(),
        "tests/values/values2.yaml".to_owned(),
    ]).unwrap();
    if let Value::Map(merged) = merged {
        assert_eq!(merged.get("a"), Some(&Value::from("hello")));
        assert_eq!(merged.get("b"), Some(&Value::from("doom")));
        assert_eq!(merged.get("c"), Some(&Value::from("something")));
    } else {
        assert!(false);
    }
}
