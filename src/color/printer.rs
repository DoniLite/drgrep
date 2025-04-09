use crate::color::config::Color;



pub fn print_colored(text: &str, color: &str) {
    println!("{}{}{}", color, text, Color::RESET);
}

pub fn print_styled(text: &str, style: &str, color: &str) {
    println!("{}{}{}{}", style, color, text, Color::RESET);
}

/// Affiche plusieurs parties de texte avec leur couleur respective
pub fn print_partial_colored(parts: &[(&str, &str)]) {
    for (text, color) in parts {
        print!("{}{}{}", color, text, Color::RESET);
    }
    println!(); // saut de ligne à la fin
}