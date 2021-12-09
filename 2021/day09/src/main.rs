
fn find_low_points(heightmap: &[Vec<u8>]) -> Vec<u8> {
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
                low_points.push(current_item);
            }
        }
    }

    low_points
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.chars().map(|h| h.to_digit(10).unwrap() as u8).collect::<Vec<_>>())
        .collect();


    let danger_level: i32 = find_low_points(&input).iter()
        .map(|i| (i + 1) as i32)
        .sum();

    println!("Danger level: {}", danger_level);
}
