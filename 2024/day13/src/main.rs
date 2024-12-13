#[macro_use] extern crate scan_fmt;

use std::str::FromStr;

use nalgebra::{matrix, vector, Matrix2};
use scan_fmt::parse::ScanError;



#[derive(Debug)]
struct ClawMachine {
    prize_location: (i64, i64),
    button_a: (i64, i64),
    button_b: (i64, i64),
}

impl ClawMachine {
    const BUTTON_A_PRICE: i64 = 3;
    const BUTTON_B_PRICE: i64 = 1;

    pub fn fewest_tokens(&self) -> Option<i64> {
        // Using Cramer's rule to solve the linear equations

        let velocity_matrix = matrix![
            self.button_a.0 as f64, self.button_b.0 as f64;
            self.button_a.1 as f64, self.button_b.1 as f64;
        ];
        let position = vector![
            self.prize_location.0 as f64,
            self.prize_location.1 as f64
        ];
        // Note: This system of equations has a unique solution
        //       finding this solution, means that we also found
        //       the minimal solution.

        let det_a = velocity_matrix.determinant();
        let a_presses = Matrix2::from_columns(
            &[position, velocity_matrix.column(1).into()]
        ).determinant() / det_a;
        let b_presses = Matrix2::from_columns(
            &[velocity_matrix.column(0).into(), position]
        ).determinant() / det_a;

        if a_presses.is_nan() || b_presses.is_nan() {
            return None;
        }

        let a_presses = a_presses.round() as i64;
        let b_presses = b_presses.round() as i64;

        // Check if the solution is valid
        if a_presses < 0 || b_presses < 0
            || a_presses * self.button_a.0 + b_presses * self.button_b.0 != self.prize_location.0
            || a_presses * self.button_a.1 + b_presses * self.button_b.1 != self.prize_location.1
        {
            return None;
        }

        Some(a_presses * Self::BUTTON_A_PRICE + b_presses * Self::BUTTON_B_PRICE)
    }

    pub fn update_prize_location(&mut self, constant: i64) {
        self.prize_location.0 += constant;
        self.prize_location.1 += constant;
    }
}

impl FromStr for ClawMachine {
    type Err = ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();

        let button_a = scan_fmt!(lines[0], "Button A: X+{}, Y+{}", i64, i64)?;
        let button_b = scan_fmt!(lines[1], "Button B: X+{}, Y+{}", i64, i64)?;
        let prize_location = scan_fmt!(lines[2], "Prize: X={}, Y={}", i64, i64)?;

        Ok(ClawMachine {
            prize_location,
            button_a,
            button_b,
        })
    }
}

fn main() {
    let mut machines: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .map(|c| ClawMachine::from_str(c).unwrap())
        .collect();


    // Part 1
    let part1_min_cost: i64 = machines.iter()
        .filter_map(|m| m.fewest_tokens())
        .sum();
    println!("[Part 1] Fewest tokens to win them 'all': {part1_min_cost:14}");


    // Part 2
    const PRIZE_LOCATION_ERROR: i64 = 10_000_000_000_000;
    for m in machines.iter_mut() {
        m.update_prize_location(PRIZE_LOCATION_ERROR);
    }

    let part2_min_cost: i64 = machines.iter()
        .filter_map(|m| m.fewest_tokens())
        .sum();
    println!("[Part 2] Fewest tokens to win them 'all': {part2_min_cost:14}");
}
