use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use ndarray::Array2;

fn main() {
    let input = File::open("../input.txt").expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();
    let rows = 162;
    let cols: usize = 55;
    let x_offset = 479; //493 for example
    let y_offset = 6;
    let mut cave = Array2::<i32>::default((rows, cols));
    for l in lines {
        let line = l.expect("Could not parse line").trim().to_string();
        let walls: Vec<_> = line.split(" -> ").collect();
        for i in 0..walls.len() - 1 {
            let mut wall_vec:Vec<_> = walls[i].split(',').collect();
            let start_x: i32 = wall_vec[0].parse::<i32>().unwrap() - x_offset;
            let start_y:i32 = wall_vec[1].parse::<i32>().unwrap() - y_offset;
            wall_vec = walls[i + 1].split(',').collect();
            let end_x: i32 = wall_vec[0].parse::<i32>().unwrap() - x_offset;
            let end_y:i32 = wall_vec[1].parse::<i32>().unwrap() - y_offset;
            if start_x == end_x && start_y < end_y {
                for y in start_y..=end_y {
                    cave[[y as usize, start_x as usize]] = 8;
                }
            } else if start_x == end_x && start_y > end_y {
                for y in end_y..=start_y {
                    cave[[y as usize, start_x as usize]] = 8;
                }
            } else if start_y == end_y && start_x < end_x {
                for x in start_x..=end_x {
                    cave[[start_y as usize, x as usize]] = 8;
                }
            } else if start_y == end_y && start_x > end_x {
                for x in end_x..=start_x {
                    cave[[start_y as usize, x as usize]] = 8;
                }
            }
        }
    }
    let mut total: i32 = 0;
    'outer: for i in 1..=1000 {
        let mut x = 500 - x_offset as usize; let mut y = 0;
        loop {
            if y + 1 == rows { 
                total = i - 1;
                break 'outer; 
            }
            if cave[[y + 1, x]] != 0 {
                if cave[[y + 1, x - 1]] == 0 {
                    x -= 1;
                } else if cave[[y + 1, x + 1]] == 0 {
                    x += 1;
                } else {
                    cave[[y, x]] = i;
                    break;
                }
            }
            y += 1;
        }
    }
    for j in 0..rows {
        for k in 0..cols {
            if cave[[j, k]] == 0 {
                print!(" ");
            } else if cave[[j, k]] == 8 {
                print!("▒");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!("\n{total} grains of sand fell in the cave.");
}