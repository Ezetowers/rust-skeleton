use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// This function will return a type that implements the Error trait, 
// but we don’t have to specify what particular type the return value will be
// That's the meaning of dyn, the error will be known in deploy time (polymorphism)
// I don't know what the Box thing does yet U_U
//
// Regarding the return value of the Ok part of the result, we are returning () which
// is the void value of Rust. It is the value that would be used if no result would
// be used here. This is the correct way in Rust to return nothing
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}