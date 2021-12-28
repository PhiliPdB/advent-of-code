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
        match &self.map[&rule] {
            Rule::Character(c) => {
                if input.chars().nth(0).unwrap() == *c {
                    Some(&input[1..])
                } else {
                    None
                }
            },
            Rule::Single(rules) => {
                rules.iter().fold(Some(input), |acc, r| {
                    if let Some(i) = acc {
                        self.matches(*r, i)
                    } else {
                        None
                    }
                })
            },
            Rule::Or(rule1, rule2) => {
                let rule1_result = rule1.iter().fold(Some(input), |acc, r| {
                    if let Some(i) = acc {
                        self.matches(*r, i)
                    } else {
                        None
                    }
                });

                rule1_result.or_else(|| {
                    rule2.iter().fold(Some(input), |acc, r| {
                        if let Some(i) = acc {
                            self.matches(*r, i)
                        } else {
                            None
                        }
                    })
                })
            },
        }
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

    let rules = Rules::from_str(rules).unwrap();
    let messages: Vec<_> = messages.lines().collect();

    // println!("{:?}", rules);

    let valid_messages = messages.iter()
        .filter(|m| {
            if let Some(rest) = rules.matches(0, m) {
                rest.is_empty()
            } else {
                false
            }
        })
        .count();

    println!("Rules matched: {}", valid_messages);
}
