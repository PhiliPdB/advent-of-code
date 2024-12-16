use std::str::FromStr;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty, Wall,
}

impl Space {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Space::Empty,
            '#' => Space::Wall,
            _ => panic!("Invalid space character: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Heading {
    North, East, South, West,
}

impl Heading {
    fn turn_left(&self) -> Self {
        match self {
            Heading::North => Heading::West,
            Heading::East => Heading::North,
            Heading::South => Heading::East,
            Heading::West => Heading::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Heading::North => Heading::East,
            Heading::East => Heading::South,
            Heading::South => Heading::West,
            Heading::West => Heading::North,
        }
    }

    fn direction(&self) -> (isize, isize) {
        match self {
            Heading::North => (0, -1),
            Heading::East => (1, 0),
            Heading::South => (0, 1),
            Heading::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeapItem {
    cost: u32,
    position: (usize, usize),
    heading: Heading,
    previous_node: ((usize, usize), Heading),
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}


#[derive(Debug)]
struct Map {
    map: Vec<Vec<Space>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn best_score(&self) -> (u32, HashSet<(usize, usize)>) {
        let mut queue = BinaryHeap::new();
        queue.push(HeapItem {
            cost: 0,
            position: self.start,
            heading: Heading::East,
            previous_node: (self.start, Heading::East),
        });


        let mut best_cost = vec![vec![vec![u32::MAX; 4]; self.map[0].len()]; self.map.len()];
        let mut predecessors = vec![vec![vec![Vec::new(); 4]; self.map[0].len()]; self.map.len()];

        while let Some(HeapItem {
            cost,
            position: (px, py),
            heading,
            previous_node
        }) = queue.pop() {
            // Check if we have already found a better path to this node
            match cost.cmp(&best_cost[py][px][heading as usize]) {
                Ordering::Less => { // Found a better path
                    // Reset the predecessors list
                    let p = &mut predecessors[py][px][heading as usize];
                    p.clear();
                    p.push(previous_node);
                    // Save the newly found best cost
                    best_cost[py][px][heading as usize] = cost;
                },
                Ordering::Equal => { // Found an equal path
                    // Add the predecessor to the list
                    predecessors[py][px][heading as usize].push(previous_node);
                    continue;
                },
                Ordering::Greater => {
                    continue;
                },
            }

            if (px, py) == self.end {
                continue;
            }

            for (h, heading_cost) in [
                (heading, 0),
                (heading.turn_left(), 1_000),
                (heading.turn_right(), 1_000),
            ] {
                let (dx, dy) = h.direction();
                let (nx, ny) = (px as isize + dx, py as isize + dy);
                if nx < 0 || ny < 0 {
                    continue;
                }

                let (nx, ny) = (nx as usize, ny as usize);
                if nx >= self.map[0].len() || ny >= self.map.len()
                    || self.map[ny][nx] == Space::Wall
                {
                    continue;
                }

                queue.push(HeapItem {
                    cost: cost + 1 + heading_cost,
                    position: (nx, ny),
                    heading: h,
                    previous_node: ((px, py), heading),
                });
            }
        }
        // Clear the predecessors stored at the start to avoid an infinite loop.
        predecessors[self.start.1][self.start.0][Heading::East as usize].clear();

        let (heading, &shortest_path) = best_cost[self.end.1][self.end.0].iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();

        // Backtrack to find all the best paths
        let mut path_tiles = HashSet::new();
        let mut queue = vec![((self.end, heading))];

        while let Some(((px, py), heading)) = queue.pop() {
            // Add to path
            path_tiles.insert((px, py));
            // List predecessors as items to visit
            for &predecessor in &predecessors[py][px][heading] {
                queue.push((predecessor.0, predecessor.1 as usize));
            }
        }

        (shortest_path, path_tiles)
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut start = None;
        let mut end = None;

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Some((x, y));
                    row.push(Space::Empty);
                } else if c == 'E' {
                    end = Some((x, y));
                    row.push(Space::Empty);
                } else {
                    let space = Space::from_char(c);
                    row.push(space);
                }
            }
            map.push(row);
        }

        Ok(Map {
            map,
            start: start.ok_or("No start found")?,
            end: end.ok_or("No end found")?,
        })
    }
}

fn main() {
    let maze = Map::from_str(include_str!("../input.txt")).unwrap();

    let (best_score, path_tiles) = maze.best_score();
    println!("[Part 1] Best score: {best_score}");
    println!("[Part 2] Path tiles: {}", path_tiles.len());
}
