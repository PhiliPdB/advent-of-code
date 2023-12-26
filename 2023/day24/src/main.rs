use std::str::FromStr;

use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};

#[derive(Debug, Clone, Copy, Default)]
struct Hail {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl Hail {
    fn intersects(&self, other: &Self) -> bool {
        const MIN: f64 = 200_000_000_000_000.0;
        const MAX: f64 = 400_000_000_000_000.0;

        if let Some((x, y)) = self.intersection_position(other) {
            (MIN..=MAX).contains(&x) && (MIN..=MAX).contains(&y)
        } else {
            false
        }
    }

    fn intersection_position(&self, other: &Self) -> Option<(f64, f64)> {
        // Using Cramer's rule to solve the linear equations

        let velocity_matrix = Matrix2::new(
            self.velocity.0, -other.velocity.0,
            self.velocity.1, -other.velocity.1
        );
        let position = Vector2::new(
            other.position.0 - self.position.0,
            other.position.1 - self.position.1
        );


        let det_a = velocity_matrix.determinant();
        let a = Matrix2::from_columns(
            &[position, velocity_matrix.column(1).into()]
        ).determinant() / det_a;
        let b = Matrix2::from_columns(
            &[velocity_matrix.column(0).into(), position]
        ).determinant() / det_a;

        // Note 0/0 == NaN, while x/0 == +-inf for x != 0.
        // Here we use -1 as some sort of (horrible) indication that there are infinitely many solutions
        if a.is_nan() || b.is_nan() {
            return Some((-1.0, -1.0));
        }

        if a < 0.0 || b < 0.0 {
            // One of the intersections happened in the past
            return None;
        }

        let x = self.position.0 + a.round() * self.velocity.0;
        let y = self.position.1 + a.round() * self.velocity.1;

        Some((x, y))
    }

    fn adjust_velocity(&self, v: (f64, f64)) -> Self {
        Self {
            position: self.position,
            velocity: (
                self.velocity.0 - v.0,
                self.velocity.1 - v.1,
                self.velocity.2,
            )
        }
    }
}

impl FromStr for Hail {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [position, velocity] = s.split(" @ ").collect::<Vec<_>>()
            .try_into().map_err(|_| "Invalid hail format")?;

        let position: Vec<_> = position.split(", ")
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let position = (position[0], position[1], position[2]);

        let velocity: Vec<_> = velocity.split(", ")
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let velocity = (velocity[0], velocity[1], velocity[2]);

        Ok(Self { position, velocity })
    }
}

fn main() {
    let mut hail_stones: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| Hail::from_str(l).unwrap())
        .collect();

    let total_intersections = hail_stones.iter()
        .combinations(2)
        .filter(|hs| hs[0].intersects(hs[1]))
        .count();
    println!("[Part 1] Total intersections: {total_intersections}");

    // Knowing 3 hail stones is enough
    hail_stones.resize(3, Hail::default());

    'velocity_search: for vx in -500..500 {
        'next_vy: for vy in -500..500 {
            let stones: Vec<_> = hail_stones.iter()
                .map(|h| h.adjust_velocity((vx as f64, vy as f64)))
                .collect();

            let position = stones[0].intersection_position(&stones[1]);
            if position.is_none() {
                continue 'next_vy;
            }
            let (mut x, mut y) = position.unwrap();
            for hs in stones.into_iter().combinations(2) {
                if let Some((px, py)) = hs[0].intersection_position(&hs[1]) {
                    if px == -1.0 && py == -1.0 {
                        continue;
                    }

                    if x == -1.0 && y == -1.0 {
                        x = px;
                        y = py;
                    } else if !((px - x).abs() < 0.1 && (py - y).abs() < 0.1) {
                        continue 'next_vy;
                    }
                } else {
                    continue 'next_vy;
                }
            }

            // Figure out the correct value for vz and z

            let t1 = (x - hail_stones[0].position.0) / (hail_stones[0].velocity.0 - vx as f64);
            let z1 = hail_stones[0].position.2 + t1 * hail_stones[0].velocity.2;

            let t2 = (x - hail_stones[1].position.0) / (hail_stones[1].velocity.0 - vx as f64);
            let z2 = hail_stones[1].position.2 + t2 * hail_stones[1].velocity.2;

            let vz = (z2 - z1) / (t2 - t1);
            let z = z1 - t1 * vz;

            println!("[Part 2] Sum of position coordinates: {}", x + y + z);
            break 'velocity_search;
        }
    }
}
