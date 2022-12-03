use std::collections::HashSet;

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.chars().collect::<Vec<_>>()
        })
        .collect();

    // Part 1
    let priority_sum: i32 = input.iter()
        .map(|c| {
            let first_compartment: HashSet<_> = c[..c.len() / 2].iter().cloned().collect();
            let second_compartment: HashSet<_> = c[c.len() / 2..c.len()].iter().cloned().collect();

            first_compartment.intersection(&second_compartment)
                .map(|c| {
                    if c.is_uppercase() {
                        *c as i32 - 'A' as i32 + 27
                    } else {
                        *c as i32 - 'a' as i32 + 1
                    }
                })
                .sum::<i32>()
        })
        .sum();
    println!("[Part 1] Priority sum: {priority_sum}");

    // Part 2
    let badge_sum: i32 = input.chunks(3)
        .map(|elves| {
            let e0: HashSet<_> = elves[0].iter().cloned().collect();
            let e1: HashSet<_> = elves[1].iter().cloned().collect();
            let e2: HashSet<_> = elves[2].iter().cloned().collect();

            let common_e0_e1: HashSet<_> = e0.intersection(&e1).cloned().collect::<HashSet<_>>();
            common_e0_e1.intersection(&e2)
                .map(|c| {
                    if c.is_uppercase() {
                        *c as i32 - 'A' as i32 + 27
                    } else {
                        *c as i32 - 'a' as i32 + 1
                    }
                })
                .sum::<i32>()
        })
        .sum();
    println!("[Part 2] Badge sum: {badge_sum}");
}
