use std::collections::{HashMap, hash_map::Entry, VecDeque};
use std::str::FromStr;


struct OrbitMap {
    adjacency_list: Vec<Vec<usize>>,
    com_index: usize,
    you_index: usize,
    san_index: usize,
}

impl OrbitMap {
    pub fn get_total_orbits(&self) -> u32 {
        let mut orbit_lookup = vec![u32::MAX; self.adjacency_list.len()];

        let mut stack = Vec::new();
        stack.push(self.com_index);
        orbit_lookup[self.com_index] = 0;

        while let Some(current) = stack.pop() {
            for neighbour in &self.adjacency_list[current] {
                if orbit_lookup[*neighbour] > orbit_lookup[current] + 1 {
                    orbit_lookup[*neighbour] = orbit_lookup[current] + 1;
                    stack.push(*neighbour);
                }
            }
        }

        orbit_lookup.iter().sum()
    }

    pub fn min_orbits(&self) -> u32 {
        let mut distances = vec![u32::MAX; self.adjacency_list.len()];

        let mut queue = VecDeque::new();
        queue.push_back(self.you_index);
        distances[self.you_index] = 0;

        while let Some(current) = queue.pop_front() {
            if current == self.san_index {
                break;
            }

            for neighbour in &self.adjacency_list[current] {
                if distances[*neighbour] > distances[current] + 1 {
                    distances[*neighbour] = distances[current] + 1;
                    queue.push_back(*neighbour);
                }
            }
        }

        distances[self.san_index] - 2
    }
}

impl FromStr for OrbitMap {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut adjacency_list = Vec::new();
        let mut vertex_lookup = HashMap::new();
        let mut current_node_id = 0;

        let mut com_index = 0;
        let mut you_index = 0;
        let mut san_index = 0;

        for line in s.lines() {
            let mut ids = Vec::with_capacity(2);
            for v in line.split(')') {
                match vertex_lookup.entry(v) {
                    Entry::Vacant(e) => {
                        e.insert(current_node_id);
                        ids.push(current_node_id);

                        adjacency_list.push(Vec::new());

                        if v == "COM" {
                            com_index = current_node_id;
                        } else if v == "YOU" {
                            you_index = current_node_id;
                        } else if v == "SAN" {
                            san_index = current_node_id;
                        }

                        current_node_id += 1;
                    },
                    Entry::Occupied(e) => {
                        ids.push(*e.get());
                    },
                }
            }

            adjacency_list[ids[0]].push(ids[1]);
            adjacency_list[ids[1]].push(ids[0]);
        }

        Ok(Self { adjacency_list, com_index, you_index, san_index })
    }
}

fn main() {
    let orbit_map = OrbitMap::from_str(include_str!("../input.txt")).unwrap();

    println!("Total orbits: {}", orbit_map.get_total_orbits());

    println!("Min orbits: {}", orbit_map.min_orbits());
}
