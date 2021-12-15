use std::collections::BinaryHeap;
use std::cmp::Ordering;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeapItem {
    position: (usize, usize),
    risk_level: u32,
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk_level.cmp(&self.risk_level)
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn add_item(
    heap: &mut BinaryHeap<HeapItem>,
    current_item: &HeapItem,
    map: &[Vec<u32>],
    risk_levels: &mut [Vec<u32>],
    x: usize, y: usize,
) {
    let new_risk_level = current_item.risk_level + map[y][x];
    if new_risk_level < risk_levels[y][x] {
        // Found a shorter way
        heap.push(HeapItem { position: (x, y), risk_level: new_risk_level });
        risk_levels[y][x] = new_risk_level;
    }
}

fn dijkstra(map: &[Vec<u32>], start: (usize, usize), end: (usize, usize), width: usize, height: usize) -> u32 {
    let mut risk_levels = vec![vec![u32::MAX; width]; height];
    let mut heap = BinaryHeap::new();

    risk_levels[start.1][start.0] = 0;
    heap.push(HeapItem { position: start, risk_level: 0 });

    while let Some(current_item) = heap.pop() {
        // Stop when we reach our end location
        if current_item.position == end {
            break;
        }

        let pos_x = current_item.position.0;
        let pos_y = current_item.position.1;
        // Already found a better path, so skip.
        if current_item.risk_level > risk_levels[pos_y][pos_x] {
            continue;
        }

        // Add new edges
        if pos_x > 0 {
            add_item(&mut heap, &current_item, map, &mut risk_levels, pos_x - 1, pos_y);
        }
        if pos_x < width - 1 {
            add_item(&mut heap, &current_item, map, &mut risk_levels, pos_x + 1, pos_y);
        }

        if pos_y > 0 {
            add_item(&mut heap, &current_item, map, &mut risk_levels, pos_x, pos_y - 1);
        }
        if pos_y < height - 1 {
            add_item(&mut heap, &current_item, map, &mut risk_levels, pos_x, pos_y + 1);
        }
    }

    risk_levels[end.1][end.0]
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect();

    let height = input.len();
    let width = input[0].len();

    // Create the extended graph for part 2
    let extended_input: Vec<_> = (0..5 * height)
        .map(|y| {
            (0..5 * width)
                .map(|x| {
                    let mut risk_level = input[y % height][x % width] + (y / height) as u32 + (x / width) as u32;
                    if risk_level > 9 {
                        risk_level -= 9;
                    }

                    risk_level
                })
                .collect::<Vec<_>>()
        })
        .collect();


    println!("[Part 1] Path risk: {:#4}", dijkstra(&input, (0, 0), (width - 1, height - 1), width, height));
    println!("[Part 2] Path risk: {:#4}", dijkstra(&extended_input, (0, 0), (5 * width - 1, 5 * height - 1), 5 * width, 5 * height));
}
