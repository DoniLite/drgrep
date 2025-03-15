use std::{error::Error, fs, path};

pub struct Config {
    pub search_key: String,
    pub file_path: String,
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
    println!("The text content :\n{}", content);
    Ok(())
}

pub fn search<'a>(search_content: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(search_content) {
            result.push(line);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runner() {
        let config = Config {
            search_key: "hello".to_string(),
            file_path: "poem.txt".to_string(),
        };
        assert!(run(config).is_ok());
    }

    #[test]
    fn test_searcher() {
        let search_key = "duct";
        let content = "\
    Rust:sécurité, rapidité, productivité.
    Obtenez les trois en même temps.";
        assert_eq!(
            vec!["sécurité, rapidité, productivité."],
            search(search_key, content)
        );
    }
}
