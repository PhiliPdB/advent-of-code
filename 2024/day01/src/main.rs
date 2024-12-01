fn main() {
    let (mut left_list, mut right_list): (Vec<_>, Vec<_>) = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (l1, l2) = l.split_once("   ").unwrap();
            let l1: u32 = l1.parse().unwrap();
            let l2: u32 = l2.parse().unwrap();

            (l1, l2)
        })
        .unzip();

    left_list.sort_unstable();
    right_list.sort_unstable();

    let distance: u32 = left_list.iter()
        .zip(right_list.iter())
        .map(|(l, r)| u32::abs_diff(*l,  *r))
        .sum();
    println!("[Part 1] Total distance: {distance}");

    let similarity: u32 = left_list.iter()
        .map(|l| {
            let right_list_count = right_list.iter()
                .filter(|r| *r == l)
                .count() as u32;

            l * right_list_count
        })
        .sum();
    println!("[Part 2] Similarity: {similarity}");
}
