use rand::seq::SliceRandom;

fn main() {
    println!("Hello, world!");

    let length: usize = 12;
    let char_pool: Vec<char> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_+-="
            .chars()
            .collect();

    let password: String = (0..length)
        .map(|_| *char_pool.choose(&mut rand::thread_rng()).unwrap())
        .collect();

    println!("Password is: {}", password);
}
