use std::collections::{HashMap, hash_map::Entry};
use std::str::FromStr;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    CoolLooking, Musical, Aerodynamic, Shiny
}

impl Category {
    fn from_char(c: char) -> Self {
        match c {
            'x' => Self::CoolLooking,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => panic!("Invalid category: {c}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rule {
    LessThan(Category, u32, usize),
    GreaterThan(Category, u32, usize),
    Rule(usize),
    Reject, Accept,
}

#[derive(Debug)]
struct Rules {
    rules: Vec<Vec<Rule>>,
    rule_map: HashMap<String, usize>,
}

impl Rules {
    fn accepts_part(&self, part: &Part) -> bool {
        let mut current_rule = self.rule_map["in"];
        loop {
            'rule_steps: for rule in &self.rules[current_rule] {
                match rule {
                    Rule::LessThan(c, n, r) => {
                        if part.rating(*c) < *n {
                            current_rule = *r;
                            break 'rule_steps;
                        }
                    },
                    Rule::GreaterThan(c, n, r) => {
                        if part.rating(*c) > *n {
                            current_rule = *r;
                            break 'rule_steps;
                        }
                    },
                    Rule::Rule(r) => {
                        current_rule = *r;
                        break 'rule_steps;
                    },
                    Rule::Reject => return false,
                    Rule::Accept => return true,
                }
            }
        }
    }

    fn total_accepted_parts(&self) -> u64 {
        let mut total_accepted_parts = 0;

        let mut queue = Vec::new();
        queue.push((self.rule_map["in"], [(1, 4000), (1, 4000), (1, 4000), (1, 4000)]));

        while let Some((current_rule, mut ranges)) = queue.pop() {
            for rule in &self.rules[current_rule] {
                match rule {
                    Rule::LessThan(c, n, r) => {
                        let new_range = match *c {
                            Category::CoolLooking => [(ranges[0].0, ranges[0].1.min(*n - 1)), ranges[1], ranges[2], ranges[3]],
                            Category::Musical     => [ranges[0], (ranges[1].0, ranges[1].1.min(*n - 1)), ranges[2], ranges[3]],
                            Category::Aerodynamic => [ranges[0], ranges[1], (ranges[2].0, ranges[2].1.min(*n - 1)), ranges[3]],
                            Category::Shiny       => [ranges[0], ranges[1], ranges[2], (ranges[3].0, ranges[3].1.min(*n - 1))],
                        };
                        queue.push((*r, new_range));

                        let current_max = ranges[*c as usize].1;
                        if current_max < *n {
                            break;
                        }

                        ranges = match *c {
                            Category::CoolLooking => [(ranges[0].1.min(*n), ranges[0].1), ranges[1], ranges[2], ranges[3]],
                            Category::Musical     => [ranges[0], (ranges[1].1.min(*n), ranges[1].1), ranges[2], ranges[3]],
                            Category::Aerodynamic => [ranges[0], ranges[1], (ranges[2].1.min(*n), ranges[2].1), ranges[3]],
                            Category::Shiny       => [ranges[0], ranges[1], ranges[2], (ranges[3].1.min(*n), ranges[3].1)],
                        };
                    },
                    Rule::GreaterThan(c, n, r) => {
                        let new_range = match *c {
                            Category::CoolLooking => [(ranges[0].0.max(*n + 1), ranges[0].1), ranges[1], ranges[2], ranges[3]],
                            Category::Musical     => [ranges[0], (ranges[1].0.max(*n + 1), ranges[1].1), ranges[2], ranges[3]],
                            Category::Aerodynamic => [ranges[0], ranges[1], (ranges[2].0.max(*n + 1), ranges[2].1), ranges[3]],
                            Category::Shiny       => [ranges[0], ranges[1], ranges[2], (ranges[3].0.max(*n + 1), ranges[3].1)],
                        };
                        queue.push((*r, new_range));

                        let current_min = ranges[*c as usize].0;
                        if current_min > *n {
                            break;
                        }

                        ranges = match *c {
                            Category::CoolLooking => [(ranges[0].0, ranges[0].0.max(*n)), ranges[1], ranges[2], ranges[3]],
                            Category::Musical     => [ranges[0], (ranges[1].0, ranges[1].0.max(*n)), ranges[2], ranges[3]],
                            Category::Aerodynamic => [ranges[0], ranges[1], (ranges[2].0, ranges[2].0.max(*n)), ranges[3]],
                            Category::Shiny       => [ranges[0], ranges[1], ranges[2], (ranges[3].0, ranges[3].0.max(*n))],
                        };
                    },
                    Rule::Rule(r) => {
                        queue.push((*r, ranges));
                    },
                    Rule::Reject => break,
                    Rule::Accept => {
                        total_accepted_parts += ranges.iter()
                            .fold(1, |acc, r| acc * (r.1 - r.0 + 1) as u64)
                    },
                }
            }
        }

        total_accepted_parts
    }
}

impl FromStr for Rules {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = Vec::new();
        rules.push(vec![Rule::Accept]);
        rules.push(vec![Rule::Reject]);

        let mut rule_map = HashMap::new();
        rule_map.insert(String::from("A"), 0);
        rule_map.insert(String::from("R"), 1);

        for rule in s.lines() {
            let [name, rule] = rule.split('{').collect::<Vec<_>>()
                .try_into().map_err(|_| "Invalid rule format")?;

            let rule_index = match rule_map.entry(name.to_owned()) {
                Entry::Occupied(e) => *e.get(),
                Entry::Vacant(e) => {
                    let new_index = rules.len();
                    e.insert(new_index);

                    rules.push(Vec::new());
                    new_index
                },
            };

            for p in rule[..rule.len()-1].split(',') {
                if p.contains('<') {
                    let [category, rest] = p.split('<').collect::<Vec<_>>()
                        .try_into().map_err(|_| "Invalid rule format: No <")?;
                    let [n, r] = rest.split(':').collect::<Vec<_>>()
                        .try_into().map_err(|_| "Invalid rule format: No :")?;

                    let category = Category::from_char(category.chars().next().unwrap());
                    let n = n.parse().map_err(|_| "Invalid number")?;

                    let rule_id = match rule_map.entry(r.to_owned()) {
                        Entry::Occupied(e) => *e.get(),
                        Entry::Vacant(e) => {
                            let new_index = rules.len();
                            e.insert(new_index);

                            rules.push(Vec::new());
                            new_index
                        },
                    };

                    rules[rule_index].push(Rule::LessThan(category, n, rule_id));
                } else if p.contains('>') {
                    let [category, rest] = p.split('>').collect::<Vec<_>>()
                        .try_into().map_err(|_| "Invalid rule format: No >")?;
                    let [n, r] = rest.split(':').collect::<Vec<_>>()
                        .try_into().map_err(|_| "Invalid rule format: No :")?;

                    let category = Category::from_char(category.chars().next().unwrap());
                    let n = n.parse().map_err(|_| "Invalid number")?;

                    let rule_id = match rule_map.entry(r.to_owned()) {
                        Entry::Occupied(e) => *e.get(),
                        Entry::Vacant(e) => {
                            let new_index = rules.len();
                            e.insert(new_index);

                            rules.push(Vec::new());
                            new_index
                        },
                    };

                    rules[rule_index].push(Rule::GreaterThan(category, n, rule_id));
                } else {
                    let rule_id = match rule_map.entry(p.to_owned()) {
                        Entry::Occupied(e) => *e.get(),
                        Entry::Vacant(e) => {
                            let new_index = rules.len();
                            e.insert(new_index);

                            rules.push(Vec::new());
                            new_index
                        },
                    };

                    rules[rule_index].push(Rule::Rule(rule_id));
                }
            }
        }

        Ok(Self { rules, rule_map })
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    cool_looking: u32,
    musical: u32,
    aerodynamic: u32,
    shiny: u32,
}

impl Part {
    fn rating(&self, c: Category) -> u32 {
        match c {
            Category::CoolLooking => self.cool_looking,
            Category::Musical => self.musical,
            Category::Aerodynamic => self.aerodynamic,
            Category::Shiny => self.shiny,
        }
    }

    fn rating_sum(&self) -> u32 {
        self.cool_looking + self.musical + self.aerodynamic + self.shiny
    }
}

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Strip curly brackets
        let s = &s[1..s.len()-1];

        let part: Vec<u32> = s.split(',')
            .map(|s| s.split('=').nth(1).unwrap())
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Self {
            cool_looking: part[0], musical: part[1],
            aerodynamic: part[2], shiny: part[3],
        })
    }
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .collect();

    let rules = Rules::from_str(input[0]).unwrap();
    let parts: Vec<_> = input[1].lines()
        .map(|l| Part::from_str(l).unwrap())
        .collect();


    let accepted_sum: u32 = parts.iter()
        .filter_map(|p| {
            if rules.accepts_part(p) {
                Some(p.rating_sum())
            } else {
                None
            }
        })
        .sum();
    println!("[Part 1] Rating sum of accepted parts: {accepted_sum}");

    println!("[Part 2] Total accepted parts: {}", rules.total_accepted_parts());
}
