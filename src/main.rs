use cli_password_generator::{self, Password};

fn main() {
    match Password::new() {
        Ok(password) => {
            println!("Generated password: {}", password.as_text);
            println!("Length password: {}", password.length);
            println!("Password entropy: {:.2} bits", password.entropy);
        }
        Err(e) => println!("Error: {}", e),
    }
}
