# drgrep ✨

[![Build Status](https://github.com/DoniLite/drgrep/actions/workflows/build_release.yml/badge.svg)](https://github.com/DoniLite/drgrep/actions/workflows/build_release.yml)
[![Tests Status](https://github.com/DoniLite/drgrep/actions/workflows/rust_test.yml/badge.svg)](https://github.com/DoniLite/drgrep/actions/workflows/rust_test.yml)
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

Examples:

### Basic Regex searching in the `.github/`

```sh
cargo run -- -r Rust -p ./.github
```

Output: ![image](./assets/Capture%20d’écran%20du%202025-04-17%2020-40-36.png)

### Find "error" in the application.log file

```sh
drgrep -k error -p application.log
```

### Find 5-letter words in a file (requires supported syntax)

```sh
drgrep -r \\w\\w\\w\\w\\w -p document.txt
```

And other advanced regex features...

## 🌱 Contributing

Contributions are welcome! Feel free to open an Issue to report a bug or suggest an improvement, or a Pull Request with your changes.
Please follow standard open-source contribution practices.

## 📜 License

This project is distributed under the MIT License. See the LICENSE file for more details.
