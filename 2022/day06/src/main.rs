use std::collections::HashSet;

fn main() {
    let input: Vec<_> = include_str!("../input.txt").chars().collect();

    for (i, w) in input.windows(4).enumerate() {
        if w[0] != w[1] && w[0] != w[2] && w[0] != w[3] && w[1] != w[2] && w[1] != w[3] && w[2] != w[3] {
            println!("[Part 1] Processed {} characters", i + 4);
            break;
        }
    }

    for (i, w) in input.windows(14).enumerate() {
        if HashSet::<char>::from_iter(w.iter().cloned()).len() == 14 {
            println!("[Part 2] Processed {} characters", i + 14);
            break;
        }
    }
}
