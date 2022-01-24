use std::iter;
use std::ops::{Index, IndexMut};

use crossbeam_channel::{Receiver, Sender};

#[derive(Debug, Clone)]
pub struct Program {
    program: Vec<i64>,
    memory: Vec<i64>,
    relative_base: i64,
    ip: usize,
}

impl Program {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            program,
            memory: Vec::new(),
            relative_base: 0,
            ip: 0,
        }
    }

    pub fn run(&mut self, input: Receiver<i64>, output: Sender<i64>) {
        loop {
            let instruction = self[self.ip] % 100;
            match instruction {
                1 => {
                    let [val1, val2] = self.get_values(self.ip);
                    let loc3 = self.get_location((self[self.ip] / 10_000) % 10, self.ip + 3);

                    self[loc3] = val1 + val2;
                    self.ip += 4;
                },
                2 => {
                    let [val1, val2] = self.get_values(self.ip);
                    let loc3 = self.get_location((self[self.ip] / 10_000) % 10, self.ip + 3);

                    self[loc3] = val1 * val2;
                    self.ip += 4;
                },
                3 => {
                    let loc1 = self.get_location((self[self.ip] / 100) % 10, self.ip + 1);
                    self[loc1] = input.try_recv().unwrap_or(-1);

                    self.ip += 2;
                },
                4 => {
                    let [val1] = self.get_values(self.ip);
                    self.ip += 2;

                    // *g_tries.get_mut() -= tries;
                    // *g_tries.lock().unwrap() -= tries;
                    // tries = 0;
                    output.send(val1).unwrap();
                },
                5 => {
                    let [val1, val2] = self.get_values(self.ip);

                    if val1 != 0 {
                        self.ip = val2 as usize;
                    } else {
                        self.ip += 3;
                    }
                },
                6 => {
                    let [val1, val2] = self.get_values(self.ip);

                    if val1 == 0 {
                        self.ip = val2 as usize;
                    } else {
                        self.ip += 3;
                    }
                },
                7 => {
                    let [val1, val2] = self.get_values(self.ip);
                    let loc3 = self.get_location((self[self.ip] / 10_000) % 10, self.ip + 3);

                    if val1 < val2 {
                        self[loc3] = 1;
                    } else {
                        self[loc3] = 0;
                    }
                    self.ip += 4;
                },
                8 => {
                    let [val1, val2] = self.get_values(self.ip);
                    let loc3 = self.get_location((self[self.ip] / 10_000) % 10, self.ip + 3);

                    if val1 == val2 {
                        self[loc3] = 1;
                    } else {
                        self[loc3] = 0;
                    }
                    self.ip += 4;
                },
                9 => {
                    let [val1] = self.get_values(self.ip);
                    self.relative_base += val1;

                    self.ip += 2;
                },
                99 => break,
                _ => panic!("Invalid program"),
            }
        }
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
