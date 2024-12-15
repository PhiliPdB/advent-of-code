use std::str::FromStr;
use std::fmt::{Display, Formatter};

use crate::{Move, Space};


#[derive(Debug, Clone)]
pub struct Map {
    map: Vec<Vec<Space>>,
    robot_position: (usize, usize),
}

impl Map {
    pub fn move_robot(&mut self, m: Move) {
        let (x, y) = self.robot_position;
        let (dx, dy) = m.direction();

        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x < 0 || new_x >= self.map[0].len() as isize {
            return;
        }
        if new_y < 0 || new_y >= self.map.len() as isize {
            return;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        match self.map[new_y][new_x] {
            Space::Wall => return,
            Space::Box => {
                let mut new_box_x = new_x as isize;
                let mut new_box_y = new_y as isize;
                while self.map[new_box_y as usize][new_box_x as usize] == Space::Box {
                    new_box_x += dx;
                    new_box_y += dy;
                }

                if new_box_x < 0 || new_box_x >= self.map[0].len() as isize {
                    return;
                }

                if new_box_y < 0 || new_box_y >= self.map.len() as isize {
                    return;
                }

                let new_box_x = new_box_x as usize;
                let new_box_y = new_box_y as usize;

                if self.map[new_box_y][new_box_x] == Space::Wall {
                    return;
                }

                self.map[new_y][new_x] = Space::Empty;
                self.map[new_box_y][new_box_x] = Space::Box;
            }
            Space::BoxOpen => {
                if !self.move_big_box(m, (new_x, new_y)) {
                    return;
                }
            },
            Space::BoxClose => {
                if !self.move_big_box(m, (new_x - 1, new_y)) {
                    return;
                }
            }
            Space::Empty => {},
        }

        self.robot_position = (new_x, new_y);
        debug_assert!(self.map[new_y][new_x] == Space::Empty);
    }

    pub fn box_gps_sum(&self) -> usize {
        self.map.iter().enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate()
                    .filter(|(_, space)| matches!(**space, Space::Box | Space::BoxOpen))
                    .map(move |(x, _)| (x, y))
            })
            .map(|(x, y)| x + 100 * y)
            .sum()
    }

    pub fn enlarge(&mut self) {
        let width = self.map[0].len();
        let height = self.map.len();

        let mut new_map = vec![vec![Space::Empty; width * 2]; height];

        for y in 0..height {
            for x in 0..width {
                if self.map[y][x] == Space::Box {
                    new_map[y][2*x] = Space::BoxOpen;
                    new_map[y][2*x+1] = Space::BoxClose;
                } else {
                    new_map[y][2*x] = self.map[y][x];
                    new_map[y][2*x+1] = self.map[y][x];
                }
            }
        }

        self.map = new_map;
        self.robot_position = (2 * self.robot_position.0, self.robot_position.1);
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, &space) in row.iter().enumerate() {
                if (x, y) == self.robot_position {
                    print!("@");
                } else {
                    match space {
                        Space::Empty => print!("."),
                        Space::Wall => print!("#"),
                        Space::Box => print!("O"),
                        Space::BoxOpen => print!("["),
                        Space::BoxClose => print!("]"),
                    }
                }
            }
            println!();
        }
    }

    fn can_move_big_box(&self, m: Move, (box_x, box_y): (usize, usize)) -> bool {
        debug_assert!(self.map[box_y][box_x] == Space::BoxOpen);

        let (dx, dy) = m.direction();
        let new_box_x = (box_x as isize + dx) as usize;
        let new_box_y = (box_y as isize + dy) as usize;

        let left_side = m == Move::Right || match self.map[new_box_y][new_box_x] {
            Space::Wall => false,
            Space::Box => false,
            Space::BoxOpen => self.can_move_big_box(m, (new_box_x, new_box_y)),
            Space::BoxClose => self.can_move_big_box(m, (new_box_x - 1, new_box_y)),
            Space::Empty => true,
        };
        let right_side = m == Move::Left || match self.map[new_box_y][new_box_x + 1] {
            Space::Wall => false,
            Space::Box => false,
            Space::BoxOpen => self.can_move_big_box(m, (new_box_x + 1, new_box_y)),
            Space::BoxClose => true, // Checked on the left side
            Space::Empty => true,
        };

        left_side && right_side
    }

    fn move_big_box(&mut self, m: Move, (box_x, box_y): (usize, usize)) -> bool {
        if !self.can_move_big_box(m, (box_x, box_y)) {
            return false;
        }

        let (dx, dy) = m.direction();
        let new_box_x = (box_x as isize + dx) as usize;
        let new_box_y = (box_y as isize + dy) as usize;

        // Clear the current box
        self.map[box_y][box_x] = Space::Empty;
        self.map[box_y][box_x + 1] = Space::Empty;

        // Move left side
        match self.map[new_box_y][new_box_x] {
            Space::Wall => unreachable!(),
            Space::Box => unreachable!(),
            Space::BoxOpen => {
                self.move_big_box(m, (new_box_x, new_box_y));
            },
            Space::BoxClose => {
                self.move_big_box(m, (new_box_x - 1, new_box_y));
            },
            Space::Empty => {},
        }
        self.map[new_box_y][new_box_x] = Space::BoxOpen;

        // Move right side
        match self.map[new_box_y][new_box_x + 1] {
            Space::Wall => unreachable!(),
            Space::Box => unreachable!(),
            Space::BoxOpen => {
                self.move_big_box(m, (new_box_x + 1, new_box_y));
            },
            Space::BoxClose => {}, // Checked on the left side
            Space::Empty => {},
        }
        self.map[new_box_y][new_box_x + 1] = Space::BoxClose;

        true
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut robot_position = None;

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    robot_position = Some((x, y));
                    row.push(Space::Empty);
                } else {
                    let space = Space::from_char(c);
                    row.push(space);
                }
            }
            map.push(row);
        }

        let robot_position = robot_position.ok_or("No robot position found")?;

        Ok(Map { map, robot_position })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            for (x, &space) in row.iter().enumerate() {
                if (x, y) == self.robot_position {
                    write!(f, "@")?;
                } else {
                    write!(f, "{}", space)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
