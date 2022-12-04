fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let pairs = l.split_once(',').unwrap();
            let p1 = pairs.0.split_once('-').unwrap();
            let p2 = pairs.1.split_once('-').unwrap();

            (
                (p1.0.parse::<i32>().unwrap(), p1.1.parse::<i32>().unwrap()),
                (p2.0.parse::<i32>().unwrap(), p2.1.parse::<i32>().unwrap())
            )
        })
        .collect();

    let fully_overlapping_pairs = input.iter()
        .filter(|(p1, p2)| {
            (p1.0 <= p2.0 && p2.1 <= p1.1)
            || (p2.0 <= p1.0 && p1.1 <= p2.1)
        })
        .count();
    println!("[Part 1] Overlapping pairs: {fully_overlapping_pairs}");

    let overlapping_pairs = input.iter()
        .filter(|(p1, p2)| {
            (p2.0 >= p1.0 && p2.0 <= p1.1) || (p1.0 >= p2.0 && p1.0 <= p2.1)
        })
        .count();
    println!("[Part 2] Overlapping pairs: {overlapping_pairs}");
}
