use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum Line_type {
    LS,
    CD,
    DIR,
    FILE,
}
struct Node {
    path: &String,
    children: &Vec<_>,
}
fn main() {
    let input = File::open("../input.txt").expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();

    for l in lines {
        let line = l.expect("Could not parse line").trim().to_string();
        let line_type = determine_type(&line);
    }
}

fn determine_type(line: &str) -> Line_type {
    if line.contains("$") {
        if line.contains("LS") {
            return Line_type::LS;
        } else {
            return Line_type::CD;
        }
    } else {
        if line.contains("dir") {
            return Line_type::DIR;
        } else {
            return Line_type::FILE;
        }
    }
}
