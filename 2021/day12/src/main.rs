use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
pub struct BitSet(u32);

impl BitSet {
    pub fn new() -> Self {
        Self(0)
    }

    /// Check if the bit on the specified index is set
    pub fn get(&self, index: u32) -> bool {
        (self.0 >> index) & 1 == 1
    }

    /// Set bit on specified index
    pub fn set(&mut self, index: u32) {
        self.0 |= 1 << index;
    }
}


#[derive(Debug)]
pub struct Graph {
    adjacency_list: Vec<Vec<u32>>,
    big_cave: Vec<bool>,
    start_vertex: u32,
    end_vertex: u32,
}

impl Graph {
    pub fn new(s: &str) -> Self {
        let mut adjacency_list = Vec::new();
        let mut vertex_lookup = HashMap::new();
        let mut big_cave = Vec::new();
        let mut start_vertex = 0;
        let mut end_vertex = 0;
        let mut current_node_id = 0;

        for line in s.lines() {
            let parts: Vec<_> = line.split('-').collect();
            let mut ids = Vec::with_capacity(2);
            for p in parts {
                if !vertex_lookup.contains_key(&p) {
                    vertex_lookup.insert(p, current_node_id);
                    adjacency_list.push(Vec::new());

                    let is_big_cave = p.chars().all(|c| c.is_uppercase());
                    big_cave.push(is_big_cave);

                    if p == "start" {
                        start_vertex = current_node_id;
                    }
                    if p == "end" {
                        end_vertex = current_node_id;
                    }

                    current_node_id += 1;
                }

                ids.push(vertex_lookup[p]);
            }

            adjacency_list[ids[0] as usize].push(ids[1]);
            adjacency_list[ids[1] as usize].push(ids[0]);
        }

        Self { adjacency_list, big_cave, start_vertex, end_vertex }
    }
}


fn paths(graph: &Graph, allow_visiting_twice: bool) -> i32 {
    let mut paths = 0;
    let mut queue = VecDeque::from([(graph.start_vertex, BitSet::new(), false)]);

    while let Some((current, visited, visited_twice)) = queue.pop_front() {
        let mut new_visited = visited;
        let mut new_visited_twice = visited_twice;
        // Check if we are allowed to visit this vertex
        if !graph.big_cave[current as usize] && new_visited.get(current) {
            if allow_visiting_twice && !new_visited_twice {
                new_visited_twice = true;
            } else {
                continue;
            }
        }
        // Visit this vertex
        new_visited.set(current);

        if current == graph.end_vertex {
            paths += 1;
            continue;
        }

        for neighbour in &graph.adjacency_list[current as usize] {
            if *neighbour != graph.start_vertex {
                queue.push_back((*neighbour, new_visited, new_visited_twice));
            }
        }
    }

    paths
}


fn main() {
    let input = Graph::new(include_str!("../input.txt"));

    println!("[Part 1] Paths: {:#5}", paths(&input, false));
    println!("[Part 2] Paths: {:#5}", paths(&input, true));
}
