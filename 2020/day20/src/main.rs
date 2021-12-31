use std::fmt::Display;
use std::str::FromStr;
use std::collections::{HashSet, HashMap};


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

impl Display for TileSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileSpace::Black => write!(f, "#"),
            TileSpace::White => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    id: u32,
    borders: [Border; 4],
    flip: bool,
    rotations: u8,
}

impl Tile {
    pub fn matches(&mut self, top_border: &Option<Border>, left_border: &Option<Border>) -> Vec<Self> {
        let mut matches = Vec::new();

        // Check rotations
        for _ in 0..4 {
            if Tile::match_border(&self.borders[0], top_border) && Tile::match_border(&self.borders[3], left_border) {
                matches.push(*self);
            }
            self.rotate_clockwise();
        }
        debug_assert_eq!(self.rotations, 0);

        // Check rotations when flipped
        self.flip_horizontal();
        for _ in 0..4 {
            if Tile::match_border(&self.borders[0], top_border) && Tile::match_border(&self.borders[3], left_border) {
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

        self.rotations += 1;
        self.rotations %= 4;
    }

    fn flip_horizontal(&mut self) {
        self.borders[0].reverse();
        self.borders[2].reverse();

        self.borders.swap(1, 3);

        self.flip = !self.flip;
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
            .parse()
            .map_err(|_| "Couldn't parse tile id")?;

        let top_border = lines[1].chars()
            .map(TileSpace::from_char)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "Couldn't convert vec into border array")?;

        let bottom_border = lines[lines.len() - 1].chars()
            .map(TileSpace::from_char)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "Couldn't convert vec into border array")?;

        let left_border = lines.iter().skip(1)
            .map(|l| l.chars().next().unwrap())
            .map(TileSpace::from_char)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "Couldn't convert vec into border array")?;

        let right_border = lines.iter().skip(1)
            .map(|l| l.chars().nth(TILE_BORDER - 1).unwrap())
            .map(TileSpace::from_char)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "Couldn't convert vec into border array")?;

        Ok(Self {
            id, borders: [top_border, right_border, bottom_border, left_border],
            flip: false,
            rotations: 0,
        })
    }
}


fn find_image(remaining_tiles: &HashSet<Tile>, image: Box<Image>, (next_x, next_y): (usize, usize)) -> Option<Box<Image>> {
    if remaining_tiles.is_empty() {
        return Some(image);
    }

    debug_assert!(next_x < IMAGE_SIZE && next_y < IMAGE_SIZE);

    let left_border = (next_x > 0).then(|| image[next_y][next_x - 1].unwrap().borders[1]);
    let top_border = (next_y > 0).then(|| image[next_y - 1][next_x].unwrap().borders[2]);

    for tile in remaining_tiles {
        let mut matched_tile = *tile;
        for matched_tile in matched_tile.matches(&top_border, &left_border) {
            let mut updated_image = image.clone();
            updated_image[next_y][next_x] = Some(matched_tile);

            let mut remaining = remaining_tiles.clone();
            remaining.remove(tile);
            debug_assert!(remaining.len() < remaining_tiles.len());

            let next_coord =
                if next_x + 1 < IMAGE_SIZE {
                    (next_x + 1, next_y)
                } else {
                    (0, next_y + 1)
                };

            let complete_image = find_image(&remaining, updated_image, next_coord);
            if complete_image.is_some() {
                return complete_image;
            }
        }
    }

    None
}


fn main() {
    let str_tiles: Vec<_> = INPUT
        .split("\n\n")
        .collect();
    assert!(str_tiles.len() == IMAGE_SIZE * IMAGE_SIZE);

    // Part 1

    let remaining_tiles: HashSet<_> = str_tiles.iter()
        .map(|s| Tile::from_str(s).unwrap())
        .collect();
    let image = Box::new([[None; IMAGE_SIZE]; IMAGE_SIZE]);
    let image = find_image(&remaining_tiles, image, (0, 0)).unwrap();

    println!("Result: {}",
        image[0][0].unwrap().id as u64 * image[0][IMAGE_SIZE - 1].unwrap().id as u64 *
        image[IMAGE_SIZE - 1][0].unwrap().id as u64 * image[IMAGE_SIZE - 1][IMAGE_SIZE - 1].unwrap().id as u64
    );

    // Part 2
    let mut img_tiles: HashMap<_, _> = str_tiles.iter()
        .map(|s| {
            let lines: Vec<_> = s.lines().collect();
            let id = lines[0]
                .replace("Tile ", "")
                .replace(":", "")
                .parse::<u32>().unwrap();

            let content: Vec<_> = lines.iter()
                .skip(2).take(TILE_BORDER - 2)
                .map(|l| {
                    l.chars().skip(1).take(TILE_BORDER - 2)
                        .map(TileSpace::from_char)
                        .collect::<Vec<_>>()
                })
                .collect();

            (id, content)
        })
        .collect();

    let mut img_map: Vec<Vec<TileSpace>> = vec![Vec::with_capacity(IMAGE_SIZE * (TILE_BORDER - 2)); IMAGE_SIZE * (TILE_BORDER - 2)];

    // Rotate each tile in the correct orientation and fill in the image map
    for y in 0..IMAGE_SIZE {
        for x in 0..IMAGE_SIZE {
            let tile = image[y][x].unwrap();
            let img_tile = img_tiles.get_mut(&tile.id).unwrap();

            if tile.flip {
                // Flip the image tile
                for r in img_tile.iter_mut() {
                    r.reverse();
                }
            }

            for _ in 0..tile.rotations {
                // Rotate the image
                for i in 0..(TILE_BORDER - 2) {
                    for j in 0..i {
                        let temp = img_tile[i][j];
                        img_tile[i][j] = img_tile[j][i];
                        img_tile[j][i] = temp;
                    }
                }

                for r in img_tile.iter_mut() {
                    r.reverse();
                }
            }

            // Put the image in the correct place in the big map
            for r in 0..(TILE_BORDER - 2) {
                img_map[y * (TILE_BORDER - 2) + r].extend(img_tile[r].iter());
            }
        }
    }

    // Verify the dimensions of the created image
    const MAP_SIZE: usize = IMAGE_SIZE * (TILE_BORDER - 2);
    debug_assert!(img_map.iter().all(|r| r.len() == MAP_SIZE));
    debug_assert_eq!(img_map.len(), MAP_SIZE);


    // Search and mark the monster squares

    // Hardcoded sea monster squares in different orientations
    let sea_monster_orientations = [
        [ // 'Normal orientation
            (0, 18),
            (1, 0), (1, 5), (1, 6), (1, 11), (1, 12), (1, 17), (1, 18), (1, 19),
            (2, 1), (2, 4), (2, 7), (2, 10), (2, 13), (2, 16)
        ],
        [ // Rotated 90deg
            (18, 2),
            (0, 1), (5, 1), (6, 1), (11, 1), (12, 1), (17, 1), (18, 1), (19, 1),
            (1, 0), (4, 0), (7, 0), (10, 0), (13, 0), (16, 0)
        ],
        [ // Rotated 180deg
            (2, 1),
            (1, 19), (1, 14), (1, 13), (1, 8), (1, 7), (1, 2), (1, 1), (1, 0),
            (0, 18), (0, 15), (0, 12), (0, 9), (0, 6), (0, 3)
        ],
        [ // Rotated 270deg
            (1, 0),
            (19, 1), (14, 1), (13, 1), (8, 1), (7, 1), (2, 1), (1, 1), (0, 1),
            (18, 2), (15, 2), (12, 2), (9, 2), (6, 2), (3, 2)
        ],
        // Flipped sea monsters
        [ // 'Normal orientation
            (0, 1),
            (1, 0), (1, 1), (1, 2), (1, 7), (1, 8), (1, 13), (1, 14), (1, 19),
            (2, 3), (2, 6), (2, 9), (2, 12), (2, 15), (2, 18)
        ],
        [ // Rotated 90deg
            (1, 2),
            (0, 1), (1, 1), (2, 1), (7, 1), (8, 1), (13, 1), (14, 1), (19, 1),
            (3, 0), (6, 0), (9, 0), (12, 0), (15, 0), (18, 0)
        ],
        [ // Rotated 180deg
            (2, 18),
            (1, 19), (1, 18), (1, 17), (1, 12), (1, 11), (1, 5), (1, 6), (1, 0),
            (0, 16), (0, 13), (0, 10), (0, 7), (0, 4), (0, 1)
        ],
        [ // Rotated 270deg
            (18, 0),
            (19, 1), (18, 1), (17, 1), (12, 1), (11, 1), (6, 1), (5, 1), (0, 1),
            (16, 2), (13, 2), (10, 2), (7, 2), (4, 2), (1, 2)
        ],
    ];

    let mut sea_monsters = 0;
    for (i, sea_monster) in sea_monster_orientations.iter().enumerate() {
        let (my, mx) =
            if i % 2 == 0 {
                (2, 19)
            } else {
                (19, 2)
            };

        for y in 0..(MAP_SIZE - my) {
            for x in 0..(MAP_SIZE - mx) {
                if sea_monster.iter().all(|(sy, sx)| img_map[y + sy][x + sx] == TileSpace::Black) {
                    sea_monsters += 1;
                }
            }
        }

        if sea_monsters > 0 {
            break;
        }
    }

    // NOTE: Assumes non-overlapping sea monsters
    let water_roughness: usize = img_map.iter()
        .map(|r| r.iter().filter(|s| **s == TileSpace::Black).count())
        .sum::<usize>() - (sea_monsters * 15);

    println!("Sea monsters: {}", sea_monsters);
    println!("Water roughness: {}", water_roughness);
}
