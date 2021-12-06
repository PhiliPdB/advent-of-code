const TOTAL_DAYS: i32 = 80;

fn main() {
    let mut fish: Vec<_> = include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    for _current_day in 0..TOTAL_DAYS {
        // Reduce day count
        fish = fish.into_iter().map(|x| x - 1).collect();

        // Create new fish
        let new_fish = fish.iter().filter(|&&x| x == -1).count();
        fish.extend_from_slice(&vec![8; new_fish]);

        // Reset fish timers
        fish = fish.into_iter().map(|x| if x == -1 {
            6
        } else {
            x
        }).collect();
    }


    println!("Total fish: {:?}", fish.len());
}
