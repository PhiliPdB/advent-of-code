use std::collections::hash_map::Entry;
use std::collections::{VecDeque, HashSet, HashMap};
use std::str::FromStr;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Trail {
    Path, Forest,
    SlopeNorth, SlopeSouth,
    SlopeWest, SlopeEast,
}

impl Trail {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::SlopeNorth,
            'v' => Self::SlopeSouth,
            '<' => Self::SlopeWest,
            '>' => Self::SlopeEast,
            _ => panic!("Invalid trail char: {c}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North, East, South, West,
}

impl Direction {
    const fn step(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Self::North => (x, y - 1),
            Self::East  => (x + 1, y),
            Self::South => (x, y + 1),
            Self::West  => (x - 1, y),
        }
    }

    const fn continue_in(&self) -> [Direction; 3] {
        match self {
            Self::North => [Self::North, Self::East, Self::West],
            Self::East  => [Self::North, Self::East, Self::South],
            Self::South => [Self::East, Self::South, Self::West],
            Self::West  => [Self::North, Self::South, Self::West],
        }
    }
}


#[derive(Debug)]
struct Map {
    map: Vec<Vec<Trail>>,
    start: (usize, usize),
    goal: (usize, usize),
}

impl Map {
    fn longest_path<const WITH_SLOPES: bool>(&self) -> u32 {
        let mut queue = VecDeque::new();
        queue.push_back((1, self.start, HashSet::new()));

        let mut longest_path_length = 0;
        while let Some((l, (x, y), mut visited)) = queue.pop_front() {
            if (x, y) == self.goal {
                // println!("Found path of length: {l}");
                longest_path_length = u32::max(longest_path_length, l);
                continue;
            }

            // Check if visited
            if !visited.insert((x, y)) {
                continue;
            }

            // Generate next step
            for d in [Direction::North, Direction::East, Direction::South, Direction::West] {
                let (steps, (new_x, new_y)) = self.next_crossing::<WITH_SLOPES>((x, y), d);
                if steps == 0 || visited.contains(&(new_x, new_y)) {
                    continue;
                }
                queue.push_back((l + steps, (new_x, new_y), visited.clone()));
            }
        }

        longest_path_length
    }

    fn next_crossing<const WITH_SLOPES: bool>(&self, (mut x, mut y): (usize, usize), mut direction: Direction) -> (u32, (usize, usize)) {
        // Check if we can even walk in this direction
        let (next_x, next_y) = direction.step((x, y));
        if !self.can_continue::<WITH_SLOPES>((next_x, next_y), direction) {
            return (0, (x, y));
        }
        (x, y) = (next_x, next_y);

        let mut steps = 1;
        loop {
            if (x, y) == self.goal {
                return (steps, (x, y));
            }

            let can_continue_in: Vec<_> = direction.continue_in().into_iter()
                .filter(|d| {
                    let (sx, sy) = d.step((x, y));
                    self.can_continue::<WITH_SLOPES>((sx, sy), *d)
                })
                .collect();

            if can_continue_in.len() > 1 || can_continue_in.len() == 0 {
                return (steps, (x, y));
            }
            direction = can_continue_in[0];

            (x, y) = direction.step((x, y));
            steps += 1;
        }
    }

    fn can_continue<const WITH_SLOPES: bool>(&self, (x, y): (usize, usize), direction: Direction) -> bool {
        match self.map[y][x] {
            Trail::Path => true,
            Trail::Forest => false,
            Trail::SlopeNorth => {
                if !WITH_SLOPES || direction == Direction::North {
                    true
                } else {
                    false
                }
            },
            Trail::SlopeSouth => {
                if !WITH_SLOPES || direction == Direction::South {
                    true
                } else {
                    false
                }
            },
            Trail::SlopeWest => {
                if !WITH_SLOPES || direction == Direction::West {
                    true
                } else {
                    false
                }
            },
            Trail::SlopeEast => {
                if !WITH_SLOPES || direction == Direction::East {
                    true
                } else {
                    false
                }
            },
        }
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: Vec<Vec<_>> = s.lines()
            .map(|l| l.chars().map(Trail::from_char).collect())
            .collect();

        map[0][1] = Trail::Forest;
        let start = (1, 1);
        let goal = (map[0].len() - 2, map.len() - 1);

        Ok(Self { map, start, goal })
    }
}

#[derive(Debug)]
struct Graph {
    outgoing_arcs: Vec<Vec<(usize, u32)>>,
    start: usize,
    goal: usize,
}

impl Graph {
    fn longest_path(&self) -> u32 {
        let mut queue = VecDeque::new();
        queue.push_back((1, self.start, HashSet::new()));

        let mut longest_path_length = 0;
        while let Some((l, n, mut visited)) = queue.pop_front() {
            if n == self.goal {
                longest_path_length = u32::max(longest_path_length, l);
                continue;
            }

            // Check if visited
            if !visited.insert(n) {
                continue;
            }

            // Generate next step
            for (next, steps) in &self.outgoing_arcs[n] {
                if visited.contains(next) {
                    continue;
                }

                queue.push_back((l + *steps, *next, visited.clone()));
            }
        }

        longest_path_length
    }
}

impl From<Map> for Graph {
    fn from(map: Map) -> Self {
        let mut node_lookup = HashMap::new();
        node_lookup.insert(map.start, 0);
        node_lookup.insert(map.goal, 1);

        let mut outgoing_arcs = Vec::new();
        outgoing_arcs.push(Vec::new());
        outgoing_arcs.push(Vec::new());


        let mut queue = VecDeque::new();
        queue.push_back(map.start);

        let mut visited = HashSet::new();

        while let Some((x, y)) = queue.pop_front() {
            if !visited.insert((x, y)) {
                continue;
            }

            if (x, y) == map.goal {
                continue;
            }

            let node_index = match node_lookup.entry((x, y)) {
                Entry::Occupied(e) => *e.get(),
                Entry::Vacant(e) => {
                    let i = outgoing_arcs.len();
                    e.insert(i);

                    outgoing_arcs.push(Vec::new());

                    i
                },
            };

            // Generate next step
            for d in [Direction::North, Direction::East, Direction::South, Direction::West] {
                let (steps, (new_x, new_y)) = map.next_crossing::<false>((x, y), d);
                if steps == 0 {
                    continue;
                }

                queue.push_back((new_x, new_y));

                let new_node_index = match node_lookup.entry((new_x, new_y)) {
                    Entry::Occupied(e) => *e.get(),
                    Entry::Vacant(e) => {
                        let i = outgoing_arcs.len();
                        e.insert(i);

                        outgoing_arcs.push(Vec::new());

                        i
                    },
                };

                outgoing_arcs[node_index].push((new_node_index, steps));
            }
        }

        Self { outgoing_arcs, start: 0, goal: 1 }
    }
}

fn main() {
    let map = Map::from_str(include_str!("../input.txt")).unwrap();

    println!("[Part 1] Scenic path length: {}", map.longest_path::<true>());
    // println!("[Part 2] Scenic path length: {}", map.longest_path::<false>());

    println!();
    let graph = Graph::from(map);
    println!("{graph:?}, N={}", graph.outgoing_arcs.len());
    println!("[Part 2] Scenic path length: {}", graph.longest_path());
}
