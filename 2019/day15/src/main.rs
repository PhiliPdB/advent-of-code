use std::collections::{VecDeque, HashSet};

use intcode::Program;

mod intcode;

fn new_position(dir: i64, (x, y): (i32, i32)) -> (i32, i32) {
    match dir {
        1 => (x, y - 1),
        2 => (x, y + 1),
        3 => (x - 1, y),
        4 => (x + 1, y),
        _ => panic!("Invalid direction")
    }
}

fn main() {
    let program = Program::new(
        include_str!("../input.txt")
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    );

    // Part 1

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((0, (0, 0), program));
    visited.insert((0, 0));

    let mut min_depth = i32::MAX;
    let mut final_position = (0, 0);
    let mut final_program = None;

    'bfs_loop: while let Some((depth, pos, p)) = queue.pop_front() {
        for dir in [1, 2, 3, 4] {
            let mut new_p = p.clone();
            let out = new_p.run(dir);

            match out {
                0 => continue,
                1 => {
                    let new_pos = new_position(dir, pos);
                    if !visited.insert(new_pos) {
                        continue;
                    }

                    queue.push_back((depth + 1, new_pos, new_p));
                },
                2 => {
                    min_depth = depth + 1;
                    final_position = pos;
                    final_program = Some(new_p);

                    break 'bfs_loop;
                }
                _ => panic!("Invalid output"),
            }
        }
    }

    println!("Minimum moves required: {}", min_depth);

    // Part 2

    queue.clear();
    queue.push_back((0, final_position, final_program.unwrap()));
    visited.clear();
    visited.insert(final_position);

    let mut max_depth = 0;

    while let Some((depth, pos, p)) = queue.pop_front() {
        if depth > max_depth {
            max_depth = depth;
        }

        for dir in [1, 2, 3, 4] {
            let mut new_p = p.clone();
            let out = new_p.run(dir);

            match out {
                0 => continue,
                1 | 2 => {
                    let new_pos = new_position(dir, pos);
                    if !visited.insert(new_pos) {
                        continue;
                    }

                    queue.push_back((depth + 1, new_pos, new_p));
                },
                _ => panic!("Invalid output"),
            }
        }
    }

    println!("Need {} minutes to fill the area", max_depth);
}
