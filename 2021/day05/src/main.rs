use std::cmp::Ordering;
use std::str::FromStr;

pub type Coordinate = (usize, usize);

#[derive(Debug)]
pub struct LineSegment {
    start: Coordinate,
    end: Coordinate,
}

impl LineSegment {
    pub fn start(&self) -> Coordinate {
        self.start
    }

    pub fn end(&self) -> Coordinate {
        self.end
    }

    pub fn coordinates_on_segment(&self, diagonal: bool) -> Vec<Coordinate> {
        let mut coordinates = Vec::new();

        let (mut x, mut y) = self.start;
        let x_increment = match self.start.0.cmp(&self.end.0) {
            Ordering::Less    =>  1,
            Ordering::Equal   =>  0,
            Ordering::Greater => -1,
        };
        let y_increment = match self.start.1.cmp(&self.end.1) {
            Ordering::Less    =>  1,
            Ordering::Equal   =>  0,
            Ordering::Greater => -1,
        };

        // Check if we need to return diagonal points
        if !diagonal && !(x_increment == 0 || y_increment == 0) {
            return coordinates;
        }

        while (x, y) != self.end {
            coordinates.push((x, y));

            x = (x as i32 + x_increment) as usize;
            y = (y as i32 + y_increment) as usize;
        }
        coordinates.push(self.end);

        coordinates
    }
}

impl FromStr for LineSegment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<_> = s
            .split(" -> ")
            .flat_map(|c| c.split(','))
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        Ok(Self {
            start: (splitted[0], splitted[1]),
            end:   (splitted[2], splitted[3]),
        })
    }
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| LineSegment::from_str(s).unwrap())
        .collect();

    // Get max y and x values to determine coordinates of the diagram
    let max_x = input.iter()
        .flat_map(|l| vec![l.start().0, l.end().0])
        .max().unwrap();
    let max_y = input.iter()
        .flat_map(|l| vec![l.start().1, l.end().1])
        .max().unwrap();

    // Create the diagram
    let mut diagram = vec![vec![0; max_x + 1]; max_y + 1];
    for line in input {
        for (x, y) in line.coordinates_on_segment(true) {
            diagram[y][x] += 1;
        }
    }

    let dangerous_areas_count = diagram.iter()
        .flatten()
        .filter(|&&i| i >= 2)
        .count();
    println!("Dangerous areas: {}", dangerous_areas_count);
}
