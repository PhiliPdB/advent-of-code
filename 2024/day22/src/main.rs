const SECRET_NUMBERS: u32 = 2000;
const MAX_KEYS: usize = 19_usize.pow(4);


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


#[inline(always)]
fn to_int(digits: &[i32]) -> usize {
    digits.iter()
        .fold(0, |acc, &digit| acc * 19 + (digit + 9) as usize)
}

fn to_digits(mut n: usize) -> Vec<i32> {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % 19) as i32 - 9);
        n /= 19;
    }
    digits.reverse();
    digits
}


fn main() {
    let secret_numbers: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    // Keep track of total profit per sequence
    let mut sequence_profit_map = vec![0; MAX_KEYS];
    // Keep track of which sequences we have seen in an iteration (as only the first occurrence counts)
    let mut seen = vec![false; MAX_KEYS];
    let mut history = Vec::with_capacity(SECRET_NUMBERS as usize);

    let mut part1_sum = 0;
    for secret_number in &secret_numbers {
        let mut result = *secret_number;
        history.clear();

        for _ in 0..SECRET_NUMBERS {
            let next_result = next_number(result);
            let price = next_result % 10;
            let price_diff = price as i32 - (result % 10) as i32;

            history.push(price_diff);
            if history.len() >= 4 {
                let key = to_int(&history[history.len() - 4..]);
                let is_seen = unsafe { seen.get_unchecked_mut(key) };
                if !*is_seen {
                    sequence_profit_map[key] += price;
                    *is_seen = true;
                }
            }
            result = next_result;
        }

        // Add the resulting number to the part 1 answer
        part1_sum += result;

        // Reset seen array (this way we don't need to create it for every number)
        for s in seen.iter_mut() {
            *s = false;
        }
    }

    println!("[Part 1] Sum of 2000th secret number: {part1_sum}");


    let (best_sequence, profit) = sequence_profit_map.into_iter()
        .enumerate()
        .max_by_key(|(_, profit)| *profit)
        .unwrap();
    let best_sequence = to_digits(best_sequence);
    println!("[Part 2] Max profit of {profit} with sequence {best_sequence:?}");
}
