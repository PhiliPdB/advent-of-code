fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .collect();

    let part1_calibration: u32 = input.iter()
        .map(|s| {
            let chars: Vec<_> = s.chars()
                .filter_map(|c| c.to_digit(10))
                .collect();

            10 * chars[0] + chars[chars.len() - 1]
        })
        .sum();
    println!("[Part 1] Sum of calibration: {part1_calibration}");


    let matches = [
        ("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'),
        ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9')
    ];
    let part2_calibration: u32 = input.iter()
        .map(|s| {
            let mut occurrences: Vec<_> = matches
                .iter()
                .flat_map(|m| {
                    s.match_indices(m.0).map(|x| (x.0, m.1)).collect::<Vec<_>>()
                })
                .collect();
            occurrences.sort_unstable_by(|a, b| a.0.cmp(&b.0));

            let mut chars: Vec<_> = s.chars().collect();
            if occurrences.len() > 0 {
                let first_match = occurrences[0];
                let last_match = occurrences[occurrences.len() - 1];

                chars[first_match.0] = first_match.1;
                chars[last_match.0] = last_match.1;
            }

            let chars: Vec<_> = chars.into_iter()
                .filter_map(|c| c.to_digit(10))
                .collect();

            10 * chars[0] + chars[chars.len() - 1]
        })
        .sum();
    println!("[Part 2] Sum of calibration: {part2_calibration}");
}
