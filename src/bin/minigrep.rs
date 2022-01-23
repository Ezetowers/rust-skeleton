use std::env;
use std::process;
use rust_skeleton::Config;

fn main() {
    // Error managed with closures (not seeing yet). Closures are rust
    // anonymous functions. unwrap_or_else return the ok value from the
    // Result<Config, &str> (in this case, Config) if no error occurs,
    // and calls a function (the closure) if an error occurs
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
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

    // Since the run function returns () in the Ok value of the Result,
    // there is nothing to unwrap and we can check the error directly
    if let Err(e) = rust_skeleton::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}