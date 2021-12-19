use hashbrown::HashSet;

const OVERLAPPING: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    NorthEast, NorthWest, NorthUp, NorthDown,
    EastNorth, EastSouth, EastUp, EastDown,
    SouthEast, SouthWest, SouthUp, SouthDown,
    WestNorth, WestSouth, WestUp, WestDown,
    UpNorth, UpEast, UpSouth, UpWest,
    DownNorth, DownEast, DownSouth, DownWest,
}

impl Facing {
    pub const fn facings() -> [Facing; 24] {
        [
            Facing::NorthEast, Facing::NorthWest, Facing::NorthUp, Facing::NorthDown,
            Facing::EastNorth, Facing::EastSouth, Facing::EastUp, Facing::EastDown,
            Facing::SouthEast, Facing::SouthWest, Facing::SouthUp, Facing::SouthDown,
            Facing::WestNorth, Facing::WestSouth, Facing::WestUp, Facing::WestDown,
            Facing::UpNorth, Facing::UpEast, Facing::UpSouth, Facing::UpWest,
            Facing::DownNorth, Facing::DownEast, Facing::DownSouth, Facing::DownWest,
        ]
    }
}

pub const fn transform_to_north((x, y, z): (i32, i32, i32), facing: Facing) -> (i32, i32, i32) {
    match facing {
        Facing::NorthEast  => (x, z, -y),
        Facing::NorthWest  => (x, -z, y),
        Facing::NorthUp    => (x, y, z),
        Facing::NorthDown  => (x, -y, -z),
        Facing::EastNorth  => (z, x, y),
        Facing::EastSouth  => (-z, x, -y),
        Facing::EastUp     => (-y, x, z),
        Facing::EastDown   => (y, x, -z),
        Facing::SouthEast  => (-x, z, y),
        Facing::SouthWest  => (-x, -z, -y),
        Facing::SouthUp    => (-x, -y, z),
        Facing::SouthDown  => (-x, y, -z),
        Facing::WestNorth  => (z, -x, -y),
        Facing::WestSouth  => (-z, -x, y),
        Facing::WestUp     => (y, -x, z),
        Facing::WestDown   => (-y, -x, -z),
        Facing::UpNorth    => (z, -y, x),
        Facing::UpEast     => (y, z, x),
        Facing::UpSouth    => (-z, y, x),
        Facing::UpWest     => (-y, -z, x),
        Facing::DownNorth  => (z, y, -x),
        Facing::DownEast   => (-y, z, -x),
        Facing::DownSouth  => (-z, -y, -x),
        Facing::DownWest   => (y, -z, -x),
    }
}


#[inline]
const fn align_coords((x1, y1, z1): (i32, i32, i32), (x2, y2, z2): (i32, i32, i32)) -> (i32, i32, i32) {
    (x1 - x2, y1 - y2, z1 - z2)
}

#[inline]
const fn add_coords((x1, y1, z1): (i32, i32, i32), (x2, y2, z2): (i32, i32, i32)) -> (i32, i32, i32) {
    (x1 + x2, y1 + y2, z1 + z2)
}

#[inline]
const fn manhattan_distance((x1, y1, z1): (i32, i32, i32), (x2, y2, z2): (i32, i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
}


fn find_scanner_pos(total_scan: &mut HashSet<(i32, i32, i32)>, scanner: &[(i32, i32, i32)]) -> Option<(i32, i32, i32)> {
    for rel_orientation in Facing::facings() {
        // Rotate all the point of the scanner
        let rotated_scanner: Vec<_> = scanner.iter()
            .map(|c| transform_to_north(*c, rel_orientation))
            .collect();

        for base_pos in total_scan.iter() {
            for rel_pos in &rotated_scanner {
                // Calculate vector from the reference point in the total scan and the current scanner
                let translation_vector = align_coords(*base_pos, *rel_pos);
                let translated_scanner = rotated_scanner.iter()
                    .map(|c| add_coords(*c, translation_vector));

                if translated_scanner.clone().filter(|c| total_scan.contains(c)).count() >= OVERLAPPING {
                    total_scan.extend(translated_scanner);
                    return Some(translation_vector);
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
                .collect::<Vec<_>>()
        })
        .collect();

    // Calculate positions of each scanner and update the set with all beacons

    let mut total_scan: HashSet<_> = input.remove(0).into_iter().collect();

    let mut positions = Vec::with_capacity(input.len());
    while !input.is_empty() {
        input.retain(|scanner| {
            if let Some(pos) = find_scanner_pos(&mut total_scan, scanner) {
                positions.push(pos);
                false
            } else {
                true
            }
        })
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
