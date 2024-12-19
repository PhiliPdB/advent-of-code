use std::collections::HashMap;

use regex::Regex;


/// Count the total number of arrangements of a towel
fn total_arrangements<'a>(towel: &'a str, rules: &HashMap<char, Vec<&str>>, cache: &mut HashMap<&'a str, u64>) -> u64 {
    if towel.is_empty() {
        return 1;
    }
    if let Some(&cached) = cache.get(towel) {
        return cached;
    }

    let Some(patterns) = rules.get(&towel.chars().next().unwrap()) else {
        return 0;
    };
    let mut arrangements = 0;

    for pattern in patterns {
        if let Some(remaining_towel) = towel.strip_prefix(pattern) {
            arrangements += total_arrangements(remaining_towel, rules, cache);
        }
    }

    cache.insert(towel, arrangements);
    arrangements
}

fn main() {
    let (patterns, designs) = include_str!("../input.txt")
        .split_once("\n\n")
        .unwrap();
    let patterns: Vec<_> = patterns.split(", ").collect();
    let designs: Vec<_> = designs.lines().collect();

    // Build regex
    let pattern_string = patterns.join("|");
    let regex = Regex::new(&format!("^({})*$", pattern_string)).unwrap();

    // Count possible designs (number of matches)
    let possible_designs = designs
        .iter()
        .filter(|design| regex.is_match(design))
        .count();
    println!("[Part 1] Possible designs: {possible_designs}");


    // Build rules hashmap
    let mut rules: HashMap<char, Vec<_>> = HashMap::new();
    for pattern in patterns {
        let first_char = pattern.chars().next().unwrap();
        rules.entry(first_char).or_default().push(pattern);
    }

    // Count total arrangements of each design
    let mut memoized = HashMap::new();
    let total_arrangements: u64 = designs.iter()
        .map(|design| total_arrangements(design, &rules, &mut memoized))
        .sum();
    println!("[Part 2] Total arrangements: {total_arrangements}");
}
