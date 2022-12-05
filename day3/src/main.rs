use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use ascii_converter::string_to_decimals;

fn main() {

    let input = File::open("../input.txt").expect("Could not read input");    
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();
    let mut total_priorities: i32 = 0;
    for line in lines {
        let rucksack = line.as_ref().expect("Could not parse line").trim();
        let compartment_size = rucksack.len() / 2;
        let compartment1 = &rucksack[0..compartment_size];
        let compartment2 = &rucksack[(compartment_size)..rucksack.len()];
        for item in compartment1.chars() {
            if compartment2.contains(item) {
                let priority = calculate_priority(item);
                total_priorities += priority as i32;
                //println!("{} + {}: {}({})", compartment1, compartment2, item, priority);
                break;
            }
        }
    }
    println!("The total of item priorities that are in both compartments is {total_priorities}.")
    println!("Elf logistics are even worse than human logistics, it seems.");
}

fn calculate_priority(item_letter: char) -> u8{
    let ascii_code = string_to_decimals(&item_letter.to_string()).unwrap()[0];
    if ascii_code >= 65 && ascii_code <=90 { //A-Z have priority
        return ascii_code - (65 - 27);       // have priority 27 - 52
    } else if ascii_code >= 97 && ascii_code <= 122 { //a - z
        return ascii_code - (97 - 1);                //have priority 1 - 26
    } else { return 0};
}