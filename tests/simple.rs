extern crate qdht;

use qdht::app;

mod common;

#[test]
fn test_simple_template() {
    let cmd = "qhdt -f tests/charts/simple/values.yaml tests/charts/simple";
    let out = app::run(cmd.split(' ')).unwrap();
    assert_eq!(out.len(), 1);
    let should = common::string_from_file("tests/charts/simple/simple.yaml.expected").unwrap();
    assert_eq!(out[0], should);
}
