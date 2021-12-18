use snailfish::Snailfish;

mod snailfish;


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let (rem, sf) = Snailfish::parse(s).unwrap();
            debug_assert!(rem.is_empty());
            sf
        })
        .collect();

    // Part 1

    let mut snailfish = input[0].clone();
    for sf in input.iter().skip(1).cloned() {
        snailfish = Snailfish::add(snailfish, sf);
    }
    println!("Magnitude: {}", snailfish.magnitude());

    // Part 2

    let mut largest_magnitude = 0;
    for i in 0..input.len() {
        for j in 0..i {
            let mag1 = Snailfish::add(input[i].clone(), input[j].clone()).magnitude();
            let mag2 = Snailfish::add(input[j].clone(), input[i].clone()).magnitude();

            if mag1 > largest_magnitude {
                largest_magnitude = mag1;
            }
            if mag2 > largest_magnitude {
                largest_magnitude = mag2;
            }
        }
    }

    println!("Largest magnitude: {}", largest_magnitude);
}
