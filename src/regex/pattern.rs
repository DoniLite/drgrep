//! # Module Pattern
//!
//! Provides advanced pattern matching and replacement functionality using the `regex` crate.
//! This implementation offers a comprehensive set of regular expression features.
//!
//! Supported patterns:
//! - All standard regular expression syntax supported by the `regex` crate
//! - Capture groups for more advanced replacement scenarios

use regex::Regex;
use std::error::Error;
use std::fmt;

/// Errors specific to pattern matching operations
#[derive(Debug)]
pub enum PatternError {
    /// Error in the regular expression pattern
    RegexError(regex::Error),
    /// Other errors
    Other(String),
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternError::RegexError(e) => write!(f, "Regex error: {}", e),
            PatternError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl Error for PatternError {}

impl From<regex::Error> for PatternError {
    fn from(err: regex::Error) -> PatternError {
        PatternError::RegexError(err)
    }
}

/// Main structure for pattern matching and replacement
#[derive(Debug)]
pub struct RegexPattern {
    regex: Regex,
    pattern: String,
}

/// Result of a match
#[derive(Debug, Clone)]
pub struct Match {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

impl RegexPattern {
    /// Creates a new regex pattern from a pattern string
    pub fn new(pattern: &str) -> Result<Self, PatternError> {
        let regex = Regex::new(pattern)?;
        Ok(RegexPattern {
            regex,
            pattern: pattern.to_string(),
        })
    }

    /// Returns the original pattern string
    pub fn get_pattern(&self) -> &str {
        self.pattern.as_str()
    }

    /// Checks if the text matches the pattern
    pub fn is_match(&self, text: &str) -> bool {
        self.regex.is_match(text)
    }

    /// Finds the first match in the text
    pub fn find(&self, text: &str) -> Option<Match> {
        self.regex.find(text).map(|m| Match {
            text: m.as_str().to_string(),
            start: m.start(),
            end: m.end(),
        })
    }

    /// Finds all matches in the text
    pub fn find_all(&self, text: &str) -> Vec<Match> {
        self.regex
            .find_iter(text)
            .map(|m| Match {
                text: m.as_str().to_string(),
                start: m.start(),
                end: m.end(),
            })
            .collect()
    }

    /// Replaces all occurrences of the pattern with the replacement string
    pub fn replace_all(&self, text: &str, replacement: &str) -> String {
        self.regex.replace_all(text, replacement).into_owned()
    }

    /// Replaces all occurrences of the pattern with the result of a function
    /// The function receives a `regex::Captures` object, allowing access to capture groups.
    pub fn replace_all_with<F>(&self, text: &str, replacement_fn: F) -> String
    where
        F: Fn(&regex::Captures) -> String,
    {
        self.regex.replace_all(text, replacement_fn).into_owned()
    }

    /// Splits the text according to the pattern
    pub fn split(&self, text: &str) -> Vec<String> {
        self.regex.split(text).map(|s| s.to_string()).collect()
    }
}

// Utility functions
pub fn is_match(pattern: &str, text: &str) -> Result<bool, PatternError> {
    let p = RegexPattern::new(pattern)?;
    Ok(p.is_match(text))
}

pub fn find(pattern: &str, text: &str) -> Result<Option<Match>, PatternError> {
    let p = RegexPattern::new(pattern)?;
    Ok(p.find(text))
}

pub fn find_all(pattern: &str, text: &str) -> Result<Vec<Match>, PatternError> {
    let p = RegexPattern::new(pattern)?;
    Ok(p.find_all(text))
}

pub fn replace_all(pattern: &str, text: &str, replacement: &str) -> Result<String, PatternError> {
    let p = RegexPattern::new(pattern)?;
    Ok(p.replace_all(text, replacement))
}

pub fn replace_all_with<F>(
    pattern: &str,
    text: &str,
    replacement_fn: F,
) -> Result<String, PatternError>
where
    F: Fn(&regex::Captures) -> String,
{
    let p = RegexPattern::new(pattern)?;
    Ok(p.replace_all_with(text, replacement_fn))
}

pub fn split(pattern: &str, text: &str) -> Result<Vec<String>, PatternError> {
    let p = RegexPattern::new(pattern)?;
    Ok(p.split(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_match() {
        let pattern = RegexPattern::new("abc").unwrap();
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("xabcy"));
        assert!(!pattern.is_match("ab"));
    }

    #[test]
    fn test_any_char() {
        let pattern = RegexPattern::new("a.c").unwrap();
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("axc"));
        assert!(!pattern.is_match("ac"));
    }

    #[test]
    fn test_digit() {
        let pattern = RegexPattern::new(r"a\dc").unwrap();
        assert!(pattern.is_match("a1c"));
        assert!(pattern.is_match("a9c"));
        assert!(!pattern.is_match("abc"));

        let pattern = RegexPattern::new(r"a\Dc").unwrap();
        assert!(!pattern.is_match("a1c"));
        assert!(pattern.is_match("abc"));
    }

    #[test]
    fn test_quantifiers() {
        // Zero or more
        let pattern = RegexPattern::new("ab*c").unwrap();
        assert!(pattern.is_match("ac"));
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("abbc"));

        // One or more
        let pattern = RegexPattern::new("ab+c").unwrap();
        assert!(!pattern.is_match("ac"));
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("abbc"));

        // Zero or one
        let pattern = RegexPattern::new("ab?c").unwrap();
        assert!(pattern.is_match("ac"));
        assert!(pattern.is_match("abc"));
        assert!(!pattern.is_match("abbc"));
    }

    #[test]
    fn test_anchors() {
        // Start anchor
        let pattern = RegexPattern::new("^abc").unwrap();
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("abcdef"));
        assert!(!pattern.is_match("xabc"));

        // End anchor
        let pattern = RegexPattern::new("abc$").unwrap();
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("xabc"));
        assert!(!pattern.is_match("abcx"));

        // Both anchors
        let pattern = RegexPattern::new("^abc$").unwrap();
        assert!(pattern.is_match("abc"));
        assert!(!pattern.is_match("abcx"));
        assert!(!pattern.is_match("xabc"));
    }

    #[test]
    fn test_find() {
        let pattern = RegexPattern::new(r"\d+").unwrap();
        let m = pattern.find("abc123def").unwrap();
        assert_eq!(m.text, "123");
        assert_eq!(m.start, 3);
        assert_eq!(m.end, 6);
    }

    #[test]
    fn test_find_all() {
        let pattern = RegexPattern::new(r"\d+").unwrap();
        let matches = pattern.find_all("abc123def456");
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].text, "123");
        assert_eq!(matches[1].text, "456");
    }

    #[test]
    fn test_replace_all() {
        let pattern = RegexPattern::new(r"\d+").unwrap();
        let result = pattern.replace_all("abc123def456", "NUM");
        assert_eq!(result, "abcNUMdefNUM");
    }

    #[test]
    fn test_replace_all_with_closure() {
        let pattern = RegexPattern::new(r"(\w+) (\d+)").unwrap();
        let result = pattern.replace_all_with("name1 123, name2 456", |caps| {
            format!("{} - {}", &caps[1], &caps[2])
        });
        assert_eq!(result, "name1 - 123, name2 - 456");
    }

    #[test]
    fn test_split() {
        let pattern = RegexPattern::new(r"\d+").unwrap();
        let result = pattern.split("abc123def456ghi");
        assert_eq!(result, vec!["abc", "def", "ghi"]);
    }

    #[test]
    fn test_complex_regex() {
        let pattern = RegexPattern::new(r"a(b+)?c").unwrap();
        assert!(pattern.is_match("ac"));
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("abbbc"));
        let m = pattern.find("axabbbcy").unwrap();
        assert_eq!(m.text, "abbbc");
    }

    #[test]
    fn test_invalid_pattern() {
        let result = RegexPattern::new("[");
        assert!(result.is_err());
        match result.unwrap_err() {
            PatternError::RegexError(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
