//! # drgrep
//!
//! A Rust implementation of the grep software with more support and features for args, workspace scanning and CLI.
//!
//! ## Features
//!
//! * Recursive research
//! * Command Lines Parser
//! * Regex Utilities
//! * CLI coloration
//!
//! ## Examples
//!
//! ```rust
//! use drgrep::{args::parser::ArgParser, search_sensitive_case};
//!
//! fn main() {
//!     let args = ArgParser::new();
//!     println!("Results: {:?}", args);
//!     let search_key = "duct";
//!     let content = "\
//!Rust:
//!sécurité, rapidité, productivité.
//!Obtenez les trois en même temps.
//!Duck tape.";
//!     assert_eq!(
//!     vec!["sécurité, rapidité, productivité."],
//!     search_sensitive_case(search_key, content)
//! );
//! }
//! ```

pub mod args;
pub mod color;
pub mod regex;

use std::env;
use std::path::Path;
use std::{error::Error, fs, path};

pub use color::config::Color;
pub use color::printer::print_colored;
pub use color::printer::print_partial_colored;
pub use color::printer::print_styled;

/// The config struct
#[derive(Debug)]
pub struct Config<'a> {
    pub search_key: &'a str,
    pub file_path: Option<&'a str>,
    pub files: Option<Vec<&'a str>>,
    pub regex: Option<regex::pattern::SimplePattern>,
    pub sensitive: bool,
}

pub struct SearchResult<'a, 'b> {
    pub lines: Vec<LinesInfo<'a>>,
    pub word: &'b str,
    pub occurrence: usize,
    // pub source: Vec<String>
}

pub struct LinesInfo<'a> {
    pub line: &'a str,
    pub start_index: usize,
    pub end_index: usize,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a args::parser::ArgParser) -> Result<Self, &'static str> {
        if !args.has("key") && !args.has("k") {
            return Err("no search key provided");
        }
        let search_key = match args.get("key") {
            Some(value) => value.as_str(),
            None => match args.get("k") {
                Some(v) => v.as_str(),
                None => return Err("key value not found"),
            },
        };
        let split_search_key: Vec<&str> = search_key.split(',').collect();
        let file_path = match args.get("source") {
            Some(value) => Some(value.as_str()),
            None => args.get("s").as_ref().map(|v| v.as_str()),
        };
        let files = if split_search_key.len() >= 2 {
            Some(split_search_key)
        } else {
            None
        };
        let regex = match args.get("regex") {
            Some(value) => match regex::pattern::SimplePattern::new(value) {
                Ok(val) => Some(val),
                Err(_) => return Err("Error during the creating of the current regex"),
            },
            None => None,
        };
        let sensitive = match args.get("sensitive") {
            Some(_) => true,
            None => match args.get("s") {
                Some(_) => true,
                None => env::var("DRGREP_SENSITIVE_CASE").is_ok(),
            },
        };

        Ok(Config {
            search_key,
            file_path,
            files,
            sensitive,
            regex,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let Some(val) = config.file_path {
        let file_path = path::Path::new(val);
        let content = fs::read_to_string(file_path)?;
        for line in search_sensitive_case(config.search_key, &content) {
            println!("{}", line);
        }
    }
    let files = fs::read_dir(Path::new("./"))?;
    files.for_each(|el| {
        if let Ok(f) = el {
            utilities::visit_dirs(&f.path(), &|f| {
                if let Ok(f_type) = f.file_type() {
                    if f_type.is_file() {
                        if let Ok(content) = utilities::can_read_to_utf8(&f.path()) {
                            for line in search_sensitive_case(config.search_key, &content) {
                                println!("{}", line);
                            }
                        }
                    }
                }
            })
            .unwrap_or_else(|err| panic!("{}", err))
        }
    });
    Ok(())
}

pub fn search_sensitive_case<'a>(search_content: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(search_content))
        .collect()
}

pub fn search_insensitive_case<'a>(search_content: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&search_content.to_lowercase()) {
            result.push(line);
        }
    }
    result
}

pub fn search_word_sensitive_case<'a, 'b>(
    search_content: &'b str,
    content: &'a str,
) -> SearchResult<'a, 'b> {
    let mut result: SearchResult<'a, 'b> = SearchResult {
        lines: Vec::new(),
        word: search_content,
        occurrence: 0,
    };
    for line in content.lines() {
        if line.contains(search_content) {
            let word_index = line.find(search_content).unwrap();
            result.lines.push(LinesInfo {
                line,
                start_index: word_index,
                end_index: search_content.len() + word_index,
            });
            result.occurrence += 1;
        }
    }
    result
}

pub fn search_word_insensitive_case<'a, 'b>(
    search_content: &'b str,
    content: &'a str,
) -> SearchResult<'a, 'b> {
    let mut result: SearchResult<'a, 'b> = SearchResult {
        lines: Vec::new(),
        word: search_content,
        occurrence: 0,
    };
    let lowercase_search_content = search_content.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&search_content.to_lowercase()) {
            let word_index = line.to_lowercase().find(&lowercase_search_content).unwrap();
            result.lines.push(LinesInfo {
                line,
                start_index: word_index,
                end_index: search_content.len() + word_index,
            });
            result.occurrence += 1;
        }
    }
    result
}

mod utilities {

    use crate::Path;
    use std::{
        error::Error,
        fs::{self, DirEntry},
        io::{self, Read},
    };

    pub fn can_read_to_utf8(path: &Path) -> Result<String, Box<dyn Error>> {
        let mut file = fs::File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }

    pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, cb)?;
                } else {
                    cb(&entry);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_case() {
        let search_key = "duct";
        let content = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
Duck tape.";
        assert_eq!(
            vec!["sécurité, rapidité, productivité."],
            search_sensitive_case(search_key, content)
        );
    }

    #[test]
    fn insensitive_case() {
        let recherche = "rUsT";
        let content = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique.";
        assert_eq!(
            vec!["Rust:", "C'est pas rustique."],
            search_insensitive_case(recherche, content)
        );
    }

    #[test]
    fn sensitive_case_search_word() {
        let recherche = "Rust";
        let content = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique.";
        assert_eq!(1, search_word_sensitive_case(recherche, content).occurrence);
    }

    #[test]
    fn insensitive_case_search_word() {
        let recherche = "rUst";
        let content = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique.";
        assert_eq!(
            "Rust:",
            search_word_insensitive_case(recherche, content).lines[0].line
        );
    }
}
