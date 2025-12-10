use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use good_lp::{Expression, Solution, SolverModel, constraint, highs, variables};

use crate::bitset::BitSet;

mod bitset;

#[derive(Debug)]
struct Machine {
    lights: BitSet,
    buttons: Vec<Vec<u32>>,
    requirements: Vec<u32>,
}

impl Machine {
    fn fewest_button_presses(&self) -> u64 {
        let mut queue = VecDeque::new();
        queue.push_back((BitSet::new(), 0));

        let mut seen = HashSet::new();

        while let Some((state, presses)) = queue.pop_front() {
            if !seen.insert(state) {
                continue;
            }

            if state == self.lights {
                return presses;
            }

            for button in &self.buttons {
                let mut new_state = state;
                for b in button {
                    new_state.toggle_bit(*b);
                }

                queue.push_back((new_state, presses + 1));
            }
        }

        unreachable!()
    }

    fn fewest_to_satisfy(&self) -> u64 {
        variables! {problem:
            0 <= button_presses[self.buttons.len()] (integer);
        };

        // Create objective
        let mut total_presses: Expression = 0.into();
        for b in &button_presses {
            total_presses += b;
        }

        // Set constraints to equal the required joltage
        let mut constraints = Vec::new();
        for (i, requirement) in self.requirements.iter().enumerate() {
            let mut joltage: Expression = 0.into();
            for (b_index, b) in self.buttons.iter().enumerate() {
                if b.contains(&(i as u32)) {
                    joltage += button_presses[b_index];
                }
            }

            constraints.push(constraint!(joltage == *requirement as i32));
        }

        // Create HiGHs model
        let model = problem
            .minimise(&total_presses)
            .using(highs)
            .with_all(constraints);
        // Solve
        let solution = model.solve().unwrap();

        // Return objective value
        solution.eval(total_presses).round() as u64
    }
}

impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();

        let lights: Vec<_> = parts[0]
            .chars()
            .filter_map(|c| match c {
                '#' => Some(true),
                '.' => Some(false),
                _ => None,
            })
            .collect();
        let mut lights_flag = BitSet::new();
        for i in lights
            .into_iter()
            .enumerate()
            .filter_map(|(i, l)| l.then_some(i))
        {
            lights_flag.set(i as u32);
        }

        let mut buttons = vec![];
        for button_set_str in parts
            .iter()
            .skip(1)
            .take(parts.len() - 2)
        {
            let butten_set_str = &button_set_str[1..button_set_str.len() - 1];
            let button = butten_set_str
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            buttons.push(button);
        }

        let requirement_part = parts[parts.len() - 1];
        let requirement_str = &requirement_part[1..requirement_part.len() - 1];
        let requirements = requirement_str
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Self {
            lights: lights_flag,
            buttons,
            requirements,
        })
    }
}

fn main() {
    let machines: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| Machine::from_str(l).unwrap())
        .collect();

    let part1_fewest_presses: u64 = machines
        .iter()
        .map(|m| m.fewest_button_presses())
        .sum();
    println!("[Part 1] Fewest button presses: {part1_fewest_presses:5}");

    let part2_fewest_presses: u64 = machines
        .iter()
        .map(|m| m.fewest_to_satisfy())
        .sum();
    println!("[Part 2] Fewest button presses: {part2_fewest_presses:5}");
}
