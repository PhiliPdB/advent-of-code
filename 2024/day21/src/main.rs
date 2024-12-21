use std::collections::HashMap;

use keypad::{DirectionalKeypad, Keypad, NumericKeypad};

mod keypad;


fn shortest_path_length(
    cache: &mut HashMap<(Vec<char>, u32, u32), u64>,
    sequence: &[char], depth: u32, max_depth: u32
) -> u64 {
    let key = (sequence.to_vec(), depth, max_depth);
    if let Some(&length) = cache.get(&key) {
        return length;
    }

    let mut current = 'A';

    let mut length = 0;
    for next in sequence {
        let move_sequences =
            if depth == 0 {
                NumericKeypad::move_sequences(current, *next)
            } else {
                DirectionalKeypad::move_sequences(current, *next)
            };

        if depth >= max_depth {
            // At the limit, so count the sequence length
            length += move_sequences.iter()
                .min_by(|a, b| a.len().cmp(&b.len()))
                .unwrap()
                .len() as u64;
        } else {
            // Recurse to the next keypad
            length += move_sequences.iter()
                .map(|sequence| shortest_path_length(cache, sequence, depth + 1, max_depth))
                .min().unwrap();
        }
        current = *next;
    }

    cache.insert(key, length);
    length
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let multiplier: u64 = line[..3].parse().unwrap();
            let sequence: Vec<_> = line.chars().collect();
            (multiplier, sequence)
        })
        .collect();
    let mut cache = HashMap::new();

    // Part 1
    let part1_complexity: u64 = input.iter()
        .map(|(m, sequence)| *m * shortest_path_length(&mut cache, sequence, 0, 2))
        .sum();
    println!("[Part 1] Complexity: {part1_complexity:15}");

    // Part 2
    let part2_complexity: u64 = input.iter()
        .map(|(m, sequence)| *m * shortest_path_length(&mut cache, sequence, 0, 25))
        .sum();
    println!("[Part 2] Complexity: {part2_complexity:15}");
}
