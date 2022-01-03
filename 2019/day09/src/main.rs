use std::{ops::{Index, IndexMut}, collections::HashMap};


#[derive(Debug, Clone)]
pub struct Program {
    program: Vec<i64>,
    memory: HashMap<usize, i64>,
}

impl Program {
    pub fn new(program: Vec<i64>) -> Self {
        Self { program, memory: HashMap::new() }
    }
}

impl Index<usize> for Program {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.program.len() {
            &self.program[index]
        } else {
            &self.memory[&index]
        }
    }
}

impl IndexMut<usize> for Program {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < self.program.len() {
            &mut self.program[index]
        } else {
            self.memory.entry(index).or_insert(0)
        }
    }
}


fn get_value(program: &Program, mode: i64, location: usize, relative_base: i64) -> i64 {
    match mode {
        0 => program[program[location] as usize],
        1 => program[location],
        2 => program[(program[location] + relative_base) as usize],
        _ => unreachable!(),
    }
}


fn run_program(program: &mut Program, input: i64) -> i64 {
    let mut output = 0;

    // Set instruction pointer
    let mut ip = 0;
    let mut relative_base = 0;
    loop {
        let instruction = ((program[ip] / 10) % 10) * 10 + program[ip] % 10;

        match instruction {
            1 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1, relative_base);
                let val2 = get_value(program, (program[ip] / 1_000) % 10, ip + 2, relative_base);
                let mut loc3 = program[ip + 3];
                if (program[ip] / 10_000) % 10 == 2 {
                    loc3 += relative_base;
                }

                program[loc3 as usize] = val1 + val2;
                ip += 4;
            },
            2 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1, relative_base);
                let val2 = get_value(program, (program[ip] / 1_000) % 10, ip + 2, relative_base);
                let mut loc3 = program[ip + 3];
                if (program[ip] / 10_000) % 10 == 2 {
                    loc3 += relative_base;
                }

                program[loc3 as usize] = val1 * val2;
                ip += 4;
            },
            3 => {
                let mut loc1 = program[ip + 1];
                if (program[ip] / 100) % 10 == 2 {
                    loc1 += relative_base;
                }

                program[loc1 as usize] = input;
                ip += 2;
            },
            4 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1, relative_base);

                output = val1;
                ip += 2;
            },
            5 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1, relative_base);
                let val2 = get_value(program, (program[ip] / 1_000) % 10, ip + 2, relative_base);

                if val1 != 0 {
                    ip = val2 as usize;
                } else {
                    ip += 3;
                }
            },
            6 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1, relative_base);
                let val2 = get_value(program, (program[ip] / 1_000) % 10, ip + 2, relative_base);

                if val1 == 0 {
                    ip = val2 as usize;
                } else {
                    ip += 3;
                }
            },
            7 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1, relative_base);
                let val2 = get_value(program, (program[ip] / 1_000) % 10, ip + 2, relative_base);
                let mut loc3 = program[ip + 3];
                if (program[ip] / 10_000) % 10 == 2 {
                    loc3 += relative_base;
                }

                if val1 < val2 {
                    program[loc3 as usize] = 1;
                } else {
                    program[loc3 as usize] = 0;
                }
                ip += 4;
            },
            8 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1, relative_base);
                let val2 = get_value(program, (program[ip] / 1_000) % 10, ip + 2, relative_base);
                let mut loc3 = program[ip + 3];
                if (program[ip] / 10_000) % 10 == 2 {
                    loc3 += relative_base;
                }

                if val1 == val2 {
                    program[loc3 as usize] = 1;
                } else {
                    program[loc3 as usize] = 0;
                }
                ip += 4;
            },
            9 => {
                let val1 = get_value(program, (program[ip] / 100) % 10, ip + 1, relative_base);
                relative_base += val1;

                ip += 2;
            },
            99 => break,
            _ => panic!("Invalid program"),
        }
    }

    output
}

fn main() {
    let program = Program::new(
        include_str!("../input.txt")
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    );


    println!("BOOST keycode: {}", run_program(&mut program.clone(), 1));

    println!("Coordinates of distress signal: {}", run_program(&mut program.clone(), 2));
}
