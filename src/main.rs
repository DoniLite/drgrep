use std::process::exit;

use drgrep::{args::parser::ArgParser, run, Config, DEFAULT_MESSAGE};

fn main() {
    let args: &mut ArgParser = &mut Default::default();

    if args.has("version") || args.has("v") {
        println!("{}", drgrep::VERSION);
        exit(0);
    }

    if args.has("help") || args.has("h") {
        println!("{}", drgrep::DEFAULT_MESSAGE);
        exit(0);
    }

    // Catching the stdin if the user is using pipe
    if args.has("content") {
        match args.get("content") {
            Some(content) => {
                if content.as_str() == "@" {
                    if let Ok(stdin_content) = drgrep::read_stdin() {
                        args.set("content", stdin_content);
                    }
                }
            }
            None => (),
        }
    }
    if args.has("c") {
        match args.get("c") {
            Some(content) => {
                // println!("content key {}", content);
                if content.as_str() == "@" {
                    if let Ok(stdin_content) = drgrep::read_stdin() {
                        // println!("stdin content {}", stdin_content);
                        args.set("c", stdin_content);
                    }
                }
            }
            None => (),
        }
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
