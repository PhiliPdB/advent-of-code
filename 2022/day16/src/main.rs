use std::collections::{HashMap, hash_map::Entry, VecDeque, HashSet};

use itertools::Itertools;


#[derive(Debug)]
struct Neighbour {
    node: usize,
    distance: u32,
}

impl Neighbour {
    fn new(node: usize, distance: u32) -> Self {
        Self { node, distance }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct StackItem {
    node: usize,
    time: u32,
    pressure_released: u32,
    visited: HashSet<usize>,
}

impl StackItem {
    fn new(node: usize, time: u32, pressure_released: u32, visited: HashSet<usize>) -> Self {
        Self { node, time, pressure_released, visited }
    }
}


fn get_distances(start_node: usize, flow_rates: &Vec<u32>, adjacency_list: &Vec<Vec<usize>>) -> Vec<Neighbour> {
    let mut distances = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back((start_node, 0));
    let mut visited = HashSet::new();

    while let Some((node, distance)) = queue.pop_front() {
        if !visited.insert(node) {
            continue;
        }

        if flow_rates[node] != 0 && node != start_node {
            distances.push(Neighbour::new(node, distance));
        }

        for n in &adjacency_list[node] {
            queue.push_back((*n, distance + 1));
        }
    }

    distances
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .collect();

    let total_nodes = input.len();

    let mut node_map = HashMap::new();
    let mut flow_rates = Vec::with_capacity(total_nodes);
    let mut adjacency_list = Vec::with_capacity(total_nodes);

    for l in input {
        let valve = l[6..8].to_owned();

        let new = node_map.len();
        let index = match node_map.entry(valve) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                e.insert(new);
                flow_rates.push(0);
                adjacency_list.push(Vec::new());
                new
            },
        };

        let flow_rate = l.split_once("rate=").unwrap().1
            .split_once(';').unwrap().0
            .parse::<u32>().unwrap();
        flow_rates[index] = flow_rate;

        let neighbours: Vec<_> = l.split_once("valves ")
            .unwrap_or_else(|| l.split_once("valve ").unwrap()).1
            .split(", ")
            .map(|n| {
                let new = node_map.len();
                match node_map.entry(n.to_owned()) {
                    Entry::Occupied(e) => *e.get(),
                    Entry::Vacant(e) => {
                        e.insert(new);
                        flow_rates.push(0);
                        adjacency_list.push(Vec::new());
                        new
                    },
                }
            })
            .collect();
        adjacency_list[index] = neighbours;
    }

    let node_map = node_map;
    let flow_rates = flow_rates;

    let start = node_map[&String::from("AA")];

    // Find distances between flowing valves
    let mut distances = Vec::with_capacity(total_nodes);
    for n in 0..total_nodes {
        distances.push(
            if flow_rates[n] != 0 || n == start {
                get_distances(n, &flow_rates, &adjacency_list)
            } else {
                Vec::new()
            }
        )
    }

    // Part 1: Use DFS to find the maximum release pressure

    const PART1_TIME_LIMIT: u32 = 30;

    let mut stack = Vec::new();
    stack.push(StackItem::new(start, 0, 0, HashSet::new()));

    let mut max_pressure = 0;
    while let Some(current) = stack.pop() {
        if current.time >= PART1_TIME_LIMIT {
            if current.pressure_released > max_pressure {
                max_pressure = current.pressure_released;
            }
            continue;
        }

        for n in &distances[current.node] {
            if current.time + n.distance + 1 > PART1_TIME_LIMIT {
                continue;
            }

            let mut new_visited = current.visited.clone();
            if !new_visited.insert(n.node) {
                continue;
            }

            let pressure_release = (PART1_TIME_LIMIT - current.time - n.distance - 1) * flow_rates[n.node];
            stack.push(StackItem::new(n.node, current.time + n.distance + 1, current.pressure_released + pressure_release, new_visited));
        }

        stack.push(StackItem::new(current.node, PART1_TIME_LIMIT, current.pressure_released, current.visited.clone()));
    }
    println!("[Part 1] Released pressure: {max_pressure}");

    // Part 2
    const PART2_TIME_LIMIT: u32 = 26;
    let mut reachable: HashMap<Vec<usize>, u32> = HashMap::new();

    stack.clear();
    stack.push(StackItem::new(start, 0, 0, HashSet::new()));

    while let Some(current) = stack.pop() {
        if current.time >= PART2_TIME_LIMIT {
            let visited = current.visited.iter()
                .cloned()
                .sorted()
                .collect_vec();

            match reachable.entry(visited) {
                Entry::Occupied(mut e) => {
                    if *e.get() < current.pressure_released {
                        e.insert(current.pressure_released);
                    }
                },
                Entry::Vacant(v) => {
                    v.insert(current.pressure_released);
                },
            };
            continue;
        }

        for n in &distances[current.node] {
            if current.time + n.distance + 1 > PART2_TIME_LIMIT {
                continue;
            }

            let mut new_visited = current.visited.clone();
            if !new_visited.insert(n.node) {
                continue;
            }

            let pressure_release = (PART2_TIME_LIMIT - current.time - n.distance - 1) * flow_rates[n.node];
            stack.push(StackItem::new(n.node, current.time + n.distance + 1, current.pressure_released + pressure_release, new_visited));
        }

        stack.push(StackItem::new(current.node, PART2_TIME_LIMIT, current.pressure_released, current.visited.clone()));
    }


    let all_valves: Vec<_> = flow_rates.iter().enumerate()
        .filter_map(|(i, f)| {
            if *f > 0 {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let mut max_result = 0;
    for k in 0..(all_valves.len() / 2 + 2) {
        for p1 in all_valves.iter().cloned().combinations(k) {
            if !reachable.contains_key(&p1) {
                continue;
            }
            let p1_score = reachable[&p1];

            // Calculate p2
            for m in 0..(all_valves.len() - k) {
                let remaining = all_valves.iter()
                    .filter(|v| !p1.contains(v))
                    .cloned();

                for p2 in remaining.combinations(m) {
                    if !reachable.contains_key(&p2) {
                        continue;
                    }

                    let score = p1_score + reachable[&p2];
                    if score > max_result {
                        max_result = score;
                    }
                }
            }
        }
    }
    println!("[Part 2] Released pressure: {max_result}");
}
