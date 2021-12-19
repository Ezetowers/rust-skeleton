use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Error managed with closures (not seeing yet). Closures are rust
    // anonymous functions. unwrap_or_else return the ok value from the
    // Result<Config, &str> (in this case, Config) if no error occurs,
    // and calls a function (the closure) if an error occurs
    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    // Error managed with common matches
    // let cfg = Config::new(&args);
    // let config = match cfg {
    //     Ok(some_config) => some_config,
    //     Err(msg) => {
    //         println!("Problem parsing arguments: {}", msg);
    //         process::exit(1);
    //     },
    // };

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Since the run function returns () in the Ok value of the Result,
    // there is nothing to unwrap and we can check the error directly
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
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
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}