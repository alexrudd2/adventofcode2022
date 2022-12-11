use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use ndarray::{Array2, s};


fn main() {
    let input = File::open("../input.txt").expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();
    let rows = lines.len();
    let cols: usize = <Vec<Result<String, std::io::Error>> as AsRef<Vec<_>>>::as_ref(&lines)[0].as_ref().unwrap().len();
    let mut forest = Array2::<i8>::default((rows, cols));
    let mut visibility = Array2::<i8>::zeros((rows, cols));
    let mut scenic = Array2::<i32>::zeros((rows, cols));

    let mut row: usize = 0;
    for l in lines {
        let line = l.expect("Could not parse line").trim().to_string();
        let mut col: usize = 0;
        for char in line.chars() {
            let num = char.to_digit(10).expect("Could not parse digit") as i8;
            forest[[row, col]] = num;
            col += 1;
        }
        row += 1;
    }

    let mut total_trees = 0;
    let mut max_scenic_score: i32 = 0;
    for row in 0..rows {
        for col in 0..cols {
            if row == 0 || col == 0 || row == rows - 1 || col == cols - 1{
                visibility[[row, col]] = 1; total_trees += 1;
            } else {
                let height = forest[[row, col]];
                let top_slice = forest.slice(s![0..row, col]);
                let bot_slice = forest.slice(s![row + 1..rows,  col]);
                let left_slice = forest.slice(s![row, 0..col]);
                let right_slice = forest.slice(s![row, col+1..cols]);
                let tallest_top = top_slice.iter().max().expect("Empty array");
                let tallest_bot = bot_slice.iter().max().expect("Empty array");
                let tallest_left = left_slice.iter().max().expect("Empty array");
                let tallest_right = right_slice.iter().max().expect("Empty array");
                if height > *tallest_top || height > *tallest_bot || height > *tallest_right || height > *tallest_left {
                    visibility[[row, col]] = height as i8; total_trees += 1;
                }
                let mut scenic_score: i32;
                let mut trees_visible = 0;
                for other_tree in top_slice.iter().rev() {
                    trees_visible += 1;
                    if height <= *other_tree { break }
                }
                scenic_score = trees_visible;

                trees_visible = 0;
                for other_tree in bot_slice.iter() {
                    trees_visible += 1;
                    if height <= *other_tree { break }
                }
                scenic_score = scenic_score * trees_visible;

                trees_visible = 0;
                for other_tree in left_slice.iter().rev() {
                    trees_visible += 1;
                    if height <= *other_tree { break }
                }
                scenic_score = scenic_score * trees_visible;

                trees_visible = 0;
                for other_tree in right_slice.iter() {
                    trees_visible += 1;
                    if height <= *other_tree { break }
                }
                scenic[[row, col]] = scenic_score;
                scenic_score = scenic_score * trees_visible;
                if scenic_score > max_scenic_score { max_scenic_score = scenic_score;}
            }
        }
    }
    println!("There are {total_trees} trees visible."); //1820
    println!("The most scenic has a score of {max_scenic_score}."); //385112
    println!("Perhaps if asked nicely the elves allow guests in their tree house?")

}
