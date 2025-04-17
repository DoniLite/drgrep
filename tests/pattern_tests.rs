// Location: tests/pattern_test.rs

use drgrep::regex::pattern::{self, PatternError, SimplePattern};

#[test]
fn test_pattern_creation() {
    // Valid patterns
    assert!(SimplePattern::new("abc").is_ok());
    assert!(SimplePattern::new("a.c").is_ok());
    assert!(SimplePattern::new("a\\dc").is_ok());
    assert!(SimplePattern::new("a\\w+").is_ok());

    assert!(SimplePattern::new("a*").is_ok()); // "a*" is valid

    // Invalid patterns (quantifier at the beginning)
    let result_invalid = SimplePattern::new("*a");
    assert!(result_invalid.is_err(), "Pattern '*a' should be invalid");
    if let Err(PatternError::InvalidPattern(msg)) = result_invalid {
        assert!(
            msg.contains("Quantifier '*' without preceding element"),
            "Incorrect error message for '*a'"
        );
    } else {
        panic!(
            "Expected InvalidPattern error for '*a', got {:?}",
            result_invalid
        );
    }
    let result_invalid_plus = SimplePattern::new("+a");
    assert!(
        result_invalid_plus.is_err(),
        "Pattern '+a' should be invalid"
    );
    if let Err(PatternError::InvalidPattern(msg)) = result_invalid_plus {
        assert!(
            msg.contains("Quantifier '+' without preceding element"),
            "Incorrect error message for '+a'"
        );
    } else {
        panic!(
            "Expected InvalidPattern error for '+a', got {:?}",
            result_invalid_plus
        );
    }
}

#[test]
fn test_pattern_matching_features() {
    // Test different pattern features
    let text = "Hello123 World_456";

    // Word boundaries
    assert!(pattern::is_match("\\w+", text).unwrap());

    // Digits
    let digit_matches = pattern::find_all("\\d+", text).unwrap();
    assert_eq!(digit_matches.len(), 2);
    assert_eq!(digit_matches[0].text, "123");
    assert_eq!(digit_matches[1].text, "456");

    // Words with underscore
    let word_matches = pattern::find_all("\\w+", text).unwrap();
    assert_eq!(word_matches.len(), 2);
    assert_eq!(word_matches[0].text, "Hello123");
    assert_eq!(word_matches[1].text, "World_456");

    // Combined patterns
    let combined = pattern::find("\\w+\\s\\w+", text).unwrap();
    assert!(combined.is_some());
    assert_eq!(combined.unwrap().text, "Hello123 World_456");
}

#[test]
fn test_replace_functionality() {
    let text = "Version 1.2.3 was released on 2023-04-17";

    // Replace version numbers
    let result = pattern::replace_all("\\d+\\.\\d+\\.\\d+", text, "X.Y.Z").unwrap();
    assert_eq!(result, "Version X.Y.Z was released on 2023-04-17");

    // Multiple replacements
    let result = pattern::replace_all("\\d+", text, "#").unwrap();
    assert_eq!(result, "Version #.#.# was released on #-#-#");
}

#[test]
fn test_split_functionality() {
    let text = "apple,banana,cherry";

    // Split by comma
    let pattern = SimplePattern::new(",").unwrap();
    let parts = pattern.split(text);
    assert_eq!(parts, vec!["apple", "banana", "cherry"]);

    // Split by digits
    let text = "abc123def456ghi";
    let pattern = SimplePattern::new("\\d+").unwrap();
    let parts = pattern.split(text);
    assert_eq!(parts, vec!["abc", "def", "ghi"]);

    // Split with anchors
    let text = "start\nmiddle\nend";
    let pattern = SimplePattern::new("^middle$").unwrap();
    let parts = pattern.split(text);
    assert_eq!(parts.len(), 2);
}

#[test]
fn test_complex_patterns() {
    // Email-like pattern
    let pattern = SimplePattern::new("\\w+@\\w+\\.\\w+").unwrap();
    let text = "Contact us at info@example.com or support@domain.org";

    let matches = pattern.find_all(text);
    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0].text, "info@example.com");
    assert_eq!(matches[1].text, "support@domain.org");

    // More complex pattern with multiple features
    let pattern = SimplePattern::new("^\\w+\\s\\d+$").unwrap(); // Word followed by space followed by digits
    assert!(pattern.is_match("Test 123"));
    assert!(!pattern.is_match("Test123"));
    assert!(!pattern.is_match(" Test 123"));
    assert!(!pattern.is_match("Test 123 "));
}

// Location: tests/pattern_tests.rs

#[test]
fn test_utility_functions() {
    let text = "The quick brown fox jumps over the lazy dog";

    // is_match
    assert!(pattern::is_match("fox", text).unwrap());
    assert!(!pattern::is_match("cat", text).unwrap());

    // find
  // NOTE: {n} quantifier is not supported. Using repeated \w instead.
   let result = pattern::find("\\w\\w\\w\\w\\w", text).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().text, "quick");

    // find_all
    let results = pattern::find_all("the", text).unwrap(); // Note: case-sensitive by default
    assert_eq!(results.len(), 1); // Only finds "the", not "The"

    // Case sensitivity test adjusted
    assert!(pattern::is_match("The", text).unwrap()); // Matches "The"
   let find_lower_the_in_upper = pattern::is_match("the", "The").unwrap();
  assert!(!find_lower_the_in_upper, "Should not find lowercase 'the' in 'The'");

   // Let's also check find_all for case sensitivity
   let results_the_lower = pattern::find_all("the", text).unwrap();
   assert_eq!(results_the_lower.len(), 1);
   assert_eq!(results_the_lower[0].text, "the");

   let results_the_upper = pattern::find_all("The", text).unwrap();
   assert_eq!(results_the_upper.len(), 1);
   assert_eq!(results_the_upper[0].text, "The");

}
