use std::collections::VecDeque;

mod bitset;
mod graph;

use bitset::BitSet;
use graph::Graph;


fn paths(graph: &Graph, allow_visiting_twice: bool) -> i32 {
    let mut paths = 0;
    let mut queue = VecDeque::from([(graph.start_vertex(), BitSet::new(), false)]);

    while let Some((current, visited, visited_twice)) = queue.pop_front() {
        let mut new_visited = visited;
        let mut new_visited_twice = visited_twice;
        // Check if we are allowed to visit this vertex
        if !graph.big_cave()[current as usize] && new_visited.get(current) {
            if allow_visiting_twice && !new_visited_twice {
                new_visited_twice = true;
            } else {
                continue;
            }
        }
        // Visit this vertex
        new_visited.set(current);

        if current == graph.end_vertex() {
            paths += 1;
            continue;
        }

        for neighbour in &graph.adjacency_list()[current as usize] {
            if *neighbour != graph.start_vertex() {
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
