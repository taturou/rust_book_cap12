use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });

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

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
}
