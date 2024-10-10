use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    
    // Generate a password of 12 random alphanumeric characters
    let password: String = (0..12)
        .map(|_| {
            // Sample a random alphanumeric character (u8) and convert it to char
            let ch = rng.sample(rand::distributions::Alphanumeric);
            ch as char
        })
        .collect();
    
    println!("Generated password: {}", password);
}

