use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeapItem {
    position: (i32, i32),
    time: u32,
}

impl HeapItem {
    fn new(position: (i32, i32), time: u32) -> Self {
        Self { position, time }
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
            .then_with(|| self.position.1.cmp(&other.position.1))
            .then_with(|| self.position.0.cmp(&other.position.0))
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn has_blizzard(
    (y, x): (i32, i32), t: u32,
    (width, height): (i32, i32),
    ups: &Vec<Vec<bool>>, downs: &Vec<Vec<bool>>,
    lefts: &Vec<Vec<bool>>, rights: &Vec<Vec<bool>>
) -> bool {
    if y < 0 || y >= height {
        return false;
    }

    lefts[y as usize][(x as usize + t as usize) % width as usize]
        || rights[y as usize][(x - t as i32).rem_euclid(width) as usize]
        || downs[x as usize][(y - t as i32).rem_euclid(height) as usize]
        || ups[x as usize][(y as usize + t as usize) % height as usize]
}

fn dijkstra(
    start: (i32, i32), goal: (i32, i32), start_time: u32,
    (width, height): (i32, i32),
    ups: &Vec<Vec<bool>>, downs: &Vec<Vec<bool>>,
    lefts: &Vec<Vec<bool>>, rights: &Vec<Vec<bool>>
) -> u32 {
    let mut queue = BinaryHeap::new();
    queue.push(HeapItem::new(start, start_time));
    let mut distances = HashMap::new();

    while let Some(HeapItem { position: (y, x), time: t }) = queue.pop() {
        if (y, x) == goal {
            return t;
        }

        if let Some(distance) = distances.get(&((y, x), t)) {
            if *distance < t {
                continue;
            }
        }

        for (dy, dx) in [(1, 0), (0, 1), (0, -1), (-1, 0), (0, 0)] {
            let (new_y, new_x) = (y + dy, x + dx);
            // Don't move outside the grid except for the start and goal squares
            if new_x < 0 || new_x >= width || (new_y < 0 && !(new_y == -1 && new_x == 0)) || (new_y >= height && !(new_y == height && new_x == width - 1)) {
                continue;
            }
            // Can't move into a blizzard
            if has_blizzard((new_y, new_x), t + 1, (width, height), ups, downs, lefts, rights) {
                continue;
            }

            let distance = distances.entry(((new_y, new_x), t + 1)).or_insert(u32::MAX);
            if t + 1 < *distance {
                *distance = t + 1;
                queue.push(HeapItem::new((new_y, new_x), t + 1));
            }
        }
    }

    unreachable!()
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let width = input[0].len() as i32 - 2;
    let height = input.len() as i32 - 2;

    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    for y in 1..(height as usize + 1) {
        let mut left_row = Vec::new();
        let mut right_row = Vec::new();

        for x in 1..(width as usize + 1) {
            match input[y][x] {
                '<' => {
                    left_row.push(true);
                    right_row.push(false);
                },
                '>' => {
                    left_row.push(false);
                    right_row.push(true);
                },
                _ => {
                    left_row.push(false);
                    right_row.push(false);
                }
            }
        }

        lefts.push(left_row);
        rights.push(right_row);
    }
    let mut ups = Vec::new();
    let mut downs = Vec::new();
    for x in 1..(width as usize + 1) {
        let mut up_col = Vec::new();
        let mut down_col = Vec::new();

        for y in 1..(height as usize + 1) {
            match input[y][x] {
                '^' => {
                    up_col.push(true);
                    down_col.push(false);
                },
                'v' => {
                    up_col.push(false);
                    down_col.push(true);
                },
                _ => {
                    up_col.push(false);
                    down_col.push(false);
                }
            }
        }

        ups.push(up_col);
        downs.push(down_col);
    }

    let start = (-1, 0);
    let goal = (height, width - 1);

    let p1 = dijkstra(start, goal, 0, (width, height), &ups, &downs, &lefts, &rights);
    let p2 = dijkstra(goal, start, p1, (width, height), &ups, &downs, &lefts, &rights);
    let p3 = dijkstra(start, goal, p2, (width, height), &ups, &downs, &lefts, &rights);

    println!("[Part 1] Reached goal in {} minutes.", p1);
    println!("[Part 2] Reached goal in {} minutes.", p3);
}
