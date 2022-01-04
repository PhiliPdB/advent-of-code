use std::f64::consts::{PI, FRAC_PI_2};

use num::integer::gcd;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Empty,
    Asteroid,
}

impl Square {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Square::Empty,
            '#' => Square::Asteroid,
            _ => panic!("Invalid square character"),
        }
    }
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.chars().map(Square::from_char).collect::<Vec<_>>())
        .collect();

    let height = input.len();
    let width = input[0].len();


    // Part 1

    let asteroid_locations: Vec<_> = input.iter().enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, s)| (*s == Square::Asteroid).then(|| (y, x)))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut detected = vec![vec![0; width]; height];
    for (i, (y1, x1)) in asteroid_locations.iter().enumerate() {
        for (y2, x2) in &asteroid_locations[(i+1)..] {
            let mut dx = *x2 as i32 - *x1 as i32;
            let mut dy = *y2 as i32 - *y1 as i32;
            let gcd = gcd(dx, dy);
            dx /= gcd;
            dy /= gcd;

            let mut is_blocking = false;
            let mut cur_x = *x1 as i32 + dx;
            let mut cur_y = *y1 as i32 + dy;
            while cur_x as usize != *x2 || cur_y as usize != *y2 {
                if input[cur_y as usize][cur_x as usize] == Square::Asteroid {
                    is_blocking = true;
                    break;
                }

                cur_x += dx;
                cur_y += dy;
            }

            if !is_blocking {
                detected[*y1][*x1] += 1;
                detected[*y2][*x2] += 1;
            }
        }
    }

    let most_asteroids = detected.iter()
        .enumerate()
        .map(|(y, row)| {
            let (x, max) = row.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
            ((y, x), max)
        })
        .max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();

    println!("Most asteroids: {}", most_asteroids.1);

    // Part 2

    let best_location = (most_asteroids.0.0 as i32, most_asteroids.0.1 as i32);
    // println!("Best location: {:?}", best_location);

    let mut transformed_asteroid_locations: Vec<_> = asteroid_locations.iter()
        .filter(|(y, x)| *y as i32 != best_location.0 || *x as i32 != best_location.1)
        .map(|(y, x)| {
            let (ty, tx) = (best_location.0 - *y as i32, *x as i32 - best_location.1);

            let r = ((tx * tx + ty * ty) as f64).sqrt();
            let mut phi = -(ty as f64).atan2(tx as f64);

            if phi < 0.0 {
                phi += 2.0 * PI;
            }
            phi += FRAC_PI_2;
            if phi >= 2.0 * PI {
                phi -= 2.0 * PI;
            }

            ((*y, *x), (r, phi))
        })
        .collect();
    transformed_asteroid_locations.sort_unstable_by(|(_, (r1, phi1)), (_, (r2, phi2))| {
        phi1.partial_cmp(phi2).unwrap()
            .then(r1.partial_cmp(r2).unwrap())
    });


    let mut visited =  vec![false; transformed_asteroid_locations.len()];
    visited[0] = true;

    let mut visited_asteroids = 0;
    let mut last_asteroid_index = 0;
    let mut last_asteroid = transformed_asteroid_locations[0];
    while visited_asteroids < 199 {
        last_asteroid_index += 1;
        last_asteroid_index %= transformed_asteroid_locations.len();

        if visited[last_asteroid_index] || transformed_asteroid_locations[last_asteroid_index].1.1 == last_asteroid.1.1 {
            continue;
        } else {
            visited[last_asteroid_index] = true;
            visited_asteroids += 1;
            last_asteroid = transformed_asteroid_locations[last_asteroid_index];
        }
    }


    let asteroid_location = last_asteroid.0;
    println!("200th asteroid: {}", asteroid_location.1 * 100 + asteroid_location.0);
}
