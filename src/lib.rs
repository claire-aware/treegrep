use regex::Regex;
use std::collections::VecDeque;
use std::io::BufRead;

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

fn print_line(filename: &str, line_no: &usize, line: &String) {
    if filename.is_empty() {
        println!("{}", line);
    } else {
        println!("{}({}): {}", filename, line_no, line);
    }
}

fn handle_indent(
    filename: &str,
    tab_width: usize,
    max_siblings: usize,
    queues: &mut Vec<(
        usize,
        String,
        usize,
        VecDeque<(usize, String)>,
        Vec<(usize, String)>,
    )>,
    line_no: usize,
    last_indent: &mut usize,
    last_matched_indent: &mut usize,
    siblings_to_see: &mut usize,
    aunt_regexes: &Vec<Regex>,
    line: &String,
) {
    let cur_indent = indent_level(&line, tab_width);
    let mut diff = cur_indent as isize - *last_indent as isize;
    if diff != 0 {
        *siblings_to_see = 0
    }
    if diff > 0 {
        queues.push((
            line_no,
            line.clone(),
            (cur_indent as i32 - *last_indent as i32) as usize,
            VecDeque::new(),
            Vec::new(),
        ));
    } else {
        while diff < 0 && queues.len() > 0 {
            let (_last_line_no, _last_line, tab_count, _siblings, matched_aunts) =
                queues.pop().unwrap();
            diff += tab_count as isize;
            if *last_matched_indent >= cur_indent {
                for (line_no, line) in matched_aunts {
                    print_line(filename, &line_no, &line);
                }
            }
        }
        let (last_line_no, last_line, tab_count, mut siblings, mut matched_aunts) =
            queues.pop().unwrap_or_else(|| {
                (
                    0,
                    String::from(""),
                    (cur_indent as i32 - *last_indent as i32) as usize,
                    VecDeque::new(),
                    Vec::new(),
                )
            });
        if diff == 0 {
            siblings.push_back((last_line_no, last_line.clone()));
            if siblings.len() > max_siblings {
                siblings.pop_front();
            }
            if aunt_regexes.iter().any(|r| r.is_match(&last_line)) {
                matched_aunts.push((last_line_no, last_line));
            }
        }
        queues.push((line_no, line.clone(), tab_count, siblings, matched_aunts));
        *last_matched_indent = cur_indent;
    }
    *last_indent = cur_indent;
}

fn print_matches(
    filename: &str,
    max_siblings: usize,
    queues: &mut Vec<(
        usize,
        String,
        usize,
        VecDeque<(usize, String)>,
        Vec<(usize, String)>,
    )>,
    line_no: usize,
    siblings_to_see: &mut usize,
    match_regexes: &Vec<Regex>,
    line: String,
) {
    if match_regexes.iter().any(|r| r.is_match(&line)) {
        let mut iter = queues.iter();
        while iter.len() > 1 {
            let (line_no, line, _tab_count, _siblings, matched_aunts) = iter.next().unwrap();

            for (line_no, line) in matched_aunts {
                print_line(filename, line_no, line);
            }

            print_line(filename, line_no, line);
        }
        let (line_no, line, _tab_count, siblings, _matched_aunts) = iter.next().unwrap();
        for (line_no, line) in siblings {
            print_line(filename, line_no, line);
        }
        print_line(filename, line_no, line);
        queues.clear();
        *siblings_to_see = max_siblings;
    } else if *siblings_to_see > 0 {
        *siblings_to_see -= 1;
        print_line(filename, &line_no, &line);
    }
}

pub fn grep<R: BufRead>(
    patterns: &Vec<String>,
    aunts: &Vec<String>,
    filename: &str,
    reader: R,
    tab_width: usize,
    max_siblings: usize,
) {
    let mut queues: Vec<(
        usize,
        String,
        usize,
        VecDeque<(usize, String)>,
        Vec<(usize, String)>,
    )> = Vec::new(); // Line number, line, tab count, has matched children/siblings, siblings, matched aunts
    let mut line_no = 0;
    let mut last_indent = 0;
    let mut last_matched_indent = 0;
    let mut siblings_to_see = 0;

    let match_regexes = Vec::from_iter(patterns.iter().map(|pattern| Regex::new(pattern).unwrap()));
    let aunt_regexes = Vec::from_iter(aunts.iter().map(|pattern| Regex::new(pattern).unwrap()));

    for line in reader.lines() {
        let var_name = line.unwrap();
        let line = var_name;
        line_no += 1;

        handle_indent(
            filename,
            tab_width,
            max_siblings,
            &mut queues,
            line_no,
            &mut last_indent,
            &mut last_matched_indent,
            &mut siblings_to_see,
            &aunt_regexes,
            &line,
        );
        print_matches(
            filename,
            max_siblings,
            &mut queues,
            line_no,
            &mut siblings_to_see,
            &match_regexes,
            line,
        );
    }
}
