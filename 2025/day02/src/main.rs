fn is_invalid_part1_id(id: u64) -> bool {
    let string_id = id.to_string();
    if !string_id.len().is_multiple_of(2) {
        return false;
    }
    let m = string_id.len() / 2;

    string_id[..m] == string_id[m..]
}

fn is_invalid_part2_id(id: u64) -> bool {
    let string_id = id.to_string();
    for m in 1..=(string_id.len() / 2) {
        if !string_id.len().is_multiple_of(m) {
            continue;
        }

        // Slow check
        // if string_id == string_id[..m].repeat(string_id.len() / m) {
        //     return true;
        // }

        if (m..string_id.len())
            .step_by(m)
            .all(|i| string_id[..m] == string_id[i..i + m])
        {
            return true;
        }
    }

    false
}

fn main() {
    let ranges: Vec<_> = include_str!("../input.txt")
        .trim_end()
        .split(',')
        .map(|s| {
            let t = s.split_once('-').unwrap();

            (t.0.parse::<u64>().unwrap(), t.1.parse::<u64>().unwrap())
        })
        .collect();

    let mut part1_invalid_sum_id = 0;
    let mut part2_invalid_sum_id = 0;
    for range in ranges {
        for id in range.0..=range.1 {
            if is_invalid_part1_id(id) {
                part1_invalid_sum_id += id;
            }

            if is_invalid_part2_id(id) {
                part2_invalid_sum_id += id;
            }
        }
    }
    println!("[Part 1] Invalid ID sum: {part1_invalid_sum_id}");
    println!("[Part 2] Invalid ID sum: {part2_invalid_sum_id}");
}
