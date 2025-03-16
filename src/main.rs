use std::{env, process};

use drgrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("error during the execuion {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("An error occurred {}", e);
        process::exit(1);
    };
}

