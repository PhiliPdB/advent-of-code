use std::collections::BinaryHeap;
use std::cmp::Ordering;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeapItem {
    position: (usize, usize),
    risk_level: u32,
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.risk_level.partial_cmp(&self.risk_level)
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk_level.cmp(&self.risk_level)
    }
}


fn add_item(heap: &mut BinaryHeap<HeapItem>, current_item: &HeapItem, input: &[Vec<u32>], risk_levels: &mut [Vec<u32>], x: usize, y: usize) {
    let new_risk_level = current_item.risk_level + input[y][x];
    if new_risk_level < risk_levels[y][x] {
        // Found a shorter way
        heap.push(HeapItem { position: (x, y), risk_level: new_risk_level });
        risk_levels[y][x] = new_risk_level;
    }
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect();

    let height = input.len();
    let width = input[0].len();

    let mut risk_levels = vec![vec![u32::MAX; width]; height];
    let mut heap = BinaryHeap::new();
    // for y in 0..height {
    //     for x in 0..width {
    //         heap.push(HeapItem { node: (x, y), risk_level: input[y][x] });
    //     }
    // }

    risk_levels[0][0] = 0;
    heap.push(HeapItem { position: (0, 0), risk_level: 0 });

    while let Some(current_item) = heap.pop() {
        if current_item.position == (width - 1, height - 1) {
            break;
        }

        let pos_x = current_item.position.0;
        let pos_y = current_item.position.1;
        if current_item.risk_level > risk_levels[pos_y][pos_x] {
            continue;
        }

        // Add new edges
        if pos_x > 0 {
            add_item(&mut heap, &current_item, &input, &mut risk_levels, pos_x - 1, pos_y);
        }
        if pos_x < width - 1 {
            add_item(&mut heap, &current_item, &input, &mut risk_levels, pos_x + 1, pos_y);
        }

        if pos_y > 0 {
            add_item(&mut heap, &current_item, &input, &mut risk_levels, pos_x, pos_y - 1);
        }
        if pos_y < height - 1 {
            add_item(&mut heap, &current_item, &input, &mut risk_levels, pos_x, pos_y + 1);
        }
    }

    println!("Path risk: {}", risk_levels[height - 1][width - 1]);
}
