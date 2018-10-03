extern crate gtmpl_value;
extern crate sprig;

use gtmpl::{Context, Func, Template};
use sprig::SPRIG;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn template_file(file: &PathBuf, ctx: &Context) -> Result<String, String> {
    let mut f = File::open(file).map_err(|e| format!("file not found: {}", e))?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .map_err(|e| format!("unable to read file: {}", e))?;
    let tmpl = create_template(&contents)?;
    tmpl.render(ctx)
}

fn create_template(string: &str) -> Result<Template, String> {
    let mut tmpl = Template::with_name("");
    tmpl.add_funcs(SPRIG as &[(&str, Func)]);
    tmpl.parse(string)?;
    Ok(tmpl)
}
