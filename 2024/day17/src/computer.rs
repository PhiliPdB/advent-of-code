
pub struct Computer {
    registers: [i64; 3],
    program: Vec<u8>,
}

impl Computer {
    pub fn new(registers: [i64; 3], program: Vec<u8>) -> Self {
        Computer {
            registers,
            program,
        }
    }

    pub fn run(&mut self) -> Vec<i64> {
        let mut output = Vec::new();
        let mut instruction_pointer = 0;

        loop {
            if instruction_pointer >= self.program.len() {
                // Halt the program
                break;
            }

            match self.program[instruction_pointer] {
                0 => { // adv instruction
                    let operand_value = self.get_combo_value(self.program[instruction_pointer + 1]);
                    let result = self.registers[0] / 2_i64.pow(operand_value as u32);
                    self.registers[0] = result;
                },
                1 => { // bxl instruction
                    self.registers[1] ^= self.program[instruction_pointer + 1] as i64;
                }
                2 => { // bst instruction
                    let operand_value = self.get_combo_value(self.program[instruction_pointer + 1]) % 8;
                    self.registers[1] = operand_value;
                }
                3 => { // jnz instruction
                    if self.registers[0] != 0 {
                        instruction_pointer = self.program[instruction_pointer + 1] as usize;
                        continue;
                    }
                },
                4 => { // bxc instruction
                    self.registers[1] ^= self.registers[2];
                },
                5 => { // out instruction
                    let operand_value = self.get_combo_value(self.program[instruction_pointer + 1]) % 8;
                    output.push(operand_value);
                },
                6 => { // bdv
                    let operand_value = self.get_combo_value(self.program[instruction_pointer + 1]);
                    let result = self.registers[0] / 2_i64.pow(operand_value as u32);
                    self.registers[1] = result;
                },
                7 => { // cdv
                    let operand_value = self.get_combo_value(self.program[instruction_pointer + 1]);
                    let result = self.registers[0] / 2_i64.pow(operand_value as u32);
                    self.registers[2] = result;
                },
                _ => panic!("Invalid opcode"),
            }

            instruction_pointer += 2;
        }

        output
    }

    fn get_combo_value(&self, combo_operand: u8) -> i64 {
        match combo_operand {
            1..=3 => combo_operand as i64,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            7 => unreachable!(),
            _ => panic!("Invalid combo operand"),
        }
    }
}
