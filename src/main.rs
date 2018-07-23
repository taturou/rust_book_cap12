use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Searching for \"{}\" in file \"{}\"", config.query, config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    let mut contentes = String::new();
    f.read_to_string(&mut contentes).expect("something went wrong reading the file");

    println!("With the text:\n{}", contentes);
}

struct Config {
  query: String,
  filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config {query, filename}
}
