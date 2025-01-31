//use std::env;
//use std::fs;
//use  std::path::PathBuf;

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    
    println!("pattern: {:?}, path {:?}", pattern, path)
}
