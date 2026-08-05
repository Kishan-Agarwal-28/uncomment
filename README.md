# uncomment

[![Build and Release](https://github.com/Kishan-Agarwal-28/uncomment/actions/workflows/release.yml/badge.svg)](https://github.com/Kishan-Agarwal-28/uncomment/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`uncomment` is a fast, language-aware CLI tool designed to strip, count, or list comments in your source code. Instead of relying on brittle regex, it uses a custom lexer to accurately distinguish comments from code, strings, and other constructs.

## Features

- **Language-Aware Lexing**: Understands string literals and syntax rules to avoid accidental deletions.
- **Multiple Modes**: Strip comments, count them, or list them out for analysis.
- **Customizable**: Add your own language rules via a configuration file (`--langs`).
- **Dry-run**: Preview changes before modifying files (`--dry-run`).
- **Cross-Platform**: Available for Windows, macOS, and Linux.

## Installation

### Shell Script (macOS / Linux)
The quickest way to install is via the installation script:
```bash
curl -fsSL https://raw.githubusercontent.com/Kishan-Agarwal-28/uncomment/main/install.sh | bash
```

### macOS / Linux (Homebrew)
```bash
brew tap Kishan-Agarwal-28/homebrew-tap
brew install uncomment
```

### Debian / Ubuntu (apt)
```bash
echo "deb [trusted=yes] https://Kishan-Agarwal-28.github.io/uncomment/ ./" | sudo tee /etc/apt/sources.list.d/uncomment.list
sudo apt update
sudo apt install uncomment
```

### Windows (Chocolatey)
```powershell
choco install uncomment
```

### From Source (Cargo)
If you have Rust installed, you can build from source:
```bash
cargo install --git https://github.com/Kishan-Agarwal-28/uncomment.git
```

## Usage

Basic usage to strip comments in place:
```bash
uncomment src/main.rs
```

**Common Options:**
- `--mode <MODE>`: Action to perform. Choices: `strip` (default), `list`, `count`.
- `--dry-run`: Print the stripped code to stdout instead of overwriting the file.
- `-l, --lang <EXT>`: Force a specific language configuration by file extension (e.g., `rs`, `py`, `js`).
- `-o, --output <FILE>`: Output to a specific file instead of modifying in-place (only with single file).
- `--langs <FILE>`: Path to a TOML file containing additional language definitions.

### Examples

**Count comments in a Python file:**
```bash
uncomment script.py --mode count
```

**List comments in a JavaScript file:**
```bash
uncomment app.js --mode list
```

**Preview stripped output without overwriting:**
```bash
uncomment config.json --dry-run
```

**Strip comments and save to a new file:**
```bash
uncomment index.html -o index_clean.html
```

## License

This project is licensed under the MIT License - see the [LICENCE](LICENCE) file for details.
