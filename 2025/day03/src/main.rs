fn max_joltage(bank: &[u32], left: u32) -> u64 {
    if left == 0 {
        return 0;
    }

    let mut max = bank[0];
    let mut max_index = 0;
    for (i, battery) in bank
        .iter()
        .enumerate()
        .take(bank.len() - (left as usize - 1))
        .skip(1)
    {
        if *battery > max {
            max = *battery;
            max_index = i;
        }
    }
    let max = max as u64;

    max * 10_u64.pow(left - 1) + max_joltage(&bank[max_index + 1..], left - 1)
}

fn main() {
    let banks: Vec<Vec<_>> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let part1_joltage_sum: u64 = banks
        .iter()
        .map(|b| max_joltage(b, 2))
        .sum();
    println!("[Part 1] Joltage sum {part1_joltage_sum:15}");

    let part2_joltage_sum: u64 = banks
        .iter()
        .map(|b| max_joltage(b, 12))
        .sum();
    println!("[Part 2] Joltage sum {part2_joltage_sum:15}");
}
