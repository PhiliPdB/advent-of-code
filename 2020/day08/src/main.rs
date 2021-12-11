use std::str::FromStr;
use std::collections::HashSet;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(Operation::Acc),
            "jmp" => Ok(Operation::Jmp),
            "nop" => Ok(Operation::Nop),
            _     => Err("Invalid operation"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    operation: Operation,
    argument: i32,
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();

        let operation = Operation::from_str(parts[0])?;
        let argument = parts[1].parse::<i32>().map_err(|_| "Invalid arg")?;

        Ok(Self { operation, argument })
    }
}

fn eval(instructions: &[Instruction], acc: i32, sp: usize, visited_instructions: &HashSet<usize>, allow_branch: bool) -> (i32, usize) {
    let mut acc = acc;
    let mut sp = sp;
    let mut newly_visited_instructions = visited_instructions.clone();
    loop {
        if !newly_visited_instructions.insert(sp) || sp == instructions.len() {
            break;
        };

        let instruction = instructions[sp];
        match instruction.operation {
            Operation::Acc => acc += instruction.argument,
            Operation::Jmp => {
                if allow_branch {
                    let (nop_acc, nop_sp) = eval(instructions, acc, sp + 1, &newly_visited_instructions, false);
                    let (jmp_acc, jmp_sp) = eval(instructions, acc, sp + instruction.argument as usize, &newly_visited_instructions, true);

                    if nop_sp == instructions.len() {
                        return (nop_acc, nop_sp);
                    } else {
                        return (jmp_acc, jmp_sp);
                    }
                } else {
                    sp += instruction.argument as usize;
                    continue;
                }
            },
            Operation::Nop => {
                if allow_branch {
                    let (nop_acc, nop_sp) = eval(instructions, acc, sp + 1, &newly_visited_instructions, true);
                    let (jmp_acc, jmp_sp) = eval(instructions, acc, sp + instruction.argument as usize, &newly_visited_instructions, false);

                    if jmp_sp == instructions.len() {
                        return (jmp_acc, jmp_sp);
                    } else {
                        return (nop_acc, nop_sp);
                    }
                }
            },
        };

        sp += 1;
    }

    (acc, sp)
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| Instruction::from_str(s).unwrap())
        .collect();

    let (part1_acc, _) = eval(&input, 0, 0, &HashSet::with_capacity(input.len()), false);
    let (part2_acc, _) = eval(&input, 0, 0, &HashSet::with_capacity(input.len()), true);

    println!("[Part 1] Accumulator value: {:#4}", part1_acc);
    println!("[Part 2] Accumulator value: {:#4}", part2_acc);
}
