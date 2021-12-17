use std::collections::{HashMap, HashSet};


const TOTAL_CYCLES: u32 = 6;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeState {
    Active,
    InActive,
}

impl CubeState {
    pub fn from_char(c: char) -> Self {
        match c {
            '#' => CubeState::Active,
            '.' => CubeState::InActive,
            _ => panic!("Invalid char"),
        }
    }

    pub fn flip(&mut self) {
        match self {
            CubeState::Active   => *self = CubeState::InActive,
            CubeState::InActive => *self = CubeState::Active,
        }
    }
}


fn get_neighbours(x: i32, y: i32, z: i32, w: i32, use_hypercubes: bool) -> Vec<(i32, i32, i32, i32)> {
    let mut neighbours = Vec::with_capacity(27 * 3);
    let w_options =
        if use_hypercubes {
            vec![-1, 0, 1]
        } else {
            vec![0]
        };

    for dw in w_options {
        for dz in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                for dx in [-1, 0, 1] {
                    if !(dx == 0 && dy == 0 && dz == 0 && dw == 0) {
                        neighbours.push((x + dx, y + dy, z + dz, w + dw));
                    }
                }
            }
        }
    }

    neighbours
}

fn simulate(input: &[Vec<CubeState>], use_hypercubes: bool) -> usize {
    let mut cubes = HashMap::new();
    // Fill the hashmap with the input
    for (y, row) in input.iter().enumerate() {
        for (x, status) in row.iter().enumerate() {
            cubes.insert((x as i32, y as i32, 0, 0), *status);
        }
    }

    for _ in 0..TOTAL_CYCLES {
        let mut cubes_to_flip = HashSet::new();
        // Perform cycle
        for ((x, y, z, w), state) in cubes.iter() {
            if *state == CubeState::InActive {
                continue;
            }

            // Only looking at active cubes
            let neighbours: Vec<_> = get_neighbours(*x, *y, *z, *w, use_hypercubes).into_iter()
                .map(|coord| (coord, cubes.get(&coord).unwrap_or(&CubeState::InActive)))
                .collect();
            let active_neighbours = neighbours.iter().filter(|(_, s)| **s == CubeState::Active).count();
            if active_neighbours != 2 && active_neighbours != 3 {
                cubes_to_flip.insert((*x, *y, *z, *w));
            }

            // See if inactive neighbours need to be flipped
            for ((nx, ny, nz, nw), _) in neighbours.iter().filter(|(_, s)| **s == CubeState::InActive) {
                let active_count = get_neighbours(*nx, *ny, *nz, *nw, use_hypercubes).into_iter()
                    .filter(|coord| *cubes.get(coord).unwrap_or(&CubeState::InActive) == CubeState::Active)
                    .count();

                if active_count == 3 {
                    cubes_to_flip.insert((*nx, *ny, *nz, *nw));
                }
            }
        }

        for coord in cubes_to_flip {
            cubes.entry(coord).or_insert(CubeState::InActive).flip();
        }
    }

    // Return count of active cubes
    cubes.values().filter(|s| **s == CubeState::Active).count()
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.chars().map(CubeState::from_char).collect::<Vec<_>>())
        .collect();


    println!("[Part 1] Active cubes: {}", simulate(&input, false));
    println!("[Part 2] Active cubes: {}", simulate(&input, true));
}
