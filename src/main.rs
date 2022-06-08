use std::env;
use std::fs;

fn main() {
    // collect arguments in stdin as vector
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("In file: {}", filename);

    // read the file
    let contents = fs::read_to_string(filename).expect("Could not read the file!");

    println!("With text: {}\n", contents);
}

