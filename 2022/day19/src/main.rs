use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Eq)]
struct HeapItem {
    resources: (u32, u32, u32, u32),
    robots: (u32, u32, u32, u32),
    time: u32,
}

impl HeapItem {
    fn new(resources: (u32, u32, u32, u32), robots: (u32, u32, u32, u32), time: u32) -> Self {
        Self { resources, robots, time }
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.robots.3.cmp(&other.robots.3)
            .then(self.robots.2.cmp(&other.robots.2))
            .then(self.robots.1.cmp(&other.robots.1))
            .then(self.robots.0.cmp(&other.robots.0))
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obsidian_robot_cost: (u32, u32),
    geode_robot_cost: (u32, u32),
}

impl Blueprint {
    fn new(id: u32, ore_robot_cost: u32, clay_robot_cost: u32, obsidian_robot_cost: (u32, u32), geode_robot_cost: (u32, u32)) -> Self {
        Self { id, ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost }
    }

    fn max_geodes(&self, max_time: u32) -> u32 {
        let mut max = 0;

        let mut queue = VecDeque::new();
        queue.push_back(HeapItem::new((0, 0, 0, 0), (1, 0, 0, 0), 0));

        let mut visited = HashSet::new();

        let mut max_geodes = vec![0; max_time as usize];
        let mut max_geode_robots = vec![0; max_time as usize];

        while let Some(HeapItem { robots, resources, time }) = queue.pop_front() {
            if time >= max_time {
                // println!("{time} {robots:?} {resources:?}");
                if resources.3 > max {
                    max = resources.3;
                }
                continue;
            }

            if !visited.insert((robots, resources)) {
                continue;
            }

            let time_left = max_time - time;
            let to_collect = (time_left + 1)*(time_left + 2*robots.3) / 2;

            if resources.3 + to_collect <= max
                || resources.3 < (max_geodes[time as usize] - 2).max(0) as u32
                || robots.3 < (max_geode_robots[time as usize] - 2).max(0) as u32
            {
                continue;
            }

            if resources.3 > max_geodes[time as usize] as u32 {
                max_geodes[time as usize] = resources.3 as i32;
            }
            if robots.3 > max_geode_robots[time as usize] as u32 {
                max_geode_robots[time as usize] = robots.3 as i32;
            }

            // Update resources
            let new_resources = (
                resources.0 + robots.0,
                resources.1 + robots.1,
                resources.2 + robots.2,
                resources.3 + robots.3,
            );
            queue.push_back(HeapItem::new(new_resources, robots, time + 1));

            // Check if we can make new robots
            if resources.0 >= self.ore_robot_cost {
                queue.push_back(HeapItem::new(
                    (new_resources.0 - self.ore_robot_cost, new_resources.1, new_resources.2, new_resources.3),
                    (robots.0 + 1, robots.1, robots.2, robots.3),
                    time + 1,
                ));
            }

            if resources.0 >= self.clay_robot_cost {
                queue.push_back(HeapItem::new(
                    (new_resources.0 - self.clay_robot_cost, new_resources.1, new_resources.2, new_resources.3),
                    (robots.0, robots.1 + 1, robots.2, robots.3),
                    time + 1,
                ));
            }

            if resources.0 >= self.obsidian_robot_cost.0 && resources.1 >= self.obsidian_robot_cost.1 {
                queue.push_back(HeapItem::new(
                    (new_resources.0 - self.obsidian_robot_cost.0, new_resources.1 - self.obsidian_robot_cost.1, new_resources.2, new_resources.3),
                    (robots.0, robots.1, robots.2 + 1, robots.3),
                    time + 1,
                ));
            }

            if resources.0 >= self.geode_robot_cost.0 && resources.2 >= self.geode_robot_cost.1 {
                queue.push_back(HeapItem::new(
                    (new_resources.0 - self.geode_robot_cost.0, new_resources.1, new_resources.2 - self.geode_robot_cost.1, new_resources.3),
                    (robots.0, robots.1, robots.2, robots.3 + 1),
                    time + 1,
                ));
            }
        }

        max
    }
}


fn main() {
    let blueprints: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let s = scan_fmt!(
                l,
                "Blueprint {d}: Each ore robot costs {d} ore. Each clay robot costs {d} ore. Each obsidian robot costs {d} ore and {d} clay. Each geode robot costs {d} ore and {d} obsidian.",
                u32, u32, u32, u32, u32, u32, u32
            ).unwrap();

            Blueprint::new(s.0, s.1, s.2, (s.3, s.4), (s.5, s.6))
        })
        .collect();

    let quality_sum: u32 = blueprints.iter()
        .map(|b| {
            let g = b.max_geodes(24);
            println!("{} {g}", b.id);

            b.id * g
        })
        .sum();
    println!("[Part 1] Quality sum: {quality_sum}");

    let product: u32 = blueprints[..3].iter()
        .map(|b| b.max_geodes(32))
        .product();
    println!("[Part 2] Product: {product}");
}
