// tests/integration_test.rs

use drgrep::{
    args::parser::ArgParser, regex::pattern::RegexPattern, search_insensitive_case,
    search_sensitive_case, Config,
};
use std::collections::HashMap;

#[test]
fn test_search_functionality() {
    // Test basic search functionality
    let content = "\
Hello world
This is a test
Find me in this line
Nothing here";

    // Test sensitive case search
    let results = search_sensitive_case("Find", content);
    assert_eq!(vec!["Find me in this line"], results);

    // Test insensitive case search
    let results = search_insensitive_case("hello", content);
    assert_eq!(vec!["Hello world"], results);
}

#[test]
fn test_config_creation() {
    // Create a mock ArgParser
    let mut args_map = HashMap::new();
    args_map.insert("key".to_string(), Some("test".to_string()));
    args_map.insert("path".to_string(), Some("./src".to_string()));
    args_map.insert("sensitive".to_string(), Some("true".to_string()));

    let args = ArgParser { args: args_map };

    // Create config from args
    let config = Config::new(&args).unwrap();

    // Test values
    assert_eq!("test", config.search_key.unwrap());
    assert_eq!(Some("./src"), config.file_path);
    assert!(config.sensitive);
}

#[test]
fn test_pattern_usage() {
    // Test pattern creation and matching
    let pattern = RegexPattern::new("\\w+").unwrap();

    assert!(pattern.is_match("Hello"));
    assert!(!pattern.is_match(" "));

    // Test find functionality
    let text = "Hello, World!";
    let result = pattern.find(text).unwrap();
    assert_eq!("Hello", result.text);

    // Test find_all functionality
    let results = pattern.find_all(text);
    assert_eq!(2, results.len());
    assert_eq!("Hello", results[0].text);
    assert_eq!("World", results[1].text);

    // Test replace functionality
    let replaced = pattern.replace_all(text, "TEXT");
    assert_eq!("TEXT, TEXT!", replaced);
}

#[test]
fn test_config_with_regex() {
    // Create a mock ArgParser with regex
    let mut args_map = HashMap::new();
    args_map.insert("key".to_string(), Some("test".to_string()));
    args_map.insert("regex".to_string(), Some("\\w+".to_string()));

    let args = ArgParser { args: args_map };

    // Create config from args
    let config = Config::new(&args).unwrap();

    // Test regex pattern
    assert!(config.regex.is_some());
    let pattern = config.regex.as_ref().unwrap();
    assert!(pattern.is_match("Hello"));
    assert!(!pattern.is_match(" "));
}

#[test]
fn test_error_handling() {
    // Test missing key argument
    let args = ArgParser {
        args: HashMap::new(),
    };

    let result = Config::new(&args);
    assert!(result.is_err());
    assert_eq!("no search key/regex provided", result.unwrap_err());

    // Test invalid regex pattern
    let mut args_map = HashMap::new();
    args_map.insert("key".to_string(), Some("test".to_string()));
    args_map.insert("regex".to_string(), Some("*invalid".to_string())); // Invalid pattern (starts with quantifier)

    let args = ArgParser { args: args_map };

    let result = Config::new(&args);
    assert!(result.is_err());
}
