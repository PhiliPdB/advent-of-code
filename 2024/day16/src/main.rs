use std::str::FromStr;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

use hashbrown::{HashMap, HashSet};


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
    path: Vec<(usize, usize)>,
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
            path: vec![self.start],
        });

        let mut visited = HashMap::new();

        let mut best_cost = u32::MAX;
        let mut path_tiles = HashSet::new();


        while let Some(current) = queue.pop() {
            let (px, py) = current.position;

            if current.cost > best_cost {
                continue;
            }

            if (px, py) == self.end {
                if current.cost <= best_cost {
                    best_cost = best_cost.min(current.cost);
                    for t in &current.path {
                        path_tiles.insert(*t);
                    }

                    continue;
                } else {
                    break;
                }
            }

            if let Some(&visited_cost) = visited.get(&((px, py), current.heading)) {
                if visited_cost < current.cost {
                    continue;
                }
            }
            visited.insert(((px, py), current.heading), current.cost);


            for h in [
                current.heading,
                current.heading.turn_left(),
                current.heading.turn_right(),
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

                let mut new_path = current.path.clone();
                new_path.push((nx, ny));

                let heading_cost =
                    if h == current.heading {
                        0
                    } else {
                        1_000
                    };
                queue.push(HeapItem {
                    cost: current.cost + 1 + heading_cost,
                    position: (nx, ny),
                    heading: h,
                    path: new_path,
                });
            }
        }

        (best_cost, path_tiles)
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
