use std::slice;

use num::Integer;


#[derive(Debug, Clone, Copy)]
enum Var {
    Old, Value(u64)
}

impl Var {
    fn value(&self, old_val: u64) -> u64 {
        match self {
            Var::Old => old_val,
            Var::Value(v) => *v,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Var, Var),
    Mul(Var, Var),
}

impl Operation {
    fn execute(&self, old_val: u64) -> u64 {
        match self {
            Operation::Add(v1, v2) => v1.value(old_val) + v2.value(old_val),
            Operation::Mul(v1, v2) => v1.value(old_val) * v2.value(old_val),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    if_true: u32,
    if_false: u32,
}

impl Monkey {
    fn new(items: Vec<u64>, operation: Operation, test: u64, if_true: u32, if_false: u32) -> Self {
        Self { items, operation, test, if_true, if_false }
    }
}

fn main() {
    let input_monkeys: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .map(|m| {
            let lines: Vec<_> = m.lines().collect();
            let items: Vec<u64> = lines[1][18..].split(", ")
                .map(|v| v.parse().unwrap())
                .collect();

            let op_parts: Vec<_> = lines[2].split(' ').collect();
            let var1 = match op_parts[5] {
                "old" => Var::Old,
                v => Var::Value(v.parse().unwrap()),
            };
            let var2 = match op_parts[7] {
                "old" => Var::Old,
                v => Var::Value(v.parse().unwrap()),
            };
            let operation = match op_parts[6] {
                "+" => Operation::Add(var1, var2),
                "*" => Operation::Mul(var1, var2),
                _ => panic!(),
            };

            let test = lines[3].split(' ').last().unwrap().parse().unwrap();
            let if_true = lines[4].split(' ').last().unwrap().parse().unwrap();
            let if_false = lines[5].split(' ').last().unwrap().parse().unwrap();

            Monkey::new(items, operation, test, if_true, if_false)
        })
        .collect();


    let mut monkeys = input_monkeys.clone();
    let mut monkey_inspections = vec![0_u64; input_monkeys.len()];
    let ptr = monkeys.as_mut_ptr();
    for _round in 0..20 {
        for m in 0..monkeys.len() {
            for i in &monkeys[m].items {
                let new_item = monkeys[m].operation.execute(*i) / 3;
                if new_item % monkeys[m].test == 0 {
                    let new_monkey = unsafe {
                        &mut slice::from_raw_parts_mut(ptr.add(monkeys[m].if_true as usize), 1)[0]
                    };
                    new_monkey.items.push(new_item);
                } else {
                    let new_monkey = unsafe {
                        &mut slice::from_raw_parts_mut(ptr.add(monkeys[m].if_false as usize), 1)[0]
                    };
                    new_monkey.items.push(new_item);
                }

                monkey_inspections[m] += 1;
            }
            monkeys[m].items.clear();
        }
    }

    monkey_inspections.sort_by(|a, b| b.cmp(a));
    println!("[Part 1] Monkey business: {}", monkey_inspections[0] * monkey_inspections[1]);

    monkey_inspections = vec![0_u64; input_monkeys.len()];
    monkeys = input_monkeys.clone();
    let tests: Vec<_> = monkeys.iter().map(|m| m.test).collect();
    let mut lcm = tests[0];
    for i in 1..tests.len() {
        lcm = lcm.lcm(&tests[i]);
    }

    let ptr = monkeys.as_mut_ptr();
    for _round in 0..10_000 {
        for m in 0..monkeys.len() {
            for i in &monkeys[m].items {
                let new_item = monkeys[m].operation.execute(*i) % lcm;
                if new_item % monkeys[m].test == 0 {
                    let new_monkey = unsafe {
                        &mut slice::from_raw_parts_mut(ptr.add(monkeys[m].if_true as usize), 1)[0]
                    };
                    new_monkey.items.push(new_item);
                } else {
                    let new_monkey = unsafe {
                        &mut slice::from_raw_parts_mut(ptr.add(monkeys[m].if_false as usize), 1)[0]
                    };
                    new_monkey.items.push(new_item);
                }

                monkey_inspections[m] += 1;
            }
            monkeys[m].items.clear();
        }
    }

    monkey_inspections.sort_by(|a, b| b.cmp(a));
    println!("[Part 2] Monkey business: {}", monkey_inspections[0] * monkey_inspections[1]);
}
