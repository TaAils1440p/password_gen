use clap::Parser;
use rand::Rng;

/// Simple password generator
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Length of the password
    #[arg(short, long, default_value_t = 12)]
    length: usize,

    /// Include numbers
    #[arg(short = 'n', long)]
    numbers: bool,

    /// Include symbols
    #[arg(short = 's', long)]
    symbols: bool,
}

fn main() {
    let args = Args::parse();
    let password = generate_password(args.length, args.numbers, args.symbols);
    println!("Generated password: {}", password);
}

/// Generates a random password based on the given options
pub fn generate_password(length: usize, use_numbers: bool, use_symbols: bool) -> String {
    const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS: &str = "0123456789";
    const SYMBOLS: &str = "!@#$%^&*()-_=+[]{};:,.<>?";

    let mut charset = LETTERS.to_string();
    if use_numbers {
        charset.push_str(NUMBERS);
    }
    if use_symbols {
        charset.push_str(SYMBOLS);
    }

    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}