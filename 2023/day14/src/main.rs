use std::collections::HashMap;
use std::fmt::{Display, self};
use std::str::FromStr;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty, Rock, RoundRock,
}

impl Field {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Rock,
            'O' => Self::RoundRock,
            _ => panic!("Invalid field: {c}"),
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Empty => write!(f, "."),
            Field::Rock => write!(f, "#"),
            Field::RoundRock => write!(f, "O"),
        }
    }
}

struct Map {
    map: Vec<Vec<Field>>,
}

impl Map {
    fn tilt_north(&mut self) {
        for y in 1..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] != Field::RoundRock {
                    continue;
                }

                let mut new_y = y;
                while new_y > 0 {
                    if self.map[new_y - 1][x] == Field::Empty {
                        new_y -= 1;
                        continue;
                    } else {
                        break;
                    }
                }

                self.map[y][x] = Field::Empty;
                self.map[new_y][x] = Field::RoundRock;
            }
        }
    }

    fn tilt_east(&mut self) {
        for x in (0..self.map[0].len()-1).rev() {
            for y in 0..self.map.len() {
                if self.map[y][x] != Field::RoundRock {
                    continue;
                }

                let mut new_x = x;
                while new_x < self.map[y].len() - 1 {
                    if self.map[y][new_x + 1] == Field::Empty {
                        new_x += 1;
                        continue;
                    } else {
                        break;
                    }
                }

                self.map[y][x] = Field::Empty;
                self.map[y][new_x] = Field::RoundRock;
            }
        }
    }

    fn tilt_south(&mut self) {
        for y in (0..self.map.len()-1).rev() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] != Field::RoundRock {
                    continue;
                }

                let mut new_y = y;
                while new_y < self.map.len() - 1 {
                    if self.map[new_y + 1][x] == Field::Empty {
                        new_y += 1;
                        continue;
                    } else {
                        break;
                    }
                }

                self.map[y][x] = Field::Empty;
                self.map[new_y][x] = Field::RoundRock;
            }
        }
    }

    fn tilt_west(&mut self) {
        for x in 1..self.map[0].len() {
            for y in 0..self.map.len() {
                if self.map[y][x] != Field::RoundRock {
                    continue;
                }

                let mut new_x = x;
                while new_x > 0 {
                    if self.map[y][new_x - 1] == Field::Empty {
                        new_x -= 1;
                        continue;
                    } else {
                        break;
                    }
                }

                self.map[y][x] = Field::Empty;
                self.map[y][new_x] = Field::RoundRock;
            }
        }
    }

    fn beam_load(&self) -> usize {
        self.map.iter().enumerate()
            .map(|(y, row)| (self.map.len() - y) * row.iter().filter(|f| **f == Field::RoundRock).count())
            .sum()
    }

    fn hash(&self) -> usize {
        self.map.iter().enumerate()
            .map(|(y, row)| {
                y * row.iter().enumerate()
                    .filter(|(_, f)| **f == Field::RoundRock)
                    .map(|(x, _)| x * 10)
                    .sum::<usize>()
            })
            .sum()
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s.lines()
            .map(|l| l.chars().map(Field::from_char).collect())
            .collect();

        Ok(Self { map })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.map {
            for field in row {
                write!(f, "{}", field)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut map = Map::from_str(include_str!("../input.txt")).unwrap();

    // Tilt north
    map.tilt_north();
    println!("[Part 1] Beam load: {:6}", map.beam_load());

    let mut scores = vec![map.beam_load()];
    let mut seen = HashMap::new();
    seen.insert(map.hash(), 0);


    let mut iteration = 1;
    let cycle_start;
    let cycle_length;
    loop {
        map.tilt_north();
        map.tilt_west();
        map.tilt_south();
        map.tilt_east();

        if let Some(o) = seen.insert(map.hash(), iteration) {
            cycle_start = o;
            cycle_length = iteration - o;
            break;
        }
        scores.push(map.beam_load());

        iteration += 1;
    }
    const TOTAL_ITERATIONS: usize = 1_000_000_000;
    let index = (TOTAL_ITERATIONS - cycle_start) % cycle_length;

    println!("[Part 2] Beam load: {:6}", scores[cycle_start + index]);
}
