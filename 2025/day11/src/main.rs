use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug)]
struct Connections<'a> {
    node_map: HashMap<&'a str, usize>,
    adjacency: Vec<Vec<usize>>,
}

impl<'a> Connections<'a> {
    pub fn total_paths(&self, start: &str, end: &str) -> u64 {
        let mut path_cache = vec![None; self.node_map.len()];
        self.paths(self.node_map[start], self.node_map[end], &mut path_cache)
    }

    fn paths(&self, node: usize, target: usize, path_cache: &mut Vec<Option<u64>>) -> u64 {
        if let Some(cache) = path_cache[node] {
            return cache;
        }

        let result = if node == target {
            1
        } else {
            self.adjacency[node]
                .iter()
                .map(|n| self.paths(*n, target, path_cache))
                .sum()
        };
        path_cache[node] = Some(result);
        result
    }

    fn total_server_paths(&self) -> u64 {
        let svr_fft = self.total_paths("svr", "fft");
        let fft_dac = self.total_paths("fft", "dac");
        let dac_fft = self.total_paths("dac", "fft");
        let dac_out = self.total_paths("dac", "out");

        // NOTE: Either the connection fft->dac or dac->fft exists
        svr_fft * u64::max(fft_dac, dac_fft) * dac_out
    }

    fn from_str(s: &'a str) -> Self {
        let mut node_map = HashMap::new();
        let mut adjacency = vec![];

        for l in s.lines() {
            let (node, connections) = l.split_once(": ").unwrap();
            let node_id = match node_map.entry(node) {
                Entry::Occupied(occupied_entry) => *occupied_entry.get(),
                Entry::Vacant(vacant_entry) => {
                    let index = adjacency.len();
                    adjacency.push(vec![]);

                    vacant_entry.insert(index);

                    index
                }
            };

            for c in connections.split(' ') {
                let id = match node_map.entry(c) {
                    Entry::Occupied(occupied_entry) => *occupied_entry.get(),
                    Entry::Vacant(vacant_entry) => {
                        let index = adjacency.len();
                        adjacency.push(vec![]);

                        vacant_entry.insert(index);

                        index
                    }
                };
                adjacency[node_id].push(id);
            }
        }

        Self {
            node_map,
            adjacency,
        }
    }
}

fn main() {
    let rack = Connections::from_str(include_str!("../input.txt"));

    println!("[Part 1] Paths: {:15}", rack.total_paths("you", "out"));
    println!("[Part 2] Paths: {:15}", rack.total_server_paths());
}
