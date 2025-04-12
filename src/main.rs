use std::process;

use drgrep::{args::parser::ArgParser, print_colored, print_partial_colored, print_styled, run, Color, Config};

fn main() {

    let args = &ArgParser::new();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("error during the execuion {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("An error occurred {}", e);
        process::exit(1);
    };

    print_colored("Hello in red!", Color::RED);
    print_colored("Bright blue text", Color::BRIGHT_BLUE);
    print_styled("Bold cyan!", Color::BOLD, Color::CYAN);
    print_styled("Underlined yellow", Color::UNDERLINE, Color::YELLOW);
    print_partial_colored(&[
        ("Bonjour ", Color::GREEN),
        ("Doni", Color::BRIGHT_YELLOW),
        (" ! Tu vas bien ?", Color::WHITE),
    ]);

    print_partial_colored(&[
        ("Error: ", Color::RED),
        ("file not found", Color::BRIGHT_RED),
        (".", Color::WHITE),
    ]);
}

