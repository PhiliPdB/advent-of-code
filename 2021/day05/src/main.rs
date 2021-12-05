use std::str::FromStr;

type Coordinate = (usize, usize);

#[derive(Debug)]
struct LineSegment {
    pub start: Coordinate,
    pub end: Coordinate,
}

impl LineSegment {
    pub fn get_coordinates_on_segment(&self) -> Vec<Coordinate> {
        let mut coordinates = Vec::new();

        if self.start.0 == self.end.0 {
            // Horizontal line

            let range =
                if self.start.1 <= self.end.1 {
                    self.start.1..=self.end.1
                } else {
                    self.end.1..=self.start.1
                };

            for y in range {
                coordinates.push((self.start.0, y));
            }
        } else if self.start.1 == self.end.1 {
            // Vertical line

            let range =
                if self.start.0 <= self.end.0 {
                    self.start.0..=self.end.0
                } else {
                    self.end.0..=self.start.0
                };

            for x in range {
                coordinates.push((x, self.start.1));
            }
        } else {
            // Diagonal
            let (mut x, mut y) = self.start;
            let x_increment =
                if self.start.0 <= self.end.0 {
                    1
                } else {
                    -1
                };
            let y_increment =
                if self.start.1 <= self.end.1 {
                    1
                } else {
                    -1
                };

            while (x, y) != self.end {
                coordinates.push((x, y));

                x = (x as i32 + x_increment) as usize;
                y = (y as i32 + y_increment) as usize;
            }
            coordinates.push(self.end);
        }

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
        .flat_map(|l| vec![l.start.0, l.end.0])
        .max().unwrap();
    let max_y = input.iter()
        .flat_map(|l| vec![l.start.1, l.end.1])
        .max().unwrap();

    // Create the diagram
    let mut diagram = vec![vec![0; max_x + 1]; max_y + 1];
    for line in input {
        for (x, y) in line.get_coordinates_on_segment() {
            diagram[y][x] += 1;
        }
    }

    let dangerous_areas_count = diagram.iter()
        .flatten()
        .filter(|&&i| i >= 2)
        .count();
    println!("Dangerous areas: {}", dangerous_areas_count);
}
