use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::iter::Iterator;
use std::path::Path;
use std::path::PathBuf;

use glob::glob;
use yaml_rust::{Yaml, YamlLoader};

use gtmpl::Value;

pub fn merge(values: Value, update: Value) -> Value {
    if let Value::Map(mut left) = values {
        match update {
            Value::Map(right) => {
                for (k, v) in right {
                    if let Some((left_k, left_v)) = left.remove_entry(&k) {
                        left.insert(left_k, merge(left_v, v));
                    } else {
                        left.insert(k, v);
                    }
                }
            }
            _ => {
                return update.clone();
            }
        };
        return Value::Map(left);
    }
    update.clone()
}

pub fn load_strvals(strings: &str) -> Result<Value, String> {
    let values: Result<HashMap<String, Value>, String> =strings.split(',').map(|s| {
        let kv: Vec<&str> = s.splitn(2, '=').collect();
        match kv.len() {
            1 => Ok((String::from(kv[0]), Value::from(""))),
            2 => Ok((String::from(kv[0]), Value::from(kv[1]))),
            _ => Err(String::from("please use key1=value1,key2=value2"))
        }
    }).collect();
    values.map(|v| Value::from(v))
}

fn yaml_to_gtmpl_value(yaml: Yaml) -> Value {
    match yaml {
        Yaml::String(s) => Value::from(s),
        Yaml::Boolean(b) => Value::from(b),
        Yaml::Integer(i) => Value::from(i),
        Yaml::Array(a) => Value::Array(a.into_iter().map(yaml_to_gtmpl_value).collect()),
        Yaml::Hash(mut h) => Value::Map(
            h.entries()
                .filter_map(|e| match (e.key().clone().into_string(), e.remove()) {
                    (Some(k), v) => Some((k, yaml_to_gtmpl_value(v))),
                    _ => None,
                }).collect(),
        ),
        Yaml::Real(_) => yaml
            .as_f64()
            .map(Value::from)
            .unwrap_or_else(|| Value::NoValue),
        _ => Value::Nil,
    }
}

fn load_yaml(file_name: &str) -> Result<Vec<Yaml>, String> {
    let mut file = File::open(file_name).map_err(|e| format!("{}", e))?;
    let mut yml_str = String::new();
    file.read_to_string(&mut yml_str)
        .map_err(|e| format!("{}", e))?;
    YamlLoader::load_from_str(&yml_str).map_err(|e| format!("{}", e))
}

pub fn load_values_yaml(values: &[String]) -> Result<Value, String> {
    let mut v = vec![];
    for file_name in values {
        v.append(&mut load_yaml(file_name)?);
    }
    Ok(v.into_iter()
        .map(yaml_to_gtmpl_value)
        .fold(Value::Map(HashMap::new()), merge))
}

pub fn load_chart(dir_name: &str) -> Result<Vec<PathBuf>, String> {
    let chart_path = Path::new(dir_name);
    let glob_path = format!("{}/templates/*.yaml", chart_path.to_string_lossy());
    glob(&glob_path)
        .map_err(|e| format!("{}", e))?
        .map(|p| p.map_err(|e| format!("{}", e)))
        .collect::<Result<Vec<PathBuf>, String>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flat_merge() {
        let a = Value::from(hashmap! {
            "a".to_owned() => 1,
            "b".to_owned() => 2,
        });
        let b = Value::from(hashmap! {
            "b".to_owned() => 3,
            "c".to_owned() => 4,
        });
        let c = merge(a, b);
        if let Value::Map(c) = c {
            assert_eq!(c.get("a"), Some(&Value::from(1)));
            assert_eq!(c.get("b"), Some(&Value::from(3)));
            assert_eq!(c.get("c"), Some(&Value::from(4)));
        } else {
            assert!(false);
        }
    }
    #[test]
    fn test_override_map_merge() {
        let a = Value::from(hashmap! {
            "a".to_owned() => hashmap! { "b".to_owned() => 2 },
        });
        let b = Value::from(hashmap! {
            "a".to_owned() => 3,
        });
        let c = merge(a, b);
        if let Value::Map(c) = c {
            assert_eq!(c.get("a"), Some(&Value::from(3)));
        } else {
            assert!(false);
        }
    }
}
