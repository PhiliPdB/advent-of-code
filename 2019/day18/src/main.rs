use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

use hashbrown::{HashMap, HashSet};

use bitset::BitSet;

mod bitset;


#[derive(Debug, PartialEq, Eq)]
struct HeapItem<const N: usize> {
    coords: [(usize, usize); N],
    keys: BitSet,
    distance: u32,
}

impl<const N: usize> Ord for HeapItem<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
            .then(self.keys.set_bits().cmp(&other.keys.set_bits()))
    }
}

impl<const N: usize> PartialOrd for HeapItem<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SmallHeapItem {
    coords: (usize, usize),
    distance: u32,
}

impl Ord for SmallHeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for SmallHeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Open,
    Wall,
    Entrance,
    Door(char),
    Key(char),
}

impl Space {
    pub fn is_key(&self) -> bool {
        matches!(self, Space::Key(_))
    }

    pub fn get_key(&self) -> char {
        match self {
            Space::Key(c) => *c,
            _ => panic!("Space is not a key"),
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Space::Open,
            '#' => Space::Wall,
            '@' => Space::Entrance,
            _ if c.is_ascii_uppercase() => Space::Door(c),
            _ if c.is_ascii_lowercase() => Space::Key(c),
            _ => panic!("Invalid space char"),
        }
    }
}

#[derive(Debug)]
struct Vault<const N: usize> {
    map: Vec<Vec<Space>>,
    adjacency_list: HashMap<(usize, usize), Vec<((usize, usize), u32)>>,
    entrances: [(usize, usize); N],
    key_locations: Vec<(char, (usize, usize))>,
    total_keys: u32,
}

impl Vault<1> {
    /// Upgrade to the part 2 vault
    pub fn upgrade(&self) -> Vault<4> {
        let mut map = self.map.clone();
        let (x, y) = self.entrances[0];

        // Update the map

        map[y - 1][x - 1] = Space::Entrance;
        map[y - 1][x]     = Space::Wall;
        map[y - 1][x + 1] = Space::Entrance;

        map[y][x - 1] = Space::Wall;
        map[y][x]     = Space::Wall;
        map[y][x + 1] = Space::Wall;

        map[y + 1][x - 1] = Space::Entrance;
        map[y + 1][x]     = Space::Wall;
        map[y + 1][x + 1] = Space::Entrance;

        let mut vault = Vault {
            map,
            adjacency_list: HashMap::new(),
            entrances: [(y - 1, x - 1), (y - 1, x + 1), (y + 1, x - 1), (y + 1, x + 1)],
            key_locations: self.key_locations.clone(),
            total_keys: self.total_keys,
        };
        for (ex, ey) in vault.entrances {
            vault.dfs((ex, ey));
        }

        vault
    }
}

impl<const N: usize> Vault<N> {
    pub fn key_distances(&self, (x, y): (usize, usize), keys: BitSet) -> Vec<(char, (usize, usize), u32)> {
        let mut key_distances = Vec::new();
        for (key, (tx, ty)) in &self.key_locations {
            if keys.get((*key as u8 - 97) as u32) {
                // Already grabbed this key
                continue;
            }

            // For part 2: Quick (and dirty) check to see if a key is reachable
            //             by checking if the coordinates are in the same quadrant
            //             of the map.
            if N == 4 && self.quadrant((x, y)) != self.quadrant((*tx, *ty)) {
                continue;
            }

            // Dijkstra to target
            if let Some(distance) = self.dijkstra((x, y), (*tx, *ty), keys) {
                key_distances.push((*key, (*tx, *ty), distance));
            }
        }

        key_distances
    }

    fn dijkstra(&self, (sx, sy): (usize, usize), (tx, ty): (usize, usize), keys: BitSet) -> Option<u32> {
        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();

        // Initialize queue
        queue.push(SmallHeapItem { coords: (sx, sy), distance: 0 });

        while let Some(SmallHeapItem { coords, distance }) = queue.pop() {
            if !visited.insert(coords) {
                continue;
            }

            if coords.0 == tx && coords.1 == ty {
                return Some(distance);
            }


            for ((mx, my), move_distance) in &self.adjacency_list[&coords] {
                if let Space::Door(k) = self.map[*my][*mx] {
                    if !keys.get((k.to_ascii_lowercase() as u8 - 97) as u32) {
                        continue;
                    }
                }

                queue.push(SmallHeapItem { coords: (*mx, *my), distance: distance + move_distance });
            }
        }

        None
    }

    /// Dfs to explore the map and create the adjacency list.
    fn dfs(&mut self, (sx, sy): (usize, usize)) {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        stack.push((sx, sy));

        while let Some((x, y)) = stack.pop() {
            if !visited.insert((x, y)) {
                continue;
            }

            for ((nx, ny), dist) in self.next_moves((x, y)) {
                self.adjacency_list.entry((x, y)).or_insert(Vec::new()).push(((nx, ny), dist));

                stack.push((nx, ny));
            }
        }
    }

    fn next_moves(&self, (x, y): (usize, usize)) -> Vec<((usize, usize), u32)> {
        let mut next_moves = Vec::with_capacity(4);

        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let mut last_x = x;
            let mut last_y = y;

            let mut nx = (x as i32 + dx) as usize;
            let mut ny = (y as i32 + dy) as usize;

            if self.map[ny][nx] == Space::Wall {
                continue;
            }

            let mut distance = 1;

            while let [(move_x, move_y)] = self.move_possibilities((nx, ny), (last_x, last_y))[..] {
                if self.map[ny][nx] != Space::Open {
                    break;
                }

                last_x = nx;
                last_y = ny;

                nx = move_x;
                ny = move_y;

                distance += 1;
            }

            next_moves.push(((nx, ny), distance));
        }

        next_moves
    }

    fn move_possibilities(&self, (x, y): (usize, usize), (last_x, last_y): (usize, usize)) -> Vec<(usize, usize)> {
        [(-1, 0), (0, -1), (1, 0), (0, 1)].into_iter()
            .filter_map(|(dx, dy)| {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;

                if nx == last_x && ny == last_y {
                    return None;
                }

                (self.map[ny][nx] != Space::Wall)
                    .then(|| (nx, ny))
            })
            .collect()
    }

    fn quadrant(&self, (x, y): (usize, usize)) -> u32 {
        let pos1 = (x <= (self.map[0].len() / 2)) as u32;
        let pos2 = (y <= (self.map.len() / 2)) as u32;

        pos1 << 1 | pos2
    }
}

impl<const N: usize> FromStr for Vault<N> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut entrances = Vec::new();
        let mut key_locations = Vec::new();
        let mut total_keys = 0;

        for (y, l) in s.lines().enumerate() {
            map.push(Vec::new());
            for (x, c) in l.chars().enumerate() {
                if c == '@' {
                    entrances.push((x, y));
                }

                map[y].push(Space::from_char(c));

                if map[y][x].is_key() {
                    key_locations.push((map[y][x].get_key(), (x, y)));
                    total_keys += 1;
                }
            }
        }

        let entrances = entrances.try_into().map_err(|_| "Invalid entrance size")?;

        let mut vault = Self {
            map, adjacency_list: HashMap::new(), entrances, key_locations, total_keys
        };
        for (ex, ey) in vault.entrances {
            vault.dfs((ex, ey));
        }

        Ok(vault)
    }
}

fn dijkstra<const N: usize>(vault: &Vault<N>) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    // Initialize queue
    queue.push(HeapItem { coords: vault.entrances, keys: BitSet::default(), distance: 0 });

    while let Some(HeapItem { coords, keys, distance }) = queue.pop() {
        if !visited.insert((coords, keys)) {
            continue;
        }

        if keys.set_bits() == vault.total_keys {
            return distance;
        }

        for i in 0..coords.len() {
            let mut new_coords = coords.clone();

            for (k, (mx, my), move_distance) in vault.key_distances(coords[i], keys) {
                let mut new_keys = keys.clone();
                new_keys.set((k as u8 - 97) as u32);

                new_coords[i] = (mx, my);
                queue.push(HeapItem { coords: new_coords, keys: new_keys, distance: distance + move_distance });
            }
        }
    }

    unreachable!()
}


fn main() {
    let vault = Vault::<1>::from_str(include_str!("../input.txt")).unwrap();
    println!("[Part 1] Shortest path: {}", dijkstra(&vault));

    let part2_vault = vault.upgrade();
    println!("[Part 2] Shortest path: {}", dijkstra(&part2_vault));
}
