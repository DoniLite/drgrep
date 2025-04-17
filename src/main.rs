use std::process;

use drgrep::{
    args::parser::ArgParser, run, Config, DEFAULT_MESSAGE,
};

fn main() {
    let args: &ArgParser = &Default::default();

    let config = Config::new(args).unwrap_or_else(|_| {
        println!("{}", DEFAULT_MESSAGE);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("An error occurred {}", e);
        process::exit(1);
    };
}
