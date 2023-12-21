use std::collections::VecDeque;
use std::str::FromStr;

use hashbrown::HashSet;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Plot {
    Garden, Rock
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Plot>>,
    starting_position: (i32, i32),
}

impl Map {
    fn reachable_in_exact<const INFINITE_MAP: bool>(&self, steps: u32) -> u64 {
        let height = self.map.len() as i32;
        let width = self.map[0].len() as i32;

        let mut reachable = 0;

        let mut queue = VecDeque::new();
        queue.push_back((self.starting_position, 0));
        let mut visited = HashSet::new();

        let is_steps_even = steps % 2 == 0;
        while let Some(((y, x), s)) = queue.pop_front() {
            if !visited.insert((y, x)) {
                continue;
            }

            // Note we could reach this position again in an even number of steps,
            // so check parity with steps required
            if (is_steps_even && s % 2 == 0) || (!is_steps_even && s % 2 == 1)  {
                reachable += 1;
            }
            if s == steps {
                continue;
            }

            for (dy, dx) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let new_y = y + dy;
                let new_x = x + dx;

                if !INFINITE_MAP
                    && 0 <= new_y && new_y < height
                    && 0 <= new_x && new_x < width
                    && self.map[new_y as usize][new_x as usize] != Plot::Rock
                {
                    queue.push_back(((new_y, new_x), s + 1));
                } else if INFINITE_MAP
                    && self.map[new_y.rem_euclid(width) as usize][new_x.rem_euclid(height) as usize] != Plot::Rock
                {
                    queue.push_back(((new_y, new_x), s + 1));
                }
            }
        }

        reachable
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut starting_position = (0, 0);
        let map = s.lines().enumerate()
            .map(|(y, l)| {
                l.chars().enumerate()
                    .map(|(x, c)| match c {
                        '.' => Plot::Garden,
                        '#' => Plot::Rock,
                        'S' => {
                            starting_position = (y as i32, x as i32);
                            Plot::Garden
                        },
                        _ => panic!("Invalid plot char: {c}"),
                    })
                    .collect()
            })
            .collect();

        Ok(Self { map, starting_position })
    }
}


fn main() {
    let map = Map::from_str(include_str!("../input.txt")).unwrap();

    println!("[Part 1] Reachable: {:15}", map.reachable_in_exact::<false>(64));

    //
    // Part 2
    //

    const STEP_GOAL: u32 = 26_501_365;
    let grid_size = map.map.len() as u32;

    // Wolfram Alpha found a quadratic function when analyzing the reachable steps
    // where we increased the number of steps by the size of the grid.
    // For this we start at STEP_GOAL mod grid size, such that after n increases,
    // we reach the step goal
    let start = STEP_GOAL % grid_size;
    let n = (STEP_GOAL / grid_size) as u64;
    
    // Find values for x = 0, .., x = 2
    // This enough the calculate the quadratic function
    let values: Vec<_> = (0..3).into_iter()
        .map(|i| map.reachable_in_exact::<true>(start + i * grid_size))
        .collect();

    // Values for ax^2 + bx + c
    let a = (values[2] + values[0]) / 2 - values[1];
    let b = values[1] - values[0] - a;
    let c = values[0];
    
    // Then find the final answer using n and the quadratic function
    println!("[Part 2] Reachable: {:15}", a * n * n + b * n + c);
}
