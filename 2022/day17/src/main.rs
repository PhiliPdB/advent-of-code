use std::collections::VecDeque;


#[derive(Debug, Clone, Copy)]
enum Direction {
    Left, Right
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Unsupported char"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Rock, Air
}

const WIDTH: usize = 7;

fn tower_size<const N: usize>(input: Vec<Direction>) -> usize {
    let mut tower_size = 0;

    let input_len = input.len();
    let mut pattern = input.into_iter().cycle();
    let mut taken = 0;

    let mut grid: VecDeque<[Space; WIDTH]> = VecDeque::new();

    let mut tower_size_start = 0;
    let mut repeat_start = 0;

    let mut iteration = 0;
    while iteration < N {
        let mut rock = match iteration % 5 {
            0 => {
                vec![[Space::Air, Space::Air, Space::Rock, Space::Rock, Space::Rock, Space::Rock, Space::Air]]
            },
            1 => {
                vec![
                    [Space::Air, Space::Air, Space::Air,  Space::Rock, Space::Air,  Space::Air, Space::Air],
                    [Space::Air, Space::Air, Space::Rock, Space::Rock, Space::Rock, Space::Air, Space::Air],
                    [Space::Air, Space::Air, Space::Air,  Space::Rock, Space::Air,  Space::Air, Space::Air],
                ]
            },
            2 => {
                vec![
                    [Space::Air, Space::Air, Space::Rock, Space::Rock, Space::Rock, Space::Air, Space::Air],
                    [Space::Air, Space::Air, Space::Air,  Space::Air,  Space::Rock, Space::Air, Space::Air],
                    [Space::Air, Space::Air, Space::Air,  Space::Air,  Space::Rock, Space::Air, Space::Air],
                ]
            },
            3 => {
                vec![
                    [Space::Air, Space::Air, Space::Rock, Space::Air, Space::Air, Space::Air, Space::Air],
                    [Space::Air, Space::Air, Space::Rock, Space::Air, Space::Air, Space::Air, Space::Air],
                    [Space::Air, Space::Air, Space::Rock, Space::Air, Space::Air, Space::Air, Space::Air],
                    [Space::Air, Space::Air, Space::Rock, Space::Air, Space::Air, Space::Air, Space::Air],
                ]
            },
            4 => {
                vec![
                    [Space::Air, Space::Air, Space::Rock, Space::Rock, Space::Air, Space::Air, Space::Air],
                    [Space::Air, Space::Air, Space::Rock, Space::Rock, Space::Air, Space::Air, Space::Air],
                ]
            },
            _ => unreachable!(),
        };

        let mut height = grid.len() + 3;
        'moving_down: loop {
            taken += 1;
            match pattern.next().unwrap() {
                Direction::Left  => {
                    // Check if possible
                    let mut possible = true;
                    'left_search: for r in &rock {
                        if r[0] == Space::Rock {
                            possible = false;
                            break;
                        }

                        if height < grid.len() {
                            let max = (grid.len() - height).min(rock.len());
                            for (i, r) in rock[0..max].iter().enumerate() {
                                for s in 1..r.len() {
                                    if r[s - 1] == Space::Air && r[s] == Space::Rock {
                                        if grid[height + i][s - 1] == Space::Rock {
                                            possible = false;
                                            break 'left_search;
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    if possible {
                        for row in rock.iter_mut() {
                            row.rotate_left(1);
                        }
                    }
                },
                Direction::Right => {
                    // Check if possible
                    let mut possible = true;
                    'right_search: for r in &rock {
                        if r[6] == Space::Rock {
                            possible = false;
                            break 'right_search;
                        }

                        if height < grid.len() {
                            let max = (grid.len() - height).min(rock.len());
                            for (i, r) in rock[0..max].iter().enumerate() {
                                for s in (0..(r.len() - 1)).rev() {
                                    if r[s + 1] == Space::Air && r[s] == Space::Rock {
                                        if grid[height + i][s + 1] == Space::Rock {
                                            possible = false;
                                            break 'right_search;
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    if possible {
                        for row in rock.iter_mut() {
                            row.rotate_right(1);
                        }
                    }
                },
            }

            // Move down if possible
            if height == 0 {
                break 'moving_down;
            }

            // Check if we can move down
            for (i, r) in rock.iter().enumerate() {
                if height + i > grid.len() {
                    break;
                }

                for (j, s) in grid[height + i - 1].iter().enumerate() {
                    if r[j] == Space::Rock && *s == Space::Rock {
                        break 'moving_down;
                    }
                }
            }

            height -= 1;
        }

        for i in 0..rock.len() {
            if height + i >= grid.len() {
                grid.push_back([Space::Air; WIDTH]);
            }

            for (j, s) in grid[height + i].iter_mut().enumerate() {
                if rock[i][j] == Space::Rock {
                    debug_assert_ne!(*s, Space::Rock);
                    *s = Space::Rock;
                }
            }

        }

        // Check for each line if it is filled completely (tetris-like)
        for i in (0..rock.len()).rev() {
            if grid[height + i].iter().all(|s| *s == Space::Rock) {
                tower_size += height + i + 1;
                for _ in 0..(height + i + 1) {
                    grid.pop_front();
                }

                // Values found by trial and error on my input... :(
                if grid.len() == 0 && iteration % 5 == 0 && taken % input_len == 397 {
                    if repeat_start == 0 {
                        repeat_start = iteration;
                        tower_size_start = tower_size;
                    } else {
                        let repeat_size = iteration - repeat_start;
                        let tower_growth = tower_size - tower_size_start;

                        // Calculate to which iteration to jump to

                        let iterations_left = N - iteration;
                        let repetitions_left = iterations_left / repeat_size;

                        let total_repetitions = (N - repeat_start) / repeat_size;

                        tower_size += repetitions_left * tower_growth;
                        debug_assert_eq!(tower_size, tower_size_start + total_repetitions * tower_growth);

                        iteration += repetitions_left * repeat_size;
                        debug_assert_eq!(iteration, repeat_start + total_repetitions * repeat_size);
                    }
                }
                break;
            }
        }

        iteration += 1;
    }

    tower_size + grid.len()
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .chars()
        .map(Direction::from_char)
        .collect();

    println!("[Part 1] Height: {}", tower_size::<2022>(input.clone()));
    println!("[Part 2] Height: {}", tower_size::<1_000_000_000_000>(input.clone()));
}
