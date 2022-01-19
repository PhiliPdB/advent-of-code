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
    // When to jump:
    // (!A || !B || !C) && D

    let part1_instructions = [
        "NOT A J",
        "NOT B T",
        "OR T J",
        "NOT C T",
        "OR T J",
        "AND D J",
        "WALK\n",
    ];

    let damage = program.clone().run(
        part1_instructions.join("\n")
            .chars()
            .map(|c| c as u8)
    );
    println!("[Part 1] Damage: {:#10}", damage);

    // Part 2
    // When to jump:
    // (!A || !B || !C) && D && (E || H)

    let part2_instructions = [
        "NOT A J",
        "NOT B T",
        "OR T J",
        "NOT C T",
        "OR T J",
        "AND D J",

        "NOT J T",
        "OR E T",
        "OR H T",
        "AND T J",

        "RUN\n",
    ];

    let damage = program.run(
        part2_instructions.join("\n")
            .chars()
            .map(|c| c as u8)
    );
    println!("[Part 2] Damage: {:#10}", damage);
}
