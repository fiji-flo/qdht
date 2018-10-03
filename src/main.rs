extern crate qdht;

use std::env;

use qdht::app;

fn main() -> Result<(), String> {
    for o in app::run(env::args_os())? {
        println!("{}", o);
    }
    Ok(())
}
