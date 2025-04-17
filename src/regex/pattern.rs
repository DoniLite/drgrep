//! # Module Pattern
//!
//! Provides basic pattern matching functionality without external dependencies.
//! This implementation supports a limited subset of regular expression features.
//!
//! Supported patterns:
//! - Literal characters (e.g., "abc")
//! - Dot (.) for any character
//! - Predefined character classes:
//!   - \d - Digits
//!   - \w - Alphanumeric characters + underscore
//!   - \s - Whitespace
//! - Negation of classes: \D, \W, \S
//! - Simple quantifiers:
//!   - * - Zero or more
//!   - + - One or more
//!   - ? - Zero or one
//! - Anchors:
//!   - ^ - Start of line
//!   - $ - End of line
//! - Escaping with \

use std::error::Error;
use std::fmt;

/// Errors specific to pattern matching operations
#[derive(Debug)]
pub enum PatternError {
    /// Syntax error in the pattern
    InvalidPattern(String),
    /// Other errors
    Other(String),
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternError::InvalidPattern(msg) => write!(f, "Invalid pattern: {}", msg),
            PatternError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl Error for PatternError {}

/// Pattern element type
#[derive(Debug, Clone, PartialEq)]
enum PatternElement {
    Literal(char),
    AnyChar,
    Digit(bool),      // bool indicates negation
    Word(bool),       // bool indicates negation
    Whitespace(bool), // bool indicates negation
    StartAnchor,
    EndAnchor,
    Quantifier(Box<PatternElement>, QuantifierType),
}

/// Quantifier types
#[derive(Debug, Clone, PartialEq)]
enum QuantifierType {
    ZeroOrMore,
    OneOrMore,
    ZeroOrOne,
}

/// Main structure for pattern matching
#[derive(Debug)]
pub struct SimplePattern {
    elements: Vec<PatternElement>,
    pattern: String,
}

/// Result of a match
#[derive(Debug, Clone)]
pub struct Match {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

impl SimplePattern {
    /// Creates a new pattern from a pattern string
    pub fn new(pattern: &str) -> Result<Self, PatternError> {
        let mut elements = Vec::new();
        let mut chars = pattern.chars().peekable();

        let mut is_start_anchor = false;
        let mut is_end_anchor = false;

        // Check for start anchor
        if let Some('^') = chars.peek() {
            is_start_anchor = true;
            chars.next();
        }

        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    // Escape character
                    match chars.next() {
                        Some('d') => elements.push(PatternElement::Digit(false)),
                        Some('D') => elements.push(PatternElement::Digit(true)),
                        Some('w') => elements.push(PatternElement::Word(false)),
                        Some('W') => elements.push(PatternElement::Word(true)),
                        Some('s') => elements.push(PatternElement::Whitespace(false)),
                        Some('S') => elements.push(PatternElement::Whitespace(true)),
                        Some(escaped) => elements.push(PatternElement::Literal(escaped)),
                        None => {
                            return Err(PatternError::InvalidPattern(
                                "Incomplete escape sequence".to_string(),
                            ))
                        }
                    }
                }
                '.' => elements.push(PatternElement::AnyChar),
                '$' => {
                    // If $ is the last character, it's an end anchor
                    if chars.peek().is_none() {
                        is_end_anchor = true;
                    } else {
                        elements.push(PatternElement::Literal('$'));
                    }
                }
                '*' | '+' | '?' => {
                    // Quantifiers
                    if elements.is_empty() {
                        return Err(PatternError::InvalidPattern(format!(
                            "Quantifier '{}' without preceding element",
                            c
                        )));
                    }

                    let last_elem = elements.pop().unwrap();
                    let quantifier_type = match c {
                        '*' => QuantifierType::ZeroOrMore,
                        '+' => QuantifierType::OneOrMore,
                        '?' => QuantifierType::ZeroOrOne,
                        _ => unreachable!(),
                    };

                    elements.push(PatternElement::Quantifier(
                        Box::new(last_elem),
                        quantifier_type,
                    ));
                }
                _ => elements.push(PatternElement::Literal(c)),
            }
        }

        let mut result = SimplePattern {
            elements,
            pattern: pattern.to_string(),
        };

        if is_start_anchor {
            result.elements.insert(0, PatternElement::StartAnchor);
        }

        if is_end_anchor {
            result.elements.push(PatternElement::EndAnchor);
        }

        Ok(result)
    }

    /// Returns the original pattern string
    pub fn get_pattern(&self) -> &str {
        self.pattern.as_str()
    }

    /// Checks if the text matches the pattern
    pub fn is_match(&self, text: &str) -> bool {
        self.find(text).is_some()
    }

    /// Finds the first match in the text
    pub fn find(&self, text: &str) -> Option<Match> {
        // Optimization for patterns with start anchor
        if self.elements.first() == Some(&PatternElement::StartAnchor) {
            return self.find_from(text, 0);
        }

        for start in 0..text.len() {
            if let Some(m) = self.find_from(text, start) {
                return Some(m);
            }
        }
        None
    }

    /// Finds all matches in the text
    pub fn find_all(&self, text: &str) -> Vec<Match> {
        let mut results = Vec::new();
        let mut start_pos = 0;

        while start_pos <= text.len() {
            if let Some(m) = self.find_from(text, start_pos) {
                results.push(m.clone());
                start_pos = if m.end > m.start { m.end } else { m.start + 1 };
            } else {
                start_pos += 1;
            }
        }

        results
    }

    /// Finds a match from a specific position
    fn find_from(&self, text: &str, start_pos: usize) -> Option<Match> {
        let text_chars: Vec<char> = text.chars().collect();
        let has_start_anchor = self.elements.first() == Some(&PatternElement::StartAnchor);
        let has_end_anchor = self.elements.last() == Some(&PatternElement::EndAnchor);
        let actual_elements = {
            let mut elements = self.elements.as_slice();
            if has_start_anchor {
                elements = &elements[1..];
            }
            if has_end_anchor {
                elements = &elements[..elements.len() - 1];
            }
            elements
        };

        if has_start_anchor
            && start_pos > 0
            && (start_pos >= text_chars.len() || text_chars[start_pos - 1] != '\n')
        {
            return None;
        }

        let mut current_pos = start_pos;
        let mut elem_pos = 0;
        let match_start = start_pos;

        while elem_pos < actual_elements.len() && current_pos <= text_chars.len() {
            let element = &actual_elements[elem_pos];

            match element {
                PatternElement::Quantifier(elem, quantifier_type) => {
                    match quantifier_type {
                        QuantifierType::ZeroOrMore => {
                            // Try to match as much as possible
                            while current_pos < text_chars.len()
                                && Self::matches_element(elem, text_chars[current_pos])
                            {
                                current_pos += 1;
                            }
                        }
                        QuantifierType::OneOrMore => {
                            let start_count = current_pos;
                            while current_pos < text_chars.len()
                                && Self::matches_element(elem, text_chars[current_pos])
                            {
                                current_pos += 1;
                            }
                            if current_pos == start_count {
                                return None; // Need at least one match
                            }
                        }
                        QuantifierType::ZeroOrOne => {
                            if current_pos < text_chars.len()
                                && Self::matches_element(elem, text_chars[current_pos])
                            {
                                current_pos += 1;
                            }
                        }
                    }
                }
                _ => {
                    if current_pos >= text_chars.len()
                        || !Self::matches_element(element, text_chars[current_pos])
                    {
                        return None;
                    }
                    current_pos += 1;
                }
            }
            elem_pos += 1;
        }

        // Check end anchor AFTER processing elements
        // Check end anchor condition AFTER matching elements
        if has_end_anchor // If the pattern requires an end anchor
        && !( // Check if we are NOT at a valid end position
           current_pos == text_chars.len() // Valid if we reached the end of the text
           || (current_pos < text_chars.len() && text_chars[current_pos] == '\n') // Valid if the next char is newline
        ) {
            return None;
        }

        let actual_start = text
            .char_indices()
            .nth(match_start)
            .map_or(0, |(idx, _)| idx);
        let actual_end = if current_pos >= text_chars.len() {
            text.len()
        } else {
            text.char_indices()
                .nth(current_pos)
                .map_or(text.len(), |(idx, _)| idx)
        };

        Some(Match {
            text: text[actual_start..actual_end].to_string(),
            start: actual_start,
            end: actual_end,
        })
    }

    /// Checks if a character matches a pattern element
    fn matches_element(element: &PatternElement, c: char) -> bool {
        match element {
            PatternElement::Literal(expected) => c == *expected,
            PatternElement::AnyChar => true,
            PatternElement::Digit(negate) => {
                let is_digit = c.is_ascii_digit();
                if *negate {
                    !is_digit
                } else {
                    is_digit
                }
            }
            PatternElement::Word(negate) => {
                let is_word = c.is_alphanumeric() || c == '_';
                if *negate {
                    !is_word
                } else {
                    is_word
                }
            }
            PatternElement::Whitespace(negate) => {
                let is_ws = c.is_whitespace();
                if *negate {
                    !is_ws
                } else {
                    is_ws
                }
            }
            PatternElement::StartAnchor => false, // Should not be tested directly
            PatternElement::EndAnchor => false,   // Should not be tested directly
            PatternElement::Quantifier(_, _) => {
                panic!("Quantifiers cannot be tested directly")
            }
        }
    }

    /// Replaces all occurrences of the pattern with the replacement string
    pub fn replace_all(&self, text: &str, replacement: &str) -> String {
        let matches = self.find_all(text);

        if matches.is_empty() {
            return text.to_string();
        }

        let mut result = String::new();
        let mut last_end = 0;

        for m in matches {
            // Add text between the last match and this one
            result.push_str(&text[last_end..m.start]);
            // Add replacement
            result.push_str(replacement);
            last_end = m.end;
        }

        // Add the rest of the text
        result.push_str(&text[last_end..]);
        result
    }

    /// Splits the text according to the pattern
    pub fn split(&self, text: &str) -> Vec<String> {
        let matches = self.find_all(text);

        if matches.is_empty() {
            return vec![text.to_string()];
        }

        let mut result = Vec::new();
        let mut last_end = 0;

        for m in matches {
            if m.start > last_end {
                result.push(text[last_end..m.start].to_string());
            } else if last_end == 0 {
                // If match is at start, add empty string
                result.push(String::new());
            }

            last_end = m.end;
        }

        // Add the rest of the text
        if last_end < text.len() {
            result.push(text[last_end..].to_string());
        } else {
            // If the last match is at the end, add empty string
            result.push(String::new());
        }

        result
    }
}

// Utility functions
pub fn is_match(pattern: &str, text: &str) -> Result<bool, PatternError> {
    let p = SimplePattern::new(pattern)?;
    Ok(p.is_match(text))
}

pub fn find(pattern: &str, text: &str) -> Result<Option<Match>, PatternError> {
    let p = SimplePattern::new(pattern)?;
    Ok(p.find(text))
}

pub fn find_all(pattern: &str, text: &str) -> Result<Vec<Match>, PatternError> {
    let p = SimplePattern::new(pattern)?;
    Ok(p.find_all(text))
}

pub fn replace_all(pattern: &str, text: &str, replacement: &str) -> Result<String, PatternError> {
    let p = SimplePattern::new(pattern)?;
    Ok(p.replace_all(text, replacement))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_match() {
        let pattern = SimplePattern::new("abc").unwrap();
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("xabcy"));
        assert!(!pattern.is_match("ab"));
    }

    #[test]
    fn test_any_char() {
        let pattern = SimplePattern::new("a.c").unwrap();
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("axc"));
        assert!(!pattern.is_match("ac"));
    }

    #[test]
    fn test_digit() {
        let pattern = SimplePattern::new("a\\dc").unwrap();
        assert!(pattern.is_match("a1c"));
        assert!(pattern.is_match("a9c"));
        assert!(!pattern.is_match("abc"));

        let pattern = SimplePattern::new("a\\Dc").unwrap();
        assert!(!pattern.is_match("a1c"));
        assert!(pattern.is_match("abc"));
    }

    #[test]
    fn test_quantifiers() {
        // Zero or more
        let pattern = SimplePattern::new("ab*c").unwrap();
        assert!(pattern.is_match("ac"));
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("abbc"));

        // One or more
        let pattern = SimplePattern::new("ab+c").unwrap();
        assert!(!pattern.is_match("ac"));
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("abbc"));

        // Zero or one
        let pattern = SimplePattern::new("ab?c").unwrap();
        assert!(pattern.is_match("ac"));
        assert!(pattern.is_match("abc"));
        assert!(!pattern.is_match("abbc"));
    }

    #[test]
    fn test_anchors() {
        // Start anchor
        let pattern = SimplePattern::new("^abc").unwrap();
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("abcdef"));
        assert!(!pattern.is_match("xabc"));

        // End anchor
        let pattern = SimplePattern::new("abc$").unwrap();
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("xabc"));
        assert!(!pattern.is_match("abcx"));

        // Both anchors
        let pattern = SimplePattern::new("^abc$").unwrap();
        assert!(pattern.is_match("abc"));
        assert!(!pattern.is_match("abcx"));
        assert!(!pattern.is_match("xabc"));
    }

    #[test]
    fn test_find() {
        let pattern = SimplePattern::new("\\d+").unwrap();
        let m = pattern.find("abc123def").unwrap();
        assert_eq!(m.text, "123");
        assert_eq!(m.start, 3);
        assert_eq!(m.end, 6);
    }

    #[test]
    fn test_find_all() {
        let pattern = SimplePattern::new("\\d+").unwrap();
        let matches = pattern.find_all("abc123def456");
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].text, "123");
        assert_eq!(matches[1].text, "456");
    }

    #[test]
    fn test_replace_all() {
        let pattern = SimplePattern::new("\\d+").unwrap();
        let result = pattern.replace_all("abc123def456", "NUM");
        assert_eq!(result, "abcNUMdefNUM");
    }

    #[test]
    fn test_split() {
        let pattern = SimplePattern::new("\\d+").unwrap();
        let result = pattern.split("abc123def456ghi");
        assert_eq!(result, vec!["abc", "def", "ghi"]);
    }
}
