use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {

    let input = File::open("../input.txt").expect("Could not read input");    
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();

    let mut total_contains: i32 = 0;
    let mut total_overlaps: i32 = 0;

    for line in lines {
        let assignments = line.expect("Could not parse line").trim().to_string();
        let assign_vec: Vec<&str> = assignments.split(',').collect();

        let first_assign = assign_vec[0];
        let first_vec: Vec<&str> = first_assign.split('-').collect();
        let first_start = first_vec[0].parse::<i8>().unwrap();
        let first_end = first_vec[1].parse::<i8>().unwrap();

        let second_assign = assign_vec[1];
        let second_vec: Vec<&str> = second_assign.split('-').collect();
        let second_start = second_vec[0].parse::<i8>().unwrap();
        let second_end = second_vec[1].parse::<i8>().unwrap();
        if (first_start <= second_start && first_end >= second_end) ||
           (second_start <= first_start && second_end >= first_end) {
            //println!("{}-{}, {}-{}", first_start, first_end, second_start, second_end);
            total_contains += 1;
        }
        if (first_start <= second_start && second_start <= first_end) || 
           (second_start <= first_start && first_start <= second_end ) {
            //println!("{}-{}, {}-{}", first_start, first_end, second_start, second_end);
            total_overlaps += 1;
     }

    }
    println!("The total of total assignment overlaps is {total_contains}.");
    println!("The total of partial assignment overlaps is {total_overlaps}.");
    println!("Elves are always looking for a way to get out of doing work.");
}
