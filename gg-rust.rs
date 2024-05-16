use std::io::stdin;


fn main() {
    'outer_loop: loop {
    let number: u32 = 10;
    println!("Pick a number >>>");

    loop {
        let mut line = String::new();
        let input = stdin().read_line(&mut line);
        
        let guess: Option<u32> = input.ok().
            map_or(None, |_| line.trim().parse().ok());
        
        match guess {
            None => println!("Enter a number..."),
            Some(n) if n == number => {
                println!("Correct");
                break 'outer_loop;
            }
            Some(n) if n < number => println!("Too low"), 
            Some(n) if n > number => println!("Too high"),
            Some(_) => println!("Error"),
        }}}}
