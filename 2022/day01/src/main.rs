fn main() {
    let mut calories: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .map(|l| {
            l.lines()
                .map(|s| s.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();
    calories.sort_unstable_by(|a, b| b.cmp(a));

    println!("[Part 1] Calories: {:6}", calories[0]);
    println!("[Part 2] Calories: {:6}", calories.iter().take(3).sum::<i32>());
}
