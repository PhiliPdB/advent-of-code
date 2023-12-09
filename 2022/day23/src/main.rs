use std::collections::{HashMap, hash_map::Entry, HashSet};


fn main() {
    let mut elves: HashSet<_> = include_str!("../input.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some((y as i32, x as i32))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut round = 0;
    loop {
        if round == 10 {
            let min_x = elves.iter().map(|(_y, x)| x).min().unwrap();
            let max_x = elves.iter().map(|(_y, x)| x).max().unwrap();
            let min_y = elves.iter().map(|(y, _x)| y).min().unwrap();
            let max_y = elves.iter().map(|(y, _x)| y).max().unwrap();

            let empty_squares = (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32;
            println!("[Part 1] Empty squares: {empty_squares}");
        }

        let mut proposals: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        // Create proposals
        for &(y, x) in &elves {
            let positions = [
                (y - 1, x - 1), (y - 1, x) , (y - 1, x + 1),
                (y, x - 1), (y, x + 1),
                (y + 1, x - 1), (y + 1, x) , (y + 1, x + 1),
            ];
            if positions.iter().all(|p| !elves.contains(p)) {
                continue;
            }

            // Propose
            let mut elf_proposal = [None; 4];
            // Check north
            if !elves.contains(&(y - 1, x - 1)) && !elves.contains(&(y - 1, x)) && !elves.contains(&(y - 1, x + 1)) {
                elf_proposal[0] = Some((y - 1, x));
            }
            // Check south
            if !elves.contains(&(y+ 1, x - 1)) && !elves.contains(&(y+ 1, x)) && !elves.contains(&(y+ 1, x + 1)) {
                elf_proposal[1] = Some((y + 1, x));
            }
            // Check west
            if !elves.contains(&(y - 1, x - 1)) && !elves.contains(&(y, x - 1)) && !elves.contains(&(y + 1, x - 1)) {
                elf_proposal[2] = Some((y, x - 1));
            }
            // Check east
            if !elves.contains(&(y - 1, x + 1)) && !elves.contains(&(y, x + 1)) && !elves.contains(&(y + 1, x + 1)) {
                elf_proposal[3] = Some((y, x + 1));
            }

            elf_proposal.rotate_left(round % 4);
            if let Some(p) = elf_proposal.iter().find(|p| p.is_some()) {
                let p = p.unwrap();
                match proposals.entry(p) {
                    Entry::Occupied(mut e) => {
                        e.get_mut().push((y, x));
                    },
                    Entry::Vacant(e) => {
                        e.insert(vec![(y, x)]);
                    },
                }
            }
        }

        if proposals.is_empty() {
            println!("[Part 2] No movement in round: {}", round + 1);
            break;
        }

        for (to, from) in proposals {
            if from.len() != 1 {
                continue;
            }

            // Move
            elves.remove(&from[0]);
            elves.insert(to);
        }

        round += 1;
    }
}
