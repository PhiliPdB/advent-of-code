use std::str::FromStr;

use hashbrown::HashSet;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Heading {
    North, East, South, West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    MirrorSWNE,
    MirrorSENW,
    SplitVertical,
    SplitHorizontal,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.'  => Self::Empty,
            '/'  => Self::MirrorSWNE,
            '\\' => Self::MirrorSENW,
            '|'  => Self::SplitVertical,
            '-'  => Self::SplitHorizontal,
            _    => panic!("Invalid tile: {c}"),
        }
    }
}

#[derive(Debug)]
pub struct Facility {
    map: Vec<Vec<Tile>>,
}

impl Facility {
    pub fn find_max_energized_tiles(&self) -> usize {
        let mut max_energized_tiles = 0;

        let height = self.map.len() as i32;
        let width = self.map[0].len() as i32;

        for x in 0..width {
            let point_south = self.energized_tiles(((x, 0), Heading::South)).len();
            max_energized_tiles = max_energized_tiles.max(point_south);

            let point_north = self.energized_tiles(((x, height - 1), Heading::North)).len();
            max_energized_tiles = max_energized_tiles.max(point_north);
        }

        for y in 0..height {
            let point_east = self.energized_tiles(((0, y), Heading::East)).len();
            max_energized_tiles = max_energized_tiles.max(point_east);

            let point_west = self.energized_tiles(((width - 1, y), Heading::West)).len();
            max_energized_tiles = max_energized_tiles.max(point_west);
        }

        max_energized_tiles
    }

    pub fn energized_tiles(&self, starting_beam: ((i32, i32), Heading)) -> HashSet<(i32, i32)> {
        let mut energized_tiles = HashSet::new();
        let mut visited = HashSet::new();

        let mut beams = vec![starting_beam];
        while let Some(((x, y), heading)) = beams.pop() {
            // Check if we've already visited this tile with this heading
            // In other to avoid infinite loops
            if !visited.insert(((x, y), heading)) {
                continue;
            }

            // Energize the current tile
            energized_tiles.insert((x, y));

            match (self.map[y as usize][x as usize], heading) {
                (Tile::Empty, h) => if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), h) {
                    beams.push(new_loc);
                },
                (Tile::MirrorSWNE, Heading::North) => if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), Heading::East) {
                    beams.push(new_loc);
                },
                (Tile::MirrorSWNE, Heading::East) => if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), Heading::North) {
                    beams.push(new_loc);
                },
                (Tile::MirrorSWNE, Heading::South) => if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), Heading::West) {
                    beams.push(new_loc);
                },
                (Tile::MirrorSWNE, Heading::West) => if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), Heading::South) {
                    beams.push(new_loc);
                }
                (Tile::MirrorSENW, Heading::North) => if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), Heading::West) {
                    beams.push(new_loc);
                },
                (Tile::MirrorSENW, Heading::East) => if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), Heading::South) {
                    beams.push(new_loc);
                },
                (Tile::MirrorSENW, Heading::South) => if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), Heading::East) {
                    beams.push(new_loc);
                },
                (Tile::MirrorSENW, Heading::West) => if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), Heading::North) {
                    beams.push(new_loc);
                },
                (Tile::SplitVertical, Heading::East|Heading::West) => {
                    if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), Heading::North) {
                        beams.push(new_loc);
                    }
                    if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), Heading::South) {
                        beams.push(new_loc);
                    }
                },
                (Tile::SplitVertical, h) => if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), h) {
                    beams.push(new_loc);
                },
                (Tile::SplitHorizontal, Heading::North|Heading::South) => {
                    if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), Heading::East) {
                        beams.push(new_loc);
                    }
                    if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), Heading::West) {
                        beams.push(new_loc);
                    }
                },
                (Tile::SplitHorizontal, h) => if let Some(new_loc) = self.get_new_location(&mut energized_tiles, (x, y), h) {
                    beams.push(new_loc);
                },
            }
        }

        energized_tiles
    }

    /// Find the next location where the beam encounters a non-empty tile energizing all the empty tiles in between
    /// Returns None if the beam goes out of the map
    fn get_new_location(&self, energized_tiles: &mut HashSet<(i32, i32)>, (x, y): (i32, i32), new_heading: Heading) -> Option<((i32, i32), Heading)> {
        match new_heading {
            Heading::North => {
                for y in (0..y).rev() {
                    if self.map[y as usize][x as usize] != Tile::Empty {
                        return Some(((x, y), Heading::North));
                    }

                    energized_tiles.insert((x, y));
                }
                None
            },
            Heading::East => {
                for x in x+1..self.map[0].len() as i32 {
                    if self.map[y as usize][x as usize] != Tile::Empty {
                        return Some(((x, y), Heading::East));
                    }

                    energized_tiles.insert((x, y));
                }
                None
            },
            Heading::South => {
                for y in y+1..self.map.len() as i32 {
                    if self.map[y as usize][x as usize] != Tile::Empty {
                        return Some(((x, y), Heading::South));
                    }

                    energized_tiles.insert((x, y));
                }
                None
            },
            Heading::West => {
                for x in (0..x).rev() {
                    if self.map[y as usize][x as usize] != Tile::Empty {
                        return Some(((x, y), Heading::West));
                    }

                    energized_tiles.insert((x, y));
                }
                None
            },
        }
    }
}

impl FromStr for Facility {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s.lines()
            .map(|l| l.chars().map(Tile::from_char).collect())
            .collect();

        Ok(Facility { map })
    }
}


fn main() {
    let facility = Facility::from_str(include_str!("../input.txt")).unwrap();

    println!("[Part 1] Number of energized tiles: {}", facility.energized_tiles(((0, 0), Heading::East)).len());
    println!("[Part 2] Number of energized tiles: {}", facility.find_max_energized_tiles());
}
