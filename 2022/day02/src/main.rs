
fn score_part1(my_play: char, opponent: char) -> i32 {
    #[allow(clippy::identity_op)]
    match (my_play, opponent) {
        ('X', 'A') => 3 + 1,
        ('X', 'B') => 0 + 1,
        ('X', 'C') => 6 + 1,
        ('Y', 'A') => 6 + 2,
        ('Y', 'B') => 3 + 2,
        ('Y', 'C') => 0 + 2,
        ('Z', 'A') => 0 + 3,
        ('Z', 'B') => 6 + 3,
        ('Z', 'C') => 3 + 3,
        _ => unreachable!()
    }
}

fn score_part2(my_play: char, opponent: char) -> i32 {
    #[allow(clippy::identity_op)]
    match (my_play, opponent) {
        ('X', 'A') => 0 + 3,
        ('X', 'B') => 0 + 1,
        ('X', 'C') => 0 + 2,
        ('Y', 'A') => 3 + 1,
        ('Y', 'B') => 3 + 2,
        ('Y', 'C') => 3 + 3,
        ('Z', 'A') => 6 + 2,
        ('Z', 'B') => 6 + 3,
        ('Z', 'C') => 6 + 1,
        _ => unreachable!()
    }
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let c: Vec<_> = l.chars().collect();
            (c[2], c[0])
        })
        .collect();

    println!("[Part 1] Score: {:5}", input.iter().map(|(my_play, opponent)| score_part1(*my_play, *opponent)).sum::<i32>());
    println!("[Part 2] Score: {:5}", input.iter().map(|(my_play, opponent)| score_part2(*my_play, *opponent)).sum::<i32>());
}
