use std::collections::{VecDeque, HashSet};


fn find_low_points(heightmap: &[Vec<u8>]) -> Vec<(u8, usize, usize)> {
    let mut low_points = Vec::new();

    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            let mut is_low_point = true;
            let current_item = heightmap[y][x];
            if x > 0 {
                is_low_point = is_low_point && current_item < heightmap[y][x - 1];
            }
            if x < heightmap[y].len() - 1 {
                is_low_point = is_low_point && current_item < heightmap[y][x + 1];
            }

            if y > 0 {
                is_low_point = is_low_point && current_item < heightmap[y - 1][x];
            }
            if y < heightmap.len() - 1 {
                is_low_point = is_low_point && current_item < heightmap[y + 1][x];
            }

            if is_low_point {
                low_points.push((current_item, x, y));
            }
        }
    }

    low_points
}

fn basin_size(heightmap: &[Vec<u8>], x: usize, y: usize) -> (i32, HashSet<(usize, usize)>) {
    // Perform BFS from lowest point, to find out the basin size.
    let mut basin_size = 0;
    let mut queue = VecDeque::from([(x, y)]);
    let mut visited = HashSet::new();

    while let Some((next_x, next_y)) = queue.pop_front() {
        if visited.contains(&(next_x, next_y)) {
            continue;
        }

        visited.insert((next_x, next_y));

        if heightmap[next_y][next_x] == 9 {
            continue;
        }

        basin_size += 1;

        // Push neighbours to the queue
        if next_x != 0 {
            queue.push_back((next_x - 1, next_y));
        }
        if next_x != heightmap[next_y].len() - 1 {
            queue.push_back((next_x + 1, next_y));
        }
        if next_y != 0 {
            queue.push_back((next_x, next_y - 1));
        }
        if next_y != heightmap.len() - 1 {
            queue.push_back((next_x, next_y + 1));
        }
    }

    (basin_size, visited)
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.chars().map(|h| h.to_digit(10).unwrap() as u8).collect::<Vec<_>>())
        .collect();

    let low_points = find_low_points(&input);

    // Part 1

    let danger_level: i32 = low_points.iter()
        .map(|(i, _, _)| (i + 1) as i32)
        .sum();

    println!("Danger level: {}", danger_level);

    // Part 2

    let mut basin_sizes = Vec::with_capacity(low_points.len());
    let mut low_point_coordinates = HashSet::with_capacity(low_points.len());
    for (_, x, y) in low_points {
        let (size, visited) = basin_size(&input, x, y);
        // Check if we didn't already count this basin
        if visited.intersection(&low_point_coordinates).count() == 0 {
            basin_sizes.push(size);
        }

        low_point_coordinates.insert((x, y));
    }
    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));

    println!("Basin size product: {}", basin_sizes.iter().take(3).product::<i32>());
}
