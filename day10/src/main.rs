use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input = File::open("../input.txt").expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();
    let mut x_register = vec![1];
    let mut i: usize = 0;
    for l in lines {
        i += 1;
        let line = l.as_ref().expect("Could not parse line").trim().to_string();
        let line_vec: Vec<&str> = line.split(' ').collect();
        let command = line_vec[0].chars().next().unwrap();
        if command == 'a' {
            let amount = line_vec[1].parse::<i8>().unwrap();
            x_register.push(*x_register.last().unwrap());
            x_register.push(*x_register.last().unwrap() + amount);
        } else {
            //noop
            x_register.push(*x_register.last().unwrap());
        }
    }
    let mut total_strength: i32 = 0;
    i = 20;
    while i < x_register.len() {
        total_strength += x_register[i - 1] as i32 * i as i32;
        i += 40;
    }
    println!("The total signal strength is {total_strength}."); //12640
    println!("Measured using an Elf-o-matic 4000.");
    println!("Image is below (you'll need an Elf-o-matic 5000 or 4000SE for color)");
    for i in 0..6 {
        for j in 0..40 {
            if (x_register[j + i * 40] - j as i8).abs() < 2 {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    // ####.#..#.###..####.#....###....##.###..
    // #....#..#.#..#....#.#....#..#....#.#..#.
    // ###..####.###....#..#....#..#....#.#..#.
    // #....#..#.#..#..#...#....###.....#.###..
    // #....#..#.#..#.#....#....#.#..#..#.#.#..
    // ####.#..#.###..####.####.#..#..##..#..#.
}
