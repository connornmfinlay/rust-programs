use std::env;
use std::fs;
use std::path::PathBuf;
use std::os::macos::fs::MetadataExt;

fn main() {
    let args: Vec<_> = env::args().collect();
    let paths: Vec<PathBuf> = if args.len() > 1 {
        args[1..]
            .iter()
            .map(|s| PathBuf::from(s))
            .collect()
    } else {
        vec![env::current_dir().unwrap()]
    };

    for path in paths {
        if path.is_dir() {
            print_dir_size(&path);
        } else {
            print_file_size(&path);
        }
    }
}

fn print_dir_size(path: &PathBuf) {
    let size = dir_size(path);
    println!("{:<10} {}", format_size(size), path.display());
}

fn print_file_size(path: &PathBuf) {
    let size = file_size(path);
    println!("{:<10} {}", format_size(size), path.display());
}

fn dir_size(path: &PathBuf) -> u64 {
    let mut total_size = 0;
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
            total_size += dir_size(&entry.path());
        } else {
            total_size += metadata.st_size();
        }
    }
    total_size
}

fn file_size(path: &PathBuf) -> u64 {
    fs::metadata(path).unwrap().st_size()
}

fn format_size(size: u64) -> String {
    let units = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut index = 0;
    let mut formatted_size = size as f64;
    while formatted_size >= 1024.0 && index < units.len() - 1 {
        formatted_size /= 1024.0;
        index += 1;
    }
    format!("{:.2} {}", formatted_size, units[index])
}
