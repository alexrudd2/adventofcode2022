use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum Crane {
    CrateMover9000,
    CrateMover9001,
}
fn main() {
    let input = File::open("../input.txt").expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();

    let stack1 = String::from("RNFVLJSM");
    let stack2 = String::from("PNDZFJWH");
    let stack3 = String::from("WRCDG");
    let stack4 = String::from("NBS");
    let stack5 = String::from("MZWPCBFN");
    let stack6 = String::from("PRMW");
    let stack7 = String::from("RTNGLSW");
    let stack8 = String::from("QTHFNBV");
    let stack9 = String::from("MLHZNF");
    let mut stacks = vec![
        stack1, stack2, stack3, stack4, stack5, stack6, stack7, stack8, stack9,
    ];

    let crane = Crane::CrateMover9000;

    for line in lines {
        let rearrangement = line.expect("Could not parse line").trim().to_string();
        if !rearrangement.contains("move") {
            //ignore intial stack defintion
            continue;
        }
        let rearrangement_vec: Vec<&str> = rearrangement.split(' ').collect();
        let quantity = rearrangement_vec[1].parse::<i8>().unwrap();
        let origin = rearrangement_vec[3].parse::<i8>().unwrap() as usize;
        let destination = rearrangement_vec[5].parse::<i8>().unwrap() as usize;
        move_crates(&crane, &mut stacks, origin, destination, quantity);
    }
    println!("\nThese elves should really build stronger crate to avoid this nonsense.");
    print!("The top crates are ");
    for stack in &stacks {
        let top = stack.chars().last().unwrap();
        print!("{top}");
    }
    println!(".");
}

fn move_crates(
    crane: &Crane,
    stacks: &mut Vec<String>,
    origin: usize,
    destination: usize,
    mut quantity: i8,
) {
    match crane {
        Crane::CrateMover9000 => {
            while quantity > 0 {
                let _crate: char = stacks[origin - 1].pop().expect("empty stack");
                stacks[destination - 1].push(_crate);
                quantity -= 1;
            }
        }
        Crane::CrateMover9001 => {
            let split = stacks[origin - 1].len() - quantity as usize;
            let _crates = stacks[origin - 1].split_off(split);
            stacks[destination - 1].push_str(&_crates);
        }
    }
}
