use drgrep::glob::*;
use drgrep::temp_dir::create_temp_dir;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

// Helper function to create test directory structure
fn create_test_file_structure(dir: &Path) -> std::io::Result<()> {
    // Create directories
    fs::create_dir_all(dir.join("src/lib"))?;
    fs::create_dir_all(dir.join("src/bin"))?;
    fs::create_dir_all(dir.join("tests"))?;
    fs::create_dir_all(dir.join("docs"))?;
    fs::create_dir_all(dir.join(".git"))?;

    // Create Rust files
    File::create(dir.join("src/lib/utils.rs"))?.write_all(b"// Utils module")?;
    File::create(dir.join("src/lib/errors.rs"))?.write_all(b"// Error types")?;
    File::create(dir.join("src/bin/main.rs"))?.write_all(b"fn main() {}")?;
    File::create(dir.join("tests/test_utils.rs"))?.write_all(b"#[test] fn test() {}")?;
    File::create(dir.join(".git/conf"))?.write_all(b"#[test] fn test() {}")?;

    // Create other files
    File::create(dir.join("Cargo.toml"))?.write_all(b"[package]")?;
    File::create(dir.join("README.md"))?.write_all(b"# Test Project")?;
    File::create(dir.join("docs/guide.md"))?.write_all(b"# User Guide")?;
    File::create(dir.join("docs/api.html"))?.write_all(b"<html>API</html>")?;

    Ok(())
}

#[test]
fn test_find_files_simple() -> std::io::Result<()> {
    let temp_dir = create_temp_dir()?;
    create_test_file_structure(temp_dir.path())?;

    // Find all Rust files
    let pattern = GlobPattern::new(&format!("{}/**/*.rs", temp_dir.path().display()));
    let matches = pattern.find_files(temp_dir.path())?;

    assert_eq!(matches.len(), 4);
    assert!(matches.iter().any(|p| p.ends_with("src/lib/utils.rs")));
    assert!(matches.iter().any(|p| p.ends_with("src/lib/errors.rs")));
    assert!(matches.iter().any(|p| p.ends_with("src/bin/main.rs")));
    assert!(matches.iter().any(|p| p.ends_with("tests/test_utils.rs")));

    let pattern = GlobPattern::new(&format!("{}/.git/**", temp_dir.path().display()));
    let matches = pattern.find_files(temp_dir.path())?;
    assert_eq!(matches.len(), 1);
    assert!(matches.iter().any(|p| p.ends_with(".git/conf")));
    assert!(pattern.matches(&format!("{}/.git/conf", temp_dir.path().display())));

    Ok(())
}

#[test]
fn test_find_files_with_alternatives() -> std::io::Result<()> {
    let temp_dir = create_temp_dir()?;
    create_test_file_structure(temp_dir.path())?;

    // Find files in src/lib or tests
    let pattern = GlobPattern::new(&format!(
        "{}/{{src/lib,tests}}/*.rs",
        temp_dir.path().display()
    ));
    let matches = pattern.find_files(temp_dir.path())?;

    assert_eq!(matches.len(), 3);
    assert!(matches.iter().any(|p| p.ends_with("src/lib/utils.rs")));
    assert!(matches.iter().any(|p| p.ends_with("src/lib/errors.rs")));
    assert!(matches.iter().any(|p| p.ends_with("tests/test_utils.rs")));
    assert!(!matches.iter().any(|p| p.ends_with("src/bin/main.rs")));

    Ok(())
}

#[test]
fn test_find_files_with_nested_alternatives() -> std::io::Result<()> {
    let temp_dir = create_temp_dir()?;
    create_test_file_structure(temp_dir.path())?;

    // Test with nested alternatives
    let pattern = GlobPattern::new(&format!(
        "{}/{{src/{{lib,bin}},tests}}/*.rs",
        temp_dir.path().display()
    ));
    let matches = pattern.find_files(temp_dir.path())?;

    assert_eq!(matches.len(), 4);
    assert!(matches.iter().any(|p| p.ends_with("src/lib/utils.rs")));
    assert!(matches.iter().any(|p| p.ends_with("src/lib/errors.rs")));
    assert!(matches.iter().any(|p| p.ends_with("src/bin/main.rs")));
    assert!(matches.iter().any(|p| p.ends_with("tests/test_utils.rs")));

    Ok(())
}

#[test]
fn test_find_files_with_character_class() -> std::io::Result<()> {
    let temp_dir = create_temp_dir()?;
    create_test_file_structure(temp_dir.path())?;

    // Find markdown files
    let pattern = GlobPattern::new(&format!("{}/**.[mM][dD]", temp_dir.path().display()));
    let matches = pattern.find_files(temp_dir.path())?;

    assert_eq!(matches.len(), 2);
    assert!(matches.iter().any(|p| p.ends_with("README.md")));
    assert!(matches.iter().any(|p| p.ends_with("docs/guide.md")));

    Ok(())
}

#[test]
fn test_combined_alternatives_and_wildcards() -> std::io::Result<()> {
    let temp_dir = create_temp_dir()?;
    create_test_file_structure(temp_dir.path())?;

    // Test combination of alternatives and wildcards
    let pattern = GlobPattern::new(&format!(
        "{}/{{src/*/,tests/}}*.[rt]*",
        temp_dir.path().display()
    ));
    let matches = pattern.find_files(temp_dir.path())?;

    assert_eq!(matches.len(), 4);
    assert!(matches.iter().any(|p| p.ends_with("src/lib/utils.rs")));
    assert!(matches.iter().any(|p| p.ends_with("src/lib/errors.rs")));
    assert!(matches.iter().any(|p| p.ends_with("src/bin/main.rs")));
    assert!(matches.iter().any(|p| p.ends_with("tests/test_utils.rs")));

    Ok(())
}
