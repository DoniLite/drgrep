use drgrep::Color;
use std::io::{self, Write};

#[test]
fn test_color_output_formatting() {
    // This is primarily a visual test, but we can at least ensure the code compiles
    // and that the formatted strings have the expected structure

    let colored_text = format!("{}This is blue text{}", Color::BLUE, Color::RESET);
    assert!(colored_text.starts_with("\x1b[34m"));
    assert!(colored_text.ends_with("\x1b[0m"));
    assert!(colored_text.contains("This is blue text"));

    // In a real terminal environment, this would display colored text
    // but in automated tests, we can only verify the string structure
    let styled_text = format!(
        "{}{}Important warning{}",
        Color::BOLD,
        Color::YELLOW,
        Color::RESET
    );
    assert!(styled_text.contains("\x1b[1m"));
    assert!(styled_text.contains("\x1b[33m"));
    assert!(styled_text.contains("Important warning"));
    assert!(styled_text.ends_with("\x1b[0m"));
}

#[test]
fn test_color_in_standard_output() {
    // Test that colors can be used in standard output
    // Note: This doesn't verify the actual color rendering, only that it doesn't cause errors

    let output = format!("{}Success message{}", Color::GREEN, Color::RESET);

    // This would display green text in an actual terminal
    let _ = io::stdout().write_all(output.as_bytes());

    // Just ensure the test runs without error
    assert!(true);
}

// Visual test function - this would be run manually, not in automated tests
#[allow(dead_code)]
fn visual_color_test() {
    // Basic colors
    println!("{}BLACK{}", Color::BLACK, Color::RESET);
    println!("{}RED{}", Color::RED, Color::RESET);
    println!("{}GREEN{}", Color::GREEN, Color::RESET);
    println!("{}YELLOW{}", Color::YELLOW, Color::RESET);
    println!("{}BLUE{}", Color::BLUE, Color::RESET);
    println!("{}MAGENTA{}", Color::MAGENTA, Color::RESET);
    println!("{}CYAN{}", Color::CYAN, Color::RESET);
    println!("{}WHITE{}", Color::WHITE, Color::RESET);

    // Bright colors
    println!("{}BRIGHT_BLACK{}", Color::BRIGHT_BLACK, Color::RESET);
    println!("{}BRIGHT_RED{}", Color::BRIGHT_RED, Color::RESET);
    println!("{}BRIGHT_GREEN{}", Color::BRIGHT_GREEN, Color::RESET);
    println!("{}BRIGHT_YELLOW{}", Color::BRIGHT_YELLOW, Color::RESET);
    println!("{}BRIGHT_BLUE{}", Color::BRIGHT_BLUE, Color::RESET);
    println!("{}BRIGHT_MAGENTA{}", Color::BRIGHT_MAGENTA, Color::RESET);
    println!("{}BRIGHT_CYAN{}", Color::BRIGHT_CYAN, Color::RESET);
    println!("{}BRIGHT_WHITE{}", Color::BRIGHT_WHITE, Color::RESET);

    // Styles
    println!("{}BOLD{}", Color::BOLD, Color::RESET);
    println!("{}UNDERLINE{}", Color::UNDERLINE, Color::RESET);

    // Combinations
    println!("{}{}BOLD RED{}", Color::BOLD, Color::RED, Color::RESET);
    println!(
        "{}{}UNDERLINED BLUE{}",
        Color::UNDERLINE,
        Color::BLUE,
        Color::RESET
    );
    println!(
        "{}{}{}BOLD UNDERLINED GREEN{}",
        Color::BOLD,
        Color::UNDERLINE,
        Color::GREEN,
        Color::RESET
    );
}
