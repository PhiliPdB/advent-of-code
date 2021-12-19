use hashbrown::HashSet;

const OVERLAPPING: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down
}

impl Facing {
    pub const fn facings() -> [(Facing, Facing); 24] {
        [
            (Facing::North, Facing::East), (Facing::North, Facing::West),
            (Facing::North, Facing::Up), (Facing::North, Facing::Down),
            (Facing::East, Facing::North), (Facing::East, Facing::South),
            (Facing::East, Facing::Up), (Facing::East, Facing::Down),
            (Facing::South, Facing::East), (Facing::South, Facing::West),
            (Facing::South, Facing::Up), (Facing::South, Facing::Down),
            (Facing::West, Facing::North), (Facing::West, Facing::South),
            (Facing::West, Facing::Up), (Facing::West, Facing::Down),
            (Facing::Up, Facing::North), (Facing::Up, Facing::East),
            (Facing::Up, Facing::South), (Facing::Up, Facing::West),
            (Facing::Down, Facing::North), (Facing::Down, Facing::East),
            (Facing::Down, Facing::South), (Facing::Down, Facing::West),
        ]
    }
}

pub const fn transform_to_north((x, y, z): (i32, i32, i32), facing: (Facing, Facing)) -> (i32, i32, i32) {
    match facing {
        (Facing::North, Facing::East)  => (x, z, -y),
        (Facing::North, Facing::West)  => (x, -z, y),
        (Facing::North, Facing::Up)    => (x, y, z),
        (Facing::North, Facing::Down)  => (x, -y, -z),
        (Facing::East, Facing::North)  => (z, x, y),
        (Facing::East, Facing::South)  => (-z, x, -y),
        (Facing::East, Facing::Up)     => (-y, x, z),
        (Facing::East, Facing::Down)   => (y, x, -z),
        (Facing::South, Facing::East)  => (-x, z, y),
        (Facing::South, Facing::West)  => (-x, -z, -y),
        (Facing::South, Facing::Up)    => (-x, -y, z),
        (Facing::South, Facing::Down)  => (-x, y, -z),
        (Facing::West, Facing::North)  => (z, -x, -y),
        (Facing::West, Facing::South)  => (-z, -x, y),
        (Facing::West, Facing::Up)     => (y, -x, z),
        (Facing::West, Facing::Down)   => (-y, -x, -z),
        (Facing::Up, Facing::North)    => (z, -y, x),
        (Facing::Up, Facing::East)     => (y, z, x),
        (Facing::Up, Facing::South)    => (-z, y, x),
        (Facing::Up, Facing::West)     => (-y, -z, x),
        (Facing::Down, Facing::North)  => (z, y, -x),
        (Facing::Down, Facing::East)   => (-y, z, -x),
        (Facing::Down, Facing::South)  => (-z, -y, -x),
        (Facing::Down, Facing::West)   => (y, -z, -x),
        _ => unreachable!(),
    }
}


const fn align_coords((x1, y1, z1): (i32, i32, i32), (x2, y2, z2): (i32, i32, i32)) -> (i32, i32, i32) {
    (x1 - x2, y1 - y2, z1 - z2)
}

const fn add_coords((x1, y1, z1): (i32, i32, i32), (x2, y2, z2): (i32, i32, i32)) -> (i32, i32, i32) {
    (x1 + x2, y1 + y2, z1 + z2)
}

const fn manhattan_distance((x1, y1, z1): (i32, i32, i32), (x2, y2, z2): (i32, i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
}


fn find_scanner_pos(total_scan: &mut HashSet<(i32, i32, i32)>, scanner: &HashSet<(i32, i32, i32)>) -> Option<(i32, i32, i32)> {
    for base_pos in total_scan.iter() {
        for rel_pos in scanner {
            for rel_orientation in Facing::facings() {
                let s_coords = align_coords(*base_pos, transform_to_north(*rel_pos, rel_orientation));
                let translated_coords = scanner.iter()
                    .map(|c| add_coords(transform_to_north(*c, rel_orientation), s_coords));

                if translated_coords.clone().filter(|c| total_scan.contains(c)).count() >= OVERLAPPING {
                    total_scan.extend(translated_coords);
                    return Some(s_coords);
                }
            }
        }
    }

    None
}


fn main() {
    let mut input: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|b| {
                    let coord: Vec<_> = b.split(',').map(|c| c.parse::<i32>().unwrap()).collect();
                    (coord[0], coord[1], coord[2])
                })
                .collect::<HashSet<_>>()
        })
        .collect();

    // Calculate positions of each scanner and update the set with all beacons

    let mut total_scan: HashSet<_> = input.swap_remove(0).into_iter().collect();

    let mut positions = Vec::with_capacity(input.len());
    while !input.is_empty() {
        for i in 0..input.len() {
            if let Some(pos) = find_scanner_pos(&mut total_scan, &input[i]) {
                positions.push(pos);
                input.swap_remove(i);
                break;
            }
        }
    }

    // Part 1
    println!("Unique positions: {}", total_scan.len());

    // Part 2
    let mut max_distance = 0;
    for i in 0..positions.len() {
        for j in 0..i {
            let distance = manhattan_distance(positions[i], positions[j]);
            if distance > max_distance {
                max_distance = distance;
            }
        }
    }

    println!("Max distance: {}", max_distance);
}
