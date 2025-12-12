use std::str::FromStr;

use crate::present::Present;

#[derive(Debug)]
pub struct Region {
    size: (u32, u32),
    requirement: Vec<u32>,
}

impl Region {
    pub fn fits_presents(&self, presents: &[Present]) -> bool {
        let required_tiles = self
            .requirement
            .iter()
            .zip(presents)
            .map(|(r, p)| *r * p.required_tiles())
            .sum();

        self.size.0 * self.size.1 >= required_tiles
    }
}

impl FromStr for Region {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, requirement) = s
            .split_once(": ")
            .ok_or("Invalid format: Could not find ':'")?;

        let (x, y) = size.split_once('x').unwrap();
        let requirement = requirement
            .split(' ')
            .map(|r| r.parse().unwrap())
            .collect();

        Ok(Self {
            size: (
                x.parse()
                    .map_err(|_| "x not a number")?,
                y.parse()
                    .map_err(|_| "y not a number")?,
            ),
            requirement,
        })
    }
}
