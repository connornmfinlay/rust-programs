use clap::Parser;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
   
    let file = File::open(&args.path).expect("Could not open file");
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        match line_result { 
        Ok(line) => {
            if line.contains(&args.pattern) {
                println!("{}", line);
            }
        }
        Err(err) => {
            eprintln!("Error reading line: {}", err);
            }
        }
    }
}
