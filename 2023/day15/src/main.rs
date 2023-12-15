
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add(u32), Remove
}

fn ascii_hash(s: &str) -> u32 {
    s.chars()
        .map(|c| c as u32)
        .fold(0, |acc, c| {
            ((acc + c) * 17) % 256
        })
}

fn main() {
    let sequence: Vec<_> = include_str!("../input.txt").trim()
        .split(',')
        .collect();

    let hash_sum: u32 = sequence.iter()
        .map(|s| ascii_hash(s))
        .sum();
    println!("[Part 1] Sum of hashes: {hash_sum}");

    // Convert into operations
    let operations: Vec<_> = sequence.into_iter()
        .map(|s| {
            if s.contains('=') {
                let [label, n] = s.split('=').collect::<Vec<_>>()
                    .try_into().unwrap();

                (label.to_owned(), Operation::Add(n.parse().unwrap()))
            } else {
                (s[..s.len()-1].to_owned(), Operation::Remove)
            }
        })
        .collect();


    const TOTAL_BOXES: usize = 256;
    let mut boxes: Vec<Vec<(String, u32)>> = Vec::with_capacity(TOTAL_BOXES);
    for _ in 0..TOTAL_BOXES {
        boxes.push(Vec::new());
    }

    // Execute operations
    for (label, op) in operations {
        let i = ascii_hash(&label) as usize;
        match op {
            Operation::Add(n) => {
                if let Some(f) = boxes[i].iter_mut().find(|(l, _)| *l == label) {
                    f.1 = n;
                } else {
                    boxes[i].push((label, n));
                }
            },
            Operation::Remove => boxes[i].retain(|(l, _)| *l != label),
        }
    }

    let focussing_power: usize = boxes.iter().enumerate()
        .flat_map(|(b, items)| {
            items.iter().enumerate()
                .map(move |(s, (_, f))| (b + 1) * (s + 1) * *f as usize)
        })
        .sum();
    println!("[Part 2] Focussing power: {focussing_power}");
}
