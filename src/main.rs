use rand::seq::SliceRandom;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Password generator", about = "A strong password generator")]
struct Arguments {
    #[structopt(long = "length", short = "l", default_value = "8")]
    length: usize,
}

fn main() {
    let args = Arguments::from_args();

    println!("Length is: {}", args.length);

    let char_pool: Vec<char> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_+-="
            .chars()
            .collect();

    let password: String = (0..args.length)
        .map(|_| *char_pool.choose(&mut rand::thread_rng()).unwrap())
        .collect();

    println!("Password is: {}", password);
}
