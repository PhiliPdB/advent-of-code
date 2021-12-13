use std::collections::HashMap;

const NUMBER: usize = 30_000_000;

fn main() {
    let input = [2, 20, 0, 4, 1, 17];

    let mut spoken: HashMap<_, _> = HashMap::from_iter(
        input.iter().enumerate()
            .map(|(i, n)| (*n, i + 1))
    );

    let mut turn = input.len() + 1;
    let mut next_number = 0;
    while turn < NUMBER {
        if let Some(last_turn) = spoken.insert(next_number, turn) {
            next_number = turn - last_turn;
        } else {
            next_number = 0;
        }

        turn += 1;
    }

    println!("Number: {}", next_number);
}
