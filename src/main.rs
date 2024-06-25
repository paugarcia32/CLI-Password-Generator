use cli_password_generator::{self, Password};

fn main() {
    let password_1 = Password::new();
    println!("The password entity is: {:#?}", password_1);
}
