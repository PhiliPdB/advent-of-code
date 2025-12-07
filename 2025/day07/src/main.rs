use std::str::FromStr;

struct Teleporter {
    width: u32,
    start: u32,
    splitters: Vec<Vec<bool>>,
}

impl Teleporter {
    fn splitters_visited(&self) -> u32 {
        let mut splitters_visited = 0;

        let mut current_beams = vec![false; self.width as usize];
        let mut next_beams = vec![false; self.width as usize];

        // Set start beam
        current_beams[self.start as usize] = true;

        // Go through each splitter line
        for has_splitter in &self.splitters {
            for beam in current_beams
                .iter()
                .enumerate()
                // Filter out beams that are 'off'
                .filter_map(|(beam, on)| on.then_some(beam))
            {
                if has_splitter[beam] {
                    splitters_visited += 1;

                    // Split the beam
                    next_beams[beam - 1] = true;
                    next_beams[beam + 1] = true;
                } else {
                    // Continue this beam
                    next_beams[beam] = true;
                }
            }

            // Put next in current
            std::mem::swap(&mut current_beams, &mut next_beams);
            // Reset the next beams
            for beam in next_beams.iter_mut() {
                *beam = false;
            }
        }

        splitters_visited
    }

    fn timelines(&self) -> u64 {
        let mut current_beams = vec![0; self.width as usize];
        let mut next_beams = vec![0; self.width as usize];

        // Set start beam
        current_beams[self.start as usize] = 1;

        // Go through the splitters
        for has_splitter in &self.splitters {
            for (beam, count) in current_beams.iter().enumerate() {
                if has_splitter[beam] {
                    next_beams[beam - 1] += count;
                    next_beams[beam + 1] += count;
                } else {
                    next_beams[beam] += count;
                }
            }

            // Put next in current
            std::mem::swap(&mut current_beams, &mut next_beams);
            // Reset the next beams
            for beam in next_beams.iter_mut() {
                *beam = 0;
            }
        }

        current_beams.iter().sum()
    }
}

impl FromStr for Teleporter {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Err("Start not found");
        let mut splitters = vec![];

        let ls: Vec<_> = s.lines().collect();
        let width = ls[0].len() as u32;
        for line in ls {
            let mut current_splitters = vec![false; width as usize];
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = Ok(x as u32);
                    }
                    '^' => {
                        current_splitters[x] = true;
                    }
                    _ => (),
                }
            }

            if current_splitters.iter().any(|s| *s) {
                splitters.push(current_splitters);
            }
        }

        Ok(Self {
            start: start?,
            width,
            splitters,
        })
    }
}

fn main() {
    let teleporter = Teleporter::from_str(include_str!("../input.txt")).unwrap();

    println!(
        "[Part 1] Splitters visited: {}",
        teleporter.splitters_visited()
    );
    println!("[Part 2] Timelines: {}", teleporter.timelines());
}
