

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split('\n')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();


    const WINDOW_SIZE: usize = 3;

    let mut increases = 0;
    let mut last_measurement: i32 = input[0..WINDOW_SIZE].iter().sum();
    for i in WINDOW_SIZE..input.len() {
        let new_measurement = last_measurement - input[i - WINDOW_SIZE] + input[i];
        if new_measurement > last_measurement {
            increases += 1;
        }
        last_measurement = new_measurement;
    }

    println!("Measurement increases: {}", increases);
}
