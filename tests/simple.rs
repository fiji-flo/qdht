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

#[test]
fn test_simple_override_template() {
    let cmd = "qhdt -f tests/charts/simple/values.yaml --set docker_hub_name=simple-company-override,port=8080 tests/charts/simple";
    let out = app::run(cmd.split(' ')).unwrap();
    assert_eq!(out.len(), 1);
    let should =
        common::string_from_file("tests/charts/simple/simple-override.yaml.expected").unwrap();
    assert_eq!(out[0], should);
}
