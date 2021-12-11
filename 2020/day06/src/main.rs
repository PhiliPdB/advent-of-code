use std::collections::HashSet;

fn main() {
    let groups: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .collect();

    // Part 1
    let unique_answers: usize = groups.iter()
        .map(|g| {
            let person_answers: HashSet<char> = HashSet::from_iter(
                g.lines().flat_map(|l| l.chars())
            );

            person_answers.len()
        })
        .sum();

    println!("Sum of unique answers per group: {}", unique_answers);

    let answers: usize = groups.iter()
        .map(|g| {
            let all_answers: HashSet<char> = HashSet::from_iter(
                g.lines().flat_map(|l| l.chars())
            );

            g.lines().fold(all_answers, |acc, p| {
                acc.intersection(&HashSet::from_iter(p.chars()))
                    .cloned()
                    .collect::<HashSet<char>>()
            }).len()
        })
        .sum();

    println!("Sum of answers per group: {}", answers);
}
