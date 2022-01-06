use intcode::Program;


mod intcode;

fn main() {
    let mut program = Program::new(
        include_str!("../input.txt")
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    );

    // Part 1
    let mut part1_program = program.clone();
    part1_program.run();

    println!("Block tiles: {}", part1_program.block_tiles);


    // Part 2
    // Play for free
    program[0] = 2;
    let score = program.run();

    println!("Score: {}", score);
}
