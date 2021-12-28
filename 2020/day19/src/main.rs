use std::iter::repeat;
use std::str::FromStr;

use hashbrown::HashMap;


#[derive(Debug)]
pub enum Rule {
    Character(char),
    Single(Vec<i32>),
    Or(Vec<i32>, Vec<i32>),
}

#[derive(Debug)]
pub struct Rules {
    map: HashMap<i32, Rule>,
}

impl Rules {
    pub fn matches<'a>(&self, rule: i32, input: &'a str) -> Option<&'a str> {
        if input.is_empty() {
            return None;
        }

        match &self.map[&rule] {
            Rule::Character(c) => {
                if input.chars().nth(0).unwrap() == *c {
                    Some(&input[1..])
                } else {
                    None
                }
            },
            Rule::Single(rules) => self.match_chain(rules, input),
            Rule::Or(rule1, rule2) => {
                let rule1_result = self.match_chain(rule1, input);

                rule1_result.or_else(|| self.match_chain(rule2, input))
            },
        }
    }

    #[inline]
    fn match_chain<'a>(&self, rules: &[i32], input: &'a str) -> Option<&'a str> {
        rules.iter().fold(Some(input), |acc, r| {
            if let Some(i) = acc {
                self.matches(*r, i)
            } else {
                None
            }
        })
    }
}

impl FromStr for Rules {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules: Vec<_> = s.lines().collect();
        let mut map = HashMap::with_capacity(rules.len());

        for rule in rules {
            let (id, rule) = rule.split_once(": ").ok_or("Invalid rule can't find ': '")?;
            let id = id.parse().map_err(|_| "Can't parse id")?;

            let rule =
                if rule.starts_with("\"") {
                    Rule::Character(rule.chars().nth(1).unwrap())
                } else if rule.contains("|") {
                    let (rule1, rule2) = rule.split_once(" | ").unwrap();

                    Rule::Or(
                        rule1.split(" ").map(|n| n.parse().unwrap()).collect(),
                        rule2.split(" ").map(|n| n.parse().unwrap()).collect()
                    )
                } else {
                    Rule::Single(rule.split(" ").map(|n| n.parse().unwrap()).collect())
                };

            map.insert(id, rule);
        }

        Ok(Self { map })
    }
}

fn main() {
    let (rules, messages) = include_str!("../input.txt")
        .split_once("\n\n")
        .unwrap();

    let mut rules = Rules::from_str(rules).unwrap();
    let messages: Vec<_> = messages.lines().collect();

    // Part 1

    let part1_valid_messages = messages.iter()
        .filter(|m| {
            if let Some(rest) = rules.matches(0, m) {
                rest.is_empty()
            } else {
                false
            }
        })
        .count();

    println!("[Part 1] Rules matched: {}", part1_valid_messages);

    // Part 2
    // For part 2, rules 8 and 11 are only used in rule 0.
    // This means that for a message to be valid, we first need to match rule 42 i times,
    // then rule 31 has to be matched j times with 1 <= j < i.

    let part2_valid_messages = messages.iter()
        .filter(|m| {
            for i in 1.. {
                // Update rule 8
                rules.map.insert(8, Rule::Single(repeat(42).take(i).collect()));
                if let Some(rule8_remaining) = rules.matches(8, m) {
                    for j in 1..i {
                        // Update rule 11
                        rules.map.insert(11, Rule::Single(repeat(31).take(j).collect()));
                        if let Some(rule11_remaining) = rules.matches(11, rule8_remaining) {
                            if rule11_remaining.is_empty() {
                                return true;
                            }
                        }
                    }
                } else {
                    return false;
                }
            }

            unreachable!()
        })
        .count();

    println!("[Part 2] Rules matched: {}", part2_valid_messages);
}
