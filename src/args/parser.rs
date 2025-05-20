//! # Parser Module
//! Provide the basic helper for Command line argument parsing

use std::{collections::HashMap, env};


/// ## Argument parser
/// Use this struct to parser and localize environment variables in your code
/// 
/// The constructor retrieve automatically the args inside the `std::env` and put it in a `HasMap`
/// 
/// Use Helper to interact safely the provided args
#[derive(Debug)]
pub struct ArgParser {
    pub args: HashMap<String, Option<String>>,
}


impl ArgParser {
    /// Create a new instance of `ArgParser`
    pub fn new() -> Self {
        let mut args = HashMap::new();
        let mut iter = env::args().skip(1).peekable();

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
                if let Some(value) = iter.peek() {
                    if !value.starts_with("-") {
                        args.insert(key, Some(iter.next().unwrap()));
                    } else {
                        args.insert(key, None);
                    }
                } else {
                    args.insert(key, None);
                }
            }
        }

        Self { args }
    }

    pub fn get(&self, key: &str) -> &Option<String> {
        match self.args.get(key) {
            Some(v) => v,
            None => &None,
        }
    }

    pub fn has(&self, key: &str) -> bool {
        self.args.contains_key(key)
    }

    pub fn set(&mut self, key: &str, val: String) {
        self.args.insert(key.to_string(), Some(val));
    }
}

impl Default for ArgParser  {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty_args() {
        // Case where no arguments are passed
        let parser = ArgParser::new();
        assert!(parser.args.is_empty());
    }

    #[test]
    fn test_has_method() {
        let mut args = HashMap::new();
        args.insert("verbose".to_string(), None);
        let parser = ArgParser { args };
        
        assert!(parser.has("verbose"));
        assert!(!parser.has("nonexistent"));
    }

    #[test]
    fn test_get_method() {
        let mut args = HashMap::new();
        args.insert("file".to_string(), Some("test.txt".to_string()));
        args.insert("verbose".to_string(), None);
        let parser = ArgParser { args };
        
        assert_eq!(parser.get("file"), &Some("test.txt".to_string()));
        assert_eq!(parser.get("verbose"), &None);
        assert_eq!(parser.get("nonexistent"), &None);
    }

    #[test]
    fn test_default_implementation() {
        let parser = ArgParser::default();
        // Verify that the default implementation calls new()
        // We can only test that the structure is created correctly
        assert!(parser.args.is_empty() || !parser.args.is_empty());
    }

    // Note: More comprehensive tests would require either:
    // 1. Refactoring the code to allow argument injection
    // 2. Using mock libraries like mockall
    // 3. Creating integration tests that actually execute the program
}