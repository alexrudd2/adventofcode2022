use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {

    let mut total_cals: Vec<i32> = Vec::new();
    
    let input = File::open("../input").expect("Could not read input");
    
    let reader = BufReader::new(input);

    let lines: Vec<_> = reader.lines().collect();
    let mut calories: i32 = 0;
    for line in lines {
        let cals: i32 = match line.expect("Could not parse line").trim().parse() 
        {
            Ok(num) => num,
            Err(_) => 0,
        };
        if cals > 0 {
            calories += cals;
        } else {
            total_cals.push(calories);
            calories = 0;
        }
    }
    // println!("{:?}", total_cals);
    let max = total_cals.iter().max().unwrap();
    println!("The elf with the most calories has {max}.");
}
