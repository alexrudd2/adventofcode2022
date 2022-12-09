use core::slice::Windows;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input = File::open("../input.txt").expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();

    let comms_stream = lines[0].as_ref().expect("Could not parse input").as_bytes();

    let start_of_packet = find_unique(comms_stream, 4);
    let start_of_message = find_unique(comms_stream, 14);

    println!("The first start-of-packet is after character {start_of_packet}.");
    println!("The first start-of-message is after character {start_of_message}.");
    println!("What a ridiculous communica{{#`%${{%&`+'${{`%&NO CARRIER.");
}

fn find_unique(comms_stream: &[u8], size: usize) -> i32 {
    let groups = comms_stream.windows(size);
    let mut chars = size as i32 - 1;
    for group in groups {
        chars += 1;
        'wrapper: {
            for i in 0..size {
                for j in i + 1..size {
                    if group[i] == group[j] {
                        break 'wrapper;
                    }
                }
            }
            return chars;
        }
    }
    return 0;
}
