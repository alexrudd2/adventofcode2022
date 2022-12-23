use ndarray::Array2;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input = File::open("../input.txt").expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();
    let rows: usize = 500;
    let cols: usize = 500;
    let mut head_x: usize = 250;
    let mut head_y: usize = 250;
    let mut tail_x: usize = 250;
    let mut tail_y: usize = 250;
    let mut locations = Array2::<i8>::zeros((rows, cols));
    let mut total_locations: i32 = 0;

    for l in lines {
        let line = l.expect("Could not parse line").trim().to_string();
        let line_vec: Vec<&str> = line.split(' ').collect();
        let direction = line_vec[0].chars().next().unwrap();
        let distance = line_vec[1].parse::<i8>().unwrap();
        for _ in 0..distance {
            if locations[[rows - tail_y - 1, tail_x]] == 0 {
                locations[[rows - tail_y - 1, tail_x]] = 1;
                total_locations += 1;
            }
            move_head(direction, &mut head_x, &mut head_y);
            move_tail(direction, head_x, head_y, &mut tail_x, &mut tail_y)
        }
    }
    println!("The tail of the snake visited {total_locations} locations.  Wonder if he got frequent-slider miles?");
    //6037
}
fn move_head(direction: char, x: &mut usize, y: &mut usize) {
    match direction {
        'U' => *y += 1,
        'D' => *y -= 1,
        'R' => *x += 1,
        'L' => *x -= 1,
        _ => return,
    }
}
fn move_tail(
    direction: char,
    head_x: usize,
    head_y: usize,
    tail_x: &mut usize,
    tail_y: &mut usize,
) {
    match direction {
        'U' => {
            if head_y > *tail_y + 1 {
                *tail_x = head_x;
                *tail_y = head_y - 1;
            }
        }
        'D' => {
            if head_y < *tail_y - 1 {
                *tail_x = head_x;
                *tail_y = head_y + 1;
            }
        }
        'R' => {
            if head_x > *tail_x + 1 {
                *tail_y = head_y;
                *tail_x = head_x - 1;
            }
        }
        'L' => {
            if head_x < *tail_x - 1 {
                *tail_y = head_y;
                *tail_x = head_x + 1;
            }
        }
        _ => return,
    }
}
