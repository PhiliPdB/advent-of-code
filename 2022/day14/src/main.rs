use std::{fmt::Display, cmp};

const HEIGHT: usize = 180;
const WIDTH: usize = 500;
const X_START: usize = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Air,
    Rock,
    Sand,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::Air  => write!(f, "."),
            Space::Rock => write!(f, "#"),
            Space::Sand => write!(f, "o"),
        }
    }
}

fn get_sand_units(mut grid: Vec<Vec<Space>>) -> u32 {
    let mut sand_units = 0;
    const SAND_START: (usize, usize) = (500 - X_START, 0);
    'new_sand: loop {
        if grid[SAND_START.1][SAND_START.0] == Space::Sand {
            // Cannot create new sand
            break;
        }

        // Calculate path
        let mut current_x = SAND_START.0;
        let mut current_y = SAND_START.1;

        while current_y + 1 < grid.len() {
            if grid[current_y + 1][current_x] == Space::Air {
                current_y += 1;
            } else if grid[current_y + 1][current_x - 1] == Space::Air {
                current_y += 1;
                current_x -= 1;
            } else if grid[current_y + 1][current_x + 1] == Space::Air {
                current_y += 1;
                current_x += 1;
            } else {
                sand_units += 1;
                grid[current_y][current_x] = Space::Sand;
                continue 'new_sand;
            }
        }

        // Sand falls out of the 'world'
        break;
    }

    sand_units
}


fn main() {
    let mut grid = vec![Vec::new(); HEIGHT];
    for row in grid.iter_mut() {
        *row = vec![Space::Air; WIDTH];
    }

    let mut highest_y = 0;
    for line in include_str!("../input.txt").lines() {
        let coords: Vec<_> = line.split(" -> ")
            .map(|c| {
                let (x, y) = c.split_once(',').unwrap();
                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
            .collect();

        for c in coords.windows(2) {
            let (x1, y1) = c[0];
            let (x2, y2) = c[1];

            if y1 > highest_y || y2 > highest_y {
                highest_y = cmp::max(y1, y2);
            }

            if x1 == x2 {
                let y_min = cmp::min(y1, y2);
                let y_max = cmp::max(y1, y2);

                // Move y
                #[allow(clippy::needless_range_loop)]
                for y in y_min..=y_max {
                    grid[y][x1 - X_START] = Space::Rock;
                }
            } else if y1 == y2 {
                let x_min = cmp::min(x1, x2);
                let x_max = cmp::max(x1, x2);

                // Move x
                #[allow(clippy::needless_range_loop)]
                for x in x_min..=x_max {
                    grid[y1][x - X_START] = Space::Rock;
                }
            } else {
                unreachable!()
            }
        }
    }

    // Part 1
    println!("[Part 1] Sand units created: {:5}", get_sand_units(grid.clone()));

    // Part 2
    for x in 0..grid[0].len() {
        grid[highest_y + 2][x] = Space::Rock;
    }
    println!("[Part 2] Sand units created: {:5}", get_sand_units(grid.clone()));
}
