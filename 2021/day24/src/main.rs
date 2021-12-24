use std::str::FromStr;

use hashbrown::HashMap;
use itertools::Either;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Var {
    W, X, Y, Z,
    Num(i32),
}

impl Var {
    pub const fn get_memory_index(&self) -> usize {
        match self {
            Var::W => 0,
            Var::X => 1,
            Var::Y => 2,
            Var::Z => 3,
            Var::Num(_) => panic!("Not a variable"),
        }
    }
}

impl FromStr for Var {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Var::W),
            "x" => Ok(Var::X),
            "y" => Ok(Var::Y),
            "z" => Ok(Var::Z),
            _   => Ok(Var::Num(s.parse::<i32>().map_err(|_| "Cannot parse number")?))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Input(Var),
    Add(Var, Var),
    Mul(Var, Var),
    Div(Var, Var),
    Mod(Var, Var),
    Eql(Var, Var),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<_> = s.split(' ').collect();

        match splitted[0] {
            "inp" => Ok(Instruction::Input(Var::from_str(splitted[1])?)),
            "add" => Ok(Instruction::Add(Var::from_str(splitted[1])?, Var::from_str(splitted[2])?)),
            "mul" => Ok(Instruction::Mul(Var::from_str(splitted[1])?, Var::from_str(splitted[2])?)),
            "div" => Ok(Instruction::Div(Var::from_str(splitted[1])?, Var::from_str(splitted[2])?)),
            "mod" => Ok(Instruction::Mod(Var::from_str(splitted[1])?, Var::from_str(splitted[2])?)),
            "eql" => Ok(Instruction::Eql(Var::from_str(splitted[1])?, Var::from_str(splitted[2])?)),
            _ => Err("Invalid instruction"),
        }
    }
}

const fn get_value(var: &Var, mem: &[i32; 4]) -> i32 {
    match var {
        Var::Num(n) => *n,
        _ => mem[var.get_memory_index()]
    }
}

fn execute(instructions: &[Instruction], memory_init: [i32; 4]) -> i32 {
    let mut memory = memory_init;

    for instruction in instructions {
        match instruction {
            Instruction::Input(_) => unimplemented!(),
            Instruction::Add(x, y) => {
                memory[x.get_memory_index()] += get_value(y, &memory);
            },
            Instruction::Mul(x, y) => {
                memory[x.get_memory_index()] *= get_value(y, &memory);
            },
            Instruction::Div(x, y) => {
                memory[x.get_memory_index()] /= get_value(y, &memory);
            },
            Instruction::Mod(x, y) => {
                memory[x.get_memory_index()] %= get_value(y, &memory);
            },
            Instruction::Eql(x, y) => {
                memory[x.get_memory_index()] =
                    if memory[x.get_memory_index()] == get_value(y, &memory) {
                        1
                    } else {
                        0
                    };
            },
        }
    }

    memory[Var::Z.get_memory_index()]
}



fn digit_range(reversed: bool) -> Either<impl Iterator<Item = i32>, impl Iterator<Item = i32>> {
    if reversed {
        Either::Left((1..10).rev())
    } else {
        Either::Right(1..10)
    }
}

fn find_number(mem: &mut [HashMap<i32, Option<i64>>], instructions: &[&[Instruction]], start_z: i32, index: usize, highest: bool) -> Option<i64> {
    if index >= 14 {
        if start_z == 0 {
            return Some(0);
        } else {
            return None;
        }
    }

    if let Some(sol) = mem[index].get(&start_z) {
        return *sol;
    }


    for digit in digit_range(highest) {
        let z = execute(instructions[index], [digit, 0, 0, start_z]);
        if let Some(mut solution) = find_number(mem, instructions, z, index + 1, highest) {
            solution += digit as i64 * 10_i64.pow(13 - index as u32);

            mem[index].insert(start_z, Some(solution));
            return Some(solution);
        }
    }

    mem[index].insert(start_z, None);
    None
}

fn main() {
    let instructions: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect();

    let per_digit: Vec<_> =
        instructions.split(|i| {
            match i {
                Instruction::Input(_) => true,
                _ => false,
            }
        })
        .skip(1)
        .collect();

    debug_assert_eq!(per_digit.len(), 14);

    let mut memoization_table = [
        HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(),
        HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(),
        HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(),
        HashMap::new(), HashMap::new(),
    ];
    println!("Highest possible MONAD: {}", find_number(&mut memoization_table, &per_digit, 0, 0, true).unwrap());

    // Clear memory to start part 2
    for hm in memoization_table.iter_mut() {
        hm.clear();
    }

    println!("Lowest possible MONAD:  {}", find_number(&mut memoization_table, &per_digit, 0, 0, false).unwrap());
}
