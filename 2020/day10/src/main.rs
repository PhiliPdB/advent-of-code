
fn arrangements(differences: &[i32]) -> u64 {
    let mut groups = Vec::new();

    let mut iter = differences.iter().enumerate();
    while let Some((i, n)) = iter.next() {
        if *n == 3 {
            groups.push(1);
            continue;
        } else {
            let group_size = differences.iter()
                .skip(i)
                .take_while(|i| **i == 1)
                .count();

            groups.push(group_size);
            iter.nth(group_size - 1);
        }
    }

    groups.iter()
        .map(|group_size| match group_size {
            0 => 1,
            1 => 1,
            2 => 2,
            3 => 4,
            4 => 7,
            _ => panic!("Unsupported group size"),
        })
        .product()
}

fn main() {
    let mut input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    input.push(0);
    input.sort_unstable();
    input.push(input[input.len() - 1] + 3);

    let differences: Vec<_> = input.windows(2)
        .map(|w| w[1] - w[0])
        .collect();
    let one_jolt_diffs = differences.iter().filter(|d| **d == 1).count();
    let three_jolt_diffs = differences.iter().filter(|d| **d == 3).count();
    println!("Chain value: {}", one_jolt_diffs * three_jolt_diffs);


    println!("Adapter arrangements: {}", arrangements(&differences));
}
