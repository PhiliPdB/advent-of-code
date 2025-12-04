use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    Paper,
}

impl Field {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Empty),
            '@' => Some(Self::Paper),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Field>>,
}

impl Grid {
    pub fn forklift_accessible(&self) -> u32 {
        let mut accessible = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.is_forklift_accessible(x, y) {
                    accessible += 1;
                }
            }
        }

        accessible
    }

    pub fn remove_accessible(&mut self) -> u32 {
        let mut accessible = vec![];

        // Find accessible paper rolls
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.is_forklift_accessible(x, y) {
                    accessible.push((x, y));
                }
            }
        }

        // Then remove them
        for &(x, y) in &accessible {
            self.grid[y][x] = Field::Empty;
        }

        // And return how many rolls were removed
        accessible.len() as u32
    }

    fn is_forklift_accessible(&self, x: usize, y: usize) -> bool {
        if self.grid[y][x] == Field::Empty {
            return false;
        }

        let height = self.grid.len();
        let width = self.grid[0].len();

        let adjacent = [
            (x.overflowing_sub(1).0, y.overflowing_sub(1).0),
            (x, y.overflowing_sub(1).0),
            (x + 1, y.overflowing_sub(1).0),
            (x.overflowing_sub(1).0, y),
            (x + 1, y),
            (x.overflowing_sub(1).0, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        let paper_count = adjacent
            .into_iter()
            .filter(|(ax, ay)| *ax < width && *ay < height && self.grid[*ay][*ax] == Field::Paper)
            .count();

        paper_count < 4
    }
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| Field::from_char(c).unwrap())
                    .collect()
            })
            .collect();

        Ok(Grid { grid })
    }
}

fn main() {
    let grid = Grid::from_str(include_str!("../input.txt")).unwrap();

    println!("[Part 1] Rolls accessible: {}", grid.forklift_accessible());

    // NOTE: Need to make the grid mutable for part 2
    let mut grid = grid;

    let mut removed = 0;
    loop {
        let r = grid.remove_accessible();
        removed += r;
        if r == 0 {
            break;
        }
    }
    println!("[Part 2] Rolls removed: {removed}");
}
