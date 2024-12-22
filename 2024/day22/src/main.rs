use hashbrown::{HashMap, HashSet};


const SECRET_NUMBERS: u32 = 2000;

const fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

const fn prune(a: u64) -> u64 {
    a % 16777216
}

const fn next_number(secret_number: u64) -> u64 {
    let mut s = mix(secret_number, secret_number * 64);
    s = prune(s);

    s = mix(s, s / 32);
    s = prune(s);

    s = mix(s, s * 2048);
    s = prune(s);

    s
}

fn main() {
    let secret_numbers: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    // Keep track of total profit per sequence
    let mut sequence_profit_map: HashMap<[i32; 4], u64> = HashMap::new();

    let mut part1_sum = 0;
    for secret_number in &secret_numbers {
        let mut result = *secret_number;
        let mut seen = HashSet::new();

        let mut history = [0; 4];
        for i in 0..SECRET_NUMBERS {
            let next_result = next_number(result);
            let price = next_result % 10;
            let price_diff = price as i32 - (result % 10) as i32;

            if i >= 3 {
                history[3] = price_diff;

                if seen.insert(history) {
                    let current_profit = sequence_profit_map.entry(history).or_default();
                    *current_profit += price;
                }

                history.rotate_left(1);
            } else {
                history[i as usize] = price_diff;
            }
            result = next_result;
        }

        part1_sum += result;
    }

    println!("[Part 1] Sum of 2000th secret number: {part1_sum}");


    let (best_sequence, profit) = sequence_profit_map.iter()
        .max_by_key(|(_, &profit)| profit)
        .unwrap();
    println!("[Part 2] Max profit of {profit} with sequence {best_sequence:?}");
}
