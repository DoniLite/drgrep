use std::process::{exit};

use drgrep::{
    args::parser::ArgParser, run, Config, DEFAULT_MESSAGE,
};

fn main() {
    let args: &ArgParser = &Default::default();

    if args.has("version") || args.has("v") {
        println!("{}", drgrep::VERSION);
        exit(0);
    }

    let config = Config::new(args).unwrap_or_else(|_: &str| {
        println!("{}", DEFAULT_MESSAGE);
        exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("An error occurred {}", e);
        exit(1);
    };
}
