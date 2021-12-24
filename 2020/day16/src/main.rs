use std::collections::{HashSet, HashMap};

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .collect();

    // Parse the tickets

    let my_ticket = input[1].lines()
        .skip(1)
        .map(|s| s.split(',').map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .next().unwrap();
    let mut nearby_tickets: Vec<_> = input[2].lines()
        .skip(1)
        .map(|s| s.split(',').map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .collect();

    // Keep track of all the possible numbers
    let mut seen_numbers = HashSet::with_capacity(1000);
    let mut all_labels = HashSet::new();
    let mut valid_numbers = HashMap::new();

    for l in input[0].lines() {
        let (label, ranges) = l.split_once(": ").unwrap();
        let (range1, range2) = ranges.split_once(" or ").unwrap();

        all_labels.insert(label);

        let range1: Vec<_> = range1.split('-').map(|n| n.parse::<u32>().unwrap()).collect();
        let range2: Vec<_> = range2.split('-').map(|n| n.parse::<u32>().unwrap()).collect();

        seen_numbers.extend(range1[0]..=range1[1]);
        seen_numbers.extend(range2[0]..=range2[1]);

        valid_numbers.insert(label, HashSet::new());
        valid_numbers.get_mut(label).unwrap().extend(range1[0]..=range1[1]);
        valid_numbers.get_mut(label).unwrap().extend(range2[0]..=range2[1]);
    }

    // Part 1

    let ticket_error_rate: u32 = nearby_tickets.iter()
        .flat_map(|ticket| {
            ticket.iter()
                .filter(|n| !seen_numbers.contains(*n))
        })
        .sum();

    println!("Ticket error rate: {}", ticket_error_rate);

    // Part 2

    // Discard invalid tickets
    nearby_tickets.retain(|ticket| {
        ticket.iter().all(|n| seen_numbers.contains(n))
    });

    // Calculate all the possible labels for each column
    let mut possible_labels = vec![all_labels.clone(); my_ticket.len()];
    for (index, pl) in possible_labels.iter_mut().enumerate() {
        pl.retain(|label| {
            nearby_tickets.iter()
                .map(|row| row[index])
                .all(|n| valid_numbers[*label].contains(&n))
        });
    }

    // Try to reduce the possibilities for each column by the process of elimination
    let mut made_changes = true;
    while made_changes {
        let mut to_remove = Vec::new();
        for (i, labels) in possible_labels.iter().enumerate() {
            if labels.len() == 1 {
                to_remove.push((i, <&str>::clone(labels.iter().next().unwrap())));
            }
        }

        made_changes = !to_remove.is_empty() && to_remove.len() != possible_labels.len();
        for (index, labels) in possible_labels.iter_mut().enumerate() {
            for (i, l) in &to_remove {
                if index != *i {
                    labels.retain(|label| label != l);
                }
            }
        }
    }

    // Multiply values of fields starting with departure
    let field_score = my_ticket.iter().enumerate()
        .fold(1, |acc, (i, v)| {
            if possible_labels[i].iter().next().unwrap().starts_with("departure") {
                acc * (*v as u64)
            } else {
                acc
            }
        });

    println!("Field score: {}", field_score);
}
