use std::env;
use std::process;

use grep_in_rust::{run, Config};

fn main() {
    // collect arguments in stdin as vector
    let args: Vec<String> = env::args().collect();

    // try to create a new Config with args;
    // if err print err;
    // else unwrap from Result<Config, ...>
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    // explicitly checking for error as there is no need to unwrap.
    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}



