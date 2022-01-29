use std::collections::HashSet;
use std::str::FromStr;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    Bug,
}

impl TryFrom<char> for Space {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Bug),
            _ => Err("Invalid space character"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    state: [[Space; 5]; 5],
}

impl Tile {
    pub fn next(&self) -> Tile {
        let mut next_state = *self;
        for y in 0..5 {
            for x in 0..5 {
                let alive_neighbours = self.alive_neighbours(x, y);

                match self.state[y][x] {
                    Space::Empty if alive_neighbours == 1 || alive_neighbours == 2 => {
                        next_state.state[y][x] = Space::Bug;
                    },
                    Space::Bug if alive_neighbours != 1 => {
                        next_state.state[y][x] = Space::Empty;
                    },
                    _ => (),
                }
            }
        }

        next_state
    }

    pub fn biodiversity(&self) -> u32 {
        let mut biodiversity = 0;
        for y in 0..5 {
            for x in 0..5 {
                if self.state[y][x] == Space::Bug {
                    biodiversity += 2_u32.pow((y * 5 + x) as u32);
                }
            }
        }
        biodiversity
    }

    fn alive_neighbours(&self, x: usize, y: usize) -> usize {
        [(x.wrapping_sub(1), y), (x, y.wrapping_sub(1)), (x + 1, y), (x, y + 1)]
            .iter()
            .map(|(nx, ny)| {
                if *nx >= 5 || *ny >= 5 || self.state[*ny][*nx] == Space::Empty {
                    0
                } else {
                    1
                }
            })
            .sum()
    }
}

impl FromStr for Tile {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = s.lines()
            .map(|l| {
                l.chars().map(|c| Space::try_from(c).unwrap()).collect::<Vec<_>>().try_into().unwrap()
            })
            .collect::<Vec<_>>()
            .try_into().unwrap();

        Ok(Self { state })
    }
}

#[derive(Debug)]
struct RecursiveTile {
    alive_spaces: HashSet<(i32, i32, i32)>
}

impl RecursiveTile {

    pub fn next(&mut self) {
        let mut new_alive_spaces = HashSet::new();
        for &(x, y, level) in self.alive_spaces.iter() {
            let mut alive_neighbours = 0;

            for (nx, ny, nlevel) in Self::neighbours(x, y, level) {
                if self.is_alive(nx, ny, nlevel) {
                    alive_neighbours += 1;
                } else {
                    // Check neighbour if it should come to live
                    let adjacent_alive = Self::neighbours(nx, ny, nlevel).into_iter()
                        .filter(|&(nnx, nny, nnlevel)| self.is_alive(nnx, nny, nnlevel))
                        .count();
                    if adjacent_alive == 1 || adjacent_alive == 2 {
                        new_alive_spaces.insert((nx, ny, nlevel));
                    }
                }
            }

            // Check if the cell should still be alive
            if alive_neighbours == 1 {
                new_alive_spaces.insert((x, y, level));
            }
        }

        self.alive_spaces = new_alive_spaces;
    }

    fn is_alive(&self, x: i32, y: i32, level: i32) -> bool {
        self.alive_spaces.contains(&(x, y, level))
    }

    fn neighbours(x: i32, y: i32, level: i32) -> Vec<(i32, i32, i32)> {
        let mut neighbours = Vec::with_capacity(8);
        for (nx, ny) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
            // Check outer border
            if nx <= -3 || nx >= 3 {
                neighbours.push((nx.signum(), 0, level - 1));
            } else if ny <= -3 || ny >= 3 {
                neighbours.push((0, ny.signum(), level - 1));
            } else if nx == 0 && ny == 0 {
                if x == 0 {
                    debug_assert!(y == 1 || y == -1);
                    neighbours.extend((-2..=2).map(|n| (n, 2 * y, level + 1)));
                } else {
                    debug_assert_eq!(y, 0);
                    debug_assert!(x == 1 || x == -1);
                    neighbours.extend((-2..=2).map(|n| (2 * x, n, level + 1)));
                }
            } else {
                neighbours.push((nx, ny, level));
            }
        }

        neighbours
    }
}

impl From<Tile> for RecursiveTile {
    fn from(t: Tile) -> Self {
        let mut alive_spaces = HashSet::new();

        for (y, row) in t.state.iter().enumerate() {
            for (x, s) in row.iter().enumerate() {
                if *s == Space::Bug {
                    alive_spaces.insert((x as i32 - 2, y as i32 - 2, 0));
                }
            }
        }

        Self { alive_spaces }
    }
}


fn main() {
    let input = Tile::from_str(include_str!("../input.txt")).unwrap();

    // Part 1

    let mut current_state = input;
    let mut seen_states = HashSet::new();
    while seen_states.insert(current_state) {
        current_state = current_state.next();
    }

    println!("Biodiversity: {}", current_state.biodiversity());

    // Part 2

    let mut recursive_state = RecursiveTile::from(input);
    for _ in 0..200 {
        recursive_state.next();
    }

    println!("Alive bugs: {}", recursive_state.alive_spaces.len());
}
