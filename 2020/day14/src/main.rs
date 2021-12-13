use std::collections::HashMap;

pub enum Instruction {
    Mask(u64, u64, Vec<u32>),
    Memory(u64, u64),
}


fn parse_mask(mask: &str) -> Instruction {
    let mut one_mask = 0_u64;
    let mut zero_mask = !one_mask;
    let mut floating = Vec::new();

    for (i, c) in mask.chars().rev().enumerate() {
        match c {
            '0' => zero_mask &= !(1 << i),
            '1' => one_mask |= 1 << i,
            'X' => floating.push(i as u32),
            _   => panic!("Invalid mask char"),
        }
    }

    Instruction::Mask(zero_mask, one_mask, floating)
}

fn run(instructions: &[Instruction], is_part1: bool) -> u64 {
    let mut memory = HashMap::new();
    let mut current_mask = (!0, 0, &Vec::new());
    for instruction in instructions {
        match instruction {
            Instruction::Mask(zero_mask, one_mask, floating) => {
                current_mask = (*zero_mask, *one_mask, floating);
            },
            Instruction::Memory(loc, val) => {
                if is_part1 {
                    memory.insert(*loc, (*val | current_mask.1) & current_mask.0);
                } else {
                    let locations = generate_floating_numbers(*loc | current_mask.1, current_mask.2);
                    for location in locations {
                        memory.insert(location, *val);
                    }
                }
            },
        }
    }

    memory.values().sum()
}

fn generate_floating_numbers(initial: u64, floating: &[u32]) -> Vec<u64> {
    let mut numbers = Vec::with_capacity(2_usize.pow(floating.len() as u32));
    let mut floating_zero = !0;
    for &i in floating {
        floating_zero &= !(1 << i);
    }
    let initial_value = initial & floating_zero;

    for &index in floating {
        let one_mask  = 1 << index;

        if numbers.is_empty() {
            numbers.push(initial_value);
            numbers.push(initial_value | one_mask);
        } else {
            let new_numbers: Vec<_> = numbers.iter()
                .map(|n| *n | one_mask)
                .collect();
            numbers.extend(new_numbers);
        }
    }

    numbers
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .collect();

    let instructions: Vec<_> = input.iter()
        .map(|s| {
            if s.starts_with("mask") {
                parse_mask(s.replace("mask = ", "").as_str())
            } else {
                let s = s.replace("mem[", "");
                let parts: Vec<_> = s.split("] = ").collect();

                Instruction::Memory(parts[0].parse::<u64>().unwrap(), parts[1].parse::<u64>().unwrap())
            }
        })
        .collect();

    println!("[Part 1] Memory sum: {:#14}", run(&instructions, true));
    println!("[Part 2] Memory sum: {:#14}", run(&instructions, false));
}
