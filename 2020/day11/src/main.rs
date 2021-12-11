use std::mem;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeatStatus {
    Empty,
    Occupied,
    Floor
}

impl FromStr for SeatStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(SeatStatus::Empty),
            "#" => Ok(SeatStatus::Occupied),
            "." => Ok(SeatStatus::Floor),
            _   => Err("Invalid seat symbol"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Layout {
    seats: Vec<SeatStatus>,
    width: usize,
    height: usize,
    next_seats: Vec<SeatStatus>,
}

impl Layout {
    pub fn perform_step_part1(&mut self) -> bool {
        self.perform_step(4, false)
    }

    pub fn perform_step_part2(&mut self) -> bool {
        self.perform_step(5, true)
    }

    pub fn perform_step(&mut self, tolerance: u32, first_seat: bool) -> bool {
        let mut changed_state = false;

        for y in 0..self.height {
            for x in 0..self.width {
                let current_status = self.get_seat(x, y);
                let occupied_neighbours = self.occupied_neighbours(x, y, first_seat);

                match current_status {
                    SeatStatus::Empty if occupied_neighbours == 0 => {
                        changed_state = true;
                        self.set_next_seat(x, y, SeatStatus::Occupied)
                    },
                    SeatStatus::Occupied if occupied_neighbours >= tolerance => {
                        changed_state = true;
                        self.set_next_seat(x, y, SeatStatus::Empty)
                    },
                    _ => self.set_next_seat(x, y, current_status),
                }
            }
        }
        mem::swap(&mut self.seats, &mut self.next_seats);

        changed_state
    }

    pub fn occupied_seats(&self) -> u32 {
        self.seats.iter()
            .filter(|s| **s == SeatStatus::Occupied)
            .count() as u32
    }

    fn get_seat(&self, x: usize, y: usize) -> SeatStatus {
        self.seats[y * self.width + x]
    }

    fn set_next_seat(&mut self, x: usize, y: usize, status: SeatStatus) {
        self.next_seats[y * self.width + x] = status;
    }

    fn occupied_neighbours(&self, x: usize, y: usize, first_seat: bool) -> u32 {
        let mut occupied_neighbours = 0;
        for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
            let mut nx = (x as i32 + dx) as usize;
            let mut ny = (y as i32 + dy) as usize;
            if first_seat {
                while nx < self.width && ny < self.height
                    && self.get_seat(nx, ny) == SeatStatus::Floor
                {
                    nx = (nx as i32 + dx) as usize;
                    ny = (ny as i32 + dy) as usize;
                }
            }

            if nx < self.width && ny < self.height
                && self.get_seat(nx, ny) == SeatStatus::Occupied
            {
                occupied_neighbours += 1;
            }
        }
        occupied_neighbours
    }
}

impl FromStr for Layout {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();

        let seats: Vec<_> = s.chars()
            .filter_map(|c| SeatStatus::from_str(&c.to_string()).ok())
            .collect();

        if seats.len() == height * width {
            let next_seats = seats.clone();
            Ok(Self { seats, width, height, next_seats })
        } else {
            Err("Couldn't parse map")
        }
    }
}


fn main() {
    let input = Layout::from_str(include_str!("../input.txt")).unwrap();

    // Part 1

    let mut part1_layout = input.clone();
    while part1_layout.perform_step_part1() {
    }

    println!("[Part 1] Occupied seats: {}", part1_layout.occupied_seats());

    // Part 2

    let mut part2_layout = input;
    while part2_layout.perform_step_part2() {
    }

    println!("[Part 2] Occupied seats: {}", part2_layout.occupied_seats());
}
