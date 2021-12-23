#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Space {
    Open,
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Space {
    pub const fn multiplier(&self) -> u32 {
        match self {
            Space::Open   => panic!("Can't move an open space"),
            Space::Amber  =>    1,
            Space::Bronze =>   10,
            Space::Copper =>  100,
            Space::Desert => 1000,
        }
    }

    pub const fn room_index(&self) -> usize {
        match self {
            Space::Open   => panic!("No room"),
            Space::Amber  => 0,
            Space::Bronze => 1,
            Space::Copper => 2,
            Space::Desert => 3,
        }
    }

    pub fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            'A' => Ok(Space::Amber),
            'B' => Ok(Space::Bronze),
            'C' => Ok(Space::Copper),
            'D' => Ok(Space::Desert),
            '.' => Ok(Space::Open),
            _   => Err("Invalid space character"),
        }
    }
}
