#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    multiplier: i8,
    adder: i8,
    modulo: i8,
    true_target: usize,
    false_target: usize,
    inspected: i64,
}

fn main() {
    // example monkeys
    // let mut Dasher0 = Monkey {
    //     items: vec![79, 98],
    //     multiplier: 19,
    //     adder: 0,
    //     modulo: 23,
    //     true_target: 2,
    //     false_target: 3,
    //     inspected: 0,
    // };
    // let mut Dancer1 = Monkey {
    //     items: vec![54, 65, 75, 74],
    //     multiplier: 1,
    //     adder: 6,
    //     modulo: 19,
    //     true_target: 2,
    //     false_target: 0,
    //     inspected: 0,
    // };
    // let mut Prancer2 = Monkey {
    //     items: vec![79, 60, 97],
    //     multiplier: 1,
    //     adder: 0,
    //     modulo: 13,
    //     true_target: 1,
    //     false_target: 3,
    //     inspected: 0,
    // };
    // let mut Vixen3 = Monkey {
    //     items: vec![74],
    //     multiplier: 1,
    //     adder: 3,
    //     modulo: 17,
    //     true_target: 0,
    //     false_target: 1,
    //     inspected: 0,
    // };
    // let mut monkeys = [
    //     Dasher0, Dancer1, Prancer2, Vixen3,
    // ];

    let dasher0 = Monkey {
        items: vec![83, 88, 96, 79, 86, 88, 70],
        multiplier: 5,
        adder: 0,
        modulo: 11,
        true_target: 2,
        false_target: 3,
        inspected: 0,
    };
    let dancer1 = Monkey {
        items: vec![59, 63, 98, 85, 68, 72],
        multiplier: 11,
        adder: 0,
        modulo: 5,
        true_target: 4,
        false_target: 0,
        inspected: 0,
    };
    let prancer2 = Monkey {
        items: vec![90, 79, 97, 52, 90, 94, 71, 70],
        multiplier: 1,
        adder: 2,
        modulo: 19,
        true_target: 5,
        false_target: 6,
        inspected: 0,
    };
    let vixen3 = Monkey {
        items: vec![97, 55, 62],
        multiplier: 1,
        adder: 5,
        modulo: 13,
        true_target: 2,
        false_target: 6,
        inspected: 0,
    };
    let comet4 = Monkey {
        items: vec![74, 54, 94, 76],
        adder: 0,
        multiplier: 1, // actually it's old * old hard-coded in the inspect code)
        modulo: 7,
        true_target: 0,
        false_target: 3,
        inspected: 0,
    };
    let cupid5 = Monkey {
        items: vec![58],
        adder: 4,
        multiplier: 1,
        modulo: 17,
        true_target: 7,
        false_target: 1,
        inspected: 0,
    };
    let dunder6 = Monkey {
        items: vec![66, 63],
        adder: 6,
        multiplier: 1,
        modulo: 2,
        true_target: 7,
        false_target: 5,
        inspected: 0,
    };
    let blixem7 = Monkey {
        items: vec![56, 56, 90, 96, 68],
        adder: 7,
        multiplier: 1,
        modulo: 3,
        true_target: 4,
        false_target: 1,
        inspected: 0,
    };

    let mut monkeys = [
        dasher0, dancer1, prancer2, vixen3, comet4, cupid5, dunder6, blixem7,
    ];
    let mut monkeys2 = monkeys.clone();
    for _round in 1..=20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            for item in monkey.items {
                let mut new_item: i64 = 0;
                if i == 4 {
                    // fuck you Comet for old * old
                    new_item = (item * item) / 3;
                } else {
                    new_item = (item * monkey.multiplier as i64 + monkey.adder as i64) / 3;
                }
                if new_item % monkey.modulo as i64 == 0 {
                    monkeys[monkey.true_target].items.push(new_item);
                } else {
                    monkeys[monkey.false_target].items.push(new_item);
                }
                monkeys[i].inspected += 1;
            }
            monkeys[i].items.clear();
        }
    }
    let mut inspections = Vec::new();
    for monkey in &monkeys {
        inspections.push(monkey.inspected);
    }
    inspections.sort();
    let monkey_madness = inspections[6] * inspections[7];
    println!("Monkey business is {monkey_madness:?} after 20 rounds."); //64032

    //part 2
    let mut greatest_common_denominator: i64 = 1;
    for m in &monkeys2 {
        greatest_common_denominator *= m.modulo as i64;
    }
    for _round in 1..=10000 {
        for i in 0..monkeys2.len() {
            let monkey = monkeys2[i].clone();
            for item in monkey.items {
                let mut new_item: i64 = 0;
                if i == 4 {
                    // fuck you Comet for old * old
                    new_item = item * item;
                } else {
                    new_item = item * monkey.multiplier as i64 + monkey.adder as i64;
                }
                new_item = new_item % greatest_common_denominator;
                if new_item % monkey.modulo as i64 == 0 {
                    monkeys2[monkey.true_target].items.push(new_item);
                } else {
                    monkeys2[monkey.false_target].items.push(new_item);
                }
                monkeys2[i].inspected += 1;
            }
            monkeys2[i].items.clear();
        }
    }
    let mut inspections: Vec<i64> = Vec::new();
    for monkey in &monkeys2 {
        inspections.push(monkey.inspected);
    }
    inspections.sort();
    let monkey_madness = inspections[6] * inspections[7];
    println!("Monkey business is {monkey_madness:?} after 10,000 rounds of worrisome monkeys."); //12729522272
    println!("(This doesn't count their reindeer cosplay insanity...)");
}
