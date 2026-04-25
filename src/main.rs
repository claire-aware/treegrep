use std::env;
use std::fs::exists;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

fn show_help() {
    eprintln!(
        "Usage: {} [OPTIONS]... PATTERN [FILES]..., {} [OPTIONS INCLUDING -p]... [FILES]...
Options:
    -h, --help: Display this message
    -p [PATTERN], --pattern [PATTERN]: Add pattern to match
    -t [NUMBER], --tab-width [NUMBER]: Set tab width for tree structure. Defaults to 4",
        env::args().next().expect(""),
        env::args().next().expect("")
    );
    std::process::exit(1);
}

fn main() {
    let mut tab_width = 4;
    let mut patterns: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    if env::args().len() <= 2 { //Progam name and file/pattern with no partner
        show_help();
    }
    let mut last_flag: String = String::from("");
    for arg in env::args().skip(1) { //Skip program name
        if arg.starts_with("-") {
            match arg.as_str() {
                "-h" | "--help" => show_help(),
                _ => last_flag = arg,
            }
        } else {
            match last_flag.as_str() {
                "-t" | "--tab_width" => {
                    tab_width = arg.parse().unwrap();
                }
                "-p" | "--pattern" => {
                    patterns.push(arg);
                }
                "" => {
                    if patterns.len() <= 0 || (files.len() <= 0 && !exists(&arg).unwrap()) {
                        patterns.push(arg);
                    } else {
                        files.push(arg);
                    }
                }
                _ => show_help(),
            }
        }
    }

    if files.len() <= 0 {
        let stdin = io::stdin();
        treegrep::grep(&patterns, "", stdin.lock(), tab_width);
        return;
    }

    let multiple = files.len() > 1;

    for file in &files {
        let path = Path::new(file);
        let f = File::open(path).unwrap_or_else(|e| {
            eprintln!("Error opening {}: {}", file, e);
            std::process::exit(1);
        });
        let filename = if multiple {
            path.to_str().unwrap_or(file)
        } else {
            ""
        };
        treegrep::grep(&patterns, filename, BufReader::new(f), tab_width);
    }
}
