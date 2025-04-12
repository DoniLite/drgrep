use std::{collections::HashMap, env};

#[derive(Debug)]
pub struct ArgParser {
    pub args: HashMap<String, Option<String>>,
}

impl ArgParser {
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
            None => &None
        }
    }

    pub fn has(&self, key: &str) -> bool {
        self.args.contains_key(key)
    }
}
