use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn rotate_right(&self, deg: i32) -> Self {
        unsafe { mem::transmute(((*self as u8) + (deg / 90) as u8) % 4) }
    }

    pub fn rotate_left(&self, deg: i32) -> Self {
        unsafe { mem::transmute(((*self as u8) - (deg / 90) as u8) % 4) }
    }
}

fn move_position(pos: &mut (i32, i32), direction: Direction, distance: i32) {
    match direction {
        Direction::North => pos.0 -= distance,
        Direction::East  => pos.1 += distance,
        Direction::South => pos.0 += distance,
        Direction::West  => pos.1 -= distance,
    }
}

fn move_to_waypoint(pos: &mut (i32, i32), (wp_x, wp_y): (i32, i32), n: i32) {
    pos.0 += n * wp_x;
    pos.1 += n * wp_y;
}

fn rotate_waypoint_counter_clockwise(wp: &mut (i32, i32), deg: i32) {
    for _ in 0..(deg / 90) {
        let tmp_wp0 = wp.0;
        wp.0 = -wp.1;
        wp.1 = tmp_wp0;
    }
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .collect();

    // Part 1

    let mut current_direction = Direction::East;
    let mut current_position = (0, 0);
    for instruction in &input {
        let (i, n) = instruction.split_at(1);
        let n = n.parse::<i32>().unwrap();
        match i {
            "N" => move_position(&mut current_position, Direction::North, n),
            "S" => move_position(&mut current_position, Direction::South, n),
            "E" => move_position(&mut current_position, Direction::East, n),
            "W" => move_position(&mut current_position, Direction::West, n),
            "L" => current_direction = current_direction.rotate_left(n),
            "R" => current_direction = current_direction.rotate_right(n),
            "F" => move_position(&mut current_position, current_direction, n),
            _   => panic!("Invalid instruction"),
        }
    }

    println!("[Part 1] Manhattan distance: {:#5}", current_position.0.abs() + current_position.1.abs());

    let mut waypoint_position = (-1, 10);
    let mut ship_position = (0, 0);
    for instruction in &input {
        let (i, n) = instruction.split_at(1);
        let n = n.parse::<i32>().unwrap();
        match i {
            "N" => move_position(&mut waypoint_position, Direction::North, n),
            "S" => move_position(&mut waypoint_position, Direction::South, n),
            "E" => move_position(&mut waypoint_position, Direction::East, n),
            "W" => move_position(&mut waypoint_position, Direction::West, n),
            "L" => rotate_waypoint_counter_clockwise(&mut waypoint_position, n),
            "R" => rotate_waypoint_counter_clockwise(&mut waypoint_position, 360 - n),
            "F" => move_to_waypoint(&mut ship_position, waypoint_position, n),
            _   => panic!("Invalid instruction"),
        }
    }

    println!("[Part 2] Manhattan distance: {:#5}", ship_position.0.abs() + ship_position.1.abs());
}
