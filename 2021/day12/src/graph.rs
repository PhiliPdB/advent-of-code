use std::collections::HashMap;


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

    /// Get a reference to the graph's adjacency list.
    #[inline]
    pub fn adjacency_list(&self) -> &Vec<Vec<u32>> {
        &self.adjacency_list
    }

    /// Get a reference to the graph's big cave.
    #[inline]
    pub fn big_cave(&self) -> &Vec<bool> {
        &self.big_cave
    }

    /// Get a reference to the graph's start vertex.
    #[inline]
    pub fn start_vertex(&self) -> u32 {
        self.start_vertex
    }

    /// Get a reference to the graph's end vertex.
    #[inline]
    pub fn end_vertex(&self) -> u32 {
        self.end_vertex
    }
}
