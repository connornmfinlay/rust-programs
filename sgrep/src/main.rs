use clap::Parser;
use anyhow::{Context, Result};
use log::{info, warn};
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use assert_fs::prelude::*;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let _ = env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file '{}'", args.path.display()))?; //shortcut for match and panic!
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
        find_matches(&content, &args.pattern, &mt std::io::stdout());
    }
    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ispum\n");
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("sgrep")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));
    Ok(())
} 
#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("sgrep")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));
    Ok(())
}
