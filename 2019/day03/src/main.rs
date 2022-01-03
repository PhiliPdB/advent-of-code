use std::collections::{HashMap, hash_map::Entry};

fn get_wire_positions(wire: &[&str]) -> HashMap<(i32, i32), i32> {
    let mut wire_locations = HashMap::new();

    let mut current_pos = (0, 0);
    let mut current_steps = 0;
    for instruction in wire {
        let (i, dist) = instruction.split_at(1);
        let dist: i32 = dist.parse().unwrap();

        let (dx, dy) = match i {
            "R" => (1, 0),
            "U" => (0, 1),
            "L" => (-1, 0),
            "D" => (0, -1),
            _ => panic!("Invalid instruction"),
        };

        for i in 1..(dist + 1) {
            current_pos.0 += dx;
            current_pos.1 += dy;

            if let Entry::Vacant(e) = wire_locations.entry(current_pos) {
                e.insert(current_steps + i);
            }
        }

        current_steps += dist;
    }

    wire_locations
}

fn main() {
    let wires: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.split(',').collect::<Vec<_>>())
        .collect();


    let wire1_locations = get_wire_positions(&wires[0]);
    let wire2_locations = get_wire_positions(&wires[1]);
    let crossings: Vec<_> = wire1_locations.into_iter()
        .filter_map(|(pos, steps)| {
            wire2_locations.get(&pos)
                .map(|s| (pos, steps, *s))
        })
        .collect();


    let closest_crossing = crossings.iter()
        .map(|((x, y), _, _)| {
            let x: i32 = *x;
            let y: i32 = *y;

            x.abs() + y.abs()
        })
        .min().unwrap();

    println!("Closest crossing: {}", closest_crossing);


    let shortest_crossing = crossings.iter()
        .map(|(_, s1, s2)| s1 + s2)
        .min().unwrap();

    println!("Shortest crossing: {}", shortest_crossing);
}
