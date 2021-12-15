use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    White,
    Black,
}

impl Tile {
    pub fn flip(&mut self) {
        match self {
            Tile::White => *self = Tile::Black,
            Tile::Black => *self = Tile::White,
        }
    }
}


fn get_tile(instruction: &str) -> (i32, i32) {
    let mut current_tile = (0, 0);

    let mut iter = instruction.chars().peekable();
    while let Some(c) = iter.next() {
        match c {
            'n' => {
                current_tile.1 -= 1;
                match iter.peek().unwrap() {
                    'e' => (),
                    'w' => current_tile.0 -= 1,
                    _ => panic!("Invalid direction")
                }
                iter.next();
            },
            'e' => current_tile.0 += 1,
            's' => {
                current_tile.1 += 1;
                match iter.peek().unwrap() {
                    'e' => current_tile.0 += 1,
                    'w' => (),
                    _ => panic!("Invalid direction")
                }
                iter.next();
            },
            'w' => current_tile.0 -= 1,
            _ => panic!("Invalid direction"),
        }
    }

    current_tile
}

/// Returns the neighbours of a coordinate
const fn get_neighbours((x, y): (i32, i32)) -> [(i32, i32); 6] {
    [
        (x - 1, y - 1), (x, y - 1),
        (x - 1, y), (x + 1, y),
        (x, y + 1), (x + 1, y + 1)
    ]
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .collect();

    // Hashmap linking a coordinate with a tile colour
    let mut flipped_tiles = HashMap::new();

    // Part 1 (and preparation for part 2...)

    for instruction in input {
        let current_tile = get_tile(instruction);
        flipped_tiles.entry(current_tile).or_insert(Tile::White).flip();
    }

    println!("Black tiles: {}", flipped_tiles.values().filter(|i| **i == Tile::Black).count());

    // Part 2

    let mut tiles_to_flip = HashSet::new();
    for _day in 0..100 {
        // Go through the black tiles to see which tiles need to flipped.
        for ((x, y), tile) in flipped_tiles.iter() {
            match tile {
                Tile::White => continue,
                Tile::Black => {
                    let neighbours: Vec<_> = get_neighbours((*x, *y))
                        .into_iter()
                        .map(|n| (n, flipped_tiles.get(&n).unwrap_or(&Tile::White)))
                        .collect();

                    let black_neighbours = neighbours.iter().filter(|(_, t)| **t == Tile::Black).count();
                    if black_neighbours == 0 || black_neighbours > 2 {
                        tiles_to_flip.insert((*x, *y));
                    }

                    // Check white neighbours, as white tiles can only be flipped if they have at least one black neighbour.
                    for c in neighbours.iter().filter_map(|(c, t)| (**t == Tile::White).then(|| c)) {
                        let black_neighbours = get_neighbours(*c).into_iter()
                            .filter(|c| *flipped_tiles.get(&c).unwrap_or(&Tile::White) == Tile::Black)
                            .count();

                        if black_neighbours == 2 {
                            tiles_to_flip.insert(*c);
                        }
                    }
                },
            }
        }

        // Flip all the tiles we marked to be flipped
        for t in &tiles_to_flip {
            flipped_tiles.entry(*t).or_insert(Tile::White).flip();
        }
        // Clear hash-set for next iteration
        tiles_to_flip.clear();
    }

    println!("Black tiles after day 100: {}", flipped_tiles.values().filter(|i| **i == Tile::Black).count());
}
