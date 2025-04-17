use crate::color::config::Color;

pub type TextParts<'a> = &'a Vec<(&'a str, &'a str)>;

pub fn print_colored(text: &str, color: &str) {
    println!("{}{}{}", color, text, Color::RESET);
}

pub fn print_styled(text: &str, style: &str, color: &str) {
    println!("{}{}{}{}", style, color, text, Color::RESET);
}

/// Affiche plusieurs parties de texte avec leur couleur respective
pub fn print_partial_colored(parts: TextParts) {
    for (text, color) in parts {
        print!("{}{}{} ", color, text, Color::RESET);
    }
    println!(); // saut de ligne à la fin
}

#[macro_export]
macro_rules! print_colored {
    ($($arg:tt)* ) => {{
        $crate::print_colored($($arg)*)
    }};
}

#[macro_export]
macro_rules! print_styled {
    ($($arg:tt)* ) => {{
        $crate::print_styled($($arg)*)
    }};
}

#[macro_export]
macro_rules! print_partial_colored {
    ($($arg:tt)* ) => {{
        $crate::print_partial_colored($($arg)*)
    }};
}
