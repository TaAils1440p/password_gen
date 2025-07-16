# 🔐 Rust Password Generator

A simple and customizable command-line password generator written in Rust.  
Supports optional inclusion of numbers and special characters.

## 🚀 Features

- Generate random passwords of a specified length
- Optionally include:
  - Numbers (`0-9`)
  - Symbols (`!@#$%^&*...`)
- Fast and secure with the `rand` crate
- Simple CLI interface using `clap`

## 📦 Requirements

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70+ recommended)

## 🛠️ Installation

Clone the repository and build the project:

```bash
git clone https://github.com/TaAils1440p/password_gen.git
cd password_gen
cargo build --release
```

## ⚙️ Usage

Run the generator with default settings:
```bash
cargo run
```

Generate a 20-character password including numbers and symbols:
```bash
cargo run -- --length 20 --numbers --symbols
```

### CLI Options
| Option            | Description                | Default |
| ----------------- | -------------------------- | ------- |
| `--length`, `-l`  | Length of the password     | 12      |
| `--numbers`, `-n` | Include numeric characters | false   |
| `--symbols`, `-s` | Include special characters | false   |

## 📁 Project Structure

```bash
password_gen/
├── src/
│   └── main.rs         # Main program logic
├── tests/
│   └── password_test.rs # Unit tests
├── Cargo.toml          # Dependencies and metadata
└── README.md
```

## 📃 License
This project is open-source and available under the [MIT License](./LICENSE).
