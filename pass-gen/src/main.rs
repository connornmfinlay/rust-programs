use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let symbols = b"!@#$%^&*()-_=+[]{}<>?/\\|";  // Define your symbol set as a byte array

    // Generate a password of 12 random characters (alphanumeric + symbols)
    let password: String = (0..12)
        .map(|_| {
            let choice = rng.gen_range(0..3);  // Randomly choose between 0 (alphanumeric) and 1 (symbols)
            match choice {
                0 => rng.sample(rand::distributions::Alphanumeric) as char,  // Alphanumeric characters
                1 => symbols[rng.gen_range(0..symbols.len())] as char,       // Random symbol
                _ => rng.sample(rand::distributions::Alphanumeric) as char,  // Fallback to alphanumeric
            }
        })
        .collect();
    
    println!("Generated password: {}", password);
}

