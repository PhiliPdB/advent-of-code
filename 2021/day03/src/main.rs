/// Size of the bits in the diagnostic report
const BIT_LENGTH: usize = 12;
const BIT_MASK: i32 = {
    let mut mask = 1;
    let mut i = BIT_LENGTH - 1;
    while i > 0 {
        mask <<= 1;
        mask |= 1;
        i -= 1;
    }

    mask
};

fn get_gamma(report: &[Vec<i32>]) -> i32 {
    let report_half_size = (report.len() / 2) as i32;

    let mut gamma = 0;
    for i in 0..BIT_LENGTH {
        // Since the report only contains 0's and 1's, the sum of them gives us
        // the amount of 1's on that bit.
        if report.iter().map(|v| v[i]).sum::<i32>() >= report_half_size {
            gamma |= 1;
        }

        gamma <<= 1;
    }
    // Did one bit shift to many...
    gamma >>= 1;

    gamma
}

fn find_rating<F>(report: &[Vec<i32>], criteria: F) -> i32
    where F: Fn(i32, i32) -> bool
{
    let mut values_left = report.to_owned();
    let mut current_bit = 0;
    while values_left.len() > 1 {
        let bit_criteria =
            if criteria(values_left.iter().map(|v| v[current_bit]).sum::<i32>(), (values_left.len() as f64 / 2_f64).ceil() as i32) {
                1
            } else {
                0
            };

        // Filter the values that are left based on the bit criteria.
        values_left.retain(|v| v[current_bit] == bit_criteria);

        // Look at the next bit
        current_bit += 1;
    }

    // Convert to decimal
    let mut value = values_left[0].iter()
        .fold(0, |acc, b| {
            (acc | b) << 1
        });
    value >>= 1;

    value
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.chars().map(|c| match c {
            '0' => 0,
            '1' => 1,
            _   => panic!("Invalid character"),
        }).collect::<Vec<_>>())
        .collect();

    // Part 1

    let gamma = get_gamma(&input);
    let epsilon = !gamma & BIT_MASK;

    println!("Power consumption rate: {}", gamma * epsilon);

    // Part 2

    let oxygen_generator_rating = find_rating(&input, |x, y| x >= y);
    let co2_scrubber_rating = find_rating(&input, |x, y| x < y);

    println!("Life support rating: {}", oxygen_generator_rating * co2_scrubber_rating);
}
