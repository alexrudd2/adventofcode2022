use std::cmp::{min, max};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use ndarray::Array2;

fn main() {
    let input = File::open("../input.txt").expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();
    let mut walls: Vec<(i32, i32, i32, i32)> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 10000000;
    let mut min_y = 10000000;
    for l in lines {
        let line = l.expect("Could not parse line").trim().to_string();
        let line_vec: Vec<_> = line.split(" -> ").collect();
        for i in 0..line_vec.len() - 1 {
            let mut wall_vec:Vec<_> = line_vec[i].split(',').collect();
            let start_x: i32 = wall_vec[0].parse::<i32>().unwrap();
            let start_y:i32 = wall_vec[1].parse::<i32>().unwrap();
            wall_vec = line_vec[i + 1].split(',').collect();
            let end_x: i32 = wall_vec[0].parse::<i32>().unwrap();
            let end_y:i32 = wall_vec[1].parse::<i32>().unwrap();
            max_x = max(max(max_x, start_x), end_x);
            max_y = max(max(max_y, start_y), end_y);
            min_x = min(min(min_x, start_x), end_x);
            min_y = min(min(min_y, start_y), end_y);
        walls.push((start_x, start_y, end_x, end_y));
        }
    }
    let rows = (max_y + 1) as usize;
    let cols: usize = (max_x - min_x + 1) as usize;
    let x_offset = min_x; //493 for example
    // let y_offset = 0;
    let mut cave = Array2::<i32>::default((rows, cols));
    for wall in walls {
        let start_x = wall.0 - x_offset;
        let start_y = wall.1;
        let end_x = wall.2 - x_offset;
        let end_y = wall.3;
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
    print_cave(&cave, rows, cols);
}
//     let mut total: i32 = 0;
//     'outer: for i in 1..=1000 {
//         let mut x = 500 - x_offset as usize; let mut y = 0;
//         loop {
//             if y + 1 == rows { 
//                 total = i - 1;
//                 break 'outer; 
//             }
//             if cave[[y + 1, x]] != 0 {
//                 if cave[[y + 1, x - 1]] == 0 {
//                     x -= 1;
//                 } else if cave[[y + 1, x + 1]] == 0 {
//                     x += 1;
//                 } else {
//                     cave[[y, x]] = 1;
//                     break;
//                 }
//             }
//             y += 1;
//         }
//     }
//     print_cave(&cave, rows, cols);
//     println!("\n{total} grains of sand fell in the cave.");

//     // part 2
//     for i in 0..cols {
//         cave[[rows - 1, i]] = 8;
//     }

//     total = 0;
//     'outer: for i in 1..=21588 {
//         let mut x = 500 - x_offset as usize; let mut y = 0;
//         println!("{i}");
//         loop {
//             if cave[[y + 1, x]] != 0 {
//                 if cave[[y + 1, x - 1]] == 0 {
//                     x -= 1;
//                 } else if cave[[y + 1, x + 1]] == 0 {
//                     x += 1;
//                 } else if y == 0 {
//                     println!("BOOM");
//                     total = i - 1;
//                     break 'outer;
//                 } else {
//                     cave[[y, x]] = 1;
//                     break;
//                 }
//             }
//             y += 1;
//         }
//     }
//     println!("{total}");
//     print_cave(&cave, rows, cols);

// }

fn print_cave(cave: &Array2<i32>, rows: usize, cols: usize) {
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
}