use std::env;
use std::fs;
use std::process;

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

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
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