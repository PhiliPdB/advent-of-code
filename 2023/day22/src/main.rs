use std::{str::FromStr, collections::{HashSet, VecDeque}};

#[derive(Debug)]
struct Brick {
    from: (u32, u32, u32),
    to: (u32, u32, u32),
}

impl FromStr for Brick {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [from, to] = s.split('~').collect::<Vec<_>>()
            .try_into().map_err(|_| "Invalid brick format")?;

        let from = from.split(',')
            .map(|i| i.parse().unwrap())
            .collect::<Vec<_>>();
        let from = (from[0], from[1], from[2]);

        let to = to.split(',')
            .map(|i| i.parse().unwrap())
            .collect::<Vec<_>>();
        let to = (to[0], to[1], to[2]);

        debug_assert!(from.0 <= to.0 && from.1 <= to.1 && from.2 <= to.2);

        Ok(Self { from, to })
    }
}

fn build_brick_map(bricks: &[Brick]) -> Vec<Vec<Vec<Option<usize>>>> {
    let width = bricks.iter().map(|b| b.to.0).max().unwrap() as usize + 1;
    let height = bricks.iter().map(|b| b.to.1).max().unwrap() as usize + 1;
    let depth = bricks.iter().map(|b| b.to.2).max().unwrap() as usize + 1;


    let mut brick_map = Vec::new();
    for z in 0..depth {
        brick_map.push(Vec::new());
        for _ in 0..height {
            brick_map[z].push(vec![None; width]);
        }
    }

    for (i, brick) in bricks.iter().enumerate() {
        for z in brick.from.2..brick.to.2+1 {
            for y in brick.from.1..brick.to.1+1 {
                for x in brick.from.0..brick.to.0+1 {
                    debug_assert!(brick_map[z as usize][y as usize][x as usize] == None);
                    brick_map[z as usize][y as usize][x as usize] = Some(i);
                }
            }
        }
    }

    brick_map
}

fn free_fall(bricks: &mut [Brick], brick_map: &mut Vec<Vec<Vec<Option<usize>>>>) {
    let mut changes = true;
    while changes {
        changes = false;
        for (i, brick) in bricks.iter_mut().enumerate() {
            let z = brick.from.2 - 1;
            if z == 0 {
                continue;
            }

            let mut is_empty = true;
            'empty_check: for y in brick.from.1..brick.to.1+1 {
                for x in brick.from.0..brick.to.0+1 {
                    if let Some(_) = brick_map[z as usize][y as usize][x as usize] {
                        is_empty = false;
                        break 'empty_check;
                    }
                }
            }

            if is_empty {
                changes = true;

                for y in brick.from.1..brick.to.1+1 {
                    for x in brick.from.0..brick.to.0+1 {
                        debug_assert!(brick_map[z as usize][y as usize][x as usize] == None);
                        brick_map[z as usize][y as usize][x as usize] = Some(i);
                    }
                }

                let z = brick.to.2;
                for y in brick.from.1..brick.to.1+1 {
                    for x in brick.from.0..brick.to.0+1 {
                        debug_assert!(brick_map[z as usize][y as usize][x as usize] == Some(i));
                        brick_map[z as usize][y as usize][x as usize] = None;
                    }
                }

                brick.from.2 -= 1;
                brick.to.2 -= 1;
            }
        }
    }
}

fn disintegrates(i: usize, supports: &[HashSet<usize>], supported_by: &[HashSet<usize>]) -> usize {
    let mut disintegrated = HashSet::new();
    disintegrated.insert(i);

    let mut queue = VecDeque::new();
    queue.push_back(i);

    while let Some(i) = queue.pop_front() {
        for j in supports[i].iter() {
            if supported_by[*j].iter()
                    .filter(|k| !disintegrated.contains(*k))
                    .count() == 0
            {
                disintegrated.insert(*j);
                queue.push_back(*j);
            }
        }
    }

    disintegrated.len() - 1
}

fn main() {
    let mut bricks: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| Brick::from_str(l).unwrap())
        .collect();

    let mut brick_map = build_brick_map(&bricks);
    // Let the bricks fall to their resting place
    free_fall(&mut bricks, &mut brick_map);

    let bricks = bricks;
    let brick_map = brick_map;

    //
    // Lookup tables
    //
    // Build lookup table to see which brick supports which
    // And by which bricks a certain brick is supported
    //

    let mut supports = Vec::with_capacity(bricks.len());
    let mut supported_by = Vec::with_capacity(bricks.len());
    for _ in 0..bricks.len() {
        supports.push(HashSet::new());
        supported_by.push(HashSet::new());
    }

    for (i, brick) in bricks.iter().enumerate() {
        let z = brick.from.2 - 1;
        if z == 0 {
            continue;
        }

        for y in brick.from.1..brick.to.1+1 {
            for x in brick.from.0..brick.to.0+1 {
                if let Some(j) = brick_map[z as usize][y as usize][x as usize] {
                    debug_assert!(i != j);

                    supports[j].insert(i);
                    supported_by[i].insert(j);
                }
            }
        }
    }

    //
    // Preparations are done, let's calculate the answers
    //

    let can_be_disintegrated = (0..bricks.len()).into_iter()
        .filter(|i| supports[*i].iter().all(|j| supported_by[*j].len() > 1))
        .count();
    println!("[Part 1] Safely disintegrated bricks: {can_be_disintegrated}");


    let disintegrates = (0..bricks.len()).into_iter()
        .map(|i| disintegrates(i, &supports, &supported_by));
    println!("[Part 2] Falling sum: {}", disintegrates.sum::<usize>());
}
