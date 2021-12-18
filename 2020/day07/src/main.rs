use std::collections::HashMap;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, u32};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{tuple, pair, preceded, terminated};


type LuggageRule = (String, Vec<(u32, String)>);

pub fn parse_line(line: & str) -> IResult<&str, LuggageRule> {
    map(tuple((
        parse_bag,
        tag(" bags contain"),
        alt((
            map(
                pair(
                    many0(preceded(tag(" "), terminated(parse_bag_item, tag(",")))),
                    preceded(tag(" "), terminated(parse_bag_item, tag(".")))
                ),
                |(mut bags, bag)| {
                    bags.push(bag);
                    bags
                }
            ),
            map(tag(" no other bags."), |_| Vec::new())
        ))
    )),|(bag, _, contain_bags)| {
        (bag, contain_bags)
    })(line)
}

fn parse_bag(input: &str) -> IResult<&str, String> {
    map(tuple((alpha1, tag(" "), alpha1)), |(s1, _, s2)| format!("{} {}", s1, s2).to_string())(input)
}

fn parse_bag_item(input: &str) -> IResult<&str, (u32, String)> {
    pair(u32, preceded(tag(" "), terminated(parse_bag, pair(tag(" bag"), opt(tag("s"))))))(input)
}


fn can_contain_bag(bag_rules: &HashMap<String, Vec<(u32, String)>>, bag: &str, search: &String) -> bool {
    let current_bag_contents: Vec<_> = bag_rules[bag].iter()
        .map(|(_, b)| b)
        .collect();

    current_bag_contents.contains(&search)
        || current_bag_contents.iter().any(|b| can_contain_bag(bag_rules, b, search))
}

fn count_containing_bags(bag_rules: &HashMap<String, Vec<(u32, String)>>, bag: &str) -> u32 {
    bag_rules[bag].iter()
        .map(|(n, b)| *n + *n * count_containing_bags(bag_rules, b))
        .sum()
}

fn main() {
    let input: HashMap<_, _> = include_str!("../input.txt")
        .lines()
        .map(|s| parse_line(s).unwrap().1)
        .collect();

    // Part 1

    let bags_containing = input.keys()
        .filter(|k| can_contain_bag(&input, k, &String::from("shiny gold")))
        .count();

    println!("Shiny gold is contained in {} bags.", bags_containing);

    // Part 2

    let contains_bags = count_containing_bags(&input, "shiny gold");
    println!("Shiny gold contains {} bags.", contains_bags);
}
