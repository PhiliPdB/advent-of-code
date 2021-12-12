use std::collections::{HashMap, VecDeque, HashSet};

#[derive(Debug)]
pub struct Graph {
    adjacency_list: Vec<Vec<i32>>,
    big_cave: Vec<bool>,
    start_vertex: i32,
    end_vertex: i32,
}

impl Graph {

    fn new(s: &str) -> Self {
        let mut adjacency_list = Vec::new();
        let mut vertex_lookup = HashMap::new();
        let mut big_cave = Vec::new();
        let mut start_vertex = -1;
        let mut end_vertex = -1;
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
    let mut queue = VecDeque::from([(graph.start_vertex, HashSet::new(), HashSet::new())]);

    while let Some((current, visited, visited_twice)) = queue.pop_front() {
        let mut new_visited = visited;
        let mut new_visited_twice = visited_twice;
        if !graph.big_cave[current as usize] && !new_visited.insert(current) {
            if allow_visiting_twice && new_visited_twice.len() < 1 {
                new_visited_twice.insert(current);
            } else {
                continue;
            }
        }

        if current == graph.end_vertex {
            paths += 1;
            continue;
        }

        for neighbour in &graph.adjacency_list[current as usize] {
            if *neighbour != graph.start_vertex {
                queue.push_back((*neighbour, new_visited.clone(), new_visited_twice.clone()));
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
