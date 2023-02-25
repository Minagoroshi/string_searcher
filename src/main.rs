use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut case_sensitive = false;
    let mut path = "";
    let mut search_string = "";

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--case-sensitive" {
            case_sensitive = true;
        } else if args[i] == "--path" {
            i += 1;
            path = &args[i];
        } else if args[i] == "--search" {
            i += 1;
            search_string = &args[i];
        }
        i += 1;
    }

    let path = Path::new(path);
    println!("Starting Search in {} for \"{}\"\n", path.display(), search_string);
    let start = std::time::Instant::now();
    walk_dir(&path, &search_string, case_sensitive);
    let end = std::time::Instant::now();
    let elapsed = end - start;
    println!("\nSearch took {} milliseconds", elapsed.as_millis());
}

fn walk_dir(path: &Path, search_string: &str, case_sensitive: bool) {
    if path.is_dir() {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    if let Ok(mut file) = File::open(&entry_path) {
                        let mut contents = String::new();
                        if let Ok(_) = file.read_to_string(&mut contents) {
                            if !case_sensitive {
                                if contents.to_lowercase().contains(&search_string.to_lowercase()) {
                                    println!("File {} contains the string \"{}\"", entry_path.display(), search_string);
                                }
                            } else {
                                if contents.contains(search_string) {
                                    println!("File {} contains the string \"{}\"", entry_path.display(), search_string);
                                }
                            }
                        }
                    }
                } else if entry_path.is_dir() {
                    walk_dir(&entry_path, search_string, case_sensitive);
                }
            }
        }
    }
}
