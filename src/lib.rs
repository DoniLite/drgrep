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

use std::cell::RefCell;
use std::env;
use std::fs::ReadDir;
use std::path::Path;
use std::rc::Rc;
use std::{error::Error, fs, path};

pub use color::config::Color;
pub use color::printer::print_colored;
pub use color::printer::print_partial_colored;
pub use color::printer::print_styled;
pub use regex::pattern::find;
pub use regex::pattern::find_all;
pub use regex::pattern::is_match;
pub use regex::pattern::replace_all;
pub use args::parser::ArgParser;

/// The config struct
#[derive(Debug)]
pub struct Config<'a> {
    pub search_key: &'a str,
    pub search_content: Option<&'a str>,
    pub file_path: Option<&'a str>,
    pub files: Option<Vec<&'a str>>,
    pub regex: Option<regex::pattern::SimplePattern>,
    pub sensitive: bool,
    path_is_dir: bool,
}

pub struct SearchResult<'a, 'b> {
    pub line: Vec<(&'a str, &'a str)>,
    pub word: &'b str,
    pub source: &'b str,
    pub idx: usize,
}

pub static DEFAULT_MESSAGE: &str = "\
drgep is a CLI searching tool
Usage:
grgrep [args]

[args]
-k key <optional:false> => The word that you want to search
-p path <optional:true>, <default: '/'> => The path of the file which you want to provide searching
";

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
        let mut is_dir = false;
        let file_path = match args.get("path") {
            Some(value) => {
                let path = Path::new(value);
                if path.is_file() {
                    Some(value.as_str())
                } else if path.is_dir() {
                    is_dir = true;
                    Some(value.as_str())
                } else {
                    None
                }
            }
            None => {
                let p = args.get("p").as_ref().map(|v| v.as_str());
                if let Some(pth) = p {
                    let path = Path::new(pth);
                    if path.is_file() {
                        p
                    } else if path.is_dir() {
                        is_dir = true;
                        p
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
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
        let search_content = match args.get("content") {
            Some(c) => Some(c.as_str()),
            None => args.get("c").as_ref().map(|v| v.as_str()),
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
            search_content,
            path_is_dir: is_dir,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if !config.path_is_dir {
        if let Some(val) = config.file_path {
            let file_path = path::Path::new(val);
            let content = fs::read_to_string(file_path)?;
            if config.sensitive {
                for result in search_word_sensitive_case(&config, val, &content) {
                    print_colored!(
                        format!("source: {}", result.source).as_str(),
                        color::config::Color::BRIGHT_BLUE
                    );
                    println!();
                    print_colored!(
                        format!("line: {}", result.idx).as_str(),
                        color::config::Color::RED
                    );
                    println!();
                    print_partial_colored!(&result.line);
                    println!("===========================");
                }
            } else {
                for result in search_word_insensitive_case(&config, val, &content) {
                    print_colored!(
                        format!("source: {}", result.source).as_str(),
                        color::config::Color::BRIGHT_BLUE
                    );
                    println!();
                    print_colored!(
                        format!("line: {}", result.idx).as_str(),
                        color::config::Color::RED
                    );
                    println!();
                    print_partial_colored!(&result.line);
                    println!("===========================");
                }
            }
            return Ok(());
        }
    }

    let files: ReadDir;
    if let Some(val) = config.file_path {
        files = fs::read_dir(Path::new(val))?;
    } else {
        files = fs::read_dir(Path::new("./"))?;
    }
    files.for_each(|el| {
        if let Ok(f) = el {
            utilities::visit_dirs(&f.path(), &|f| {
                if let Ok(f_type) = f.file_type() {
                    if f_type.is_file() {
                        if let Ok(content) = utilities::can_read_to_utf8(&f.path()) {
                            if config.sensitive {
                                for result in search_word_sensitive_case(
                                    &config,
                                    f.path().to_str().unwrap(),
                                    &content,
                                ) {
                                    print_colored!(
                                        format!("source: {}", result.source).as_str(),
                                        color::config::Color::BRIGHT_BLUE
                                    );
                                    println!();
                                    print_colored!(
                                        format!("line: {}", result.idx).as_str(),
                                        color::config::Color::RED
                                    );
                                    println!();
                                    print_partial_colored!(&result.line);
                                    println!("===========================");
                                }
                            } else {
                                for result in search_word_insensitive_case(
                                    &config,
                                    f.path().to_str().unwrap(),
                                    &content,
                                ) {
                                    print_colored!(
                                        format!("source: {}", result.source).as_str(),
                                        color::config::Color::BRIGHT_BLUE
                                    );
                                    println!();
                                    print_colored!(
                                        format!("line: {}", result.idx).as_str(),
                                        color::config::Color::RED
                                    );
                                    println!();
                                    print_partial_colored!(&result.line);
                                    println!("===========================");
                                }
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
    config: &'a Config<'b>,
    source: &'b str,
    content: &'a str,
) -> Vec<SearchResult<'a, 'b>> {
    let shared_config = Rc::new(config);
    // Used to count the number of the content lines
    // This value is incremented
    let occ = Rc::new(RefCell::new(0));
    content
        .lines()
        .filter(|l| {
            *Rc::clone(&occ).borrow_mut() += 1;
            l.contains(Rc::clone(&shared_config).search_key)
        })
        .map(|line| {
            let conf = Rc::clone(&shared_config);
            let parts = line
                .split(' ')
                .map(|w| {
                    let pattern = regex::pattern::SimplePattern::new(conf.search_key).unwrap();
                    if pattern.is_match(w) {
                        (w, color::config::Color::BRIGHT_YELLOW)
                    } else {
                        (w, color::config::Color::WHITE)
                    }
                })
                .collect();
            SearchResult {
                line: parts,
                word: conf.search_key,
                source,
                idx: *Rc::clone(&occ).borrow(),
            }
        })
        .collect()
}

pub fn search_word_insensitive_case<'a, 'b>(
    config: &'a Config<'b>,
    source: &'b str,
    content: &'a str,
) -> Vec<SearchResult<'a, 'b>> {
    let shared_config = Rc::new(config);
    // Used to count the number of the content lines
    // This value is incremented
    let occ = Rc::new(RefCell::new(0));
    content
        .lines()
        .filter(|l| {
            *Rc::clone(&occ).borrow_mut() += 1;
            l.to_lowercase()
                .contains(&Rc::clone(&shared_config).search_key.to_lowercase())
        })
        .map(|line| {
            let conf = Rc::clone(&shared_config);
            let parts = line
                .split(' ')
                .map(|w| {
                    let pattern =
                        regex::pattern::SimplePattern::new(conf.search_key.to_lowercase().as_str())
                            .unwrap();
                    if pattern.is_match(w.to_lowercase().as_str()) {
                        (w, color::config::Color::BRIGHT_YELLOW)
                    } else {
                        (w, color::config::Color::WHITE)
                    }
                })
                .collect();
            SearchResult {
                line: parts,
                word: conf.search_key,
                source,
                idx: *Rc::clone(&occ).borrow(),
            }
        })
        .collect()
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
        let config = Config {
            search_content: None,
            file_path: None,
            search_key: recherche,
            files: None,
            regex: None,
            sensitive: true,
            path_is_dir: false,
        };
        let content = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique.";
        assert_eq!(1, search_word_sensitive_case(&config, "", content).len());
    }

    #[test]
    fn insensitive_case_search_word() {
        let recherche = "rUst";
        let config = Config {
            search_content: None,
            file_path: None,
            search_key: recherche,
            files: None,
            regex: None,
            sensitive: true,
            path_is_dir: false,
        };
        let content = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique.";
        assert_eq!(
            vec![("Rust:", color::config::Color::BRIGHT_YELLOW)],
            search_word_insensitive_case(&config, recherche, content)[0].line
        );
    }
}
