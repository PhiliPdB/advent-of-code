use std::str::FromStr;

use crate::{present::Present, region::Region};

mod present;
mod region;

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .collect();

    let presents: Vec<_> = input
        .iter()
        .take(input.len() - 1)
        .map(|i| Present::from_str(i).unwrap())
        .collect();
    let regions: Vec<_> = input[input.len() - 1]
        .lines()
        .map(|l| Region::from_str(l).unwrap())
        .collect();

    let part1_count = regions
        .iter()
        .filter(|r| r.fits_presents(&presents))
        .count();
    println!("[Part 1] Regions that fit: {part1_count}");
}
