use std::str::FromStr;

use hashbrown::HashSet;
use scan_fmt::parse::ScanError;

#[macro_use] extern crate scan_fmt;


#[derive(Debug, Clone, Copy)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    pub fn position(&self, time: i32, width: i32, height: i32) -> (i32, i32) {
        let mut x = self.position.0 + time * self.velocity.0;
        let mut y = self.position.1 + time * self.velocity.1;

        x = x.rem_euclid(width);
        y = y.rem_euclid(height);

        (x, y)
    }
}

impl FromStr for Robot {
    type Err = ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, vx, vy) = scan_fmt!(s, "p={},{} v={},{}", i32, i32, i32, i32)?;

        Ok(Robot {
            position: (x, y),
            velocity: (vx, vy),
        })
    }
}


fn safety_factor(robots: &[Robot], time: i32, width: i32, height: i32) -> i32 {
    let positions = robots.iter()
        .map(|r| r.position(time, width, height));

    let mut quadrant_count = [0; 4];

    for (x, y) in positions {
        if x == width / 2 || y == height / 2 {
            continue;
        }

        match (x < width / 2, y < height / 2) {
            (true,  true)  => quadrant_count[0] += 1,
            (true,  false) => quadrant_count[1] += 1,
            (false, true)  => quadrant_count[2] += 1,
            (false, false) => quadrant_count[3] += 1,
        }
    }

    quadrant_count.into_iter().product()
}

fn has_christmas_tree(robots: &[Robot], time: i32, width: i32, height: i32) -> bool {
    let positions: HashSet<_> = robots.iter()
        .map(|r| r.position(time, width, height))
        .collect();

    positions.iter()
        .any(|&pos| is_christmas_tree(&positions, pos))
}

fn is_christmas_tree(positions: &HashSet<(i32, i32)>, (x, y): (i32, i32)) -> bool {
    debug_assert!(positions.contains(&(x, y)));

    // Check for a christmas tree like structure

    for dy in 1..10 { // Check the next 10 rows, should be enough...
        for dx in 0..dy {
            if !positions.contains(&(x + dx, y+dy)) || !positions.contains(&(x - dx, y+dy)) {
                return false;
            }
        }
    }

    true
}

fn main() {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    let robots: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| Robot::from_str(l).unwrap())
        .collect();


    let part1_safety = safety_factor(&robots, 100, WIDTH, HEIGHT);
    println!("[Part 1] Bathroom safety {part1_safety}");

    let mut t = 0;
    loop {
        if has_christmas_tree(&robots, t, WIDTH, HEIGHT) {
            println!("[Part 2] Christmas tree at time {t}");
            break;
        }
        t += 1;
    }

    // Print the christmas tree
    let positions: HashSet<_> = robots.iter()
        .map(|r| r.position(t, WIDTH, HEIGHT))
        .collect();

    println!();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if positions.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
