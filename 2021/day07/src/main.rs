
fn fuel_cost(positions: &Vec<i32>, move_to: i32) -> i32 {
    positions.iter()
        .map(|p| (p - move_to).abs())
        .sum()
}

fn main() {
    let positions: Vec<_> = include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let least_fuel_position = (min..max)
        .map(|p| fuel_cost(&positions, p))
        .min().unwrap();

    println!("Least fuel cost: {}", least_fuel_position);
}
