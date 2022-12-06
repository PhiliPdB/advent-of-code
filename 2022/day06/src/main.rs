fn main() {
    let input: Vec<_> = include_str!("../input.txt").chars().collect();

    for (i, w) in input.windows(4).enumerate() {
        if w[0] != w[1] && w[0] != w[2] && w[0] != w[3] && w[1] != w[2] && w[1] != w[3] && w[2] != w[3] {
            println!("[Part 1] Processed {} characters", i + 4);
            break;
        }
    }

    'outer: for (i, w) in input.windows(14).enumerate() {
        let mut seen: u32 = 0;
        for c in w {
            let c = *c as u32;

            if (seen >> c) & 1 == 1 {
                continue 'outer;
            } else {
                seen |= 1 << c;
            }
        }

        println!("[Part 2] Processed {} characters", i + 14);
        break;
    }
}
