use intcode::Program;

mod intcode;

fn main() {
    let program = Program::new(
        include_str!("../input.txt")
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    );

    // Part 1

    let mut affected_points = 0;
    for x in 0..50 {
        for y in 0..50 {
            affected_points += program.clone().run([x, y]);
        }
    }

    println!("Affected points: {affected_points}");


    // Part 2

    let mut current = (0, 99);
    loop {
        let o = program.clone().run([current.0, current.1]);
        if o == 0 {
            current.0 += 1;
        } else {
            if program.clone().run([current.0 + 99, current.1 - 99]) == 1 {
                // Assume the entire thing fits
                println!("Answer: {}", current.0 * 10_000 + current.1 - 99);
                break;
            } else {
                current.1 += 1;
            }
        }
    }
}
