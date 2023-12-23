use std::str::FromStr;

use crate::map::Map;
use crate::graph::Graph;

mod map;
mod graph;


fn main() {
    let map = Map::from_str(include_str!("../input.txt")).unwrap();

    let part1_graph = Graph::from_map::<true>(map.clone());
    println!("[Part 1] Scenic path length: {}", part1_graph.longest_path());

    let part2_graph = Graph::from_map::<false>(map);
    println!("[Part 2] Scenic path length: {}", part2_graph.longest_path());
}
