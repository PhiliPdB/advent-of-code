
const PREAMBLE_SIZE: usize = 25;

fn contains_sum(numbers: &[u64], target: u64) -> bool {
    for i in 0..numbers.len() {
        for j in 0..i {
            if numbers[i] + numbers[j] == target {
                return true;
            }
        }
    }

    false
}

fn find_contiguous_range(numbers: &[u64], target: u64) -> Option<(usize, usize)> {
    for i in 0..numbers.len() {
        let mut sum = numbers[i];
        let mut offset = 0;
        while sum < target {
            offset += 1;
            sum += numbers[i + offset];
        }

        if sum == target {
            return Some((i, i + offset));
        }
    }

    None
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    // Part 1
    let first_non_valid = input.iter().enumerate()
        .skip(PREAMBLE_SIZE)
        .find(|(i, n)| !contains_sum(&input[i-PREAMBLE_SIZE..*i], **n))
        .unwrap().1;
    println!("First non-valid: {}", first_non_valid);

    // Part 2
    let (min_index, max_index) = find_contiguous_range(&input, *first_non_valid).unwrap();
    let min_value = input[min_index..=max_index].iter().min().unwrap();
    let max_value = input[min_index..=max_index].iter().max().unwrap();
    println!("Encryption weakness: {}", min_value + max_value);
}
