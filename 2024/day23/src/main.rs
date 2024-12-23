use std::str::FromStr;
use std::collections::HashMap;

use itertools::Itertools;


#[derive(Debug)]
struct Graph {
    adjacency_list: Vec<Vec<usize>>,
    node_names: Vec<String>,
}

impl Graph {
    pub fn all_cliques(&self, min_size: usize) -> impl Iterator<Item = Vec<&String>> {
        let mut cliques = Vec::new();
        let mut clique_store = vec![0; self.node_names.len()];

        self.all_cliques_recursive(&mut clique_store, &mut cliques, 0, 1, min_size);

        cliques.into_iter()
            .map(|c| c.iter().map(|&i| &self.node_names[i]).collect())
    }

    fn all_cliques_recursive(&self,
        clique_store: &mut Vec<usize>, all_cliques: &mut Vec<Vec<usize>>,
        i: usize, l: usize, min_size: usize
    ) {
        for j in i+1..self.node_names.len() {
            if self.adjacency_list[j].len() < min_size {
                continue;
            }
            clique_store[l - 1] = j;

            if self.is_clique(clique_store, l) {
                if l >= min_size {
                    all_cliques.push(clique_store[..l].to_vec());
                }
                self.all_cliques_recursive(clique_store, all_cliques, j, l+1, min_size);
            }
        }
    }

    fn is_clique(&self, clique_store: &[usize], b: usize) -> bool {
        for i in 0..b {
            for j in i+1..b {
                if !self.adjacency_list[clique_store[i]].contains(&clique_store[j]) {
                    return false;
                }
            }
        }
        true
    }
}

impl FromStr for Graph {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut node_names = Vec::new();
        let mut node_lookup = HashMap::new();

        let mut adjacency_list = Vec::new();
        for line in s.lines() {
            let (from, to) = line.split_once("-")
                .ok_or("Error parsing line")?;

            let from_id = *node_lookup.entry(from)
                .or_insert_with(|| {
                    node_names.push(from.to_string());
                    adjacency_list.push(Vec::new());

                    node_names.len() - 1
                });
            let to_id = *node_lookup.entry(to)
                .or_insert_with(|| {
                    node_names.push(to.to_string());
                    adjacency_list.push(Vec::new());

                    node_names.len() - 1
                });

            adjacency_list[from_id].push(to_id);
            adjacency_list[to_id].push(from_id);
        }

        Ok(Graph {
            adjacency_list,
            node_names,
        })
    }
}

fn main() {
    let computers = Graph::from_str(include_str!("../input.txt")).unwrap();
    let cliques: Vec<_> = computers.all_cliques(3)
        .collect();


    let part1_answer = cliques.iter()
        .filter(|clique| clique.len() == 3 && clique.iter().any(|name| name.starts_with('t')))
        .count();
    println!("[Part 1] Containing computer starting with 't': {part1_answer}");


    let mut max_clique = cliques.iter()
        .max_by_key(|c| c.len())
        .unwrap().clone();
    max_clique.sort_unstable();

    println!("[Part 2] Password: {}", max_clique.iter().join(","));
}
