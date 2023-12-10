use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Horizontal, Vertical,
    NorthEast, NorthWest,
    SouthEast, SouthWest,
    Start, Ground,
}

impl Pipe {
    fn from_char(c: char) -> Self {
        match c {
            '-' => Self::Horizontal,
            '|' => Self::Vertical,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            'F' => Self::SouthEast,
            '7' => Self::SouthWest,
            'S' => Self::Start,
            '.' => Self::Ground,
            _ => panic!("Invalid pipe char: {c}"),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::Horizontal => write!(f, "-"),
            Pipe::Vertical => write!(f, "|"),
            Pipe::NorthEast => write!(f, "L"),
            Pipe::NorthWest => write!(f, "J"),
            Pipe::SouthEast => write!(f, "F"),
            Pipe::SouthWest => write!(f, "7"),
            Pipe::Start => write!(f, "S"),
            Pipe::Ground => write!(f, "."),
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Pipe>>,
    starting_point: (usize, usize),
    main_pipeline: HashSet<(usize, usize)>,
}

impl Map {
    pub fn loop_length(&mut self) -> u32 {
        let (mut prev_y, mut prev_x) = self.starting_point;
        let (mut current_y, mut current_x) = self.starting_point;
        let mut steps = 0;
        loop {
            if steps != 0 && (current_y, current_x) == self.starting_point {
                break;
            }
            steps += 1;
            self.main_pipeline.insert((current_y, current_x));

            match self.map[current_y][current_x] {
                Pipe::Horizontal => {
                    let diff = current_x as isize - prev_x as isize;

                    (prev_y, prev_x) = (current_y, current_x);
                    current_x = (current_x as isize + diff) as usize;
                },
                Pipe::Vertical => {
                    let diff: isize = current_y as isize - prev_y as isize;

                    (prev_y, prev_x) = (current_y, current_x);
                    current_y = (current_y as isize + diff) as usize;
                },
                Pipe::NorthEast => {
                    if (prev_y, prev_x) == (current_y - 1, current_x) {
                        // Coming from the north, going east
                        (prev_y, prev_x) = (current_y, current_x);
                        current_x += 1;
                    } else {
                        // Coming from east, going north
                        (prev_y, prev_x) = (current_y, current_x);
                        current_y -= 1;
                    }
                },
                Pipe::NorthWest => {
                    if (prev_y, prev_x) == (current_y - 1, current_x) {
                        // Coming from the north, going west
                        (prev_y, prev_x) = (current_y, current_x);
                        current_x -= 1;
                    } else {
                        // Coming from west, going north
                        (prev_y, prev_x) = (current_y, current_x);
                        current_y -= 1;
                    }
                },
                Pipe::SouthEast => {
                    if (prev_y, prev_x) == (current_y + 1, current_x) {
                        // Coming from the south, going east
                        (prev_y, prev_x) = (current_y, current_x);
                        current_x += 1;
                    } else {
                        // Coming from east, going south
                        (prev_y, prev_x) = (current_y, current_x);
                        current_y += 1;
                    }
                },
                Pipe::SouthWest => {
                    if (prev_y, prev_x) == (current_y + 1, current_x) {
                        // Coming from the south, going west
                        (prev_y, prev_x) = (current_y, current_x);
                        current_x -= 1;
                    } else {
                        // Coming from west, going south
                        (prev_y, prev_x) = (current_y, current_x);
                        current_y += 1;
                    }
                },
                Pipe::Start => {
                    // Look north
                    if current_y > 0 && matches!(self.map[current_y - 1][current_x], Pipe::Vertical|Pipe::SouthEast|Pipe::SouthWest) {
                        // Go north
                        current_y -= 1;
                    } else if matches!(self.map[current_y + 1][current_x], Pipe::Vertical|Pipe::NorthEast|Pipe::NorthWest) {
                        // Go south
                        current_y += 1;
                    } else if matches!(self.map[current_y][current_x + 1], Pipe::Horizontal|Pipe::NorthWest|Pipe::SouthWest) {
                        // Go east
                        current_x += 1;
                    } else if matches!(self.map[current_y][current_x - 1], Pipe::Horizontal|Pipe::NorthEast|Pipe::SouthEast) {
                        // Go west
                        current_x -= 1;
                    }
                },
                Pipe::Ground => unreachable!(),
            }
        }

        steps
    }

    pub fn expand(&self) -> Self {
        let mut map = Vec::with_capacity(self.map.len() * 2 - 1);
        let mut loop_tiles: HashSet<(usize, usize)> = self.main_pipeline.iter()
            .map(|(y, x)| (y*2,x*2))
            .collect();

        for (y, l) in self.map.iter().enumerate() {
            map.push(Vec::with_capacity(l.len() * 2 - 1));

            for (x, p) in l.iter().enumerate() {
                map[y*2].push(*p);

                if self.main_pipeline.contains(&(y, x)) && self.main_pipeline.contains(&(y, x + 1))
                    && matches!(p, Pipe::Horizontal|Pipe::NorthEast|Pipe::SouthEast|Pipe::Start)
                    && matches!(self.map[y][x + 1], Pipe::Horizontal|Pipe::NorthWest|Pipe::SouthWest|Pipe::Start)
                {
                    map[y*2].push(Pipe::Horizontal);
                    loop_tiles.insert((y*2, map[y*2].len() - 1));
                } else {
                    map[y*2].push(Pipe::Ground);
                }
            }

            map.push(Vec::with_capacity(l.len() * 2 - 1));
            for x in 0..l.len() {
                if self.main_pipeline.contains(&(y, x)) && self.main_pipeline.contains(&(y + 1, x))
                    && matches!(self.map[y][x], Pipe::Vertical|Pipe::SouthWest|Pipe::SouthEast|Pipe::Start)
                    && matches!(self.map[y + 1][x], Pipe::Vertical|Pipe::NorthWest|Pipe::NorthEast|Pipe::Start)
                {
                    map[y*2+1].push(Pipe::Vertical);
                    loop_tiles.insert((y*2+1, map[y*2+1].len() - 1));
                } else {
                    map[y*2+1].push(Pipe::Ground);
                }
                map[y*2+1].push(Pipe::Ground);
            }
        }

        Self { map, starting_point: self.starting_point, main_pipeline: loop_tiles }
    }

    pub fn flood_outside(&mut self) -> HashSet<(usize, usize)> {
        let mut outside_tiles = HashSet::new();

        let mut queue = Vec::new();
        for y in 0..self.map.len() {
            queue.push((y, 0));
            queue.push((y, self.map[y].len() - 1));
        }
        for x in 0..self.map[0].len() {
            queue.push((0, x));
            queue.push((0, self.map.len() - 1));
        }

        while let Some((y, x)) = queue.pop() {
            if outside_tiles.contains(&(y, x)) || self.main_pipeline.contains(&(y, x)) {
                continue;
            }
            outside_tiles.insert((y, x));
            if y >= self.map.len() || x >= self.map[0].len() {
                unreachable!();
            }

            for (dy, dx) in [
                (-1, -1), (-1, 0), (-1, 1),
                ( 0, -1),          ( 0, 1),
                ( 1, -1), ( 1, 0), ( 1, 1),
            ] {
                let new_location = (
                    (y as isize + dy) as usize,
                    (x as isize + dx) as usize
                );

                if new_location.0 >= self.map.len() || new_location.1 >= self.map[0].len() {
                    continue;
                }

                if !outside_tiles.contains(&new_location) && !self.main_pipeline.contains(&new_location) {
                    queue.push(new_location);
                }
            }
        }

        outside_tiles
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, l) in self.map.iter().enumerate() {
            for (x, t) in l.iter().enumerate() {
                if self.main_pipeline.contains(&(y, x)) {
                    write!(f, "{}", t)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut starting_point = (0, 0);
        let map = s.lines().enumerate()
            .map(|(y, l)| {
                l.chars().enumerate()
                    .map(|(x, c)| {
                        let p = Pipe::from_char(c);
                        if p == Pipe::Start {
                            starting_point = (y, x);
                        }
                        p
                    })
                    .collect()
            })
            .collect();

        Ok(Self { map, starting_point, main_pipeline: HashSet::new() })
    }
}

fn main() {
    let mut map = Map::from_str(include_str!("../input.txt")).unwrap();

    println!("[Part 1] Furthest away: {}", map.loop_length() / 2);


    let size = map.map.len() * map.map[0].len();
    let outside_tiles = map.expand()
        .flood_outside()
        .iter()
        .filter(|(y, x)| y % 2 == 0 && x % 2 == 0)
        .count();
    println!("[Part 2] Inside size: {}", size - outside_tiles - map.main_pipeline.len());
}
