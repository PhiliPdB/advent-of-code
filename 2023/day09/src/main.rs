use std::mem;


fn predict(history: Vec<i32>) -> (i32, i32) {
    let mut current = history;
    let mut differences = Vec::new();

    let mut first_values = vec![current[0]];
    let mut last_values = vec![current[current.len() - 1]];

    loop {
        let mut all_zero = true;
        for w in current.windows(2) {
            let diff = w[1] - w[0];
            differences.push(diff);
            all_zero &= diff == 0;
        }

        if all_zero {
            break;
        } else {
            first_values.push(differences[0]);
            last_values.push(differences[differences.len() - 1]);

            // Set the differences on current, and clear to start anew.
            mem::swap(&mut current, &mut differences);
            differences.clear();
        }
    }

    // Find new first
    for i in (0..(first_values.len() - 1)).rev() {
        first_values[i] -= first_values[i + 1];
    }

    (first_values[0], last_values.into_iter().sum())
}

fn main() {
    let histories = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        });
    let predictions: Vec<_> = histories
        .map(|h| predict(h))
        .collect();


    let sum_of_next: i32 = predictions.iter()
        .map(|h| h.1)
        .sum();
    println!("[Part 1] Sum of predictions: {sum_of_next:10}");

    let sum_of_previous: i32 = predictions.iter()
        .map(|h| h.0)
        .sum();
    println!("[Part 2] Sum of predictions: {sum_of_previous:10}");
}
