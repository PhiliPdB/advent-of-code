use std::mem;

const TOTAL_DAYS: i32 = 256;

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut current_fish = [0_u64; 9];
    // Fill current fish array
    for f in input {
        current_fish[f] += 1;
    }
    // Create temporary storage for the next iteration
    let mut next_fish = [0_u64; 9];

    for _current_day in 0..TOTAL_DAYS {
        // Let all fish decrease by 1 day.
        next_fish[0..8].copy_from_slice(&current_fish[1..9]);

        // Create new fish
        next_fish[8] = current_fish[0];

        // Reset fishes that reached day 0
        next_fish[6] += current_fish[0];

        // Push next_fish to current_fish by swapping the memory
        mem::swap(&mut current_fish, &mut next_fish);
    }

    println!("Total fish: {}", current_fish.iter().sum::<u64>());
}
