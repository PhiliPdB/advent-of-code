use std::collections::{HashSet, VecDeque};

fn main() {
    let cubes: HashSet<_> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let split: Vec<_> = l.split(',').map(|p| p.parse::<i32>().unwrap()).collect();
            (split[0], split[1], split[2])
        })
        .collect();

    // Part 1
    let mut air_cubes = HashSet::new();
    let mut part1_exposed_sides = 0;
    for &(x, y, z) in &cubes {
        let sides = [
            (x - 1, y, z), (x + 1, y, z),
            (x, y - 1, z), (x, y + 1, z),
            (x, y, z - 1), (x, y, z + 1),
        ];

        let iter = sides.into_iter()
            .filter(|s| !cubes.contains(s));

        part1_exposed_sides += iter.clone().count();
        air_cubes.extend(iter);
    }
    println!("[Part 1] Surface area: {part1_exposed_sides}");

    // Get boundaries of the cube
    let x_min = cubes.iter().map(|(x, _y, _z)| *x).min().unwrap();
    let x_max = cubes.iter().map(|(x, _y, _z)| *x).max().unwrap();
    let y_min = cubes.iter().map(|(_x, y, _z)| *y).min().unwrap();
    let y_max = cubes.iter().map(|(_x, y, _z)| *y).max().unwrap();
    let z_min = cubes.iter().map(|(_x, _y, z)| *z).min().unwrap();
    let z_max = cubes.iter().map(|(_x, _y, z)| *z).max().unwrap();

    // Part 2
    let mut part2_exposed_sides = part1_exposed_sides;
    'air_cubes: for &(x, y, z) in &air_cubes {
        let mut queue = VecDeque::new();
        queue.push_back((x, y, z));

        let mut visited = HashSet::new();
        while let Some((x, y, z)) = queue.pop_front() {
            if !(x_min..=x_max).contains(&x) || !(y_min..=y_max).contains(&y) || !(z_min..=z_max).contains(&z) {
                // Exposed to the steam
                continue 'air_cubes;
            }

            if !visited.insert((x, y, z)) {
                continue;
            }

            let sides = [
                (x - 1, y, z), (x + 1, y, z),
                (x, y - 1, z), (x, y + 1, z),
                (x, y, z - 1), (x, y, z + 1),
            ];

            for side in sides.into_iter().filter(|s| !cubes.contains(s)) {
                queue.push_back(side);
            }
        }

        // Count exposed sides of this cube
        let sides = [
            (x - 1, y, z), (x + 1, y, z),
            (x, y - 1, z), (x, y + 1, z),
            (x, y, z - 1), (x, y, z + 1),
        ];
        part2_exposed_sides -= sides.into_iter()
            .filter(|s| cubes.contains(s))
            .count();
    }
    println!("[Part 2] Surface area: {part2_exposed_sides}");
}
