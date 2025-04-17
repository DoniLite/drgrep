# drgrep ✨

[![Build Status](https://github.com/DoniLite/drgrep/actions/workflows/build_and_release.yml/badge.svg)](https://github.com/DoniLite/drgrep/actions/workflows/build_and_release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/drgrep.svg)](https://crates.io/crates/drgrep)

`drgrep` is a simple, fast command-line tool written in Rust for searching patterns in text, with minimal external dependencies (beyond Rust itself). It's inspired by `grep` but uses a **simplified** and **custom** regular expression (regex) engine, designed to be lightweight and easy to integrate.

## 🚀 Features

* **Simple Text Searching:** Find lines matching a given pattern in files or from standard input.
* **Custom Regex Engine:** Uses an internal implementation for a limited but useful subset of regex features (see [Supported Regex Syntax](https://donilite.github.io/drgrep/drgrep/regex/pattern/index.html)).
* **Fast and Lightweight:** Written in Rust for performance and memory safety.
* **Cross-Platform:** Pre-compiled binaries are available for Linux, macOS, and Windows via [GitHub Releases](https://github.com/DoniLite/drgrep/releases).
* **Minimal External Dependencies:** Designed to minimize dependencies (check your final `Cargo.toml`).

## ⚙️ Installation

You can install `drgrep` in several ways:

### 1. Pre-compiled Binaries (Recommended)

Download the binary corresponding to your operating system from the [GitHub Releases page](https://github.com/DoniLite/drgrep/releases).

1. Go to the releases page.
2. Find the latest release.
3. Download the archive for your OS (`drgrep-linux`, `drgrep-macos`, `drgrep-windows.exe`).
4. (Optional) Rename the file to `drgrep` (or `drgrep.exe`) and place it in a directory included in your system's `PATH`.

### 2. From Source (with Cargo)

If you have Rust and Cargo installed (`rustup`), you can compile from source:

```bash
# Clone the repository
git clone https://github.com/DoniLite/drgrep.git
cd drgrep

# Compile in release mode
cargo build --release

# The binary will be located at target/release/drgrep
# You can copy it to your PATH:
# sudo cp target/release/drgrep /usr/local/bin/  # Example for Linux/macOS
```

### 3. From Crates.io

You can install it directly with Cargo:

```bash
cargo install drgrep
```

## 📚 Usage

The basic syntax is:

```bash
drgrep [ARGS]
```

Bash
<PATTERN>: The (simplified) regex pattern to search for.
[FILE...]: One or more files to search within. If no file is specified, drgrep reads from standard input (stdin).
Examples:

### Find "error" in the application.log file

```bash
drgrep -k error -p application.log
```

### Find lines starting with a timestamp (e.g., 2023-...) in multiple log files

drgrep "^\\d{4}-" service1.log service2.log

# NOTE: {n} might not be supported; use \d\d\d\d- if needed

# Find 5-letter words in a file (requires supported syntax)

drgrep "\\w\\w\\w\\w\\w" document.txt

# Using with a pipe

cat data.txt | drgrep "important data"

# Searching for a pattern containing spaces (use quotes)

drgrep "user login" auth.log
Use code with caution.
Bash
(Add other relevant options and examples for your tool here, such as recursive search, case-insensitivity, etc., if you implement them).
💡 Supported Regex Syntax
drgrep uses a simplified regular expression engine. Only the following features are currently supported:
Symbol Description Example Matches
. Any single character a.c abc, axc
\d Digit (0-9) \d+ 123, 9
\w Word character (alphanumeric + _) \w+ word,_123
\s Whitespace character hello\sworld hello world
\D Non-digit \D a,
\W Non-word character \W !,
\S Non-whitespace character \S+ word, 123

* Zero or more occurrences ab*c ac, abc, abbc

* One or more occurrences ab+c abc, abbc
? Zero or one occurrence ab?c ac, abc
^ Start of line anchor ^Start Start here
$ End of line anchor end$ This is the end
\ Escape character \. . (literal)
Unsupported Features (non-exhaustive list):
Capturing groups (...)
Alternation | (OR logic)
Specific quantifiers {n}, {n,}, {n,m}
Custom character classes [...], [^...]
Lookarounds (lookahead (?=...), lookbehind (?<=...))
Backreferences \1
Inline case-insensitivity options (?i) (may be available via a CLI flag)
And other advanced regex features...
🌱 Contributing
Contributions are welcome! Feel free to open an Issue to report a bug or suggest an improvement, or a Pull Request with your changes.
Please follow standard open-source contribution practices.
📜 License
This project is distributed under the MIT License. See the LICENSE file for more details.
