fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    // Set the size of the sliding window.
    // This should be 1 for part 1 and 3 for part 2.
    const WINDOW_SIZE: usize = 3;

    let mut increases = 0;
    for i in WINDOW_SIZE..input.len() {
        // We can obtain the new measurement by subtracting input[i - WINDOW_SIZE]
        // from the previous sum and adding input[i].
        // This leads to a measurement increase iff input[i - WINDOW_SIZE] < input[i].
        // Thus, we only need to check this condition, meaning there is no need to keep
        // track of the previous total.
        if input[i - WINDOW_SIZE] < input[i] {
            increases += 1;
        }
    }

    println!("Measurement increases: {}", increases);
}
