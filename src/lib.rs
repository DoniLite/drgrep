use std::{collections::HashMap, env, error::Error, fs, path};


pub struct ArgParser {
    pub args: HashMap<String, Option<String>>,
}

impl ArgParser {
    pub fn new() -> Self {
        let mut args = HashMap::new();
        let mut iter = env::args().skip(1).peekable(); // Ignore le premier argument (nom du programme)

        while let Some(arg) = iter.next() {
            if arg.starts_with("--") {
                let key = arg.trim_start_matches("--").to_string();
                if let Some(value) = iter.peek() {
                    if !value.starts_with("--") {
                        args.insert(key, Some(iter.next().unwrap()));
                    } else {
                        args.insert(key, None);
                    }
                } else {
                    args.insert(key, None);
                }
            } else if arg.starts_with("-") {
                let key = arg.trim_start_matches("-").to_string();
                args.insert(key, None);
            }
        }

        Self { args }
    }

    pub fn get(&self, key: &str) -> Option<&Option<String>> {
        self.args.get(key)
    }

    pub fn has(&self, key: &str) -> bool {
        self.args.contains_key(key)
    }
}
/// The config struct
pub struct Config {
    pub search_key: String,
    pub file_path: String,
}

pub struct SearchResult<'a, 'b> {
    pub lines: Vec<LinesInfo<'a>>,
    pub word: &'b str,
    pub occurrence: usize,
}

pub struct LinesInfo<'a> {
    pub line: &'a str,
    pub start_index: usize,
    pub end_index: usize,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err(
                "You must provide arguments to the script example:  \n cargo run arg1 arg2",
            );
        }
        Ok(Config {
            search_key: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_path = path::Path::new(&config.file_path);
    let content = fs::read_to_string(file_path)?;
    for line in search_sensitive_case(&config.search_key, &content) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search_sensitive_case<'a>(search_content: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(search_content) {
            result.push(line);
        }
    }
    result
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
