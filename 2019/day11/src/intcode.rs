use std::collections::HashMap;
use std::iter;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    pub fn rotate_left(&mut self) {
        *self = match self {
            Facing::North => Facing::West,
            Facing::East  => Facing::North,
            Facing::South => Facing::East,
            Facing::West  => Facing::South,
        }
    }

    pub fn rotate_right(&mut self) {
        *self = match self {
            Facing::North => Facing::East,
            Facing::East  => Facing::South,
            Facing::South => Facing::West,
            Facing::West  => Facing::North,
        }
    }
}


#[derive(Debug, Clone)]
pub struct Program {
    program: Vec<i64>,
    memory: Vec<i64>,
    relative_base: i64,

    position: (i32, i32),
    facing: Facing,
    pub canvas: HashMap<(i32, i32), i64>,
}

impl Program {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            program,
            memory: Vec::new(),
            relative_base: 0,

            position: (0, 0),
            facing: Facing::North,
            canvas: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let mut output_instr = 0;

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

                    self[loc1] = *self.canvas.get(&self.position).unwrap_or(&0);
                    ip += 2;
                },
                4 => {
                    let [val1] = self.get_values(ip);

                    match output_instr {
                        0 => {
                            // Getting the color
                            self.canvas.insert(self.position, val1);
                            output_instr = 1;
                        },
                        1 => {
                            // Getting the rotate instruction
                            match val1 {
                                0 => self.facing.rotate_left(),
                                1 => self.facing.rotate_right(),
                                _ => unreachable!(),
                            }

                            // Move forward
                            match self.facing {
                                Facing::North => self.position.1 -= 1,
                                Facing::East  => self.position.0 += 1,
                                Facing::South => self.position.1 += 1,
                                Facing::West  => self.position.0 -= 1,
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
    }

    fn get_values<const N: usize>(&self, ip: usize) -> [i64; N] {
        let instruction = self[ip];

        let mut output = [0; N];
        for i in 0..N {
            let mode = (instruction / 10_i64.pow(i as u32 + 2)) % 10;
            output[i] = self.get_value(mode, ip + i + 1)
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
