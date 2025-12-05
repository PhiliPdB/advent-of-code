fn is_fresh(ranges: &[(u64, u64)], ingredient: u64) -> bool {
    ranges
        .iter()
        .any(|&(s, e)| s <= ingredient && ingredient <= e)
}

fn build_non_overlapping_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut non_overlapping_ranges = vec![ranges[0]];
    for &(s, e) in ranges.iter().skip(1) {
        let (_, le) = non_overlapping_ranges
            .last_mut()
            .unwrap();

        if s <= *le + 1 {
            *le = u64::max(*le, e);
        } else {
            non_overlapping_ranges.push((s, e));
        }
    }
    non_overlapping_ranges
}

fn main() {
    let (fresh_ranges, ingredients) = include_str!("../input.txt")
        .split_once("\n\n")
        .unwrap();

    // Parse the fresh ranges
    let mut fresh_ranges: Vec<_> = fresh_ranges
        .lines()
        .map(|l| {
            let (start, end) = l.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect();
    // Sort (by start of range)
    fresh_ranges.sort_unstable();
    // And covert to non-overlapping ranges
    let non_overlapping_ranges = build_non_overlapping_ranges(&fresh_ranges);

    // Parse ingredient list
    let ingredients: Vec<_> = ingredients
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    let fresh_ingredients = ingredients
        .iter()
        .filter(|i| is_fresh(&non_overlapping_ranges, **i))
        .count();
    println!("[Part 1] Fresh ingredients: {fresh_ingredients}");

    let total_fresh_ids: u64 = non_overlapping_ranges
        .iter()
        .map(|&(s, e)| e - s + 1)
        .sum();
    println!("[Part 2] Total fresh ids: {total_fresh_ids}");
}
