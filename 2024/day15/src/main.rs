use std::str::FromStr;

use map::Map;

mod map;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    Empty,
    Wall,
    Box, BoxOpen, BoxClose,
}

impl Space {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Space::Empty,
            '#' => Space::Wall,
            'O' => Space::Box,
            _ => panic!("Invalid space character: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn from_char(c: char) -> Self {
        match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("Invalid move character: {}", c),
        }
    }

    pub const fn direction(&self) -> (isize, isize) {
        match self {
            Move::Up => (0, -1),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
        }
    }
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .collect();

    let map = Map::from_str(input[0]).unwrap();
    let moves: Vec<_> = input[1].chars()
        .filter(|&c| c != '\n')
        .map(Move::from_char)
        .collect();


    // Part 1
    let mut part1_map = map.clone();
    for &m in &moves {
        part1_map.move_robot(m);
    }
    println!("[Part 1] Box GPS sum: {}", part1_map.box_gps_sum());


    // Part 2
    let mut part2_map = map.clone();
    part2_map.enlarge();

    for &m in &moves {
        part2_map.move_robot(m);
    }
    println!("[Part 2] Box GPS sum: {}", part2_map.box_gps_sum());

}
