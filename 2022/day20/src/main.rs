use std::collections::VecDeque;

fn mix(numbers: &Vec<i64>, iterations: u32) -> i64 {
    let mut positions: VecDeque<_> = (0..numbers.len()).collect();

    for _ in 0..iterations {
        for (i, &n) in numbers.iter().enumerate() {
            if n == 0 {
                continue;
            }

            while positions[0] != i {
                positions.rotate_left(1);
            }

            let wrap = numbers.len() - 1;
            let tmp = positions.pop_front().unwrap();
            debug_assert_eq!(tmp, i);
            if n < 0 {
                positions.rotate_right(-n as usize % wrap);
            } else {
                positions.rotate_left(n as usize % wrap);
            }
            positions.push_front(tmp);
        }
    }

    let zero_pos = numbers.iter().position(|i| *i == 0).unwrap();
    let zero_pos = positions.iter().position(|i| *i == zero_pos).unwrap();
    let n1000 = (zero_pos + 1000) % positions.len();
    let n2000 = (zero_pos + 2000) % positions.len();
    let n3000 = (zero_pos + 3000) % positions.len();

    numbers[positions[n1000]] + numbers[positions[n2000]] + numbers[positions[n3000]]
}

fn main() {
    let numbers: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    println!("[Part 1] Sum of grove coordinate: {}", mix(&numbers, 1));
    let numbers = numbers.into_iter().map(|n| n * 811589153).collect();
    println!("[Part 2] Sum of grove coordinate: {}", mix(&numbers, 10));
}

