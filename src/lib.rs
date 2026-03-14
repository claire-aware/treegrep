use std::io::BufRead;
use regex::Regex;

fn indent_level(line: &str, tabwidth: usize) -> usize {
    let mut indent = 0;

    for ch in line.chars() {
        match ch {
            ' ' => indent += 1,
            '\t' => indent += tabwidth,
            _ => return indent,
        }
    }

    indent
}

pub fn grep<R: BufRead>(pattern: &str, filename: &str, reader: R, tabwidth: usize) {
    let mut queue: Vec<(usize, String)> = Vec::new();
    let mut tabs = vec![0usize];
    let mut line_no = 0;
    let mut last_indent = 0;

    let re = Regex::new(pattern).unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        line_no += 1;

        let cur_indent = indent_level(&line, tabwidth);
        let mut diff = cur_indent as isize - last_indent as isize;

        if diff == 0 {
            queue.pop();
            queue.push((line_no, line.clone()));
        } else if diff > 0 {
            queue.push((line_no, line.clone()));
            tabs.push(cur_indent - last_indent);
        } else {
            while diff < 0 {
                diff += tabs.pop().unwrap_or(0) as isize;
                queue.pop();
            }
            queue.pop();
            queue.push((line_no, line.clone()));
        }

        if re.is_match(&line) {
            for (line_no, line) in &queue {
                if filename.is_empty() {
                    println!("{}", line);
                } else {
                    println!("{}({}): {}", filename, line_no, line);
                }
            }
            queue.clear();
        }

        last_indent = cur_indent;
    }
}
