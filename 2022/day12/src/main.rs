use std::collections::{VecDeque, HashSet};


struct BFSNode {
    position: (usize, usize),
    steps: u32,
}

impl BFSNode {
    fn new(position: (usize, usize), steps: u32) -> Self {
        Self { position, steps }
    }
}

pub fn shortest_path_length(start_location: (usize, usize), end_location: (usize, usize), elevation: &[Vec<u32>]) -> u32 {
    let width = elevation[0].len();
    let height = elevation.len();

    // Perform breath first search since all the step sizes are 1.

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(BFSNode::new(start_location, 0));

    while let Some(node) = queue.pop_front() {
        if node.position == end_location {
            return node.steps;
        }

        if !visited.insert(node.position) {
            continue;
        }

        // Generate new moves
        let current_elevation = elevation[node.position.0][node.position.1];

        if node.position.0 > 0 && elevation[node.position.0 - 1][node.position.1] <= current_elevation + 1 {
            queue.push_back(BFSNode::new((node.position.0 - 1, node.position.1), node.steps + 1));
        }

        if node.position.0 < height - 1 && elevation[node.position.0 + 1][node.position.1] <= current_elevation + 1 {
            queue.push_back(BFSNode::new((node.position.0 + 1, node.position.1), node.steps + 1));
        }

        if node.position.1 > 0 && elevation[node.position.0][node.position.1 - 1] <= current_elevation + 1 {
            queue.push_back(BFSNode::new((node.position.0, node.position.1 - 1), node.steps + 1));
        }

        if node.position.1 < width - 1 && elevation[node.position.0][node.position.1 + 1] <= current_elevation + 1 {
            queue.push_back(BFSNode::new((node.position.0, node.position.1 + 1), node.steps + 1));
        }
    }

    u32::MAX
}

fn main() {
    let mut start_location = (0, 0);
    let mut end_location = (0, 0);
    let elevation: Vec<_> = include_str!("../input.txt")
        .lines()
        .enumerate()
        .map(|(y, l)| l.chars()
            .enumerate()
            .map(|(x, c)| {
                if c == 'S' {
                    start_location = (y, x);
                    0
                } else if c == 'E' {
                    end_location = (y, x);
                    25
                } else {
                    c as u32 - 'a' as u32
                }
            })
            .collect::<Vec<_>>()
        )
        .collect();


    println!("[Part 1] Reached E in {} steps.", shortest_path_length(start_location, end_location, &elevation));

    // Part 2
    let mut min_path_length = u32::MAX;
    for y in 0..elevation.len() {
        for x in 0..elevation.len() {
            if elevation[y][x] == 0 {
                let steps = shortest_path_length((y, x), end_location, &elevation);
                if steps < min_path_length {
                    min_path_length = steps;
                }
            }
        }
    }
    println!("[Part 2] Minimum number of steps: {min_path_length}");
}
