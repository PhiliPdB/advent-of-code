
fn trees_for_slope(map: &[Vec<bool>], (dx, dy): (usize, usize)) -> u32 {
    let (mut x, mut y) = (0, 0);
    let mut total_trees = 0;
    let modulo = map[0].len();

    while y < map.len() - dy {
        x += dx;
        y += dy;

        if map[y][x % modulo] {
            total_trees += 1;
        }
    }

    total_trees
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c == '#')
                .collect::<Vec<_>>()
        })
        .collect();

    // Part 1
    println!("Trees for slope (3, 1): {}", trees_for_slope(&input, (3, 1)));

    // Part 2

    let trees_product: u32 = [
            (1, 1), (3, 1), (5, 1), (7, 1), (1, 2)
        ].iter()
        .map(|&s| trees_for_slope(&input, s))
        .product();

    println!("Product of trees for the slopes: {}", trees_product);
}
