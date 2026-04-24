use std::env;
use std::fs::exists;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

fn main() {
    let mut tabwidth = 4;
    let mut patterns: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    let emptyarguments = env::args().len() <= 1;
    let mut settingtabwidth = false;
    let mut addingpattern = false;
    for arg in env::args() {
        if emptyarguments || [String::from("-h"), String::from("--help")].contains(&arg) {
            eprintln!(
                "Usage: {} [OPTIONS]... PATTERN [FILES]..., {} [OPTIONS INCLUDING -p]... [FILES]...
Options:
    -h, --help: Display this message
    -p [PATTERN], --pattern [PATTERN]: Add pattern to match
    -t [NUMBER], --tab-width [NUMBER]: Set tab width for tree structure",
                env::args().next().expect(""),
                env::args().next().expect("")
            );
            std::process::exit(1);
        } else if [String::from("-t"), String::from("--tab-width")].contains(&arg) {
            settingtabwidth = true;
        } else if settingtabwidth {
            settingtabwidth = false;
            tabwidth = arg.parse().unwrap();
        } else if [String::from("-p"), String::from("--pattern")].contains(&arg) {
            addingpattern = true;
        } else if patterns.len() <= 0
            || addingpattern
            || (files.len() <= 0 && !exists(&arg).unwrap())
        {
            addingpattern = false;
            patterns.push(arg);
        } else {
            files.push(arg);
        }
    }

    if files.len() <= 0 {
        let stdin = io::stdin();
        treegrep::grep(&patterns, "", stdin.lock(), tabwidth);
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
        treegrep::grep(&patterns, filename, BufReader::new(f), tabwidth);
    }
}
