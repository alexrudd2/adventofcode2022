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
    let mut tail_x: Vec<usize> = vec![250, 250, 250, 250, 250, 250, 250, 250, 250];
    let mut tail_y: Vec<usize> = vec![250, 250, 250, 250, 250, 250, 250, 250, 250];
    let mut locations = Array2::<i8>::zeros((rows, cols));
    let mut total_locations_short: i32 = 0;
    let mut total_locations_long: i32 = 0;

    for l in lines {
        let line = l.expect("Could not parse line").trim().to_string();
        let line_vec: Vec<&str> = line.split(' ').collect();
        let direction = line_vec[0].chars().next().unwrap();
        let distance = line_vec[1].parse::<i8>().unwrap();
        for _ in 0..distance {
            if locations[[rows - tail_y[8] - 1, tail_x[8]]] == 0 {
                locations[[rows - tail_y[8] - 1, tail_x[8]]] = 1;
                total_locations_short += 1;
            }
            move_head(direction, &mut head_x, &mut head_y);
            for i in 0..9 {
                if i == 0 {
                    move_tail(head_x, head_y, &mut tail_x[i], &mut tail_y[i]);
                } else {
                    //print!("{i}: ");
                    move_tail(
                        tail_x[i - 1],
                        tail_y[i - 1],
                        &mut tail_x[i],
                        &mut tail_y[i],
                    );
                }
                if locations[[rows - tail_y[8] - 1, tail_x[8]]] == 0 {
                    locations[[rows - tail_y[8] - 1, tail_x[8]]] = 1;
                    total_locations_long += 1;
                }
            }
        }
    }
    println!("The tail of the short rope snake visited {total_locations_short} locations.");
    // 6037
    println!("The tail of the long rope snake visited {total_locations_long} locations.");
    // 2485
    println!("Wonder if they got frequent-slider miles?");
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
    head_x: usize,
    head_y: usize,
    tail_x: &mut usize,
    tail_y: &mut usize,
) {
    if head_y > *tail_y + 1 { //up
        if head_x > *tail_x { *tail_x += 1; }
        if head_x < *tail_x { *tail_x -= 1; }
        *tail_y += 1;
    }
    else if head_y < *tail_y - 1 { //down
        if head_x > *tail_x { *tail_x += 1; }
        if head_x < *tail_x { *tail_x -= 1; }
        *tail_y -= 1;
    }
    if head_x > *tail_x + 1 { // right
        if head_y > *tail_y { *tail_y += 1; }
        if head_y < *tail_y { *tail_y -= 1; }
        *tail_x += 1;
    }
    else if head_x < *tail_x - 1 { //left
        if head_y > *tail_y { *tail_y += 1; }
        if head_y < *tail_y { *tail_y -= 1; }
        *tail_x -= 1;
    }
}
