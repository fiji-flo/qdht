use std::fs::File;
use std::io::prelude::*;

pub fn string_from_file(file: &str) -> Result<String, String> {
    let mut f = File::open(file).map_err(|e| format!("file not found: {}", e))?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .map_err(|e| format!("unable to read file: {}", e))?;
    Ok(contents)
}
