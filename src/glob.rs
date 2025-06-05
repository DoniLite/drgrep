//! # Enhanced Glob Pattern Matcher
//!
//! A comprehensive glob pattern matcher implementation in Rust.
//! This module provides functionality to match strings against glob patterns
//! with support for:
//! - `*` (matches any sequence of characters)
//! - `?` (matches any single character)
//! - `[abc]` (matches any character in the set)
//! - `[!abc]` (matches any character not in the set)
//! - `{a,b,c}` (matches any of the comma-separated patterns)
//! - File system traversal to find matching files

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a compiled glob pattern for efficient matching.
#[derive(Debug, Clone)]
pub struct GlobPattern {
    pattern: String,
    components: Vec<Component>,
    expanded_patterns: Vec<String>, // For alternatives
}

/// Components that make up a glob pattern.
#[derive(Debug, Clone)]
enum Component {
    /// Matches a literal string
    Literal(String),
    /// Matches any single character
    SingleWildcard,
    /// Matches any sequence of characters (including empty)
    MultiWildcard,
    /// Matches any character in the set
    CharacterClass { chars: HashSet<char>, negated: bool },
    // Matches any of the comma-separated patterns
    // Alternatives(Vec<String>),
}

impl GlobPattern {
    /// Creates a new `GlobPattern` instance from a pattern string.
    ///
    /// # Arguments
    ///
    /// * `pattern` - A string slice containing the glob pattern
    ///
    /// # Examples
    ///
    /// ```
    /// use drgrep::glob::GlobPattern;
    ///
    /// let pattern = GlobPattern::new("*.rs");
    /// assert!(pattern.matches("file.rs"));
    /// assert!(!pattern.matches("file.txt"));
    /// ```
    pub fn new(pattern: &str) -> Self {
        let pattern_string = pattern.to_string();
        let (components, expanded_patterns) = Self::parse(&pattern_string);

        GlobPattern {
            pattern: pattern_string,
            components,
            expanded_patterns,
        }
    }

    /// Parse a glob pattern string into components and expanded alternative patterns.
    fn parse(pattern: &str) -> (Vec<Component>, Vec<String>) {
        // First, check if we have any braces that indicate alternatives
        if pattern.contains('{') && pattern.contains('}') {
            let expanded = Self::expand_alternatives(pattern);
            return (Vec::new(), expanded);
        }

        // If no alternatives, proceed with normal parsing
        let mut components = Vec::new();
        let mut current_literal = String::new();
        let chars: Vec<char> = pattern.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '*' => {
                    if !current_literal.is_empty() {
                        components.push(Component::Literal(current_literal));
                        current_literal = String::new();
                    }
                    components.push(Component::MultiWildcard);
                    i += 1;
                }
                '?' => {
                    if !current_literal.is_empty() {
                        components.push(Component::Literal(current_literal));
                        current_literal = String::new();
                    }
                    components.push(Component::SingleWildcard);
                    i += 1;
                }
                '[' => {
                    if !current_literal.is_empty() {
                        components.push(Component::Literal(current_literal));
                        current_literal = String::new();
                    }

                    let (char_class, new_pos) = Self::parse_character_class(&chars, i + 1);
                    components.push(char_class);
                    i = new_pos;
                }
                '\\' => {
                    // Escape the next character
                    if i + 1 < chars.len() {
                        current_literal.push(chars[i + 1]);
                        i += 2;
                    } else {
                        current_literal.push('\\');
                        i += 1;
                    }
                }
                _ => {
                    current_literal.push(chars[i]);
                    i += 1;
                }
            }
        }

        if !current_literal.is_empty() {
            components.push(Component::Literal(current_literal));
        }

        (components, Vec::new())
    }

    /// Expand alternatives in a pattern like {a,b} to multiple patterns
    fn expand_alternatives(pattern: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();

        // First, identify top-level alternative groups (not nested)
        let mut i = 0;
        let chars: Vec<char> = pattern.chars().collect();

        while i < chars.len() {
            match chars[i] {
                '{' => {
                    // Find the matching closing brace
                    let start_pos = i;
                    let mut brace_count = 1;
                    let mut end_pos = start_pos;

                    for j in (start_pos + 1)..chars.len() {
                        match chars[j] {
                            '{' => brace_count += 1,
                            '}' => {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    end_pos = j;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }

                    if brace_count != 0 || end_pos == start_pos {
                        // Malformed pattern, treat as literal
                        current.push('{');
                        i += 1;
                        continue;
                    }

                    // Extract the options within braces
                    let options_str = &pattern[start_pos + 1..end_pos];

                    // Split by commas, but respect nested braces
                    let options = Self::split_respecting_braces(options_str);

                    // Recursive expansion for each option
                    let prefix = current.clone();
                    let suffix = &pattern[end_pos + 1..];

                    // If this is the first alternative, initialize result
                    if result.is_empty() {
                        for option in options {
                            // Recursively expand any alternatives in this option
                            let expanded_option = Self::expand_alternatives(&format!(
                                "{}{}{}",
                                prefix, option, suffix
                            ));
                            result.extend(expanded_option);
                        }
                    } else {
                        // For subsequent alternatives, apply each option to every existing result
                        let mut new_results = Vec::new();

                        for existing in &result {
                            for option in &options {
                                // Recursively expand
                                let expanded_option =
                                    Self::expand_alternatives(&format!("{}{}", option, suffix));
                                for exp in expanded_option {
                                    new_results.push(format!("{}{}", existing.clone(), exp));
                                }
                            }
                        }

                        if !new_results.is_empty() {
                            result = new_results;
                        }
                    }

                    // Since we've processed this alternative completely, we can return
                    return result;
                }
                '\\' => {
                    // Escape the next character
                    if i + 1 < chars.len() {
                        current.push(chars[i + 1]);
                        i += 2;
                    } else {
                        current.push('\\');
                        i += 1;
                    }
                }
                _ => {
                    current.push(chars[i]);
                    i += 1;
                }
            }
        }

        // If we get here with no expansion, add the literal pattern
        if result.is_empty() {
            result.push(current);
        }

        result
    }

    /// Split a string by commas, but respect nested braces
    fn split_respecting_braces(s: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut brace_count = 0;

        for c in s.chars() {
            match c {
                '{' => {
                    brace_count += 1;
                    current.push(c);
                }
                '}' => {
                    brace_count -= 1;
                    current.push(c);
                }
                ',' if brace_count == 0 => {
                    result.push(current);
                    current = String::new();
                }
                _ => current.push(c),
            }
        }

        if !current.is_empty() {
            result.push(current);
        }

        result
    }

    /// Parse a character class like [abc] or [!abc]
    fn parse_character_class(chars: &[char], start: usize) -> (Component, usize) {
        let mut i = start;
        let mut char_set = HashSet::new();
        let mut negated = false;

        // Check if the class is negated
        if i < chars.len() && chars[i] == '!' {
            negated = true;
            i += 1;
        }

        // Parse the class content
        while i < chars.len() && chars[i] != ']' {
            if i + 2 < chars.len() && chars[i + 1] == '-' {
                // Character range like a-z
                let start_char = chars[i];
                let end_char = chars[i + 2];

                for c in start_char..=end_char {
                    char_set.insert(c);
                }

                i += 3;
            } else {
                char_set.insert(chars[i]);
                i += 1;
            }
        }

        // Skip closing bracket
        if i < chars.len() {
            i += 1;
        }

        (
            Component::CharacterClass {
                chars: char_set,
                negated,
            },
            i,
        )
    }

    /// Returns the original pattern string.
    pub fn as_str(&self) -> &str {
        &self.pattern
    }

    /// Checks if a string matches the glob pattern.
    ///
    /// # Arguments
    ///
    /// * `text` - The string to match against the pattern
    ///
    /// # Returns
    ///
    /// `true` if the string matches the pattern, `false` otherwise.
    pub fn matches(&self, text: &str) -> bool {
        // If we have expanded alternatives, check each of them
        if !self.expanded_patterns.is_empty() {
            return self
                .expanded_patterns
                .iter()
                .any(|pat| GlobPattern::new(pat).matches(text));
        }

        // Otherwise use the normal matching algorithm
        self.matches_components(text, &self.components, 0)
    }

    /// Match text against components starting from a position.
    fn matches_components(&self, text: &str, components: &[Component], text_pos: usize) -> bool {
        let text_chars: Vec<char> = text.chars().collect();

        self.matches_from_position(text, &text_chars, components, 0, text_pos)
    }

    /// Recursive helper function to match text from a specific position.
    fn matches_from_position(
        &self,
        text: &str,
        text_chars: &[char],
        components: &[Component],
        component_idx: usize,
        text_pos: usize,
    ) -> bool {
        // If we've reached the end of both the pattern and the text, it's a match
        if component_idx >= components.len() {
            return text_pos >= text_chars.len();
        }

        // If we've reached the end of the text but not the pattern,
        // it's only a match if the rest of the pattern can match empty string
        if text_pos >= text_chars.len() {
            // Special cases for components that can match empty strings
            match &components[component_idx] {
                Component::MultiWildcard => {
                    return self.matches_from_position(
                        text,
                        text_chars,
                        components,
                        component_idx + 1,
                        text_pos,
                    );
                }
                _ => {
                    // Check if the rest of the pattern consists only of multi-wildcards
                    for i in component_idx..components.len() {
                        if !matches!(components[i], Component::MultiWildcard) {
                            return false;
                        }
                    }
                    return true;
                }
            }
        }

        match &components[component_idx] {
            Component::Literal(lit) => {
                let lit_chars: Vec<char> = lit.chars().collect();

                if text_pos + lit_chars.len() > text_chars.len() {
                    return false;
                }

                for (i, &lit_char) in lit_chars.iter().enumerate() {
                    if text_chars[text_pos + i] != lit_char {
                        return false;
                    }
                }

                // Move past this literal in both pattern and text
                self.matches_from_position(
                    text,
                    text_chars,
                    components,
                    component_idx + 1,
                    text_pos + lit_chars.len(),
                )
            }
            Component::SingleWildcard => {
                // ? matches exactly one character, so advance both
                self.matches_from_position(
                    text,
                    text_chars,
                    components,
                    component_idx + 1,
                    text_pos + 1,
                )
            }
            Component::MultiWildcard => {
                // * can match any number of characters (including zero)

                // Option 1: * matches nothing, move to next component
                if self.matches_from_position(
                    text,
                    text_chars,
                    components,
                    component_idx + 1,
                    text_pos,
                ) {
                    return true;
                }

                // Option 2: * matches the current character, try again at next position
                self.matches_from_position(
                    text,
                    text_chars,
                    components,
                    component_idx,
                    text_pos + 1,
                )
            }
            Component::CharacterClass { chars, negated } => {
                let matches_class = chars.contains(&text_chars[text_pos]) != *negated;

                if matches_class {
                    self.matches_from_position(
                        text,
                        text_chars,
                        components,
                        component_idx + 1,
                        text_pos + 1,
                    )
                } else {
                    false
                }
            } // Component::Alternatives(alternatives) => {
              //     // This shouldn't be reached because we're expanding alternatives before matching
              //     // But just in case, implement a simple matching
              //     for alt in alternatives {
              //         if GlobPattern::new(alt).matches(&text[text_pos..]) {
              //             return true;
              //         }
              //     }
              //     false
              // }
        }
    }

    /// Find all files in a directory that match the glob pattern.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The base directory to start searching from
    ///
    /// # Returns
    ///
    /// A vector of PathBuf instances that match the pattern.
    pub fn find_files<P: AsRef<Path>>(&self, base_dir: P) -> std::io::Result<Vec<PathBuf>> {
        let mut result = Vec::new();

        // If we have expanded alternatives, search with each pattern
        if !self.expanded_patterns.is_empty() {
            for pattern in &self.expanded_patterns {
                let pattern_glob = GlobPattern::new(pattern);
                let files = pattern_glob.find_files(base_dir.as_ref())?;
                for file in files {
                    if !result.contains(&file) {
                        result.push(file);
                    }
                }
            }
            return Ok(result);
        }

        // Otherwise use normal search
        self.find_files_recursive(base_dir.as_ref(), &mut result)?;
        Ok(result)
    }

    /// Recursively search for files matching the pattern.
    fn find_files_recursive(&self, dir: &Path, results: &mut Vec<PathBuf>) -> std::io::Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // Convert path to string for pattern matching
            if let Some(path_str) = path.to_str() {
                // Check if the path matches our pattern
                if self.matches(path_str) {
                    results.push(path.clone());
                }
            }

            // Recursively search directories
            if path.is_dir() {
                self.find_files_recursive(&path, results)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_simple_literals() {
        let pattern = GlobPattern::new("hello");
        assert!(pattern.matches("hello"));
        assert!(!pattern.matches("world"));
        assert!(!pattern.matches("hello world"));
    }

    #[test]
    fn test_single_wildcard() {
        let pattern = GlobPattern::new("h?llo");
        assert!(pattern.matches("hello"));
        assert!(pattern.matches("hallo"));
        assert!(!pattern.matches("hllo"));
        assert!(!pattern.matches("helloo"));
    }

    #[test]
    fn test_multi_wildcard() {
        let pattern = GlobPattern::new("h*o");
        assert!(pattern.matches("hello"));
        assert!(pattern.matches("ho"));
        assert!(pattern.matches("hello world hello"));
        assert!(!pattern.matches("world"));
    }

    #[test]
    fn test_character_class() {
        let pattern = GlobPattern::new("h[ae]llo");
        assert!(pattern.matches("hello"));
        assert!(pattern.matches("hallo"));
        assert!(!pattern.matches("hillo"));
        assert!(!pattern.matches("hllo"));

        let pattern = GlobPattern::new("h[a-z]llo");
        assert!(pattern.matches("hello"));
        assert!(pattern.matches("hallo"));
        assert!(pattern.matches("hzllo"));
        assert!(!pattern.matches("h1llo"));

        let pattern = GlobPattern::new("h[!aeiou]llo");
        assert!(!pattern.matches("hello"));
        assert!(!pattern.matches("hallo"));
        assert!(pattern.matches("hbllo"));
        assert!(pattern.matches("hzllo"));
    }

    #[test]
    fn test_alternatives() {
        let pattern = GlobPattern::new("hello.{rs,txt,md}");
        assert!(pattern.matches("hello.rs"));
        assert!(pattern.matches("hello.txt"));
        assert!(pattern.matches("hello.md"));
        assert!(!pattern.matches("hello.go"));

        let pattern = GlobPattern::new("{src,tests}/*.rs");
        assert!(pattern.matches("src/main.rs"));
        assert!(pattern.matches("tests/test_utils.rs"));
        assert!(!pattern.matches("docs/readme.rs"));
    }

    #[test]
    fn test_nested_alternatives() {
        let pattern = GlobPattern::new("{src/{lib,bin},tests}/*.rs");
        assert!(pattern.matches("src/lib/utils.rs"));
        assert!(pattern.matches("src/bin/main.rs"));
        assert!(pattern.matches("tests/test_main.rs"));
        assert!(!pattern.matches("src/other/mod.rs"));
    }

    #[test]
    fn test_combined_patterns() {
        let pattern = GlobPattern::new("src/[a-z]*/{*.rs,*.toml}");
        assert!(pattern.matches("src/utils/mod.rs"));
        assert!(pattern.matches("src/config/settings.toml"));
        assert!(!pattern.matches("src/123/test.rs"));
        assert!(!pattern.matches("src/utils/test.js"));
    }

    #[test]
    fn test_edge_cases() {
        let pattern = GlobPattern::new("*");
        assert!(pattern.matches(""));
        assert!(pattern.matches("anything"));

        let pattern = GlobPattern::new("?");
        assert!(pattern.matches("a"));
        assert!(!pattern.matches(""));
        assert!(!pattern.matches("ab"));

        let pattern = GlobPattern::new("\\*");
        assert!(pattern.matches("*"));
        assert!(!pattern.matches("anything"));

        let empty_pattern = GlobPattern::new("");
        assert!(empty_pattern.matches(""));
        assert!(!empty_pattern.matches("anything"));
    }

    #[test]
    fn test_expand_alternatives() {
        // Test simple alternatives expansion
        let expanded = GlobPattern::expand_alternatives("a{b,c}d");
        assert_eq!(expanded, vec!["abd", "acd"]);

        // Test nested alternatives expansion
        let expanded = GlobPattern::expand_alternatives("{src/{lib,bin},tests}/*.rs");
        let expected = vec!["src/lib/*.rs", "src/bin/*.rs", "tests/*.rs"];

        for exp in &expected {
            assert!(expanded.contains(&exp.to_string()));
        }
        assert_eq!(expanded.len(), expected.len());
    }
}
