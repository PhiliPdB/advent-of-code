use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open, Wall, Wrap,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left, Right, Forward(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Facing {
    North, East, South, West,
}

impl Facing {
    fn rotate_left(&mut self) {
        *self = match self {
            Facing::North => Facing::West,
            Facing::East => Facing::North,
            Facing::South => Facing::East,
            Facing::West => Facing::South,
        }
    }

    fn rotate_right(&mut self) {
        *self = match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        }
    }

    fn next_coord(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Facing::North => (x, y - 1),
            Facing::East => (x + 1, y),
            Facing::South => (x, y + 1),
            Facing::West => (x - 1, y),
        }
    }

    fn value(&self) -> usize {
        match self {
            Facing::North => 3,
            Facing::East => 0,
            Facing::South => 1,
            Facing::West => 2,
        }
    }
}

fn main() {
    let mut grid = Vec::new();

    let (map, directions) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let map: Vec<_> = map.lines().collect();
    let direction_chars: Vec<_> = directions.chars().collect();

    let width = map.iter().map(|r| r.len()).max().unwrap();

    grid.push(vec![Tile::Wrap; width + 2]);
    for r in map {
        grid.push(vec![Tile::Wrap; width + 2]);
        let y = grid.len() - 1;
        for (x, t) in r.chars().enumerate() {
            match t {
                '.' => grid[y][x + 1] = Tile::Open,
                '#' => grid[y][x + 1] = Tile::Wall,
                ' ' => (),
                _ => unimplemented!(),
            }
        }
    }
    grid.push(vec![Tile::Wrap; width + 2]);

    let mut i = 0;
    let mut parsed_directions = Vec::new();
    while i < directions.len() {
        let mut j = i;
        while j < directions.len() && direction_chars[j].is_ascii_digit() {
            j += 1;
        }

        if j != i {
            parsed_directions.push(Direction::Forward(directions[i..j].parse().unwrap()));
            i = j;
        } else if direction_chars[i] == 'R' {
            parsed_directions.push(Direction::Right);
            i = j + 1;
        } else if direction_chars[i] == 'L' {
            parsed_directions.push(Direction::Left);
            i = j + 1;
        }
    }
    let directions = parsed_directions;

    let mut start_location = (0, 1);
    for x in 0..(width+2) {
        if grid[start_location.1][x] == Tile::Open {
            start_location.0 = x;
            break;
        }
    }
    let start_location = start_location;

    // Part 1

    let mut current_location = start_location;
    let mut current_facing = Facing::East;
    for d in &directions {
        match d {
            Direction::Left => current_facing.rotate_left(),
            Direction::Right => current_facing.rotate_right(),
            Direction::Forward(n) => {
                'forward: for _ in 0..*n {
                    let next = current_facing.next_coord(current_location);
                    // Check wrapping
                    match grid[next.1][next.0] {
                        Tile::Open => current_location = next,
                        Tile::Wall => break 'forward,
                        Tile::Wrap => {
                            match current_facing {
                                Facing::North => {
                                    for y in (0..grid.len()).rev() {
                                        match grid[y][next.0] {
                                            Tile::Open => {
                                                current_location = (next.0, y);
                                                continue 'forward;
                                            },
                                            Tile::Wall => {
                                                break 'forward;
                                            },
                                            Tile::Wrap => continue,
                                        }
                                    }
                                },
                                Facing::East => {
                                    for x in 0..(width + 2) {
                                        match grid[next.1][x] {
                                            Tile::Open => {
                                                current_location = (x, next.1);
                                                continue 'forward;
                                            },
                                            Tile::Wall => {
                                                break 'forward;
                                            },
                                            Tile::Wrap => continue,
                                        }
                                    }
                                },
                                Facing::South => {
                                    for y in 0..grid.len() {
                                        match grid[y][next.0] {
                                            Tile::Open => {
                                                current_location = (next.0, y);
                                                continue 'forward;
                                            },
                                            Tile::Wall => {
                                                break 'forward;
                                            },
                                            Tile::Wrap => continue,
                                        }
                                    }
                                },
                                Facing::West => {
                                    for x in (0..(width + 2)).rev() {
                                        match grid[next.1][x] {
                                            Tile::Open => {
                                                current_location = (x, next.1);
                                                continue 'forward;
                                            },
                                            Tile::Wall => {
                                                break 'forward;
                                            },
                                            Tile::Wrap => continue,
                                        }
                                    }
                                },
                            }
                        },
                    }
                }
            },
        }
    }
    println!("[Part 1] Password: {}", current_location.1 * 1000 + current_location.0 * 4 + current_facing.value());

    // Part 2

    // Create wrapping rules...
    let mut wrapping_rules = HashMap::new();
    for y in 1..51 {
        // 1 -> 4
        wrapping_rules.insert((50, y, Facing::West), (1, 151 - y, Facing::East));
        // 4 -> 1
        wrapping_rules.insert((0, 151 - y, Facing::West), (51, y, Facing::East));

        // 2 -> 5
        wrapping_rules.insert((151, y, Facing::East), (100, 151 - y, Facing::West));
        // 5 -> 2
        wrapping_rules.insert((101, 151 - y, Facing::East), (150, y, Facing::West));
    }
    for x in 51..101 {
        // 1 -> 6
        wrapping_rules.insert((x, 0, Facing::North), (1, 100 + x, Facing::East));
        // 6 -> 1
        wrapping_rules.insert((0, 100 + x, Facing::West), (x, 1, Facing::South));

        // 5 -> 6
        wrapping_rules.insert((x, 151, Facing::South), (50, 100 + x, Facing::West));
        // 6 -> 5
        wrapping_rules.insert((51, 100 + x, Facing::East), (x, 150, Facing::North));
    }
    for x in 101..151 {
        // 2 -> 3
        wrapping_rules.insert((x, 51, Facing::South), (100, x - 50, Facing::West));
        // 3 -> 2
        wrapping_rules.insert((101, x - 50, Facing::East), (x, 50, Facing::North));

        // 2 -> 6
        wrapping_rules.insert((x, 0, Facing::North), (x - 100, 200, Facing::North));
        // 6 -> 2
        wrapping_rules.insert((x - 100, 201, Facing::South), (x, 1, Facing::South));
    }
    for y in 51..101 {
        // 3 -> 4
        wrapping_rules.insert((50, y, Facing::West), (y - 50, 101, Facing::South));
        // 4 -> 3
        wrapping_rules.insert((y - 50, 100, Facing::North), (51, y, Facing::East));
    }

    debug_assert!(wrapping_rules.iter().all(|(k, _v)| grid[k.1][k.0] == Tile::Wrap));
    debug_assert!(wrapping_rules.iter().all(|(_k, v)| grid[v.1][v.0] != Tile::Wrap));


    current_location = start_location;
    current_facing = Facing::East;
    for d in &directions {
        match d {
            Direction::Left => current_facing.rotate_left(),
            Direction::Right => current_facing.rotate_right(),
            Direction::Forward(n) => {
                'forward: for _ in 0..*n {
                    let next = current_facing.next_coord(current_location);
                    // Check wrapping
                    match grid[next.1][next.0] {
                        Tile::Open => current_location = next,
                        Tile::Wall => break 'forward,
                        Tile::Wrap => {
                            let new_location = wrapping_rules[&(next.0, next.1, current_facing)];
                            match grid[new_location.1][new_location.0] {
                                Tile::Open => {
                                    current_location = (new_location.0, new_location.1);
                                    current_facing = new_location.2;
                                    continue 'forward;
                                },
                                Tile::Wall => {
                                    break 'forward;
                                },
                                Tile::Wrap => unreachable!(),
                            }
                        }
                    }
                }
            },
        }
    }
    println!("[Part 2] Password: {}", current_location.1 * 1000 + current_location.0 * 4 + current_facing.value());
}
