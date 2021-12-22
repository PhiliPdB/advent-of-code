use std::collections::{HashMap, HashSet};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    On,
    Off
}

#[derive(Debug, Clone, Copy)]
pub struct Command {
    command: Status,
    x_start: i32, x_end: i32,
    y_start: i32, y_end: i32,
    z_start: i32, z_end: i32,
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let (command, coords) = s.split_once(' ').unwrap();
            let command = match command {
                "on" => Status::On,
                "off" => Status::Off,
                _ => panic!("Invalid command"),
            };
            let coords: Vec<_> = coords.split(',')
                .flat_map(|c| {
                    c[2..].split("..").map(|n| n.parse::<i32>().unwrap())
                })
                .collect();

            Command {
                command,
                x_start: coords[0], x_end: coords[1],
                y_start: coords[2], y_end: coords[3],
                z_start: coords[4], z_end: coords[5],
            }
        })
        .collect();

    // let mut cube_status = HashMap::new();
    let mut on_cubes = HashSet::new();

    for command in &input {
        for x in command.x_start.max(-50)..=command.x_end.min(50) {
            for y in command.y_start.max(-50)..=command.y_end.min(50) {
                for z in command.z_start.max(-50)..=command.z_end.min(50) {
                    match command.command {
                        Status::On => on_cubes.insert((x, y, z)), //cube_status.insert((x, y, z), Status::On),
                        Status::Off => on_cubes.remove(&(x, y, z)), //cube_status.insert((x, y, z), Status::Off),
                    };
                }
            }
        }
    }

    // let on_cubes = cube_status.values()
    //     .filter(|s| **s == Status::On)
    //     .count();

    println!("On cubes: {}", on_cubes.len());
}
