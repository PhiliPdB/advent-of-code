use std::collections::{HashMap, HashSet, VecDeque, hash_map::Entry};

use crate::map::{Map, Direction};


/// A visited set that can hold up to 64 elements
#[derive(Debug, Clone, Copy)]
struct Visited(u64);

impl Visited {
    #[inline(always)]
    const fn new() -> Self {
        Self(0)
    }

    #[inline(always)]
    fn insert(&mut self, n: usize) -> bool {
        let mask = 1 << n;

        let already_visited = (self.0 & mask) != 0;
        self.0 |= mask;

        !already_visited
    }

    #[inline(always)]
    const fn contains(&self, n: &usize) -> bool {
        self.0 & (1 << *n) != 0
    }
}


#[derive(Debug)]
pub(crate) struct Graph {
    outgoing_arcs: Vec<Vec<(usize, u32)>>,
    start: usize,
    goal: usize,
}

impl Graph {
    pub(crate) fn longest_path(&self) -> u32 {
        // Check if we have at most 64 nodes, because our visited set can't handle more.
        assert!(self.outgoing_arcs.len() <= 64);

        let mut queue = Vec::new();
        queue.push((1, self.start, Visited::new()));

        let mut longest_path_length = 0;
        while let Some((l, n, mut visited)) = queue.pop() {
            if n == self.goal {
                longest_path_length = u32::max(longest_path_length, l);
                continue;
            }

            // Check if visited
            if !visited.insert(n) {
                continue;
            }

            // Generate next step
            for (next, steps) in &self.outgoing_arcs[n] {
                if visited.contains(next) {
                    continue;
                }

                queue.push((l + *steps, *next, visited));
            }
        }

        longest_path_length
    }


    /// Convert a map into a graph
    /// With or without the slopes
    pub(crate) fn from_map<const WITH_SLOPES: bool>(map: Map) -> Self {
        let mut node_lookup = HashMap::new();
        node_lookup.insert(map.start(), 0);
        node_lookup.insert(map.goal(), 1);

        let mut outgoing_arcs = Vec::new();
        outgoing_arcs.push(Vec::new());
        outgoing_arcs.push(Vec::new());


        let mut queue = VecDeque::new();
        queue.push_back(map.start());

        let mut visited = HashSet::new();

        while let Some((x, y)) = queue.pop_front() {
            if !visited.insert((x, y)) {
                continue;
            }

            if (x, y) == map.goal() {
                continue;
            }

            let node_index = match node_lookup.entry((x, y)) {
                Entry::Occupied(e) => *e.get(),
                Entry::Vacant(e) => {
                    let i = outgoing_arcs.len();
                    e.insert(i);

                    outgoing_arcs.push(Vec::new());

                    i
                },
            };

            // Generate next step
            for d in [Direction::North, Direction::East, Direction::South, Direction::West] {
                let (steps, (new_x, new_y)) = map.next_crossing::<WITH_SLOPES>((x, y), d);
                if steps == 0 {
                    continue;
                }

                queue.push_back((new_x, new_y));

                let new_node_index = match node_lookup.entry((new_x, new_y)) {
                    Entry::Occupied(e) => *e.get(),
                    Entry::Vacant(e) => {
                        let i = outgoing_arcs.len();
                        e.insert(i);

                        outgoing_arcs.push(Vec::new());

                        i
                    },
                };

                outgoing_arcs[node_index].push((new_node_index, steps));
            }
        }

        Self { outgoing_arcs, start: 0, goal: 1 }
    }
}
