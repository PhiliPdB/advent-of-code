use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Trail {
    Path, Forest,
    SlopeNorth, SlopeSouth,
    SlopeWest, SlopeEast,
}

impl Trail {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::SlopeNorth,
            'v' => Self::SlopeSouth,
            '<' => Self::SlopeWest,
            '>' => Self::SlopeEast,
            _ => panic!("Invalid trail char: {c}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    North, East, South, West,
}

impl Direction {
    const fn step(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Self::North => (x, y - 1),
            Self::East  => (x + 1, y),
            Self::South => (x, y + 1),
            Self::West  => (x - 1, y),
        }
    }

    const fn continue_in(&self) -> [Direction; 3] {
        match self {
            Self::North => [Self::North, Self::East, Self::West],
            Self::East  => [Self::North, Self::East, Self::South],
            Self::South => [Self::East, Self::South, Self::West],
            Self::West  => [Self::North, Self::South, Self::West],
        }
    }
}


#[derive(Debug, Clone)]
pub(crate) struct Map {
    map: Vec<Vec<Trail>>,
    start: (usize, usize),
    goal: (usize, usize),
}

impl Map {
    #[inline(always)]
    pub(crate) const fn start(&self) -> (usize, usize) {
        self.start
    }

    #[inline(always)]
    pub(crate) const fn goal(&self) -> (usize, usize) {
        self.goal
    }

    pub(crate) fn next_crossing<const WITH_SLOPES: bool>(&self, (mut x, mut y): (usize, usize), mut direction: Direction) -> (u32, (usize, usize)) {
        // Check if we can even walk in this direction
        let (next_x, next_y) = direction.step((x, y));
        if !self.can_continue::<WITH_SLOPES>((next_x, next_y), direction) {
            return (0, (x, y));
        }
        (x, y) = (next_x, next_y);

        let mut steps = 1;
        loop {
            if (x, y) == self.goal {
                return (steps, (x, y));
            }

            let can_continue_in: Vec<_> = direction.continue_in().into_iter()
                .filter(|d| {
                    let (sx, sy) = d.step((x, y));
                    self.can_continue::<WITH_SLOPES>((sx, sy), *d)
                })
                .collect();

            if can_continue_in.len() > 1 || can_continue_in.is_empty() {
                return (steps, (x, y));
            }
            direction = can_continue_in[0];

            (x, y) = direction.step((x, y));
            steps += 1;
        }
    }

    fn can_continue<const WITH_SLOPES: bool>(&self, (x, y): (usize, usize), direction: Direction) -> bool {
        match self.map[y][x] {
            Trail::Path   => true,
            Trail::Forest => false,
            Trail::SlopeNorth => {
                !WITH_SLOPES || direction == Direction::North
            },
            Trail::SlopeSouth => {
                !WITH_SLOPES || direction == Direction::South
            },
            Trail::SlopeWest => {
                !WITH_SLOPES || direction == Direction::West
            },
            Trail::SlopeEast => {
                !WITH_SLOPES || direction == Direction::East
            },
        }
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: Vec<Vec<_>> = s.lines()
            .map(|l| l.chars().map(Trail::from_char).collect())
            .collect();

        map[0][1] = Trail::Forest;
        let start = (1, 1);
        let goal = (map[0].len() - 2, map.len() - 1);

        Ok(Self { map, start, goal })
    }
}
