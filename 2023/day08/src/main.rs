use std::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

use num::Integer;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left, Right
}

impl Instruction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'L' => Some(Self::Left),
            'R' => Some(Self::Right),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    nodes: Vec<(u32, u32)>,
    node_lookup: HashMap<String, u32>,
}

impl Map {
    fn steps(&self) -> u32 {
        let mut steps = 0;

        let mut current = self.node_lookup["AAA"];
        let goal = self.node_lookup["ZZZ"];

        for instruction in self.instructions.iter().cycle() {
            if current == goal {
                break;
            }

            match instruction {
                Instruction::Left => current = self.nodes[current as usize].0,
                Instruction::Right => current = self.nodes[current as usize].1,
            }

            steps += 1;
        }
        steps
    }

    fn ghost_steps(&self) -> u64 {
        let mut steps = 0;

        let mut current_nodes: Vec<_> = self.node_lookup.iter()
            .filter_map(|(k, v)| {
                if k.ends_with("A") {
                    Some(*v)
                } else {
                    None
                }
            })
            .collect();
        let goals: HashSet<_> = self.node_lookup.iter()
            .filter_map(|(k, v)| {
                if k.ends_with("Z") {
                    Some(*v)
                } else {
                    None
                }
            })
            .collect();
        let mut frequencies = Vec::new();

        for instruction in self.instructions.iter().cycle() {
            let old_length = current_nodes.len();
            current_nodes.retain(|c| !goals.contains(c));
            if current_nodes.len() != old_length {
                frequencies.push(steps);
            }

            if current_nodes.is_empty() {
                break;
            }

            for c in current_nodes.iter_mut() {
                match instruction {
                    Instruction::Left => *c = self.nodes[*c as usize].0,
                    Instruction::Right => *c = self.nodes[*c as usize].1,
                }
            }

            steps += 1;
        }

        let mut total_steps = 1;
        for f in frequencies {
            total_steps = total_steps.lcm(&f);
        }

        total_steps
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [instructions, str_nodes] = s.split("\n\n")
            .collect::<Vec<_>>()
            .try_into().map_err(|_| "Invalid format")?;

        let instructions = instructions.chars()
            .map(|c| Instruction::from_char(c).unwrap())
            .collect();

        let mut node_lookup_len = 0;
        let mut node_lookup = HashMap::new();
        let mut nodes = vec![(0, 0); str_nodes.lines().count()];
        for l in str_nodes.lines() {
            let chars: Vec<_> = l.chars().collect();
            let current: String = chars[..3].iter().collect();
            let left: String = chars[7..10].iter().collect();
            let right: String = chars[12..15].iter().collect();

            let current_id = match node_lookup.entry(current) {
                Entry::Occupied(e) => *e.get(),
                Entry::Vacant(e) => {
                    let id = node_lookup_len as u32;
                    e.insert(id);
                    node_lookup_len += 1;

                    id
                },
            };
            let left_id = match node_lookup.entry(left) {
                Entry::Occupied(e) => *e.get(),
                Entry::Vacant(e) => {
                    let id = node_lookup_len as u32;
                    e.insert(id);
                    node_lookup_len += 1;

                    id
                },
            };
            let right_id = match node_lookup.entry(right) {
                Entry::Occupied(e) => *e.get(),
                Entry::Vacant(e) => {
                    let id = node_lookup_len as u32;
                    e.insert(id);
                    node_lookup_len += 1;

                    id
                },
            };

            nodes[current_id as usize] = (left_id, right_id);
        }

        Ok(Self { instructions, nodes, node_lookup })
    }
}

fn main() {
    let map = Map::from_str(include_str!("../input.txt")).unwrap();

    println!("[Part 1] Steps till the end: {:14}", map.steps());
    println!("[Part 2] Steps till the end: {:14}", map.ghost_steps());
}
