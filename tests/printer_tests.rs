// In tests/printer_tests.rs

use drgrep::color::config::Color;
use drgrep::color::printer::{print_colored, print_partial_colored, print_styled};

#[test]
fn test_printer_integration() {
    // Basic functionality tests
    // These mostly just verify that the code runs without panicking

    // Test print_colored
    print_colored("Integration test for colored printing", Color::BLUE);

    // Test print_styled
    print_styled(
        "Integration test for styled printing",
        Color::BOLD,
        Color::GREEN,
    );

    // Test print_partial_colored
    let parts = vec![
        ("First", Color::RED),
        ("Second", Color::GREEN),
        ("Third", Color::BLUE),
    ];
    print_partial_colored(&parts);

    // Successfully reaching this point means the functions executed without errors
    assert!(true);
}

#[test]
fn test_macro_integration() {
    // Test the macros function properly in an integration context

    // Import macros
    use drgrep::print_colored;
    use drgrep::print_partial_colored;
    use drgrep::print_styled;

    // Use the macros
    print_colored!("Macro integration test", Color::CYAN);

    print_styled!(
        "Styled macro integration test",
        Color::UNDERLINE,
        Color::MAGENTA
    );

    let parts = vec![
        ("MacroTest1", Color::YELLOW),
        ("MacroTest2", Color::BRIGHT_BLUE),
    ];
    print_partial_colored!(&parts);

    // If we reach here, the macros expanded and executed correctly
    assert!(true);
}

// Visual test function - this would be run manually, not in automated tests
#[allow(dead_code)]
fn visual_printer_test() {
    // Test all printing functions with various colors and styles

    println!("\n=== Basic Color Printing ===");
    print_colored("Red Text", Color::RED);
    print_colored("Green Text", Color::GREEN);
    print_colored("Blue Text", Color::BLUE);
    print_colored("Yellow Text", Color::YELLOW);

    println!("\n=== Styled Printing ===");
    print_styled("Bold Red", Color::BOLD, Color::RED);
    print_styled("Underlined Green", Color::UNDERLINE, Color::GREEN);
    print_styled("Bold Underlined Blue", Color::BOLD, Color::BLUE);

    println!("\n=== Partial Colored Printing ===");
    let parts1 = vec![("Error:", Color::RED), ("File not found", Color::WHITE)];
    print_partial_colored(&parts1);

    let parts2 = vec![
        ("Success:", Color::GREEN),
        ("Operation completed", Color::WHITE),
        ("in", Color::WHITE),
        ("5 seconds", Color::BRIGHT_CYAN),
    ];
    print_partial_colored(&parts2);

    let parts3 = vec![
        ("Warning:", Color::YELLOW),
        ("Disk space", Color::WHITE),
        ("low", Color::RED),
    ];
    print_partial_colored(&parts3);

    println!("\n=== Using Macros ===");
    drgrep::print_colored!("Macro: Cyan Text", Color::CYAN);
    drgrep::print_styled!("Macro: Bold Magenta", Color::BOLD, Color::MAGENTA);

    let parts4 = vec![
        ("Macro:", Color::BRIGHT_GREEN),
        ("Multiple", Color::BRIGHT_YELLOW),
        ("Colors", Color::BRIGHT_BLUE),
    ];
    drgrep::print_partial_colored!(&parts4);
}
