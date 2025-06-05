use drgrep::regex::pattern::{self, PatternError, RegexPattern};

#[test]
fn test_regex_pattern_creation() {
    // Valid patterns
    assert!(RegexPattern::new("abc").is_ok());
    assert!(RegexPattern::new("a.c").is_ok());
    assert!(RegexPattern::new(r"a\dc").is_ok());
    assert!(RegexPattern::new(r"a\w+").is_ok());
    assert!(RegexPattern::new(r"a*").is_ok()); // "*" is valid in regex

    // Invalid patterns for regex should still be caught
    let result_invalid = RegexPattern::new("[");
    assert!(
        result_invalid.is_err(),
        "Pattern '[' should be invalid regex"
    );
    if let Err(PatternError::RegexError(_)) = result_invalid {
        assert!(true); // Expected a RegexError
    } else {
        panic!("Expected RegexError for '[', got {:?}", result_invalid);
    }
}

#[test]
fn test_regex_pattern_matching_features() {
    // Test different regex pattern features
    let text = "Hello123 World_456";

    // Word boundaries
    assert!(pattern::is_match(r"\w+", text).unwrap());

    // Digits
    let digit_matches = pattern::find_all(r"\d+", text).unwrap();
    assert_eq!(digit_matches.len(), 2);
    assert_eq!(digit_matches[0].text, "123");
    assert_eq!(digit_matches[1].text, "456");

    // Words with underscore
    let word_matches = pattern::find_all(r"\w+", text).unwrap();
    assert_eq!(word_matches.len(), 2);
    assert_eq!(word_matches[0].text, "Hello123");
    assert_eq!(word_matches[1].text, "World_456");

    // Combined patterns
    let combined = pattern::find(r"\w+\s\w+", text).unwrap();
    assert!(combined.is_some());
    assert_eq!(combined.unwrap().text, "Hello123 World_456");
}

#[test]
fn test_regex_replace_functionality() {
    let text = "Version 1.2.3 was released on 2023-04-17";

    // Replace version numbers
    let result = pattern::replace_all(r"\d+\.\d+\.\d+", text, "X.Y.Z").unwrap();
    assert_eq!(result, "Version X.Y.Z was released on 2023-04-17");

    // Multiple replacements
    let result = pattern::replace_all(r"\d+", text, "#").unwrap();
    assert_eq!(result, "Version #.#.# was released on #-#-#");

    // Replace with capture groups - CORRECTION: Adapter le motif pour capturer les deux formats de valeurs
    let text_with_names = "Name: John Doe, Age: 30";
    let result_with_capture =
        pattern::replace_all(r"(\w+): (\w+(?:\s\w+)*)", text_with_names, "$1 is $2").unwrap();
    assert_eq!(result_with_capture, "Name is John Doe, Age is 30");

    // Replace with function
    let result_with_fn = pattern::replace_all_with(r"(\d+)", "Number: 10, Double: 20", |caps| {
        let num: i32 = caps[1].parse().unwrap();
        (num * 2).to_string()
    })
    .unwrap();
    assert_eq!(result_with_fn, "Number: 20, Double: 40");
}

#[test]
fn test_regex_split_functionality() {
    let text = "apple,banana,cherry";

    // Split by comma
    let parts = pattern::split(",", text).unwrap();
    assert_eq!(parts, vec!["apple", "banana", "cherry"]);

    // Split by digits
    let text = "abc123def456ghi";
    let parts = pattern::split(r"\d+", text).unwrap();
    assert_eq!(parts, vec!["abc", "def", "ghi"]);

    // Split with anchors (regex split on a match removes the match, so the result will be different)
    let text = "start\nmiddle\nend";
    let parts = pattern::split("middle", text).unwrap();
    assert_eq!(parts, vec!["start\n", "\nend"]);
}

#[test]
fn test_regex_complex_patterns() {
    // Email-like pattern
    let text = "Contact us at info@example.com or support@domain.org";

    let matches = pattern::find_all(r"\w+@\w+\.\w+", text).unwrap();
    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0].text, "info@example.com");
    assert_eq!(matches[1].text, "support@domain.org");

    // More complex pattern with multiple features
    assert!(pattern::is_match(r"^\w+\s\d+$", "Test 123").unwrap());
    assert!(!pattern::is_match(r"^\w+\s\d+$", "Test123").unwrap());
    assert!(!pattern::is_match(r"^\w+\s\d+$", " Test 123").unwrap());
    assert!(!pattern::is_match(r"^\w+\s\d+$", "Test 123 ").unwrap());
}

// Location: tests/pattern_tests.rs (This file seems redundant with the above, consider merging or removing)
// Keeping it for now based on your provided structure.

#[test]
fn test_regex_utility_functions() {
    let text = "The quick brown fox jumps over the lazy dog";

    // is_match
    assert!(pattern::is_match("fox", text).unwrap());
    assert!(!pattern::is_match("cat", text).unwrap());

    // find
    let result = pattern::find(r"\w{5}", text).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().text, "quick");

    // find_all
    let results = pattern::find_all("the", text).unwrap(); // Note: case-sensitive by default
    assert_eq!(results.len(), 1); // Only finds "the", not "The"

    // Case sensitivity test adjusted
    assert!(pattern::is_match("The", text).unwrap()); // Matches "The"
    let find_lower_the_in_upper = pattern::is_match("the", "The").unwrap();
    assert!(
        !find_lower_the_in_upper,
        "Should not find lowercase 'the' in 'The'"
    );

    // Let's also check find_all for case sensitivity
    let results_the_lower = pattern::find_all("the", text).unwrap();
    assert_eq!(results_the_lower.len(), 1);
    assert_eq!(results_the_lower[0].text, "the");

    let results_the_upper = pattern::find_all("The", text).unwrap();
    assert_eq!(results_the_upper.len(), 1);
    assert_eq!(results_the_upper[0].text, "The");
}
