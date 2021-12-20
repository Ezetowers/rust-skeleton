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
    Ok(())
}

// Take a look at the lifetime of this function. The return value is a 
// vector of strings, and that vector can be filled up with both
// the query or the contests input if we do not define anything. The
// explicit lifetime definition here is telling to the compiler that 
// the return values of the function will be values found in the 
// contents variable and no the query variable
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/******************************** Tests **************************************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}