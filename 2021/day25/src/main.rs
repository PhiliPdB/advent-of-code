use hashbrown::HashSet;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cucumber {
    Open,
    East,
    South,
}

impl Cucumber {
    pub const fn from_char(c: char) -> Self {
        match c {
            '.' => Cucumber::Open,
            '>' => Cucumber::East,
            'v' => Cucumber::South,
            _   => panic!("Not a valid cucumber"),
        }
    }
}


fn main() {
    let map: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.chars().map(Cucumber::from_char).collect::<Vec<_>>())
        .collect();

    let width = map[0].len();
    let height = map.len();


    // Convert map into hashsets containing the coordinates for east- and south-moving cucumbers
    let mut east_coords = HashSet::new();
    let mut south_coords = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match *c {
                Cucumber::Open  => continue,
                Cucumber::East  => east_coords.insert((x, y)),
                Cucumber::South => south_coords.insert((x, y)),
            };
        }
    }


    let mut made_changes = true;
    let mut iterations = 0;
    while made_changes {
        made_changes = false;

        // Move east moving cucumbers
        let mut new_east_coords = HashSet::with_capacity(east_coords.len());
        for (x, y) in &east_coords {
            let new_x = (x + 1) % width;

            if south_coords.contains(&(new_x, *y)) || east_coords.contains(&(new_x, *y)) {
                new_east_coords.insert((*x, *y));
                continue;
            } else {
                made_changes = true;
                new_east_coords.insert((new_x, *y));
            }
        }
        east_coords = new_east_coords;


        // Move south moving cucumbers
        let mut new_south_coords = HashSet::with_capacity(south_coords.len());
        for (x, y) in &south_coords {
            let new_y = (y + 1) % height;

            if east_coords.contains(&(*x, new_y)) || south_coords.contains(&(*x, new_y)) {
                new_south_coords.insert((*x, *y));
                continue;
            } else {
                made_changes = true;
                new_south_coords.insert((*x, new_y));
            }
        }
        south_coords = new_south_coords;


        iterations += 1;
    }

    println!("Iterations: {}", iterations);
}
