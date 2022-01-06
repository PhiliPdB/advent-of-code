use std::cmp::Ordering;
use std::iter;
use std::ops::{Index, IndexMut};



#[derive(Debug, Clone)]
pub struct Program {
    program: Vec<i64>,
    memory: Vec<i64>,
    relative_base: i64,

    pub block_tiles: i32,
    paddle_location: (i64, i64),
    ball_location: (i64, i64),
}

impl Program {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            program,
            memory: Vec::new(),
            relative_base: 0,

            block_tiles: 0,
            paddle_location: (0, 0),
            ball_location: (0, 0),
        }
    }

    pub fn run(&mut self) -> i64 {
        let mut output_instr = 0;
        let mut out_x = 0;
        let mut out_y = 0;

        let mut score = 0;

        // Set instruction pointer
        let mut ip = 0;
        loop {
            let instruction = self[ip] % 100;
            match instruction {
                1 => {
                    let [val1, val2] = self.get_values(ip);
                    let loc3 = self.get_location((self[ip] / 10_000) % 10, ip + 3);

                    self[loc3] = val1 + val2;
                    ip += 4;
                },
                2 => {
                    let [val1, val2] = self.get_values(ip);
                    let loc3 = self.get_location((self[ip] / 10_000) % 10, ip + 3);

                    self[loc3] = val1 * val2;
                    ip += 4;
                },
                3 => {
                    let loc1 = self.get_location((self[ip] / 100) % 10, ip + 1);

                    // Tilt joystick based on where the paddle is compared to the ball
                    self[loc1] =
                        match self.paddle_location.0.cmp(&self.ball_location.0) {
                            Ordering::Less    => 1,
                            Ordering::Equal   => 0,
                            Ordering::Greater => -1,
                        };

                    ip += 2;
                },
                4 => {
                    let [val1] = self.get_values(ip);

                    match output_instr {
                        0 => {
                            out_x = val1;

                            output_instr = 1;
                        },
                        1 => {
                            out_y = val1;

                            output_instr = 2;
                        },
                        2 => {
                            if out_x == -1 && out_y == 0 {
                                score = val1;
                            } else {
                                match val1 {
                                    2 => self.block_tiles += 1,
                                    3 => self.paddle_location = (out_x, out_y),
                                    4 => self.ball_location = (out_x, out_y),
                                    _ => (),
                                }
                            }

                            output_instr = 0;
                        },
                        _ => unreachable!(),
                    }

                    ip += 2;
                },
                5 => {
                    let [val1, val2] = self.get_values(ip);

                    if val1 != 0 {
                        ip = val2 as usize;
                    } else {
                        ip += 3;
                    }
                },
                6 => {
                    let [val1, val2] = self.get_values(ip);

                    if val1 == 0 {
                        ip = val2 as usize;
                    } else {
                        ip += 3;
                    }
                },
                7 => {
                    let [val1, val2] = self.get_values(ip);
                    let loc3 = self.get_location((self[ip] / 10_000) % 10, ip + 3);

                    if val1 < val2 {
                        self[loc3] = 1;
                    } else {
                        self[loc3] = 0;
                    }
                    ip += 4;
                },
                8 => {
                    let [val1, val2] = self.get_values(ip);
                    let loc3 = self.get_location((self[ip] / 10_000) % 10, ip + 3);

                    if val1 == val2 {
                        self[loc3] = 1;
                    } else {
                        self[loc3] = 0;
                    }
                    ip += 4;
                },
                9 => {
                    let [val1] = self.get_values(ip);
                    self.relative_base += val1;

                    ip += 2;
                },
                99 => break,
                _ => panic!("Invalid program"),
            }
        }

        score
    }

    fn get_values<const N: usize>(&self, ip: usize) -> [i64; N] {
        let instruction = self[ip];

        let mut output = [0; N];
        for (i, item) in output.iter_mut().enumerate() {
            let mode = (instruction / 10_i64.pow(i as u32 + 2)) % 10;
            *item = self.get_value(mode, ip + i + 1)
        }

        output
    }

    fn get_value(&self, mode: i64, location: usize) -> i64 {
        match mode {
            0 => self[self[location] as usize],
            1 => self[location],
            2 => self[(self[location] + self.relative_base) as usize],
            _ => unreachable!(),
        }
    }

    fn get_location(&self, mode: i64, location: usize) -> usize {
        match mode {
            0 => self[location] as usize,
            1 => panic!("Invalid mode for location retrieval"),
            2 => (self[location] + self.relative_base) as usize,
            _ => unreachable!(),
        }
    }
}

impl Index<usize> for Program {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.program.len() {
            &self.program[index]
        } else if index - self.program.len() < self.memory.len() {
            &self.memory[index - self.program.len()]
        } else {
            &0
        }
    }
}

impl IndexMut<usize> for Program {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < self.program.len() {
            &mut self.program[index]
        } else if index - self.program.len() < self.memory.len() {
            &mut self.memory[index - self.program.len()]
        } else {
            let grow_by = index - self.program.len() - self.memory.len() + 1;
            self.memory.extend(iter::repeat(0).take(grow_by));

            &mut self.memory[index - self.program.len()]
        }
    }
}
