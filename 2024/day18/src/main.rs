use std::collections::{HashSet, VecDeque};


const WIDTH: u32 = 71;
const HEIGHT: u32 = 71;


fn min_steps(fallen_bytes: &HashSet<(u32, u32)>) -> Option<u32> {
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));

    let mut visited = vec![vec![false; WIDTH as usize]; HEIGHT as usize];
    while let Some(((x, y), steps)) = queue.pop_front() {
        if x == WIDTH - 1 && y == HEIGHT - 1 {
            return Some(steps);
        }

        if visited[y as usize][x as usize] {
            continue;
        }
        visited[y as usize][x as usize] = true;

        if x > 0 && !fallen_bytes.contains(&(x - 1, y)) {
            queue.push_back(((x - 1, y), steps + 1));
        }
        if x < WIDTH - 1 && !fallen_bytes.contains(&(x + 1, y)) {
            queue.push_back(((x + 1, y), steps + 1));
        }
        if y > 0 && !fallen_bytes.contains(&(x, y - 1)) {
            queue.push_back(((x, y - 1), steps + 1));
        }
        if y < HEIGHT - 1 && !fallen_bytes.contains(&(x, y + 1)) {
            queue.push_back(((x, y + 1), steps + 1));
        }
    }

    None
}

fn main() {
    let falling_bytes: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x: u32 = x.parse().unwrap();
            let y: u32 = y.parse().unwrap();

            (x, y)
        })
        .collect();

    const PART1_BYTES: usize = 1024;
    let part1_bytes: HashSet<_> = falling_bytes
        .iter().take(PART1_BYTES)
        .cloned()
        .collect();
    println!("[Part 1] Steps to exit: {}", min_steps(&part1_bytes).unwrap());

    // Perform a binary search to find which byte will cut off the path
    let mut min_bytes = PART1_BYTES;
    let mut max_bytes = falling_bytes.len();
    while min_bytes < max_bytes {
        let mid_bytes = (max_bytes + min_bytes) / 2;
        let selected_bytes: HashSet<_> = falling_bytes
            .iter().take(mid_bytes)
            .cloned()
            .collect();

        if min_steps(&selected_bytes).is_some() {
            min_bytes = mid_bytes + 1;
        } else {
            max_bytes = mid_bytes;
        }
    }

    // Get the byte that causes the cutoff
    let (x, y) = falling_bytes[min_bytes - 1];
    println!("[Part 2] Byte causing cutoff: {x},{y}");
}
