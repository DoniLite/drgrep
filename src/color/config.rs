//! # Color Configuration Module
//!
//! This module provides ANSI color and style escape sequences for terminal text formatting.
//! It allows you to easily add colors and styles to your command-line output.
//!
//! ## Usage
//!
//! ```rust
//! use drgrep::color::config::Color;
//!
//! // Print colored text
//! println!("{}This is red text{}", Color::RED, Color::RESET);
//!
//! // Combine styles
//! println!("{}{}Bold and underlined{}", Color::BOLD, Color::UNDERLINE, Color::RESET);
//!
//! // Use bright variants
//! println!("{}Bright cyan text{}", Color::BRIGHT_CYAN, Color::RESET);
//! ```
//!
//! ## Available Colors
//!
//! - Basic colors: BLACK, RED, GREEN, YELLOW, BLUE, MAGENTA, CYAN, WHITE
//! - Bright variants: BRIGHT_BLACK, BRIGHT_RED, BRIGHT_GREEN, etc.
//!
//! ## Available Styles
//!
//! - BOLD: Bold text
//! - UNDERLINE: Underlined text
//!
//! ## Note
//!
//! Always remember to use `Color::RESET` after using a color or style to reset
//! the terminal formatting.

/// ANSI escape sequences for terminal text formatting
///
/// This struct provides constants for coloring and styling text in terminal outputs
/// using ANSI escape sequences.
///
/// # Examples
///
/// ```
/// use drgrep::color::config::Color;
///
/// // Print text in green
/// println!("{}Success!{}", Color::GREEN, Color::RESET);
///
/// // Print bold red error message
/// println!("{}{}Error: File not found{}", Color::BOLD, Color::RED, Color::RESET);
/// ```
pub struct Color;

impl Color {
    /// Resets all colors and styles to terminal default
    pub const RESET: &'static str = "\x1b[0m";

    // Text colors
    /// Black text color
    pub const BLACK: &'static str = "\x1b[30m";
    /// Red text color
    pub const RED: &'static str = "\x1b[31m";
    /// Green text color
    pub const GREEN: &'static str = "\x1b[32m";
    /// Yellow text color
    pub const YELLOW: &'static str = "\x1b[33m";
    /// Blue text color
    pub const BLUE: &'static str = "\x1b[34m";
    /// Magenta text color
    pub const MAGENTA: &'static str = "\x1b[35m";
    /// Cyan text color
    pub const CYAN: &'static str = "\x1b[36m";
    /// White text color
    pub const WHITE: &'static str = "\x1b[37m";

    // Bright variants
    /// Bright black text color (usually gray)
    pub const BRIGHT_BLACK: &'static str = "\x1b[90m";
    /// Bright red text color
    pub const BRIGHT_RED: &'static str = "\x1b[91m";
    /// Bright green text color
    pub const BRIGHT_GREEN: &'static str = "\x1b[92m";
    /// Bright yellow text color
    pub const BRIGHT_YELLOW: &'static str = "\x1b[93m";
    /// Bright blue text color
    pub const BRIGHT_BLUE: &'static str = "\x1b[94m";
    /// Bright magenta text color
    pub const BRIGHT_MAGENTA: &'static str = "\x1b[95m";
    /// Bright cyan text color
    pub const BRIGHT_CYAN: &'static str = "\x1b[96m";
    /// Bright white text color
    pub const BRIGHT_WHITE: &'static str = "\x1b[97m";

    // Styles
    /// Bold text style
    pub const BOLD: &'static str = "\x1b[1m";
    /// Underline text style
    pub const UNDERLINE: &'static str = "\x1b[4m";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_constants_are_correct() {
        // Basic format check for ANSI escape codes
        assert_eq!(Color::RESET, "\x1b[0m");
        assert_eq!(Color::RED, "\x1b[31m");
        assert_eq!(Color::BLUE, "\x1b[34m");
        assert_eq!(Color::BOLD, "\x1b[1m");
    }

    #[test]
    fn test_color_combination() {
        // Test that colors can be combined with styles
        let bold_red = format!("{}{}", Color::BOLD, Color::RED);
        assert_eq!(bold_red, "\x1b[1m\x1b[31m");

        // Test that reset works after combinations
        let bold_red_reset = format!("{}{}test{}", Color::BOLD, Color::RED, Color::RESET);
        assert_eq!(bold_red_reset, "\x1b[1m\x1b[31mtest\x1b[0m");
    }

    #[test]
    fn test_bright_colors() {
        // Check bright color codes
        assert_eq!(Color::BRIGHT_GREEN, "\x1b[92m");
        assert_eq!(Color::BRIGHT_YELLOW, "\x1b[93m");

        // Test bright and normal color difference
        assert_ne!(Color::GREEN, Color::BRIGHT_GREEN);
        assert_ne!(Color::MAGENTA, Color::BRIGHT_MAGENTA);
    }

    #[test]
    fn test_styles() {
        // Check style codes
        assert_eq!(Color::BOLD, "\x1b[1m");
        assert_eq!(Color::UNDERLINE, "\x1b[4m");

        // Test style combination
        let bold_underline = format!("{}{}", Color::BOLD, Color::UNDERLINE);
        assert_eq!(bold_underline, "\x1b[1m\x1b[4m");
    }
}
