use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

fn main() {
    let tabwidth = 4;

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1
        || args.iter().any(|e| {
            [String::from("-h"), String::from("--help")]
                .iter()
                .any(|a| e == a)
        })
    {
        eprintln!("Usage: {} [OPTION]... PATTERN [FILE]...", args[0]);
        std::process::exit(1);
    }

    let pattern = &args[1];

    if args.len() == 2 {
        let stdin = io::stdin();
        treegrep::grep(pattern, "", stdin.lock(), tabwidth);
        return;
    }

    let files = &args[2..];
    let multiple = files.len() > 1;

    for file in files {
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
        treegrep::grep(pattern, filename, BufReader::new(f), tabwidth);
    }
}
