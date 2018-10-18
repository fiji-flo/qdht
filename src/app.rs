use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches};
use gtmpl::{Context, Value};

use loader::*;
use template::template_file;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_args<'a, I, T>(itr: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    App::new("qdht")
        .about("template like helm - just quick and dirty")
        .version(VERSION)
        .author("Florian Merz <flomerz@gmail.com>")
        .arg(
            Arg::with_name("values")
                .short("f")
                .long("values")
                .takes_value(true)
                .number_of_values(1)
                .multiple(true)
                .help("yaml value files for template"),
        ).arg(
            Arg::with_name("overrides")
                .long("set")
                .takes_value(true)
                .number_of_values(1)
                .multiple(true)
                .help("set values from command line"),
        ).arg(
            Arg::with_name("execute")
                .short("x")
                .long("execute")
                .takes_value(true)
                .number_of_values(1)
                .multiple(true)
                .help("only template specified files"),
        ).arg(
            Arg::with_name("chart")
                .required(true)
                .required_unless("execute"),
        ).get_matches_from(itr)
}

pub fn run<I, T>(itr: I) -> Result<Vec<String>, String>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = parse_args(itr);
    let values = load_values_yaml(
        &matches
            .values_of("values")
            .unwrap_or_default()
            .map(String::from)
            .collect::<Vec<String>>(),
    )?;
    let overrides = load_strvals(&matches.value_of("overrides").unwrap_or_default())?;
    let updates = merge(values, overrides);
    let mut values = HashMap::new();
    values.insert(String::from("Values"), updates);
    let values = Value::Map(values);
    let context = Context::from(values)?;
    let templates = if let Some(template_files) = matches.values_of("execute") {
        template_files.map(PathBuf::from).collect()
    } else if let Some(chart) = matches.value_of("chart") {
        load_chart(chart).unwrap_or_default()
    } else {
        vec![]
    };
    let out = templates
        .iter()
        .map(|t| template_file(t, &context))
        .collect::<Result<Vec<String>, String>>()
        .map_err(|e| e.to_string())?;
    Ok(out)
}
