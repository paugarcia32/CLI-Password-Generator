use rand::seq::SliceRandom;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Password generator", about = "A strong password generator")]
struct Arguments {
    #[structopt(long = "length", short = "l", default_value = "8")]
    length: usize,
}

impl Arguments {
    fn get() -> Self {
        let args: Arguments = Arguments::from_args();
        let length: usize = args.length;
        Self { length }
    }
}

#[derive(Debug)]
pub struct Password {
    as_text: String,
    length: usize,
}

impl Password {
    pub fn new() -> Self {
        let args = Arguments::get();

        let char_pool: Vec<char> =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_+-="
                .chars()
                .collect();

        let as_text: String = (0..args.length)
            .map(|_| *char_pool.choose(&mut rand::thread_rng()).unwrap())
            .collect();
        Self {
            as_text,
            length: args.length,
        }
    }
}
