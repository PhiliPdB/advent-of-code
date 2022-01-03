
fn get_value(program: &[i32], mode: i32, location: usize) -> i32 {
    if mode == 0 {
        program[program[location] as usize]
    } else {
        program[location]
    }
}

fn run_program(program: &mut [i32], input: i32) -> i32 {
    let mut output = 0;

    // Set instruction pointer
    let mut ip = 0;
    loop {
        let instruction = ((program[ip] / 10) % 10) * 10 + program[ip] % 10;

        match instruction {
            1 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1);
                let val2 = get_value(program, (program[ip] / 1_000) % 10, ip + 2);
                let loc3 = program[ip + 3];

                program[loc3 as usize] = val1 + val2;
                ip += 4;
            },
            2 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1);
                let val2 = get_value(program, (program[ip] / 1_000) % 10, ip + 2);
                let loc3 = program[ip + 3];

                program[loc3 as usize] = val1 * val2;
                ip += 4;
            },
            3 => {
                let loc1 = program[ip + 1];
                program[loc1 as usize] = input;
                ip += 2;
            },
            4 => {
                let loc1 = program[ip + 1];
                let val1 =
                    if (program[ip] / 100) % 10 == 0 {
                        program[loc1 as usize]
                    } else {
                        loc1
                    };

                output = val1;
                ip += 2;
            },
            5 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1);
                let val2 = get_value(program, (program[ip] / 1_000) % 10, ip + 2);

                if val1 != 0 {
                    ip = val2 as usize;
                } else {
                    ip += 3;
                }
            },
            6 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1);
                let val2 = get_value(program, (program[ip] / 1_000) % 10, ip + 2);

                if val1 == 0 {
                    ip = val2 as usize;
                } else {
                    ip += 3;
                }
            },
            7 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1);
                let val2 = get_value(program, (program[ip] / 1_000) % 10, ip + 2);
                let loc3 = program[ip + 3];

                if val1 < val2 {
                    program[loc3 as usize] = 1;
                } else {
                    program[loc3 as usize] = 0;
                }
                ip += 4;
            },
            8 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1);
                let val2 = get_value(program, (program[ip] / 1_000) % 10, ip + 2);
                let loc3 = program[ip + 3];

                if val1 == val2 {
                    program[loc3 as usize] = 1;
                } else {
                    program[loc3 as usize] = 0;
                }
                ip += 4;
            },
            99 => break,
            _ => panic!("Invalid program"),
        }
    }

    output
}

fn main() {
    let program: Vec<_> = include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("[Part 1] Output: {:#8}", run_program(&mut program.clone(), 1));
    println!("[Part 2] Output: {:#8}", run_program(&mut program.clone(), 5));
}
