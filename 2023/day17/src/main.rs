use std::collections::BinaryHeap;
use std::cmp::Ordering;

use hashbrown::HashSet;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Heading {
    North, East, South, West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DijkstraNode {
    heat_loss: u32,
    position: (usize, usize),
    heading: Heading,
    in_same_direction: u32,
}

impl DijkstraNode {
    fn new(heat_loss: u32, position: (usize, usize), heading: Heading, in_same_direction: u32) -> Self {
        Self {
            heat_loss,
            position,
            heading,
            in_same_direction,
        }
    }
}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

fn heat_loss<const MAX_IN_SAME_DIRECTION: u32, const TURN_RADIUS: u32>(map: &Vec<Vec<u32>>) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(DijkstraNode::new(0, (0, 0), Heading::East, 0));

    let width = map[0].len();
    let height = map.len();
    while let Some(DijkstraNode { heat_loss, position: (x, y), heading, in_same_direction }) = heap.pop() {
        if (x, y) == (width - 1, height - 1) && in_same_direction > TURN_RADIUS {
            return heat_loss;
        }

        if !visited.insert(((x, y), heading, in_same_direction)) {
            continue;
        }

        for direction in [Heading::North, Heading::East, Heading::South, Heading::West] {
            let turning = heading != direction;
            if (!turning && in_same_direction == MAX_IN_SAME_DIRECTION)
                || (turning && in_same_direction > MAX_IN_SAME_DIRECTION - TURN_RADIUS)
            {
                continue;
            }

            match direction {
                Heading::North => {
                    if heading == Heading::South || y == 0 {
                        continue;
                    }

                    if turning {
                        if let Some((h, (x, y))) = turn::<TURN_RADIUS>(map, (x, y), heading, direction) {
                            heap.push(DijkstraNode::new(
                                heat_loss + h, (x, y),
                                Heading::North, 1
                            ));
                        }
                    } else {
                        heap.push(DijkstraNode::new(
                            heat_loss + map[y - 1][x], (x, y - 1),
                            Heading::North, in_same_direction + 1
                        ));
                    }
                },
                Heading::East => {
                    if heading == Heading::West || x == width - 1 {
                        continue;
                    }

                    if turning {
                        if let Some((h, (x, y))) = turn::<TURN_RADIUS>(map, (x, y), heading, direction) {
                            heap.push(DijkstraNode::new(
                                heat_loss + h, (x, y),
                                Heading::East, 1
                            ));
                        }
                    } else {
                        heap.push(DijkstraNode::new(
                            heat_loss + map[y][x + 1], (x + 1, y),
                            Heading::East, in_same_direction + 1
                        ));
                    }
                },
                Heading::South => {
                    if heading == Heading::North || y == height - 1 {
                        continue;
                    }

                    if turning {
                        if let Some((h, (x, y))) = turn::<TURN_RADIUS>(map, (x, y), heading, direction) {
                            heap.push(DijkstraNode::new(
                                heat_loss + h, (x, y),
                                Heading::South, 1
                            ));
                        }
                    } else {
                        heap.push(DijkstraNode::new(
                            heat_loss + map[y + 1][x], (x, y + 1),
                            Heading::South, in_same_direction + 1
                        ));
                    }
                },
                Heading::West => {
                    if heading == Heading::East || x == 0 {
                        continue;
                    }

                    if turning {
                        if let Some((h, (x, y))) = turn::<TURN_RADIUS>(map, (x, y), heading, direction) {
                            heap.push(DijkstraNode::new(
                                heat_loss + h, (x, y),
                                Heading::West, 1
                            ));
                        }
                    } else {
                        heap.push(DijkstraNode::new(
                            heat_loss + map[y][x - 1], (x - 1, y),
                            Heading::West, in_same_direction + 1
                        ));
                    }
                },
            }
        }
    }

    unreachable!()
}

fn turn<const TURN_RADIUS: u32>(map: &Vec<Vec<u32>>, (x, y): (usize, usize), heading: Heading, new_heading: Heading) -> Option<(u32, (usize, usize))> {
    let mut heat_loss = 0;
    let (mut x, mut y) = (x, y);

    let (dx, dy) = match heading {
        Heading::North => (0, -1),
        Heading::East => (1, 0),
        Heading::South => (0, 1),
        Heading::West => (-1, 0),
    };

    for _ in 0..TURN_RADIUS {
        x = ((x as i32) + dx) as usize;
        y = ((y as i32) + dy) as usize;

        if x >= map[0].len() || y >= map.len() {
            return None;
        }

        heat_loss += map[y][x];
    }

    match new_heading {
        Heading::North => y -= 1,
        Heading::East => x += 1,
        Heading::South => y += 1,
        Heading::West => x -= 1,
    }
    heat_loss += map[y][x];

    Some((heat_loss, (x, y)))
}

fn main() {
    let map: Vec<Vec<_>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    println!("[Part 1] Heat loss: {}", heat_loss::< 3, 0>(&map));
    println!("[Part 2] Heat loss: {}", heat_loss::<10, 3>(&map));
}
