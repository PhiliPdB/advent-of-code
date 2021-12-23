use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

use hashbrown::HashSet;

use crate::map::{Map, Map2, Map4};

mod map;
mod space;

#[derive(Debug, PartialEq, Eq)]
pub struct Node<M : Map>(M, u32);

impl<M: Map + Eq> Ord for Node<M> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

impl<M: Map + Eq> PartialOrd for Node<M> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_min_score<M>(map: M) -> u32
    where M: Map + Eq + std::hash::Hash + Copy
{
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(Node(map, 0));

    while let Some(current_item) = heap.pop() {
        if current_item.0.is_finished() {
            return current_item.1;
        }

        if !visited.insert(current_item.0) {
            continue;
        }

        // Add new moves
        for mut new_move in current_item.0.generate_moves() {
            new_move.1 += current_item.1;
            heap.push(new_move);
        }
    }

    unreachable!();
}

fn main() {
    let input  = Map2::from_str(include_str!("../input.txt")).unwrap();
    let input2 = Map4::from_str(include_str!("../input_part2.txt")).unwrap();


    let part1_score = get_min_score(input);
    println!("[Part 1] Min score: {:?}", part1_score);

    let part2_score = get_min_score(input2);
    println!("[Part 2] Min score: {:?}", part2_score);
}

