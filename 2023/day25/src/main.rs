use std::collections::VecDeque;
use std::str::FromStr;
use std::collections::{HashMap, hash_map::Entry};

#[derive(Debug)]
struct Wiring {
    adjacency_list: Vec<Vec<usize>>,
}

impl Wiring {
    fn size(&self, start: usize, cuts: &[(usize, usize)]) -> usize {
        let mut graph = Vec::new();
        for _ in 0..self.adjacency_list.len() {
            graph.push(vec![0; self.adjacency_list.len()]);
        }

        for (i, adjacent) in self.adjacency_list.iter().enumerate() {
            for &j in adjacent {
                graph[i][j] = 1;
                graph[j][i] = 1;
            }
        }

        for &(i, j) in cuts {
            graph[i][j] = 0;
            graph[j][i] = 0;
        }

        let mut size = 0;
        let mut queue = Vec::new();
        queue.push(start);

        let mut visited = vec![false; self.adjacency_list.len()];
        while let Some(n) = queue.pop() {
            #[allow(clippy::needless_range_loop)]
            for i in 0..self.adjacency_list.len() {
                if graph[n][i] > 0 && !visited[i] {
                    visited[i] = true;
                    size += 1;

                    queue.push(i);
                }
            }
        }

        size
    }

    fn max_flow(&self, (start, end): (usize, usize)) -> (u32, Vec<(usize, usize)>) {
        let mut residual_graph = Vec::new();
        for _ in 0..self.adjacency_list.len() {
            residual_graph.push(vec![0; self.adjacency_list.len()]);
        }

        for (i, adjacent) in self.adjacency_list.iter().enumerate() {
            for &j in adjacent {
                residual_graph[i][j] = 1;
                residual_graph[j][i] = 1;
            }
        }

        // Use Edmond-Karps to find a maximum flow

        let mut visited;
        let mut max_flow = 0;
        loop {
            visited = vec![false; self.adjacency_list.len()];
            let mut parent = vec![0; self.adjacency_list.len()];
            visited[start] = true;

            let mut queue = VecDeque::new();
            queue.push_back(start);

            while let Some(n) = queue.pop_front() {
                for i in 0..self.adjacency_list.len() {
                    if residual_graph[n][i] > 0 && !visited[i] {
                        parent[i] = n;
                        visited[i] = true;

                        queue.push_back(i);
                    }
                }
            }

            if !visited[end] {
                break;
            }

            let mut flow = u32::MAX;

            let mut w = end;
            while w != start {
                let v = parent[w];

                flow = u32::min(flow, residual_graph[v][w]);

                w = v;
            }

            w = end;
            while w != start {
                let v = parent[w];

                residual_graph[w][v] -= flow;
                residual_graph[v][w] -= flow;

                w = v;
            }

            max_flow += flow;
        }

        // The nodes on the boundary of our visited set, are where we cut the graph
        let mut cut = Vec::new();
        for (i, adjacent) in self.adjacency_list.iter().enumerate() {
            for &j in adjacent {
                if (visited[i] && !visited[j]) || (visited[j] && !visited[i]) {
                    cut.push((i, j));
                }
            }
        }

        (max_flow, cut)
    }
}

impl FromStr for Wiring {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut component_lookup = HashMap::new();
        let mut adjacency_list = Vec::new();

        for l in s.lines() {
            let [from, to] = l.split(": ").collect::<Vec<_>>()
                .try_into().map_err(|_| "Invalid connection format")?;

            let from_index = match component_lookup.entry(from.to_owned()) {
                Entry::Occupied(e) => *e.get(),
                Entry::Vacant(e) => {
                    let index = adjacency_list.len();
                    e.insert(index);
                    adjacency_list.push(Vec::new());

                    index
                },
            };

            for to in to.split(' ') {
                let to_index = match component_lookup.entry(to.to_owned()) {
                    Entry::Occupied(e) => *e.get(),
                    Entry::Vacant(e) => {
                        let index = adjacency_list.len();
                        e.insert(index);
                        adjacency_list.push(Vec::new());

                        index
                    },
                };

                adjacency_list[from_index].push(to_index);
            }
        }

        Ok(Self { adjacency_list })
    }
}

fn main() {
    let wiring = Wiring::from_str(include_str!("../input.txt")).unwrap();

    // Arbitrarily get the flow from node 0 till node n - 1,
    // This seems to work in my input
    let start = 0;
    let end = wiring.adjacency_list.len() - 1;
    let (flow, cuts) = wiring.max_flow((start, end));
    // Check if it really worked... The flow should be 3 for our 3 cut wires.
    assert_eq!(flow, 3);

    let group_size = wiring.size(start, &cuts);
    println!("[Part 1] Group multiplication: {}", group_size * (wiring.adjacency_list.len() - group_size))
}
