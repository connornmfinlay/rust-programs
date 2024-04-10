use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn read_info() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file '{}'", args.path.display()))?; //shortcut for match and panic!
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn main(){
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
       let _ = read_info();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
