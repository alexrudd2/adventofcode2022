use std::cmp::Ordering;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
enum LineType {
    LS,
    CD,
    DIR,
    FILE,
    ROOT,
}
struct FileNode {
    path: String,
    size: i32,
}

#[derive(Eq)]
struct DirNode {
    path: String,
    size: i32,
}
impl Ord for DirNode {
    fn cmp(&self, other: &DirNode) -> Ordering {
        self.size.cmp(&other.size)
    }
}
impl PartialOrd for DirNode {
    fn partial_cmp(&self, other: &DirNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for DirNode {
    fn eq(&self, other: &DirNode) -> bool {
        self.size == other.size
    }
}
fn main() {
    let input = File::open("../input.txt").expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();
    let mut current_path = String::new();
    let mut files: Vec<FileNode> = Vec::new();
    let mut dirs: Vec<DirNode> = Vec::new();
    dirs.push(DirNode {
        path: String::from("/"),
        size: 0,
    });

    for l in lines {
        let line = l.expect("Could not parse line").trim().to_string();
        let line_type = determine_type(&line);
        match line_type {
            LineType::CD => {
                let path = &line[5..];
                if path.contains("..") {
                    if current_path != "" {
                        let end = current_path.rfind("/").expect("Invalid path");
                        current_path = current_path[0..end].to_string();
                    }
                } else {
                    if current_path != "/" {
                        current_path.push_str(&String::from('/'));
                    }
                    current_path.push_str(path);
                    let dir = DirNode {
                        path: current_path.clone(),
                        size: 0,
                    };
                    dirs.push(dir);
                }
            }
            LineType::FILE => {
                let line_vec: Vec<&str> = line.split(' ').collect();
                let size = line_vec[0].parse::<i32>().unwrap();
                let path = current_path.clone() + "/" + line_vec[1];
                let file = FileNode { path, size };
                files.push(file);
            }
            LineType::ROOT => (),
            LineType::DIR => (),
            LineType::LS => (),
        }
    }
    let mut total_size = 0;
    for dir in &mut dirs {
        for file in &files {
            if file.path.starts_with(&dir.path) {
                dir.size += file.size;
            }
        }
        if dir.size <= 100000 {
            total_size += dir.size;
        }
    }
    println!("The total size of files under directories is {total_size}.");
    println!("Are elf files smaller than normal files?");

    let total_space = 70000000;
    let update_space = 30000000;
    let total_used = dirs[0].size;
    let required_deletes = update_space - (total_space - total_used);
    dirs.sort();
    for dir in dirs {
        if dir.size >= required_deletes {
            println!("To free enough space ({}), delete {}.", dir.size, dir.path);
            break;
        }
    }
    println!("It was probably junk, anyways.") //549173
}

fn determine_type(line: &str) -> LineType {
    if line == String::from("$ cd /") {
        return LineType::ROOT;
    }
    if line.contains("$") {
        if line.contains("$ ls") {
            return LineType::LS;
        } else {
            return LineType::CD;
        }
    } else {
        if line.contains("dir ") {
            return LineType::DIR;
        } else {
            return LineType::FILE;
        }
    }
}
