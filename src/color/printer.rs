//! # Text Printer Module
//!
//! This module provides functionality for printing colored and styled text to the terminal.
//! It leverages the color configurations from the `color::config` module and offers both
//! function-based and macro-based interfaces for convenience.
//!
//! ## Features
//!
//! - Print entire lines with a single color
//! - Print text with both style (bold, underline) and color
//! - Print multiple text segments with different colors in a single line
//! - Macros for simplified importing and usage
//!
//! ## Usage Examples
//!
//! ```rust
//! use drgrep::color::config::Color;
//! use drgrep::color::printer::{print_colored, print_styled, print_partial_colored};
//!
//! // Print a line with a single color
//! print_colored("This is an error message", Color::RED);
//!
//! // Print text with both style and color
//! print_styled("Important warning", Color::BOLD, Color::YELLOW);
//!
//! // Print multiple text segments with different colors
//! let parts = vec![
//!     ("Success:", Color::GREEN),
//!     ("File processed", Color::WHITE)
//! ];
//! print_partial_colored(&parts);
//!
//! // Using macros (requires importing them)
//! drgrep::print_colored!("Error detected", Color::RED);
//! ```

use crate::color::config::Color;

/// Type alias for text parts with their associated colors
///
/// Each element in the vector is a tuple containing:
/// - The text segment to be printed
/// - The color or style to apply to that segment
pub type TextParts<'a> = &'a Vec<(&'a str, &'a str)>;

/// Prints text in a specified color
///
/// This function prints the provided text in the specified color and
/// automatically resets the color after printing.
///
/// # Arguments
///
/// * `text` - The text to print
/// * `color` - The color to apply (from `Color` constants)
///
/// # Examples
///
/// ```
/// use drgrep::color::config::Color;
/// use drgrep::color::printer::print_colored;
///
/// print_colored("Success!", Color::GREEN);
/// print_colored("Error: File not found", Color::RED);
/// ```
pub fn print_colored(text: &str, color: &str) {
    println!("{}{}{}", color, text, Color::RESET);
}

/// Prints text with both style and color
///
/// This function applies both a text style (like bold or underline)
/// and a color to the provided text, then prints it.
///
/// # Arguments
///
/// * `text` - The text to print
/// * `style` - The style to apply (from `Color` constants)
/// * `color` - The color to apply (from `Color` constants)
///
/// # Examples
///
/// ```
/// use drgrep::color::config::Color;
/// use drgrep::color::printer::print_styled;
///
/// print_styled("Important warning", Color::BOLD, Color::YELLOW);
/// print_styled("Critical error", Color::BOLD, Color::RED);
/// ```
pub fn print_styled(text: &str, style: &str, color: &str) {
    println!("{}{}{}{}", style, color, text, Color::RESET);
}

/// Prints multiple text segments with different colors on a single line
///
/// This function takes a vector of (text, color) pairs and prints each segment
/// with its associated color on the same line, with a space between segments.
///
/// # Arguments
///
/// * `parts` - A reference to a vector of tuples, each containing text and its color
///
/// # Examples
///
/// ```
/// use drgrep::color::config::Color;
/// use drgrep::color::printer::print_partial_colored;
///
/// let parts = vec![
///     ("File:", Color::BLUE),
///     ("example.txt", Color::WHITE),
///     ("Status:", Color::BLUE),
///     ("Found", Color::GREEN)
/// ];
/// print_partial_colored(&parts);
/// ```
pub fn print_partial_colored(parts: TextParts) {
    for (text, color) in parts {
        print!("{}{}{} ", color, text, Color::RESET);
    }
    println!(); // Add newline at the end
}

/// Macro for printing colored text
///
/// This macro provides a convenient shorthand for calling the `print_colored` function.
///
/// # Examples
///
/// ```
/// use drgrep::color::config::Color;
/// use drgrep::print_colored;
///
/// print_colored!("Error message", Color::RED);
/// ```
#[macro_export]
macro_rules! print_colored {
    ($($arg:tt)* ) => {{
        $crate::print_colored($($arg)*)
    }};
}

/// Macro for printing styled and colored text
///
/// This macro provides a convenient shorthand for calling the `print_styled` function.
///
/// # Examples
///
/// ```
/// use drgrep::color::config::Color;
/// use drgrep::print_styled;
///
/// print_styled!("Warning", Color::BOLD, Color::YELLOW);
/// ```
#[macro_export]
macro_rules! print_styled {
    ($($arg:tt)* ) => {{
        $crate::print_styled($($arg)*)
    }};
}

/// Macro for printing multiple colored text segments
///
/// This macro provides a convenient shorthand for calling the `print_partial_colored` function.
///
/// # Examples
///
/// ```
/// use drgrep::color::config::Color;
/// use drgrep::print_partial_colored;
///
/// let parts = vec![
///     ("Success:", Color::GREEN),
///     ("Operation completed", Color::WHITE)
/// ];
/// print_partial_colored!(&parts);
/// ```
#[macro_export]
macro_rules! print_partial_colored {
    ($($arg:tt)* ) => {{
        $crate::print_partial_colored($($arg)*)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::config::Color;

    // Helper function to capture stdout for testing
    // Note: This approach has limitations and is primarily for demonstration
    #[allow(dead_code)]
    fn capture_stdout<F>(f: F) -> String
    where
        F: FnOnce(),
    {
        // In a real test, you would use a crate like `capture-stdout` or similar
        // This is a simplified version that doesn't actually capture output
        f();
        String::new() // Placeholder
    }

    #[test]
    fn test_print_colored() {
        // Since we can't easily capture stdout in a unit test,
        // we'll just verify the function doesn't panic
        print_colored("Test message", Color::RED);
        // If we reach here, the function didn't panic
        assert!(true);
    }

    #[test]
    fn test_print_styled() {
        // Again, just making sure it doesn't panic
        print_styled("Test styled message", Color::BOLD, Color::GREEN);
        assert!(true);
    }

    #[test]
    fn test_print_partial_colored() {
        let parts = vec![("Part1", Color::RED), ("Part2", Color::BLUE)];
        print_partial_colored(&parts);
        assert!(true);
    }

    #[test]
    fn test_empty_parts() {
        // Test with empty parts to ensure it doesn't crash
        let empty_parts: Vec<(&str, &str)> = vec![];
        print_partial_colored(&empty_parts);
        assert!(true);
    }

    // Test macros
    // Note: In real tests, you'd need a more complex setup to test macros properly
    #[test]
    fn test_macros() {
        // These should expand to the corresponding function calls
        crate::print_colored!("Macro test", Color::CYAN);
        crate::print_styled!("Styled macro test", Color::BOLD, Color::MAGENTA);

        let parts = vec![("MacroPart1", Color::GREEN), ("MacroPart2", Color::YELLOW)];
        crate::print_partial_colored!(&parts);

        // If we reach here, the macros didn't cause any panics
        assert!(true);
    }
}
