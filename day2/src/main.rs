use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {

    let input = File::open("../input.txt").expect("Could not read input");    
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();

    let mut total_score_part_1: i32 = 0;
    let mut total_score_part_2: i32 = 0;
    for line in lines {
        let round_score_1: i32 = match line.as_ref().expect("Could not parse line").trim() {
            "A X" => 3 + 1,  //rock = rock
            "B X" => 0 + 1,  //paper > rock
            "C X" => 6 + 1,  //scissors > rock
            "A Y" => 6 + 2,  //rock < paper
            "B Y" => 3 + 2,  //paper = paper
            "C Y" => 0 + 2,  //scissors > paper
            "A Z" => 0 + 3,  //rock > scissors
            "B Z" => 6 + 3,  //paper < scissors
            "C Z" => 3 + 3,  //scissors = scissors
            _ => 0,
        };
        let round_score_2: i32 = match line.expect("Could not parse line").trim() {
            "A X" => 0 + 3,  //rock > scissors
            "B X" => 0 + 1,  //paper > rock
            "C X" => 0 + 2,  //scissors > paper
            "A Y" => 3 + 1,  //rock = rock
            "B Y" => 3 + 2,  //paper = paper
            "C Y" => 3 + 3,  //scissors = scissors
            "A Z" => 6 + 2,  //rock < paper
            "B Z" => 6 + 3,  //paper < scissors
            "C Z" => 6 + 1,  //scissors < rock
            _ => 0,
        };
        total_score_part_1 += round_score_1;
        total_score_part_2 += round_score_2;
    };
    println!("Your total score for part 1 is {total_score_part_1}.");
    println!("Your total score for part 2 is {total_score_part_2}.");
    println!("You dirty, dirth match fixer.  Hope the snacks are worth it.")
}