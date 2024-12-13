use std::str::FromStr;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;


#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<(usize, usize)>>,
}

impl Map {
    fn antinodes<const PART2: bool>(&self) -> HashSet<(usize, usize)> {
        let mut locations = HashSet::new();

        for points in self.antennas.values() {
            for p in points.iter().combinations(2) {
                let (p1, p2) = (p[0], p[1]);
                let (x1, y1) = *p1;
                let (x2, y2) = *p2;

                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                let mut min_i =
                    if PART2 {
                        isize::max(-(x1 as isize) / dx, -(y1 as isize) / dy)
                    } else {
                        -1
                    };
                let mut max_i =
                    if PART2 {
                        isize::min(
                            (self.width - 1 - x1) as isize / dx,
                            (self.height - 1 - y1) as isize / dy
                        )
                    } else {
                        2
                    };
                if max_i < min_i {
                    std::mem::swap(&mut min_i, &mut max_i);
                }
                let min_i = min_i;
                let max_i = max_i;

                for i in min_i..=max_i {
                    if !PART2 && (i == 0 || i == 1) {
                        continue;
                    }

                    let x = (x1 as isize + i * dx) as usize;
                    let y = (y1 as isize + i * dy) as usize;
                    if x < self.width && y < self.height {
                        locations.insert((x, y));
                    }
                }
            }
        }

        locations
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in s.lines().enumerate() {
            height = y + 1;
            width = line.len();

            for (x, c) in line.chars().enumerate() {
                if c.is_alphanumeric() {
                    antennas.entry(c).or_default().push((x, y));
                }
            }
        }

        Ok(Map { width, height, antennas })
    }
}

fn main() {
    let map = Map::from_str(include_str!("../input.txt")).unwrap();

    let part1_antinodes = map.antinodes::<false>();
    println!("[Part 1] Number of antinodes: {}", part1_antinodes.len());

    let part2_antinodes = map.antinodes::<true>();
    println!("[Part 2] Number of antinodes: {}", part2_antinodes.len());
}
