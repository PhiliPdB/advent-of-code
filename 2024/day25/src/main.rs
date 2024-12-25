use itertools::iproduct;


fn key_fits(key: &[usize; 5], lock: &[usize; 5]) -> bool {
    key.iter().zip(lock.iter())
        .all(|(key_column, lock_column)| key_column + lock_column <= 5)
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .map(|x| {
            let grid: Vec<_> = x.lines()
                .map(|x| x.chars().collect::<Vec<_>>())
                .collect();
            let is_lock = grid[0][0] == '#';
            let mut column_heights = [0; 5];

            for row in grid.iter().skip(1).take(5) {
                for (x, &cell) in row.iter().enumerate() {
                    if cell == '#' {
                        column_heights[x] += 1;
                    }
                }
            }

            (is_lock, column_heights)
        })
        .collect();
    let locks: Vec<_> = input.iter()
        .filter(|(is_lock, _)| *is_lock)
        .map(|(_, column_heights)| *column_heights)
        .collect();
    let keys: Vec<_> = input.iter()
        .filter(|(is_lock, _)| !is_lock)
        .map(|(_, column_heights)| *column_heights)
        .collect();


    let fitting_pairs = iproduct!(locks.iter(), keys.iter())
        .filter(|(lock, key)| key_fits(key, lock))
        .count();
    println!("[Part 1] Key/locks that fit together: {fitting_pairs}");
}
