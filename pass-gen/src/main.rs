use rand::Rng;
use rand::prelude::SliceRandom;  // Import the SliceRandom trait
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn main() {
    let mut rng = rand::thread_rng();
    let symbols = b"!^()-_=+[]{}<>?";
    let numbers = b"0123456789";
    let alphanumeric = b"ABCDEFGHJKLMNOPQRSTUVWXYZabcdefghjkmnopqrstuvwxyz";

    // Generate 3 random numbers
    let num_part: String = (0..3)
        .map(|_| numbers[rng.gen_range(0..numbers.len())] as char)
        .collect();

    // Generate 3 random symbols
    let sym_part: String = (0..3)
        .map(|_| symbols[rng.gen_range(0..symbols.len())] as char)
        .collect();

    // Generate 6 random alphanumeric characters
    let alpha_part: String = (0..6)
        .map(|_| alphanumeric[rng.gen_range(0..alphanumeric.len())] as char)
        .collect();

    // Combine all parts and shuffle them to randomize the order
    let mut password = format!("{}{}{}", num_part, sym_part, alpha_part)
        .chars()
        .collect::<Vec<char>>();
    password.shuffle(&mut rng);

    let final_password: String = password.into_iter().collect();

    // Copy the generated password to the clipboard
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(final_password.clone()).unwrap();

    println!("Generated password: {}", final_password);
    println!("Password has been copied to the clipboard.");
}

