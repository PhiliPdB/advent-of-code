use std::iter;


fn fft_phase(input: &[i32]) -> Vec<i32> {
    const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

    let mut output = Vec::with_capacity(input.len());

    for i in 1..(input.len() + 1) {
        let pattern = BASE_PATTERN.into_iter()
            .flat_map(|n| iter::repeat(n).take(i))
            .cycle()
            .skip(1);

        // Calculate digit
        let digit = input.iter()
            .zip(pattern)
            .map(|(x, y)| *x * y)
            .sum::<i32>().abs() % 10;

        output.push(digit);
    }

    output
}

fn fft_fast(input: &mut [i32]) {
    for i in (0..input.len() - 2).rev() {
        input[i] += input[i + 1];
        input[i] %= 10;
    }
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    const PHASES: u32 = 100;


    // Part 1

    let mut part1_fft = input.clone();
    for _ in 0..PHASES {
        part1_fft = fft_phase(&part1_fft);
    }

    print!("[Part 1] First 8 digits: ");
    for d in part1_fft.iter().take(8) {
        print!("{d}");
    }
    println!();


    // Part 2
    let input_len = input.len();
    // Repeat the input 10_000 times.
    let mut input: Vec<_> = input.into_iter().cycle().take(input_len * 10_000).collect();
    // Calculate the offset
    let offset = input.iter().take(7)
        .fold(0, |acc, d| acc * 10 + d) as usize;
    assert!(offset > input.len() / 2, "fft_fast doesn't work when the offset is not big enough");

    for _ in 0..PHASES {
        fft_fast(&mut input[offset..]);
    }

    print!("[Part 2] First 8 digits: ");
    for d in input.iter().skip(offset).take(8) {
        print!("{d}");
    }
    println!();
}
