use std::collections::{HashMap, hash_map::Entry};


#[derive(Debug, Clone, Copy)]
enum Operator {
    Add, Subtract, Multiply, Divide,
}

#[derive(Debug, Clone)]
enum Job {
    Operation(String, Operator, String),
    Number(i64),
}

impl Job {
    fn get_parts(&self) -> (&str, &str) {
        match self {
            Job::Operation(m1, _, m2) => (m1, m2),
            Job::Number(_) => panic!(),
        }
    }
}

fn monkey_value(monkey: &str, monkeys: &HashMap<String, Job>, memory: &mut HashMap<String, i64>) -> i64 {
    if let Entry::Occupied(e) = memory.entry(monkey.to_owned()) {
        return *e.get();
    }

    match &monkeys[monkey] {
        Job::Operation(m1, op, m2) => {
            let m1_val = monkey_value(m1, monkeys, memory);
            let m2_val = monkey_value(m2, monkeys, memory);

            let result = match op {
                Operator::Add => m1_val + m2_val,
                Operator::Subtract => m1_val - m2_val,
                Operator::Multiply => m1_val * m2_val,
                Operator::Divide => m1_val / m2_val,
            };

            memory.insert(monkey.to_owned(), result);
            result
        },
        Job::Number(n) => {
            memory.insert(monkey.to_owned(), *n);
            *n
        },
    }
}

fn has_human(monkey: &str, monkeys: &HashMap<String, Job>) -> bool {
    if monkey == "humn" {
        return true;
    }

    match &monkeys[monkey] {
        Job::Operation(m1, _, m2) => {
            has_human(m1, monkeys) || has_human(m2, monkeys)
        },
        Job::Number(_) => false,
    }
}

fn require_value(monkeys: &HashMap<String, Job>, memory: &HashMap<String, i64>) -> i64 {
    let (m1, m2) = monkeys["root"].get_parts();
    let (mut current, mut goal) =
        if has_human(m1, &monkeys) {
            (m1, memory[m2])
        } else {
            (m2, memory[m1])
        };
    while current != "humn" {
        let Job::Operation(p1, op, p2) = &monkeys[current] else {
            panic!();
        };
        if has_human(p1, monkeys) {
            goal = match op {
                Operator::Add => goal - memory[p2],
                Operator::Subtract => goal + memory[p2],
                Operator::Multiply => goal / memory[p2],
                Operator::Divide => goal * memory[p2],
            };
            current = p1;
        } else {
            goal = match op {
                Operator::Add => goal - memory[p1],
                Operator::Subtract => memory[p1] - goal,
                Operator::Multiply => goal / memory[p1],
                Operator::Divide => memory[p1] / goal,
            };
            current = p2;
        }
    }

    goal
}

fn main() {
    let monkeys: HashMap<_, _> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (name, job) = l.split_once(": ").unwrap();
            let name = name.to_owned();

            if let Ok(n) = job.parse() {
                (name, Job::Number(n))
            } else {
                let parts: Vec<_> = job.split(' ').collect();
                let operator = match parts[1] {
                    "+" => Operator::Add,
                    "-" => Operator::Subtract,
                    "*" => Operator::Multiply,
                    "/" => Operator::Divide,
                    _ => unimplemented!(),
                };

                (name, Job::Operation(parts[0].to_owned(), operator, parts[2].to_owned()))
            }
        })
        .collect();

    let mut memory = HashMap::with_capacity(monkeys.len());
    println!("[Part 1] Root yells: {}", monkey_value("root", &monkeys, &mut memory));

    println!("[Part 2] I yell: {}", require_value(&monkeys, &memory));
}
