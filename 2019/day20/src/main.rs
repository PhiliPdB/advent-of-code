use std::{collections::{HashMap, VecDeque, HashSet, hash_map::Entry, BinaryHeap}, str::FromStr, cmp::Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeapItem {
    portal_index: usize,
    level: u32,
    distance: u32,
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}


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

    adjacency_list: Vec<Vec<(usize, u32)>>,
    is_outer: Vec<bool>,
    portal_lookup: Vec<usize>,
    start_portal: usize,
    end_portal: usize,
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

    pub fn dijkstra_shortest_path(&self) -> u32 {
        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();

        queue.push(HeapItem { portal_index: self.start_portal, level: 0, distance: 0 });

        while let Some(item) = queue.pop() {
            if !visited.insert((item.portal_index, item.level)) {
                continue;
            }

            if item.portal_index == self.end_portal && item.level == 0 {
                return item.distance;
            }

            for (np, dist) in &self.adjacency_list[item.portal_index] {
                // Check if this an inner or an outer portal
                if !self.is_outer[*np] {
                    // Inner portal
                    // Restrict depth of the search
                    if item.level > 10_000 {
                        continue;
                    }

                    queue.push(HeapItem { portal_index: self.portal_lookup[*np], level: item.level + 1, distance: item.distance + dist + 1 });
                } else if item.level != 0 && *np != self.end_portal {
                    // Outer portal and not on the top level
                    queue.push(HeapItem { portal_index: self.portal_lookup[*np], level: item.level - 1, distance: item.distance + dist + 1 });
                } else if *np == self.end_portal {
                    // Outer portal and on top level, check if we go to the end
                    queue.push(HeapItem { portal_index: *np, level: item.level, distance: item.distance + dist });
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

        let height = str_map.len() - 4;
        let width = str_map[0].len() - 4;

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

        let mut start = None;
        let mut end = None;
        let mut portals = HashMap::new();

        for (name, coords) in portal_lookup.into_iter() {
            if name == "AA" {
                start = Some(coords[0]);
            } else if name == "ZZ" {
                end = Some(coords[0]);
            } else {
                portals.insert(coords[0], coords[1]);
                portals.insert(coords[1], coords[0]);
            }
        }

        // Build adjacency list
        let start = start.ok_or("No start square")?;
        let end = end.ok_or("No end square")?;

        let mut adjacency_list = Vec::new();
        let mut is_outer = Vec::new();
        let mut portal_index_lookup = HashMap::new();
        let mut index_id = 0;
        for from in [start].iter().chain(portals.values()) {
            let from_index =
                match portal_index_lookup.entry(*from) {
                    Entry::Occupied(e) => *e.get(),
                    Entry::Vacant(e) => {
                        e.insert(index_id);
                        index_id += 1;

                        adjacency_list.push(Vec::new());
                        is_outer.push(from.0 == 0 || from.1 == 0 || from.0 == height - 1 || from.1 == width - 1);

                        index_id - 1
                    },
                };


            for to in [end].iter().chain(portals.values()) {
                let to_index =
                    match portal_index_lookup.entry(*to) {
                        Entry::Occupied(e) => *e.get(),
                        Entry::Vacant(e) => {
                            e.insert(index_id);
                            index_id += 1;

                            adjacency_list.push(Vec::new());
                            is_outer.push(to.0 == 0 || to.1 == 0 || to.0 == height - 1 || to.1 == width - 1);

                            index_id - 1
                        },
                    };

                // Perform BFS to find out the distance
                if let Some(distance) = bfs(&map, *from, *to) {
                    if distance != 0 {
                        adjacency_list[from_index].push((to_index, distance));
                    }
                }
            }
        }

        let mut portal_lookup = vec![0; adjacency_list.len()];
        for (coord, id) in portal_index_lookup.iter() {
            if let Some(other_coord) = portals.get(coord) {
                portal_lookup[*id] = portal_index_lookup[other_coord];
            }
        }


        Ok(Self {
            map,
            portals,
            start, end,

            adjacency_list,
            portal_lookup,
            is_outer,
            start_portal: portal_index_lookup[&start],
            end_portal: portal_index_lookup[&end],
        })
    }
}

fn bfs(map: &Vec<Vec<Space>>, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
    // Perform BFS to find out the shortest path

    let height = map.len();
    let width = map[0].len();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, 0));

    while let Some(((y, x), depth)) = queue.pop_front() {
        if !visited.insert((y, x)) {
            continue;
        }

        if (y, x) == end {
            return Some(depth);
        }

        for (ny, nx) in [(y.wrapping_sub(1), x), (y, x + 1), (y + 1, x), (y, x.wrapping_sub(1))] {
            // Check if we are out of bounds
            if ny >= height || nx >= width {
                continue;
            }

            match &map[ny][nx] {
                Space::Wall => continue,
                _ => {
                    queue.push_back(((ny, nx), depth + 1));
                },
            }
        }
    }

    None
}

fn main() {
    let maze = Maze::from_str(include_str!("../input.txt")).unwrap();

    println!("[Part 1] Shortest path: {:#4}", maze.shortest_path());
    println!("[Part 2] Shortest path: {:#4}", maze.dijkstra_shortest_path());
}
