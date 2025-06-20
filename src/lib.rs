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
pub mod glob;
pub mod regex;
pub mod temp_dir;

use std::env;
use std::fs::{DirEntry, ReadDir};
use std::path::{Path, PathBuf};
use std::{error::Error, fs, path};

pub use args::parser::ArgParser;
pub use color::config::Color;
pub use color::printer::print_colored;
pub use color::printer::print_partial_colored;
pub use color::printer::print_styled;
pub use regex::pattern::find;
pub use regex::pattern::find_all;
pub use regex::pattern::is_match;
pub use regex::pattern::replace_all;
pub use regex::pattern::RegexPattern;
pub use regex::pattern::RegexPattern as SimplePattern;
pub use utilities::read_stdin;

/// The config struct
#[derive(Debug)]
pub struct Config<'a> {
    pub search_key: Option<&'a str>,
    pub search_content: Option<&'a str>,
    pub file_path: Option<&'a str>,
    pub regex: Option<regex::pattern::RegexPattern>,
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
drgrep is a CLI searching tool
Usage:
drgrep --[args]/-[flag]

[flags]-[args]
-h --help => Print this default help message
-v --version => Print the current version of the drgrep software
-k --key <optional:false> => The word that you want to search
-p --path <optional:true>, <default: '/'> => The path of the file which you want to provide searching
-r --regex <optional:true> => The regex expression to use for matching
-c --content <optional:true> => The content in which the program will process can be provided as string
-s --sensitive <optional:true> => Use this to setup a sensitive case config you can use it with the env variables via : [DRGREP_SENSITIVE_CASE]
";

pub static VERSION: &str = "v0.2.3";

impl<'a> Config<'a> {
    pub fn new(args: &'a args::parser::ArgParser) -> Result<Self, &'static str> {
        if !args.has("key")
            && !args.has("k")
            && !args.has("regex")
            && !args.has("r")
            && !args.has("content")
            && !args.has("c")
        {
            return Err("no search key/regex provided");
        }
        let search_key = match args.get("key") {
            Some(value) => Some(value.as_str()),
            None => args.get("k").as_ref().map(|v| v.as_str()),
        };
        let mut is_dir = true;
        let file_path = match args.get("path") {
            Some(value) => {
                let path = Path::new(value);
                if path.is_file() {
                    is_dir = false;
                    Some(value.as_str())
                } else if path.is_dir() {
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
                        is_dir = false;
                        p
                    } else if path.is_dir() {
                        p
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        };
        let regex = match args.get("regex") {
            Some(value) => match regex::pattern::RegexPattern::new(value) {
                Ok(val) => Some(val),
                Err(_) => return Err("Error during the creating of the current regex"),
            },
            None => {
                if let Some(value) = args.get("r") {
                    if let Ok(r) = regex::pattern::RegexPattern::new(value) {
                        Some(r)
                    } else {
                        return Err("Error during the creating of the current regex");
                    }
                } else {
                    None
                }
            }
        };
        let search_content = match args.get("content") {
            Some(c) => Some(c.as_str()),
            None => {
                if let Some(c) = args.get("c") {
                    is_dir = false;
                    Some(c.as_str())
                } else {
                    None
                }
            }
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
            sensitive,
            regex,
            search_content,
            path_is_dir: is_dir,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let ignore = utilities::GitIgnoreFiles::load();
    let current_dir = if let Ok(p) = env::current_dir() {
        p
    } else {
        PathBuf::new()
    };
    if !config.path_is_dir {
        if let Some(val) = config.file_path {
            let file_path = path::Path::new(val);
            let content = fs::read_to_string(file_path)?;
            if let Some(reg) = config.regex {
                for result in search_with_regex(&reg, val, &content) {
                    print_colored!(
                        format!("source: {}", result.source).as_str(),
                        color::config::Color::BRIGHT_BLUE
                    );
                    print_colored!(
                        format!("line: {}", result.idx).as_str(),
                        color::config::Color::RED
                    );
                    print_partial_colored!(&result.line);
                    println!("=================================\n");
                }
                return Ok(());
            } else if config.sensitive {
                if let Some(key) = config.search_key {
                    for result in search_word_sensitive_case(key, val, &content) {
                        print_colored!(
                            format!("source: {}", result.source).as_str(),
                            color::config::Color::BRIGHT_BLUE
                        );
                        print_colored!(
                            format!("line: {}", result.idx).as_str(),
                            color::config::Color::RED
                        );
                        print_partial_colored!(&result.line);
                        println!("=================================\n");
                    }
                }
                return Ok(());
            } else if let Some(key) = config.search_key {
                for result in search_word_insensitive_case(key, val, &content) {
                    print_colored!(
                        format!("source: {}", result.source).as_str(),
                        color::config::Color::BRIGHT_BLUE
                    );
                    print_colored!(
                        format!("line: {}", result.idx).as_str(),
                        color::config::Color::RED
                    );
                    print_partial_colored!(&result.line);
                    println!("=================================\n");
                }
            }
            return Ok(());
        } else if let Some(content) = config.search_content {
            // println!("search content {}", content);
            if let Some(key) = config.search_key {
                if config.sensitive {
                    for result in search_word_sensitive_case(key, "", content) {
                        print_colored!(
                            format!("line: {}", result.idx).as_str(),
                            color::config::Color::RED
                        );
                        print_partial_colored!(&result.line);
                        println!("=================================\n");
                    }
                } else {
                    for result in search_word_insensitive_case(key, "", content) {
                        print_colored!(
                            format!("line: {}", result.idx).as_str(),
                            color::config::Color::RED
                        );
                        print_partial_colored!(&result.line);
                        println!("=================================\n");
                    }
                }
            } else if let Some(reg) = config.regex {
                for result in search_with_regex(&reg, "", content) {
                    print_colored!(
                        format!("line: {}", result.idx).as_str(),
                        color::config::Color::RED
                    );
                    print_partial_colored!(&result.line);
                    println!("=================================\n");
                }
            }
        }
        return Ok(());
    }

    let files: ReadDir;
    if let Some(val) = config.file_path {
        files = fs::read_dir(Path::new(val))?;
    } else {
        files = fs::read_dir(Path::new("./"))?;
    }

    let handle_files: &dyn Fn(&DirEntry) = &|f| {
        if let Ok(f_type) = f.file_type() {
            if f_type.is_file() && !ignore.is_ignored(&f.path(), &current_dir) {
                if let Ok(content) = utilities::can_read_to_utf8(&f.path()) {
                    if let Some(reg) = &config.regex {
                        for result in search_with_regex(reg, f.path().to_str().unwrap(), &content) {
                            print_colored!(
                                format!("source: {}", result.source).as_str(),
                                color::config::Color::BRIGHT_BLUE
                            );
                            print_colored!(
                                format!("line: {}", result.idx).as_str(),
                                color::config::Color::RED
                            );
                            print_partial_colored!(&result.line);
                            println!("=================================\n");
                        }
                        return;
                    }
                    if config.sensitive {
                        if let Some(key) = config.search_key {
                            for result in search_word_sensitive_case(
                                key,
                                f.path().to_str().unwrap(),
                                &content,
                            ) {
                                print_colored!(
                                    format!("source: {}", result.source).as_str(),
                                    color::config::Color::BRIGHT_BLUE
                                );
                                print_colored!(
                                    format!("line: {}", result.idx).as_str(),
                                    color::config::Color::RED
                                );
                                print_partial_colored!(&result.line);
                                println!("=================================\n");
                            }
                        }
                    } else if let Some(key) = config.search_key {
                        for result in
                            search_word_insensitive_case(key, f.path().to_str().unwrap(), &content)
                        {
                            print_colored!(
                                format!("source: {}", result.source).as_str(),
                                color::config::Color::BRIGHT_BLUE
                            );
                            print_colored!(
                                format!("line: {}", result.idx).as_str(),
                                color::config::Color::RED
                            );
                            print_partial_colored!(&result.line);
                            println!("=================================\n");
                        }
                    }
                }
            }
        }
    };

    files
        .filter(|f| {
            if let Ok(entry) = f {
                // println!("entry: {}", entry.path().display());
                !ignore.is_ignored(&entry.path(), &current_dir)
            } else {
                false
            }
        })
        .for_each(|el| {
            if let Ok(f) = el {
                if f.path().is_file() {
                    handle_files(&f);
                    return;
                }
                utilities::visit_dirs(&f.path(), handle_files)
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
    key: &'b str,
    source: &'b str,
    content: &'a str,
) -> Vec<SearchResult<'a, 'b>> {
    content
        .lines()
        .enumerate() // Provides a line index automatically
        .filter(|(_, line)| line.contains(key))
        .map(|(idx, line)| {
            // For example, split the line and highlight matching words
            let parts = line
                .split(' ')
                .map(|w| {
                    let color = if w.contains(key) {
                        color::config::Color::BRIGHT_YELLOW
                    } else {
                        color::config::Color::WHITE
                    };
                    (w, color)
                })
                .collect();
            SearchResult {
                line: parts,
                word: key,
                source,
                idx: idx + 1, // Using one-based line numbers
            }
        })
        .collect()
}

pub fn search_word_insensitive_case<'a, 'b>(
    key: &'b str,
    source: &'b str,
    content: &'a str,
) -> Vec<SearchResult<'a, 'b>> {
    content
        .lines()
        .enumerate() // Provides a line index automatically
        .filter(|(_, line)| line.to_lowercase().contains(&key.to_lowercase()))
        .map(|(idx, line)| {
            // For example, split the line and highlight matching words
            let parts = line
                .split(' ')
                .map(|w| {
                    let color = if w.to_lowercase().contains(&key.to_lowercase()) {
                        color::config::Color::BRIGHT_YELLOW
                    } else {
                        color::config::Color::WHITE
                    };
                    (w, color)
                })
                .collect();
            SearchResult {
                line: parts,
                word: key,
                source,
                idx: idx + 1, // Using one-based line numbers
            }
        })
        .collect()
}

pub fn search_with_regex<'a, 'b>(
    regex: &RegexPattern,
    source: &'b str,
    content: &'a str,
) -> Vec<SearchResult<'a, 'b>> {
    content
        .lines()
        .enumerate()
        .filter(|(_, l)| regex.is_match(l))
        .map(|(idx, line)| {
            let parts = line
                .split(' ')
                .map(|w| {
                    let color = if regex.is_match(w.to_lowercase().as_str()) {
                        color::config::Color::BRIGHT_YELLOW
                    } else {
                        color::config::Color::WHITE
                    };
                    (w, color)
                })
                .collect();
            SearchResult {
                line: parts,
                word: "",
                source,
                idx: idx + 1,
            }
        })
        .collect()
}

mod utilities {

    use crate::{glob::GlobPattern, Path};
    use std::{
        env,
        error::Error,
        fs::{self, DirEntry},
        io::{self, stdin, Read},
        path::PathBuf,
        rc::Rc,
    };

    #[derive(Debug)]
    pub struct GitIgnoreFiles {
        pub pattern: Vec<Rc<GlobPattern>>,
        pub entries: Vec<Rc<String>>,
    }

    impl GitIgnoreFiles {
        pub fn load() -> Self {
            let mut patterns = Vec::new();
            let mut entries = Vec::new();
            let cur_dir = if let Ok(p) = env::current_dir() {
                p
            } else {
                PathBuf::new()
            };
            if let Ok(content) = fs::read_to_string(Path::new(".gitignore")) {
                // println!("gitignore content: \n {}", content);
                content.lines().for_each(|l| {
                    patterns.push(Rc::new(GlobPattern::new(&format!(
                        "{}/{}",
                        cur_dir.display(),
                        l
                    ))));
                    entries.push(Rc::new(l.to_string()));
                });
            }
            patterns.push(Rc::new(GlobPattern::new(&format!(
                "{}/.git/**",
                cur_dir.display()
            ))));
            // println!(
            //     "format constructor for git: {}",
            //     &format!("{}/.git/**", cur_dir.display())
            // );
            Self {
                pattern: patterns,
                entries,
            }
        }

        pub fn is_ignored(&self, p: &Path, current: &Path) -> bool {
            if let Some(pth) = p.to_str() {
                let gen_path = format!("{}{}", current.display(), &pth[1..]);
                // println!("Generated path in the current {}", gen_path);
                self.pattern.iter().any(|pat| pat.matches(&gen_path))
                    || self.entries.iter().any(|e| pth.contains(e.as_str()))
            } else {
                false
            }
        }
    }

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

    pub fn read_stdin() -> io::Result<String> {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
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
            search_key: Some(recherche),
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
            1,
            search_word_sensitive_case(config.search_key.unwrap(), "", content).len()
        );
    }

    #[test]
    fn insensitive_case_search_word() {
        let recherche = "rUst";
        let config = Config {
            search_content: None,
            file_path: None,
            search_key: Some(recherche),
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
            search_word_insensitive_case(config.search_key.unwrap(), recherche, content)[0].line
        );
    }
}
