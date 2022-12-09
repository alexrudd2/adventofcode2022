use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let mut total_cals: Vec<i32> = Vec::new();

    let input = File::open("../input").expect("Could not read input");

    let reader = BufReader::new(input);

    let lines: Vec<_> = reader.lines().collect();
    let mut calories: i32 = 0;
    for line in lines {
        let cals: i32 = match line.expect("Could not parse line").trim().parse() {
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
    total_cals.sort_unstable();
    total_cals.reverse();
    //println!("{:?}", total_cals);
    let gold = total_cals[0];
    let silver = total_cals[1];
    let bronze = total_cals[2];
    let combined = gold + silver + bronze;
    println!("The elf with the most calories has {gold}.");
    println!("The elf with the next most calories has {silver}.");
    println!("The elf with the next most calories has {bronze}.");
    println!("Together the top three elves have {combined}.");
}
