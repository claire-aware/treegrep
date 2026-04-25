use std::env;
use std::fs::exists;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

fn show_help() {
    eprintln!(
        "Usage: {} [OPTIONS]... PATTERN [FILES]..., {} [OPTIONS INCLUDING -m]... [FILES]...
Options:
    -h, --help: Display this message
    -m PATTERN, --match PATTERN: Add pattern to match
    -t NUMBER, --tab-width NUMBER: Set tab width for tree structure. Defaults to 4
    -a PATTERN, --aunt PATTERN: Match pattern when sibling of parent of a regular match
    -s NUMBER, --siblings NUMBER: Show n siblings before and after match",
        env::args().next().expect(""),
        env::args().next().expect("")
    );
    std::process::exit(1);
}

fn main() {
    let mut tab_width = 4;
    let mut siblings = 0;
    let mut patterns: Vec<String> = Vec::new();
    let mut aunts: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    if env::args().len() <= 2 {
        //Progam name and file/pattern with no partner
        show_help();
    }
    let mut last_flag: String = String::from("");
    for arg in env::args().skip(1) {
        //Skip program name
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
                "-m" | "--match" => {
                    patterns.push(arg);
                }
                "-a" | "--aunt" => {
                    aunts.push(arg);
                }
                "-s" | "--siblings" => {
                    siblings = arg.parse().unwrap();
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
            last_flag = String::from("");
        }
    }

    if files.len() <= 0 {
        let stdin = io::stdin();
        treegrep::grep(&patterns, &aunts, "", stdin.lock(), tab_width, siblings);
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
        treegrep::grep(
            &patterns,
            &aunts,
            filename,
            BufReader::new(f),
            tab_width,
            siblings,
        );
    }
}
