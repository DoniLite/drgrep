//! # ArgParser Module
//!
//! A simple and lightweight command-line argument parser for Rust applications.
//!
//! ## Features
//!
//! - Parses command-line arguments in short (`-a`) and long (`--argument`) format
//! - Supports arguments with or without values
//! - Simple and intuitive interface
//! - No external dependencies
//!
//! ## Installation
//!
//! Add this dependency to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! drgrep = "0.1.0"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use drgrep::ArgParser;
//!
//! fn main() {
//!    // Initialize the parser (automatically parses command-line arguments)
//!    let args = ArgParser::new();
//!
//!    // Check if a flag is present
//!    if args.has("verbose") || args.has("v") {
//!        println!("Verbose mode enabled");
//!    }
//!
//!    // Get the value of an argument
//!    match args.get("file") {
//!        Some(file_path) => println!("Using file: {}", file_path),
//!        None => {
//!            if args.has("file") {
//!                println!("The --file option was specified without a value");
//!            } else {
//!                println!("No file specified");
//!            }
//!        }
//!    }
//! }
//! ```
//!
//! ## Command Line Examples
//!
//! The parser supports the following argument formats:
//!
//! ```sh
//! # Arguments without values (flags)
//! $ ./my_program --verbose
//! $ ./my_program -v
//!
//! # Arguments with values
//! $ ./my_program --file test.txt
//! $ ./my_program -f test.txt
//!
//! # Combination of arguments
//! $ ./my_program --verbose --file test.txt -o output.log
//! ```
//!
//! ## Current Limitations
//!
//! - No support for positional arguments (not preceded by `-` or `--`)
//! - No support for grouped arguments (like `-abc` for `-a -b -c`)
//! - No support for arguments with values in the form `--key=value`
//! - No built-in validation for required arguments
//!
//! ## Contributing
//!
//! Contributions are welcome! Feel free to open an issue or pull request.
//!
//! ## License
//!
//! This tool is distributed under the [MIT License].

pub mod parser;