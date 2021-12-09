
fn get_seat_id(boarding_pass: &str) -> i32 {
    let row = boarding_pass.chars().take(7).fold(
        (0, 127),
        |(low, up), c| {
            let mid = (low + up) / 2;
            match c {
                'F' => (low, mid),
                'B' => (mid, up),
                _   => panic!("Invalid row identifier"),
            }
        }
    ).1;

    let col = boarding_pass.chars().skip(7).fold(
        (0, 7),
        |(low, up), c| {
            let mid = (low + up) / 2;
            match c {
                'L' => (low, mid),
                'R' => (mid, up),
                _   => panic!("Invalid col identifier"),
            }
        }
    ).1;

    row * 8 + col
}

fn main() {
    let mut input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| get_seat_id(s))
        .collect();
    input.sort();

    // Part 1
    println!("Max seat id: {}", input[input.len() - 1]);

    // Part 2
    let seat = input.windows(2)
        .filter_map(|w| if w[0] + 2 == w[1] {
            Some(w[0] + 1)
        } else {
            None
        })
        .collect::<Vec<_>>()[0];
    println!("Seat id: {}", seat);
}
