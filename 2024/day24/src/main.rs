use std::str::FromStr;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    And, Or, Xor
}

impl FromStr for Operator {
    type Err = &'static str;

    fn from_str(operator: &str) -> Result<Self, Self::Err> {
        match operator {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err("Unknown operator"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Rule<'a> {
    left_input: &'a str,
    right_input: &'a str,
    operator: Operator,
    output: &'a str,
}

impl<'a> Rule<'a> {
    fn evaluate(&self, variables: &HashMap<&str, bool>) -> Option<bool> {
        if !variables.contains_key(self.left_input) || !variables.contains_key(self.right_input) {
            return None;
        }

        let lhs1 = variables[self.left_input];
        let lhs2 = variables[self.right_input];

        Some(match self.operator {
            Operator::And => lhs1 && lhs2,
            Operator::Or => lhs1 || lhs2,
            Operator::Xor => lhs1 ^ lhs2,
        })
    }

    fn has_input(&self, variable: &str) -> bool {
        self.left_input == variable || self.right_input == variable
    }

    fn parse(s: &'a str) -> Self {
        let (lhs, output) = s.split_once(" -> ").unwrap();
        let (left_input, operator, right_input) = lhs.split_whitespace().collect_tuple().unwrap();
        let operator = Operator::from_str(operator).unwrap();

        Rule {
            left_input, right_input,
            operator,
            output,
        }
    }
}

fn apply_rules<'a>(variables: &mut HashMap<&'a str, bool>, mut rules: Vec<Rule<'a>>) -> u64 {
    while !rules.is_empty() {
        rules.retain(|r| {
            if let Some(result) = r.evaluate(variables) {
                variables.insert(r.output, result);
                false
            } else {
                true
            }
        });
    }

    get_variable(variables, "z")
}

fn get_variable(variables: &HashMap<&str, bool>, variable: &str) -> u64 {
    let mut value = 0;
    for index in 0..64 {
        let var = format!("{variable}{index:02}");
        if let Some(&v) = variables.get(var.as_str()) {
            if v {
                value |= 1 << index;
            }
        }
    }
    value
}

fn main() {
    let (starting_state, rules) = include_str!("../input.txt")
        .split_once("\n\n")
        .unwrap();

    let mut variables: HashMap<&str, bool> = starting_state
        .lines()
        .map(|line| {
            let (variable, value) = line.split_once(": ").unwrap();
            (variable, value == "1")
        })
        .collect();
    let rules: Vec<_> = rules.lines()
        .map(Rule::parse)
        .collect();

    let z = apply_rules(&mut variables, rules.clone());
    println!("[Part 1] Value of z: {z}");


    // Part 2
    // Try to check if the circuit of a single bit represents a full adder
    // If not, mark the incorrect wires to be swapped

    let mut to_swap = HashSet::new();
    let mut carry = rules.iter()
        .find(|r| {
            ((r.left_input == "x00" && r.right_input == "y00") || (r.left_input == "y00" && r.right_input == "x00"))
                && r.operator == Operator::And
        })
        .unwrap()
        .output;
    for i in 1..45 {
        let x = format!("x{i:02}");
        let y = format!("y{i:02}");
        let z = format!("z{i:02}");

        let basic_add = rules.iter()
            .find(|r| {
                ((r.left_input == x && r.right_input == y) || (r.left_input == y && r.right_input == x))
                && r.operator == Operator::Xor
            }).unwrap();

        let add = rules.iter()
            .find(|r| {
                (r.has_input(carry) || r.has_input(basic_add.output))
                && r.operator == Operator::Xor
            }).unwrap();
        if add.output != z {
            to_swap.insert(add.output.to_owned());
            to_swap.insert(z);
        }

        if !add.has_input(carry) {
            to_swap.insert(carry.to_owned());
        }
        if !add.has_input(basic_add.output) {
            to_swap.insert(basic_add.output.to_owned());
        }

        // Check if the carry circuit is correct
        let should_carry = rules.iter()
            .find(|r| {
                ((r.left_input == x && r.right_input == y) || (r.left_input == y && r.right_input == x))
                && r.operator == Operator::And
            }).unwrap();

        let cascade_carry = rules.iter()
            .find(|r| {
                (r.has_input(carry) || r.has_input(basic_add.output))
                && r.operator == Operator::And
            }).unwrap();
        if !cascade_carry.has_input(carry) {
            to_swap.insert(carry.to_owned());
        }
        if !cascade_carry.has_input(basic_add.output) {
            to_swap.insert(basic_add.output.to_owned());
        }


        let carry_or = rules.iter()
            .find(|r| {
                (r.has_input(cascade_carry.output) || r.has_input(should_carry.output))
                && r.operator == Operator::Or
            }).unwrap();
        if !carry_or.has_input(cascade_carry.output) {
            to_swap.insert(cascade_carry.output.to_owned());
        }
        if !carry_or.has_input(should_carry.output) {
            to_swap.insert(should_carry.output.to_owned());
        }

        carry = carry_or.output;
    }

    let mut swapped: Vec<_> = to_swap.into_iter().collect();
    swapped.sort_unstable();
    println!("[Part 2] Wires to swap: {}", swapped.into_iter().join(","));
}
