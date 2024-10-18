use rand::Rng;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn main() {
    let mut rng = rand::thread_rng();
    let symbols = b"!@#$%^*()-_=+[]{}<>?/";  // Define your symbol set as a byte array

    let password: String = (0..8)
        .map(|_| {
            let choice = rng.gen_range(0..3);  // Randomly choose between 0 (alphanumeric) and 1 (symbols)
            match choice {
                0 => rng.sample(rand::distributions::Alphanumeric) as char,  // Alphanumeric characters
                1 => symbols[rng.gen_range(0..symbols.len())] as char,       // Random symbol
                _ => rng.sample(rand::distributions::Alphanumeric) as char,  // Fallback to alphanumeric
            }
        })
        .collect();

    // Copy the generated password to the clipboard
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(password.clone()).unwrap();

    println!("Generated password: {}", password);
    println!("Password has been copied to the clipboard.");
}
