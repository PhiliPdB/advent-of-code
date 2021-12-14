use std::collections::HashMap;
use std::mem;

const STEPS: usize = 40;


fn perform_step(template: &mut HashMap<[char; 2], u64>, rules: &HashMap<[char; 2], char>) {
    let mut new_pairs: HashMap<[char; 2], u64> = HashMap::new();

    for (pair, count) in template.iter() {
        let item = rules[pair];
        *new_pairs.entry([pair[0], item]).or_default() += *count;
        *new_pairs.entry([item, pair[1]]).or_default() += *count;
    }

    mem::swap(template, &mut new_pairs);
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .collect();

    let template: Vec<_> = input[0].chars().collect();
    let rules: HashMap<_, _> = input[1].lines()
        .map(|s| {
            let splitted: Vec<_> = s.split(" -> ")
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect();

            let item = splitted[0].clone();
            ([item[0], item[1]], splitted[1][0])
        })
        .collect();

    // Convert template in pair counts
    let mut pair_counts: HashMap<[char; 2], u64> = HashMap::new();
    for pair in template.windows(2) {
        *pair_counts.entry([pair[0], pair[1]]).or_default() += 1;
    }

    // Perform the steps
    for _ in 0..STEPS {
        perform_step(&mut pair_counts, &rules);
    }

    // Count each character in the string
    let counts: HashMap<char, u64> = pair_counts.iter()
        .fold(HashMap::new(), |mut acc: HashMap<char, u64>, ([c1, c2], count)| {
            *acc.entry(*c1).or_default() += *count;
            *acc.entry(*c2).or_default() += *count;

            acc
        })
        .into_iter()
        .map(|(k, v)| {
            // Each character is counted double (expect start and end characters), so divide by 2 and ceil.
            (k, (v as f64 / 2_f64).ceil() as u64)
        })
        .collect();

    let least_common_count = counts.values().min().unwrap();
    let most_common_count = counts.values().max().unwrap();

    println!("Count: {}", most_common_count - least_common_count);
}
