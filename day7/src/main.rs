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
struct DirNode {
    path: String,
    size: i32,
}
fn main() {
    let input = File::open("../input.txt").expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();
    let mut current_path = String::new();
    let mut path_indices: Vec<usize> = vec![0];
    let mut files: Vec<FileNode> = Vec::new();
    let mut dirs: Vec<DirNode> = Vec::new();
    dirs.push(DirNode{path: String::from('/'), size: 0});

    for l in lines {
        let line = l.expect("Could not parse line").trim().to_string();
        let line_type = determine_type(&line);
        match line_type {
            LineType::CD => {
                let path = &line[5..];
                if path.contains("..") {
                    path_indices.pop().expect("Empty path");
                    let end = path_indices.last();
                    current_path = match end {
                        Some(x) => current_path[0..=*x].to_string(),
                        None => {
                            path_indices.push(0);
                            String::from('/')
                        },
                    };
                    
                    if current_path.len() == 1 { // treat '/' as ''
                        current_path = String::new();
                    }
                } else {
                    current_path.push_str(&String::from('/'));
                    path_indices.push(path_indices.last().expect("Empty path") + path.len());
                    current_path.push_str(path);
                    let dir = DirNode{ path: current_path.clone(), size: 0};
                    dirs.push(dir);
                }
                //println!("{}", current_path);
            },
            LineType::FILE => {
                let line_vec: Vec<&str> = line.split(' ').collect();
                let size = line_vec[0].parse::<i32>().unwrap();
                let path = current_path.clone() + "/" + line_vec[1];
                //println!("{}: {}", size, path);
                let file = FileNode{path, size};
                files.push(file);
            }
            LineType::ROOT => (),
            LineType::DIR => (),
            LineType::LS => (),
        }
    }
    let mut total_size = 0;
    for mut dir in dirs {
        for file in &files {
            if file.path.starts_with(&dir.path) {
                println!("{}: {} ({})", &dir.path, file.path, file.size);
                dir.size += file.size;
            }
        }
        if dir.size <= 100000 {
            println!("> {}", dir.size);
            total_size += dir.size;
        }
        println!("{}: {}", dir.path, dir.size);
    }
    println!("The total size of files under directories is {total_size}.");
    println!("Are elf files smaller than normal files?");
}

fn determine_type(line: &str) -> LineType {
    if line == String::from("$ cd /") {
        return LineType::ROOT;
    }
    if line.contains("$") {
        if line.contains("ls") {
            return LineType::LS;
        } else {
            return LineType::CD;
        }
    } else {
        if line.contains("dir") {
            return LineType::DIR;
        } else {
            return LineType::FILE;
        }
    }
}
