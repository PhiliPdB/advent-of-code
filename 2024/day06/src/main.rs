use std::str::FromStr;

use hashbrown::HashSet;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Empty, Obstruction
}

impl State {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Empty),
            '#' => Some(Self::Obstruction),
            _ => None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Heading {
    North, East, South, West
}

impl Heading {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::North,
            '>' => Self::East,
            'v' => Self::South,
            '<' => Self::West,
            _ => panic!("Cannot parse heading"),
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East  => Self::South,
            Self::South => Self::West,
            Self::West  => Self::North,
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<State>>,
    guard: (usize, usize, Heading),
}

impl Map {
    fn guard_positions(&self) -> (HashSet<(usize, usize)>, bool) {
        let width = self.map[0].len();
        let height = self.map.len();

        let (mut x, mut y, mut heading) = self.guard;

        let mut positions = HashSet::new();
        let mut position_heading = HashSet::new();
        let mut is_loop = false;
        while x < width && y < height {
            positions.insert((x, y));
            debug_assert!(self.map[y][x] == State::Empty);

            let (next_x, next_y) = match heading {
                Heading::North => (x, y.wrapping_sub(1)),
                Heading::East  => (x.wrapping_add(1), y),
                Heading::South => (x, y.wrapping_add(1)),
                Heading::West  => (x.wrapping_sub(1), y),
            };
            if !(next_x < width && next_y < height) {
                break;
            }

            match self.map[next_y][next_x] {
                State::Empty => {
                    x = next_x;
                    y = next_y;
                },
                State::Obstruction => {
                    heading = heading.turn_right();

                    if !position_heading.insert((x, y, heading)) {
                        is_loop = true;
                        break;
                    }
                },
            }
        }
        (positions, is_loop)
    }

    fn obstacle_locations(&mut self) -> usize {
        let mut locations = 0;
        let (positions, _) = self.guard_positions();

        for (x, y) in positions.into_iter() {
            // println!("Checking: {x},{y}");
            if x == self.guard.0 && y == self.guard.1 {
                continue;
            }

            self.map[y][x] = State::Obstruction;
            let (_, is_loop) = self.guard_positions();
            self.map[y][x] = State::Empty;

            if is_loop {
                locations += 1;
            }
        }
        locations
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut guard= (0, 0, Heading::North);
        for (y, l) in s.lines().enumerate() {
            let mut line = Vec::new();
            for (x, c) in l.chars().enumerate() {
                let Some(state) = State::from_char(c) else {
                    guard = (x, y, Heading::from_char(c));
                    line.push(State::Empty);
                    continue;
                };

                line.push(state);
            }
            map.push(line);
        }

        Ok(Map{ map, guard })
    }
}


fn main() {
    let map= Map::from_str(include_str!("../input.txt")).unwrap();

    let (visited_positions, _) = map.guard_positions();
    println!("[Part 1] Visited positions: {}", visited_positions.len());

    let mut map = map;
    let obstacle_location_count = map.obstacle_locations();
    println!("[Part 2] Obstacle locations: {obstacle_location_count}");
}
