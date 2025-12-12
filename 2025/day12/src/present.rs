use std::str::FromStr;

#[derive(Debug)]
pub struct Present {
    tiles: Vec<Vec<bool>>,
}

impl Present {
    pub fn required_tiles(&self) -> u32 {
        self.tiles
            .iter()
            .flatten()
            .map(|t| if *t { 1 } else { 0 })
            .sum()
    }
}

impl FromStr for Present {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            tiles: s
                .lines()
                .skip(1)
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect(),
        })
    }
}
