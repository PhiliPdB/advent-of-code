const TOTAL_DAYS: i32 = 256;

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Merge fish with the same day together
    let mut fish: Vec<_> = (0..6)
        .filter_map(|day| {
            let total_fish = input.iter().filter(|&&f| f == day).count();
            if total_fish > 0 {
                Some((total_fish as u64, day as i8))
            } else {
                None
            }
        })
        .collect();

    for _current_day in 0..TOTAL_DAYS {
        let mut new_fish = 0;
        fish = fish.into_iter()
            .map(|(f, day)| {
                if day > 0 {
                    (f, day - 1)
                } else {
                    new_fish += f;

                    (f, 6)
                }
            })
            .collect();

        // Create new fish
        fish.push((new_fish, 8));
    }

    println!("Total fish: {:}", fish.iter().map(|(f, _)| f).sum::<u64>());
}
