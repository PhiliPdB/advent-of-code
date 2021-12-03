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

fn get_gamma(report: &Vec<Vec<i32>>) -> i32 {
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

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.chars().map(|c| match c {
            '0' => 0,
            '1' => 1,
            _   => panic!("Invalid character"),
        }).collect::<Vec<_>>())
        .collect();


    let gamma = get_gamma(&input);
    let epsilon = !gamma & BIT_MASK;

    println!("Power consumption rate: {}", gamma * epsilon);
}
