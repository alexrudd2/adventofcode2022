use ascii_converter::string_to_decimals;
use ndarray::{Array2, ArrayBase, Dim, OwnedRepr};
use pathfinding::prelude::bfs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn get_successors(
        &self,
        hill: &ArrayBase<OwnedRepr<i8>, Dim<[usize; 2]>>,
        rows: usize,
        cols: usize,
    ) -> Vec<Pos> {
        let mut successors = Vec::new();
        let &Pos(r, c) = self;
        if r > 0 && (hill[[r - 1, c]] <= hill[[r, c]] + 1) {
            successors.push(Pos(r - 1, c));
        }
        if r < rows - 1 && (hill[[r + 1, c]] <= hill[[r, c]] + 1) {
            successors.push(Pos(r + 1, c));
        }
        if c > 0 && (hill[[r, c - 1]] <= hill[[r, c]] + 1) {
            successors.push(Pos(r, c - 1));
        }
        if c < cols - 1 && (hill[[r, c + 1]] <= hill[[r, c]] + 1) {
            successors.push(Pos(r, c + 1));
        }
        successors
    }
}
fn main() {
    let input = File::open("../input.txt").expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();
    let rows = lines.len();
    let cols: usize = <Vec<Result<String, std::io::Error>> as AsRef<Vec<_>>>::as_ref(&lines)[0]
        .as_ref()
        .unwrap()
        .len();
    let mut hill = Array2::<i8>::default((rows, cols));

    let mut row: usize = 0;
    let mut start_pos = Pos(0, 0);
    let mut end_pos = Pos(0, 0);
    for l in lines {
        let line = l.expect("Could not parse line").trim().to_string();
        let mut col: usize = 0;
        for char in line.chars() {
            match char {
                'S' => {
                    start_pos = Pos(row, col);
                    hill[[row, col]] = 1; // a
                }
                'E' => {
                    end_pos = Pos(row, col);
                    hill[[row, col]] = 26; // z
                }
                _ => {
                    let num = string_to_decimals(&char.to_string()).unwrap()[0] as i8 - 96;
                    hill[[row, col]] = num;
                }
            }
            col += 1;
        }
        row += 1;
    }
    let result = bfs(
        &start_pos,
        |p| p.get_successors(&hill, rows, cols),
        |p| *p == end_pos,
    );
    let shortest_path = result.expect("no path found").len() - 1;
    println!(
        "The shortest path from Start at {:?} to End at {:?} is {}.",
        start_pos, end_pos, shortest_path
    ); //440
    println!("Wouldn't it be easier to borrow the sleigh and reindeer?");
}
