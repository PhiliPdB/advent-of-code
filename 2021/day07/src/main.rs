
fn constant_fuel_cost(positions: &[i32], move_to: i32) -> i32 {
    positions.iter()
        .map(|p| (p - move_to).abs())
        .sum()
}

fn fuel_cost(positions: &[i32], move_to: i32) -> i32 {
    positions.iter()
        .map(|p| {
            let difference = (p - move_to).abs();

            // NOTE: 1 + 2 + ... + n == n(n+1) / 2
            (difference * (difference + 1)) / 2
        })
        .sum()
}

fn main() {
    let positions: Vec<_> = include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let part1_least_fuel_cost = (min..max)
        .map(|p| constant_fuel_cost(&positions, p))
        .min().unwrap();
    let part2_least_fuel_cost = (min..max)
        .map(|p| fuel_cost(&positions, p))
        .min().unwrap();

    println!("Part1 least fuel cost: {:#8}", part1_least_fuel_cost);
    println!("Part2 least fuel cost: {:#8}", part2_least_fuel_cost);
}
