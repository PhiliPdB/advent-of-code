use std::str::FromStr;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;


#[derive(Debug)]
struct Graph {
    adjacency_list: Vec<HashSet<usize>>,
    node_names: Vec<String>,
}

impl Graph {
    pub fn three_cliques(&self) -> Vec<[&String; 3]> {
        let mut three_cliques = Vec::new();

        for n1 in 0..self.node_names.len() {
            for &n2 in self.adjacency_list[n1].iter() {
                if n1 <=  n2 {
                    continue;
                }

                for &n3 in self.adjacency_list[n2].iter() {
                    if n2 <= n3 {
                        continue;
                    }

                    if self.adjacency_list[n3].contains(&n1) {
                        three_cliques.push([&self.node_names[n1], &self.node_names[n2], &self.node_names[n3]]);
                    }
                }
            }
        }

        three_cliques
    }

    pub fn maximal_cliques(&self) -> impl Iterator<Item = Vec<&String>> {
        let mut cliques = Vec::new();
        self.bron_kerbosch(&mut cliques, HashSet::new(), (0..self.node_names.len()).collect(), HashSet::new());

        cliques.into_iter()
            .map(|c| c.iter().map(|&i| &self.node_names[i]).collect())
    }

    fn bron_kerbosch(&self, cliques: &mut Vec<HashSet<usize>>,
        r: HashSet<usize>, mut p: HashSet<usize>, mut x: HashSet<usize>
    ) {
        if p.is_empty() && x.is_empty() {
            // Found a maximal clique
            cliques.push(r);
            return;
        }

        // Choose the pivot to be the vertex in p or x with the most neighbours
        let pivot = *p.union(&x)
            .max_by_key(|&&v| self.adjacency_list[v].len())
            .unwrap();

        // Iterate over the vertices in p that are not neighbours of the pivot
        let vertices: Vec<_> = p.difference(&self.adjacency_list[pivot])
            .cloned()
            .collect();
        for v in vertices.into_iter() {
            let v_neighbours = &self.adjacency_list[v];

            let mut r_cloned = r.clone();
            r_cloned.insert(v);

            self.bron_kerbosch(
                cliques,
                r_cloned,
                p.intersection(v_neighbours).cloned().collect(),
                x.intersection(v_neighbours).cloned().collect(),
            );

            p.remove(&v);
            x.insert(v);
        }
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
                    adjacency_list.push(HashSet::new());

                    node_names.len() - 1
                });
            let to_id = *node_lookup.entry(to)
                .or_insert_with(|| {
                    node_names.push(to.to_string());
                    adjacency_list.push(HashSet::new());

                    node_names.len() - 1
                });

            adjacency_list[from_id].insert(to_id);
            adjacency_list[to_id].insert(from_id);
        }

        Ok(Graph {
            adjacency_list,
            node_names,
        })
    }
}


fn main() {
    let computers = Graph::from_str(include_str!("../input.txt")).unwrap();


    let part1_answer = computers.three_cliques().iter()
        .filter(|clique| clique.iter().any(|name| name.starts_with('t')))
        .count();
    println!("[Part 1] Containing computer starting with 't': {part1_answer}");


    let mut max_clique = computers.maximal_cliques()
        .max_by_key(|c| c.len())
        .unwrap().clone();
    max_clique.sort_unstable();

    println!("[Part 2] Password: {}", max_clique.iter().join(","));
}
