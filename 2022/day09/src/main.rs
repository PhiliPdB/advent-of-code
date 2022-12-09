use std::collections::HashSet;


enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    pub fn from_char(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32, y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Move {
    direction: Direction,
    steps: i32,
}

impl Move {
    fn new(direction: Direction, steps: i32) -> Self {
        Self { direction, steps }
    }
}

fn update_knot(head: Coord, knot: &mut Coord) {
    if head.x - knot.x >= 2 {
        knot.x += 1;
        debug_assert_eq!(head.x, knot.x + 1);

        if head.y - knot.y >= 1 {
            knot.y += 1;
        } else if knot.y - head.y >= 1 {
            knot.y -= 1;
        }
    } else if knot.x - head.x >= 2 {
        knot.x -= 1;
        debug_assert_eq!(head.x, knot.x - 1);

        if head.y - knot.y >= 1 {
            knot.y += 1;
        } else if knot.y - head.y >= 1 {
            knot.y -= 1;
        }
    } else if head.y - knot.y >= 2 {
        knot.y += 1;
        debug_assert_eq!(head.y, knot.y + 1);

        if head.x - knot.x >= 1 {
            knot.x += 1;
        } else if knot.x - head.x >= 1 {
            knot.x -= 1;
        }
    } else if knot.y - head.y >= 2 {
        knot.y -= 1;
        debug_assert_eq!(head.y, knot.y - 1);

        if head.x - knot.x >= 1 {
            knot.x += 1;
        } else if knot.x - head.x >= 1 {
            knot.x -= 1;
        }
    }
}

fn main() {
    let moves: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (d, s) = l.split_once(' ').unwrap();
            Move::new(Direction::from_char(d.chars().next().unwrap()), s.parse().unwrap())
        })
        .collect();

    let mut tail_visited = HashSet::new();
    let mut tail_position = Coord::new(0, 0);
    let mut head_position = Coord::new(0, 0);

    tail_visited.insert(tail_position);
    for m in &moves {
        for _ in 0..m.steps {
            match m.direction {
                Direction::Up => {
                    head_position.y -= 1;

                    if tail_position.y - head_position.y >= 2 {
                        tail_position.y = head_position.y + 1;
                        tail_position.x = head_position.x;
                    }
                },
                Direction::Down => {
                    head_position.y += 1;

                    if head_position.y - tail_position.y >= 2 {
                        tail_position.y = head_position.y - 1;
                        tail_position.x = head_position.x;
                    }
                },
                Direction::Left => {
                    head_position.x += 1;

                    if head_position.x - tail_position.x >= 2 {
                        tail_position.x = head_position.x - 1;
                        tail_position.y = head_position.y;
                    }
                },
                Direction::Right => {
                    head_position.x -= 1;

                    if tail_position.x - head_position.x >= 2 {
                        tail_position.x = head_position.x + 1;
                        tail_position.y = head_position.y;
                    }
                },
            }

            tail_visited.insert(tail_position);
        }
    }

    println!("[Part 1] Total visited: {}", tail_visited.len());


    let mut visited = HashSet::new();
    let mut knot_positions = [Coord::new(0, 0); 10];
    visited.insert(knot_positions[9]);

    for m in &moves {
        for _ in 0..m.steps {
            match m.direction {
                Direction::Up    => knot_positions[0].y -= 1,
                Direction::Down  => knot_positions[0].y += 1,
                Direction::Left  => knot_positions[0].x += 1,
                Direction::Right => knot_positions[0].x -= 1,
            }

            for i in 1..knot_positions.len() {
                update_knot(knot_positions[i - 1], &mut knot_positions[i]);
            }

            visited.insert(knot_positions[9]);
        }
    }

    println!("[Part 2] Total visited: {}", visited.len());
}
