use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Password generator", about = "A strong password generator")]
struct Arguments {
    #[structopt(long = "length", short = "l", default_value = "8")]
    length: usize,

    #[structopt(long = "lowercase", short = "w")]
    is_lowercase: bool,

    #[structopt(long = "uppercase", short = "u")]
    is_uppercase: bool,

    #[structopt(long = "numbers", short = "n")]
    is_numbers: bool,

    #[structopt(long = "special", short = "s")]
    is_special_symbols: bool,

    #[structopt(long = "ambiguous", short = "a")]
    is_avoid_ambiguous: bool,
}

impl Arguments {
    fn get() -> Result<Self, Box<dyn Error>> {
        let args: Arguments = Arguments::from_args_safe()?;
        let length: usize = args.length;
        if length < 12 || length > 40 {
            Err("The length must be between 12 and 40 characters")?;
        }
        Ok(Self { length, ..args })
    }
}

#[derive(Debug)]
pub struct Password {
    pub as_text: String,
    pub length: usize,
    pub entropy: f64,
}

impl Password {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let args = Arguments::get()?;

        let mut char_pool: Vec<char> = Vec::new();

        if args.is_lowercase {
            char_pool.extend('a'..='z');
        }

        if args.is_uppercase {
            char_pool.extend('A'..='Z');
        }

        if args.is_numbers {
            char_pool.extend('0'..='9');
        }

        if args.is_special_symbols {
            char_pool.extend("!@#$%^&*()_+-=[]{};:'<>,.?/".chars());
        }

        if args.is_avoid_ambiguous {
            char_pool.retain(|&c| !matches!(c, '1' | 'l' | '0' | 'O'));
        }

        if char_pool.is_empty() {
            return Err("You must select at least one character type".into());
        }

        let as_text: String = (0..args.length)
            .map(|_| *char_pool.choose(&mut OsRng).unwrap())
            .collect();

        let entropy = Password::calculate_entropy(&as_text);

        Ok(Self {
            as_text,
            length: args.length,
            entropy,
        })
    }

    fn calculate_entropy(password: &str) -> f64 {
        let length = password.len() as f64;
        let charset_size: f64 = 94.0;
        length * charset_size.log2()
    }
}
