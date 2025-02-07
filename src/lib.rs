use std::{error::Error, fs, path};

pub struct Config {
    pub search_key: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("You must provide arguments to the script example:  \n cargo run arg1 arg2");
        }
        Ok(Config {
            search_key: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_path = path::Path::new(&config.file_path);
    let content =
        fs::read_to_string(file_path)?;
    println!("The text content :\n{}", content);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let config = Config {
            search_key: "hello".to_string(),
            file_path: "poem.txt".to_string(),
        };
        assert!(run(config).is_ok());
    }
}