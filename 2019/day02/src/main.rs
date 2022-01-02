
pub fn run_program(program: &mut [usize], noun: usize, verb: usize) -> usize {
    program[1] = noun;
    program[2] = verb;

    // Set instruction pointer
    let mut ip = 0;
    loop {
        match program[ip] {
            1 => {
                let loc1 = program[ip + 1];
                let loc2 = program[ip + 2];
                let loc3 = program[ip + 3];
                program[loc3] = program[loc1] + program[loc2];
                ip += 4;
            },
            2 => {
                let loc1 = program[ip + 1];
                let loc2 = program[ip + 2];
                let loc3 = program[ip + 3];
                program[loc3] = program[loc1] * program[loc2];
                ip += 4;
            },
            99 => break,
            _ => panic!("Invalid program"),
        }
    }

    program[0]
}


fn main() {
    let program: Vec<_> = include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    println!("Output for 1202: {}", run_program(&mut program.clone(), 12, 2));

    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(&mut program.clone(), noun, verb) == 19690720 {
                println!("Found input: {}", noun * 100 + verb);
                return;
            }
        }
    }
}
