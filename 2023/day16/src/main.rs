use std::str::FromStr;

use hashbrown::HashSet;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Heading {
    North, East, South, West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    MirrorSWNE,
    MirrorSENW,
    SplitVertical,
    SplitHorizontal,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.'  => Self::Empty,
            '/'  => Self::MirrorSWNE,
            '\\' => Self::MirrorSENW,
            '|'  => Self::SplitVertical,
            '-'  => Self::SplitHorizontal,
            _    => panic!("Invalid tile: {c}"),
        }
    }
}

#[derive(Debug)]
struct Facility {
    map: Vec<Vec<Tile>>,
}

impl Facility {
    pub fn find_best_beam_config(&self) -> usize {
        let mut max_energized_tiles = 0;

        let height = self.map.len() as i32;
        let width = self.map[0].len() as i32;

        for x in 0..width {
            let point_south = self.energized_tiles(((x, 0), Heading::South)).len();
            max_energized_tiles = max_energized_tiles.max(point_south);

            let point_north = self.energized_tiles(((x, height - 1), Heading::North)).len();
            max_energized_tiles = max_energized_tiles.max(point_north);
        }

        for y in 0..height {
            let point_east = self.energized_tiles(((0, y), Heading::East)).len();
            max_energized_tiles = max_energized_tiles.max(point_east);

            let point_west = self.energized_tiles(((width - 1, y), Heading::West)).len();
            max_energized_tiles = max_energized_tiles.max(point_west);
        }

        max_energized_tiles
    }

    pub fn energized_tiles(&self, starting_beam: ((i32, i32), Heading)) -> HashSet<(i32, i32)> {
        let mut energized_tiles = HashSet::new();
        let mut visited = HashSet::new();

        let height = self.map.len();
        let width = self.map[0].len();

        let mut beams = vec![starting_beam];
        while let Some(((x, y), heading)) = beams.pop() {
            if !(0..width as i32).contains(&x) || !(0..height as i32).contains(&y) {
                continue;
            }

            if !visited.insert(((x, y), heading)) {
                continue;
            }

            energized_tiles.insert((x, y));

            match (self.map[y as usize][x as usize], heading) {
                (Tile::Empty, h) => beams.push(Self::new_location((x, y), h)),
                (Tile::MirrorSWNE, Heading::North) => beams.push(Self::new_location((x, y), Heading::East)),
                (Tile::MirrorSWNE, Heading::East)  => beams.push(Self::new_location((x, y), Heading::North)),
                (Tile::MirrorSWNE, Heading::South) => beams.push(Self::new_location((x, y), Heading::West)),
                (Tile::MirrorSWNE, Heading::West)  => beams.push(Self::new_location((x, y), Heading::South)),
                (Tile::MirrorSENW, Heading::North) => beams.push(Self::new_location((x, y), Heading::West)),
                (Tile::MirrorSENW, Heading::East)  => beams.push(Self::new_location((x, y), Heading::South)),
                (Tile::MirrorSENW, Heading::South) => beams.push(Self::new_location((x, y), Heading::East)),
                (Tile::MirrorSENW, Heading::West)  => beams.push(Self::new_location((x, y), Heading::North)),
                (Tile::SplitVertical, Heading::East|Heading::West) => {
                    beams.push(Self::new_location((x, y), Heading::North));
                    beams.push(Self::new_location((x, y), Heading::South));
                },
                (Tile::SplitVertical, h) => beams.push(Self::new_location((x, y), h)),
                (Tile::SplitHorizontal, Heading::North|Heading::South) => {
                    beams.push(Self::new_location((x, y), Heading::East));
                    beams.push(Self::new_location((x, y), Heading::West));
                },
                (Tile::SplitHorizontal, h) => beams.push(Self::new_location((x, y), h)),
            }
        }

        energized_tiles
    }

    #[inline(always)]
    const fn new_location((x, y): (i32, i32), new_heading: Heading) -> ((i32, i32), Heading) {
        match new_heading {
            Heading::North =>((x, y - 1), Heading::North),
            Heading::East  =>((x + 1, y), Heading::East),
            Heading::South =>((x, y + 1), Heading::South),
            Heading::West  =>((x - 1, y), Heading::West),
        }
    }
}

impl FromStr for Facility {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s.lines()
            .map(|l| l.chars().map(Tile::from_char).collect())
            .collect();

        Ok(Facility { map })
    }
}


fn main() {
    let facility = Facility::from_str(include_str!("../input.txt")).unwrap();

    println!("[Part 1] Number of energized tiles: {}", facility.energized_tiles(((0, 0), Heading::East)).len());
    println!("[Part 2] Number of energized tiles: {}", facility.find_best_beam_config());
}
