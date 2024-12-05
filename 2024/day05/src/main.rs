use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;


fn is_correctly_ordered(printing_order: &[u32], is_before: &HashMap<u32, HashSet<u32>>) -> bool {
    for (i, order) in printing_order.iter().enumerate() {
        let Some(before) = is_before.get(order) else {
            continue;
        };

        if printing_order[..i].iter().any(|o| before.contains(o)) {
            return false;
        }
    }
    true
}

fn fix_ordering(printing_order: &[u32], is_before: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    // Collect the new build ordering
    let mut new_order = Vec::with_capacity(printing_order.len());

    'main: for &order in printing_order {
        let Some(before) = is_before.get(&order) else {
            new_order.push(order);
            continue;
        };

        for i in 0..new_order.len() {
            let o = new_order[i];
            if before.contains(&o) {
                // Current order needs to be before o, so put at this index
                new_order.insert(i, order);
                continue 'main;
            }
        }

        new_order.push(order);
    }

    new_order
}



fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .collect();

    //
    // Input parsing
    //

    let mut is_before: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in input[0].lines() {
        let (n1, n2) = rule.split_once('|').unwrap();
        let n1 = n1.parse().unwrap();
        let n2 = n2.parse().unwrap();

        match is_before.entry(n1) {
            Entry::Occupied(mut e) => {
                e.get_mut().insert(n2);
            },
            Entry::Vacant(e) => {
                let mut new_hashset = HashSet::new();
                new_hashset.insert(n2);
                e.insert(new_hashset);
            },
        }
    }

    let printing_orders: Vec<_> = input[1]
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();


    //
    // Solving
    //

    let part1_middle_page_sum: u32 = printing_orders.iter()
        .map(|order| {
            if is_correctly_ordered(order, &is_before) {
                order[order.len() / 2]
            } else {
                0
            }
        })
        .sum();
    println!("[Part 1] Correctly ordered middle page sum: {part1_middle_page_sum}");

    let part2_middle_page_sum: u32 = printing_orders.iter()
        .map(|order| {
            if !is_correctly_ordered(order, &is_before) {
                let new_order = fix_ordering(&order, &is_before);
                new_order[new_order.len() / 2]
            } else {
                0
            }
        })
        .sum();
    println!("[Part 2] Fixed order middle page sum: {part2_middle_page_sum}");
}
