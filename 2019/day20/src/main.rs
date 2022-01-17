use std::{collections::{HashMap, VecDeque, HashSet}, str::FromStr};


#[derive(Debug, Clone, PartialEq, Eq)]
enum Space {
    Wall,
    Open,
    Portal(String),
}


#[derive(Debug)]
struct Maze {
    map: Vec<Vec<Space>>,
    portals: HashMap<(usize, usize), (usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Maze {
    pub fn shortest_path(&self) -> u32 {
        // Perform BFS to find out the shortest path

        let height = self.map.len();
        let width = self.map[0].len();

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((self.start, 0));

        while let Some(((y, x), depth)) = queue.pop_front() {
            if !visited.insert((y, x)) {
                continue;
            }

            if (y, x) == self.end {
                return depth;
            }

            for (ny, nx) in [(y.wrapping_sub(1), x), (y, x + 1), (y + 1, x), (y, x.wrapping_sub(1))] {
                // Check if we are out of bounds
                if ny >= height || nx >= width {
                    continue;
                }

                match &self.map[ny][nx] {
                    Space::Wall => continue,
                    Space::Open => {
                        queue.push_back(((ny, nx), depth + 1));
                    },
                    Space::Portal(name) => {
                        if name == "AA" || name == "ZZ" {
                            queue.push_back(((ny, nx), depth + 1));
                        } else {
                            // Go through the portal
        	                queue.push_back((self.portals[&(ny, nx)], depth + 2));
                        }
                    },
                }
            }
        }

        unreachable!()
    }
}

impl FromStr for Maze {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_map: Vec<_> = s.lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect();

        let mut map = Vec::new();
        let mut portal_lookup: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

        let height = str_map.len() - 2;
        let width = str_map[0].len() - 2;

        for (y, line) in str_map.iter().skip(2).take(height).enumerate() {
            map.push(Vec::new());

            for (x, c) in line.iter().skip(2).take(width).enumerate() {
                let mut space =
                    match c {
                        '.' => Space::Open,
                        _   => Space::Wall,
                    };

                // Check for portals
                if space == Space::Open {
                    // Look up
                    let portal_name =
                        if str_map[y + 2 - 1][x + 2].is_ascii_alphabetic() {
                            // Looking up
                            Some(format!("{}{}", str_map[y + 2 - 2][x + 2], str_map[y + 2 - 1][x + 2]))
                        } else if str_map[y + 2][x + 2 + 1].is_ascii_alphabetic() {
                            // Looking right
                            Some(format!("{}{}", str_map[y + 2][x + 2 + 1], str_map[y + 2][x + 2 + 2]))
                        } else if str_map[y + 2 + 1][x + 2].is_ascii_alphabetic() {
                            // Looking down
                            Some(format!("{}{}", str_map[y + 2 + 1][x + 2], str_map[y + 2 + 2][x + 2]))
                        } else if str_map[y + 2][x + 2 - 1].is_ascii_alphabetic() {
                            // Looking left
                            Some(format!("{}{}", str_map[y + 2][x + 2 - 2], str_map[y + 2][x + 2 - 1]))
                        } else {
                            None
                        };

                    if let Some(portal_name) = portal_name {
                        portal_lookup.entry(portal_name.clone()).or_insert(Vec::new()).push((y, x));

                        space = Space::Portal(portal_name);
                    }
                }

                // Update the map
                map[y].push(space);
            }
        }

        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut portals = HashMap::new();

        for (name, coords) in portal_lookup.into_iter() {
            if name == "AA" {
                start = coords[0];
            } else if name == "ZZ" {
                end = coords[0];
            } else {
                portals.insert(coords[0], coords[1]);
                portals.insert(coords[1], coords[0]);
            }
        }

        Ok(Self { map, portals, start, end })
    }
}

fn main() {
    let maze = Maze::from_str(include_str!("../input.txt")).unwrap();

    println!("Shortest path: {}", maze.shortest_path());
}
