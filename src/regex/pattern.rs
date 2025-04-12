//! # Module SimplePattern
//!
//! Fournit des fonctionnalités basiques de correspondance de motifs sans dépendances externes.
//! Cette implémentation supporte un sous-ensemble limité des fonctionnalités d'expressions régulières.
//!
//! Motifs supportés:
//! - Caractères littéraux (ex: "abc")
//! - Point (.) pour n'importe quel caractère
//! - Classes de caractères prédéfinies:
//!   - \d - Chiffres
//!   - \w - Caractères alphanumériques + underscore
//!   - \s - Espaces blancs
//! - Négation des classes: \D, \W, \S
//! - Quantificateurs simples:
//!   - * - Zéro ou plus
//!   - + - Un ou plus
//!   - ? - Zéro ou un
//! - Ancres:
//!   - ^ - Début de ligne
//!   - $ - Fin de ligne
//! - Échappement avec \

use std::error::Error;
use std::fmt;

/// Erreurs spécifiques aux opérations de correspondance de motifs
#[derive(Debug)]
pub enum PatternError {
    /// Erreur de syntaxe dans le motif
    InvalidPattern(String),
    /// Autres erreurs
    Other(String),
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternError::InvalidPattern(msg) => write!(f, "Motif invalide: {}", msg),
            PatternError::Other(msg) => write!(f, "Erreur: {}", msg),
        }
    }
}

impl Error for PatternError {}

/// Type d'élément de motif
#[derive(Debug, Clone, PartialEq)]
enum PatternElement {
    Literal(char),
    AnyChar,
    Digit(bool),      // bool indique négation
    Word(bool),       // bool indique négation
    Whitespace(bool), // bool indique négation
    StartAnchor,
    EndAnchor,
    Quantifier(Box<PatternElement>, QuantifierType),
}

/// Types de quantificateurs
#[derive(Debug, Clone, PartialEq)]
enum QuantifierType {
    ZeroOrMore,
    OneOrMore,
    ZeroOrOne,
}

/// Structure principale pour la correspondance de motifs
#[derive(Debug)]
pub struct SimplePattern {
    elements: Vec<PatternElement>,
    pattern: String,
}

/// Résultat d'une correspondance
#[derive(Debug, Clone)]
pub struct Match {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

impl SimplePattern {
    /// Crée un nouveau modèle à partir d'une chaîne de motif
    pub fn new(pattern: &str) -> Result<Self, PatternError> {
        let mut elements = Vec::new();
        let mut chars = pattern.chars().peekable();

        let mut is_start_anchor = false;
        let mut is_end_anchor = false;

        // Vérifier l'ancre de début
        if let Some('^') = chars.peek() {
            is_start_anchor = true;
            chars.next();
        }

        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    // Caractère d'échappement
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
                                "Caractère d'échappement incomplet".to_string(),
                            ))
                        }
                    }
                }
                '.' => elements.push(PatternElement::AnyChar),
                '$' => {
                    // Si $ est le dernier caractère, c'est une ancre de fin
                    if chars.peek().is_none() {
                        is_end_anchor = true;
                    } else {
                        elements.push(PatternElement::Literal('$'));
                    }
                }
                '*' | '+' | '?' => {
                    // Quantificateurs
                    if elements.is_empty() {
                        return Err(PatternError::InvalidPattern(format!(
                            "Quantificateur '{}' sans élément précédent",
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

    pub fn get_pattern(&self) -> &str {
        self.pattern.as_str()
    }

    /// Vérifie si le texte correspond au motif
    pub fn is_match(&self, text: &str) -> bool {
        self.find(text).is_some()
    }

    /// Trouve la première correspondance dans le texte
    pub fn find(&self, text: &str) -> Option<Match> {
        // Optimisation pour les motifs avec ancre de début
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

    /// Trouve toutes les correspondances dans le texte
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

    /// Trouve une correspondance à partir d'une position spécifique
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
                            // Essayer de faire correspondre autant que possible
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
                                return None; // Besoin d'au moins une correspondance
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

        // Vérification de l'ancre de fin APRÈS avoir parcouru les éléments
        if has_end_anchor
            && current_pos != text_chars.len()
            && (current_pos > 0 && text_chars[current_pos - 1] != '\n')
        {
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

    /// Vérifie si un caractère correspond à un élément de motif
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
            PatternElement::StartAnchor => false, // Ne devrait pas être testé directement
            PatternElement::EndAnchor => false,   // Ne devrait pas être testé directement
            PatternElement::Quantifier(_, _) => {
                panic!("Les quantificateurs ne peuvent pas être testés directement")
            }
        }
    }

    /// Remplace toutes les occurrences du motif par la chaîne de remplacement
    pub fn replace_all(&self, text: &str, replacement: &str) -> String {
        let matches = self.find_all(text);

        if matches.is_empty() {
            return text.to_string();
        }

        let mut result = String::new();
        let mut last_end = 0;

        for m in matches {
            // Ajouter le texte entre la dernière correspondance et celle-ci
            result.push_str(&text[last_end..m.start]);
            // Ajouter le remplacement
            result.push_str(replacement);
            last_end = m.end;
        }

        // Ajouter le reste du texte
        result.push_str(&text[last_end..]);
        result
    }

    /// Divise le texte selon le motif
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
                // Si la correspondance est au début, ajouter une chaîne vide
                result.push(String::new());
            }

            last_end = m.end;
        }

        // Ajouter le reste du texte
        if last_end < text.len() {
            result.push(text[last_end..].to_string());
        } else {
            // Si la dernière correspondance est à la fin, ajouter une chaîne vide
            result.push(String::new());
        }

        result
    }
}

// Fonctions utilitaires
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
        // Zéro ou plus
        let pattern = SimplePattern::new("ab*c").unwrap();
        assert!(pattern.is_match("ac"));
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("abbc"));

        // Un ou plus
        let pattern = SimplePattern::new("ab+c").unwrap();
        assert!(!pattern.is_match("ac"));
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("abbc"));

        // Zéro ou un
        let pattern = SimplePattern::new("ab?c").unwrap();
        assert!(pattern.is_match("ac"));
        assert!(pattern.is_match("abc"));
        assert!(!pattern.is_match("abbc"));
    }

    #[test]
    fn test_anchors() {
        // Ancre de début
        let pattern = SimplePattern::new("^abc").unwrap();
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("abcdef"));
        assert!(!pattern.is_match("xabc"));

        // Ancre de fin
        let pattern = SimplePattern::new("abc$").unwrap();
        assert!(pattern.is_match("abc"));
        assert!(pattern.is_match("xabc"));
        assert!(!pattern.is_match("abcx"));

        // Les deux ancres
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

// Exemple d'utilisation
#[allow(dead_code)]
fn example() -> Result<(), Box<dyn Error>> {
    let pattern = SimplePattern::new("\\d+")?;

    // Vérification de correspondance
    let contains_digits = pattern.is_match("abc123");
    println!("Contient des chiffres: {}", contains_digits);

    // Recherche de la première correspondance
    if let Some(m) = pattern.find("Version 2.3.4") {
        println!("Premier nombre trouvé: {}", m.text);
    }

    // Recherche de toutes les correspondances
    let matches = pattern.find_all("Version 2.3.4");
    println!(
        "Tous les nombres: {:?}",
        matches.iter().map(|m| &m.text).collect::<Vec<_>>()
    );

    // Remplacement
    let result = pattern.replace_all("Version 2.3.4", "X");
    println!("Après remplacement: {}", result);

    // Division
    let parts = pattern.split("abc123def456");
    println!("Parties: {:?}", parts);

    // Utilisation des fonctions utilitaires
    let email_pattern = "\\w+@\\w+\\.\\w+";
    let text = "Contact: user@example.com et admin@domain.org";
    let emails = find_all(email_pattern, text)?;
    println!(
        "Emails trouvés: {:?}",
        emails.iter().map(|m| &m.text).collect::<Vec<_>>()
    );

    Ok(())
}
