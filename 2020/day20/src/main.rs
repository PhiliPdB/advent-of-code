use std::{str::FromStr, collections::HashSet};

pub const RUN_TEST: bool = false;

pub const TILE_BORDER: usize = 10;
pub const INPUT: &str =
    if RUN_TEST {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };
pub const IMAGE_SIZE: usize =
    if RUN_TEST {
        3
    } else {
        12
    };

pub type Border = [TileSpace; TILE_BORDER];
pub type Image = [[Option<Tile>; IMAGE_SIZE]; IMAGE_SIZE];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileSpace {
    Black,
    White,
}

impl TileSpace {
    pub fn from_char(c: char) -> Self {
        match c {
            '#' => TileSpace::Black,
            '.' => TileSpace::White,
            _   => panic!("Invalid space"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    id: u32,
    borders: [Border; 4],
}

impl Tile {
    pub fn matches(&mut self, top_border: &Option<Border>, left_border: &Option<Border>) -> Vec<Self> {
        let mut matches = Vec::new();

        // Check rotations
        for _ in 0..4 {
            if Tile::match_border(&self.borders[0], &top_border) && Tile::match_border(&self.borders[3], &left_border) {
                matches.push(*self);
            }
            self.rotate_clockwise();
        }

        // Check horizontal flip
        self.flip_horizontal();
        for _ in 0..4 {
            if Tile::match_border(&self.borders[0], &top_border) && Tile::match_border(&self.borders[3], &left_border) {
                matches.push(*self);
            }
            self.rotate_clockwise();
        }
        self.flip_horizontal();

        matches
    }

    fn rotate_clockwise(&mut self) {
        self.borders.rotate_right(1);
        self.borders[0].reverse();
        self.borders[2].reverse();
    }

    fn flip_horizontal(&mut self) {
        self.borders[0].reverse();
        self.borders[2].reverse();

        self.borders.swap(1, 3);
    }

    fn match_border(border: &Border, other_border: &Option<Border>) -> bool {
        if let Some(other_border) = other_border {
            border == other_border
        } else {
            true
        }
    }
}

impl FromStr for Tile {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let id = lines[0]
            .replace("Tile ", "")
            .replace(":", "")
            .parse::<u32>()
            .map_err(|_| "Couldn't parse tile id")?;

        let top_border = lines[1].chars()
            .map(|c| TileSpace::from_char(c))
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "Couldn't convert vec into border array")?;

        let bottom_border = lines[lines.len() - 1].chars()
            .map(|c| TileSpace::from_char(c))
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "Couldn't convert vec into border array")?;

        let left_border = lines.iter().skip(1)
            .map(|l| l.chars().nth(0).unwrap())
            .map(|c| TileSpace::from_char(c))
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "Couldn't convert vec into border array")?;

        let right_border = lines.iter().skip(1)
            .map(|l| l.chars().nth(TILE_BORDER - 1).unwrap())
            .map(|c| TileSpace::from_char(c))
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "Couldn't convert vec into border array")?;

        Ok(Self { id, borders: [top_border, right_border, bottom_border, left_border] })
    }
}


fn find_image(remaining_tiles: &HashSet<Tile>, image: Box<Image>, (next_x, next_y): (usize, usize)) -> Option<Box<Image>> {
    debug_assert!(next_x < IMAGE_SIZE && next_y < IMAGE_SIZE);

    if remaining_tiles.is_empty() {
        return Some(image);
    }

    let left_border = (next_x > 0).then(|| image[next_y][next_x - 1].unwrap().borders[1]);
    let top_border = (next_y > 0).then(|| image[next_y - 1][next_x].unwrap().borders[2]);

    for tile in remaining_tiles {
        let mut matched_tile = *tile;
        for matched_tile in matched_tile.matches(&top_border, &left_border) {
            let mut updated_image = image.clone();
            updated_image[next_y][next_x] = Some(matched_tile);

            let mut remaining = remaining_tiles.clone();
            remaining.remove(&tile);
            debug_assert!(remaining.len() < remaining_tiles.len());

            let next_coord =
                if next_x + 1 < IMAGE_SIZE {
                    (next_x + 1, next_y)
                } else {
                    (0, next_y + 1)
                };

            let complete_image = find_image(&remaining, updated_image, next_coord);
            if !complete_image.is_none() {
                return complete_image;
            }
        }
    }

    None
}


fn main() {
    let tiles: Vec<_> = INPUT
        .split("\n\n")
        .map(|s| Tile::from_str(s).unwrap())
        .collect();
    assert!(tiles.len() == IMAGE_SIZE * IMAGE_SIZE);

    // Part 1

    let remaining_tiles: HashSet<_> = tiles.into_iter().collect();
    let image = Box::new([[None; IMAGE_SIZE]; IMAGE_SIZE]);
    let image = find_image(&remaining_tiles, image, (0, 0)).unwrap();

    println!("Result: {}",
        image[0][0].unwrap().id as u64 * image[0][IMAGE_SIZE - 1].unwrap().id as u64 *
        image[IMAGE_SIZE - 1][0].unwrap().id as u64 * image[IMAGE_SIZE - 1][IMAGE_SIZE - 1].unwrap().id as u64
    );

    // Part 2

}
