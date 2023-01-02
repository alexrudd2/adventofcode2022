use ndarray::Array2;
use std::cmp::{max, min};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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
            let mut wall_vec: Vec<_> = line_vec[i].split(',').collect();
            let start_x: i32 = wall_vec[0].parse::<i32>().unwrap();
            let start_y: i32 = wall_vec[1].parse::<i32>().unwrap();
            wall_vec = line_vec[i + 1].split(',').collect();
            let end_x: i32 = wall_vec[0].parse::<i32>().unwrap();
            let end_y: i32 = wall_vec[1].parse::<i32>().unwrap();
            max_x = max(max(max_x, start_x), end_x);
            max_y = max(max(max_y, start_y), end_y);
            min_x = min(min(min_x, start_x), end_x);
            min_y = min(min(min_y, start_y), end_y);
            walls.push((start_x, start_y, end_x, end_y));
        }
    }
    let mut rows = (max_y + 1) as usize;
    let mut cols: usize = (max_x - min_x + 2) as usize;
    let mut x_offset = min_x - 1; //493 for example
    let mut cave = Array2::<i32>::default((rows, cols));
    fill_walls(&mut cave, &walls, x_offset);

    // part 1
    let mut total = drop_sand(&mut cave, x_offset, rows);
    print_cave(&cave, rows, cols);
    println!("\n{total} grains of sand fell in the cave before falling into the abyss.");

    // part 2
    rows += 2;
    cols = rows * 3;
    x_offset -= rows as i32;
    cave = Array2::<i32>::default((rows, cols));
    fill_walls(&mut cave, &walls, x_offset);
    for i in 0..cols {
        cave[[rows - 1, i]] = 8;
    }
    total = drop_sand(&mut cave, x_offset, rows);
    print_cave(&cave, rows, cols);
    println!("\n{total} grains of sand fell in the cave with a floor.");
    println!("Did anyone bring skis?")
}

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

fn fill_walls(
    cave: &mut Array2<i32>,
    walls: &Vec<(i32, i32, i32, i32)>,
    x_offset: i32,
) {
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
}

fn drop_sand(cave: &mut Array2<i32>, x_offset: i32, rows: usize) -> i32 {
    for i in 1..=100000 {
        let mut x = 500 - x_offset as usize;
        let mut y: usize = 0;
        loop {
            if y + 1 == rows {
                //into the void
                return i - 1;
            }
            if cave[[y + 1, x]] != 0 {
                if cave[[y + 1, x - 1]] == 0 {
                    x -= 1;
                } else if cave[[y + 1, x + 1]] == 0 {
                    x += 1;
                } else if y == 0 {
                    //at the start
                    return i;
                } else {
                    cave[[y, x]] = 1;
                    break;
                }
            }
            y += 1;
        }
    }
    return 0;
}
