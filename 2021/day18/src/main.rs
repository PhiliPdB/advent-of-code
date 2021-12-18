use std::fmt::{Display, Formatter, Result};

use nom::{IResult, Parser};
use nom::sequence::{delimited, preceded, pair};
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::i32;


#[derive(Debug,Clone)]
pub struct Snailfish {
    left: SnailfishItem,
    right: SnailfishItem,
}

impl Snailfish {

    pub fn add(sf1: Self, sf2: Self) -> Self {
        let mut snailfish = Self {
            left: SnailfishItem::Pair(Box::new(sf1)),
            right: SnailfishItem::Pair(Box::new(sf2)),
        };
        snailfish.reduce();

        snailfish
    }

    pub fn magnitude(&self) -> i32 {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }

    pub fn get_left_most_val(&mut self) -> &mut i32 {
        self.left.get_left_most_val()
    }

    pub fn get_right_most_val(&mut self) -> &mut i32 {
        self.right.get_right_most_val()
    }

    pub fn reduce(&mut self) {
        let mut did_reduce = true;
        while did_reduce {
            did_reduce = self.explode(0, None, None) || self.split();
        }
    }

    fn explode(&mut self, depth: i32, left_val: Option<&mut SnailfishItem>, right_val: Option<&mut SnailfishItem>) -> bool {
        self.left.explode(depth + 1, left_val, Some(&mut self.right))
            || self.right.explode(depth + 1, Some(&mut self.left), right_val)
    }

    fn split(&mut self) -> bool {
        self.left.split() || self.right.split()
    }
}

impl Display for Snailfish {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}


#[derive(Debug, Clone)]
pub enum SnailfishItem {
    Number(i32),
    Pair(Box<Snailfish>),
}

impl SnailfishItem {

    pub fn magnitude(&self) -> i32 {
        match self {
            SnailfishItem::Number(n) => *n,
            SnailfishItem::Pair(sf) => sf.magnitude(),
        }
    }

    pub fn get_left_most_val(&mut self) -> &mut i32 {
        match self {
            SnailfishItem::Number(n) => n,
            SnailfishItem::Pair(sf) => sf.get_left_most_val(),
        }
    }

    pub fn get_right_most_val(&mut self) -> &mut i32 {
        match self {
            SnailfishItem::Number(n) => n,
            SnailfishItem::Pair(sf) => sf.get_right_most_val(),
        }
    }

    pub fn is_pair(&self) -> bool {
        match self {
            SnailfishItem::Number(_) => false,
            SnailfishItem::Pair(_) => true,
        }
    }

    pub fn is_deepest_pair(&self) -> bool {
        match self {
            SnailfishItem::Number(_) => false,
            SnailfishItem::Pair(sf) => {
                !sf.left.is_pair() && !sf.right.is_pair()
            },
        }
    }

    pub fn get_value(&self) -> i32 {
        match self {
            SnailfishItem::Number(n) => *n,
            SnailfishItem::Pair(_) => panic!("Not a value"),
        }
    }

    pub fn get_pair(&self) -> &Box<Snailfish> {
        match self {
            SnailfishItem::Number(_) => panic!("Not a pair"),
            SnailfishItem::Pair(sf) => sf,
        }
    }


    fn explode(&mut self, depth: i32, left_val: Option<&mut SnailfishItem>, right_val: Option<&mut SnailfishItem>) -> bool {
        match self {
            SnailfishItem::Number(_) => false,
            SnailfishItem::Pair(sf) => {
                let left = &mut sf.left;
                let right = &mut sf.right;
                if depth >= 3 && left.is_deepest_pair() {
                    // Try to explode the left pair
                    let pair = left.get_pair();
                    if right.is_pair() {
                        *right.get_left_most_val() += pair.right.get_value();
                    } else {
                        *right = SnailfishItem::Number(right.get_value() + pair.right.get_value());
                    }

                    if let Some(val) = left_val.map(|sfi| sfi.get_right_most_val()) {
                        *val += pair.left.get_value();
                        *left = SnailfishItem::Number(0);
                    } else {
                        *left = SnailfishItem::Number(0);
                    }

                    true
                } else if depth >= 3 && right.is_deepest_pair() {
                    // Try to explode the right pair
                    let pair = right.get_pair();
                    if left.is_pair() {
                        *left.get_right_most_val() += pair.left.get_value();
                    } else {
                        *left = SnailfishItem::Number(left.get_value() + pair.left.get_value());
                    }

                    if let Some(val) = right_val.map(|sfi| sfi.get_left_most_val()) {
                        *val += pair.right.get_value();
                        *right = SnailfishItem::Number(0);
                    } else {
                        *right = SnailfishItem::Number(0);
                    }

                    true
                } else {
                    left.explode(depth + 1, left_val, Some(right))
                        || right.explode(depth + 1, Some(left), right_val)
                }
            },
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailfishItem::Number(n) => {
                if *n >= 10 {
                    *self = SnailfishItem::Pair(Box::new(Snailfish {
                        left: SnailfishItem::Number(*n / 2),
                        right: SnailfishItem::Number((*n as f64 / 2_f64).ceil() as i32),
                    }));

                    true
                } else {
                    false
                }
            },
            SnailfishItem::Pair(sf) => sf.left.split() || sf.right.split(),
        }
    }
}

impl Display for SnailfishItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            SnailfishItem::Number(n) => write!(f, "{}", n),
            SnailfishItem::Pair(sf) => write!(f, "{}", sf),
        }
    }
}


fn parse_snailfish(input: &str) -> IResult<&str, Snailfish> {
    delimited(
        tag("["),
        pair(parse_snailfish_input, preceded(tag(","), parse_snailfish_input))
            .map(|(left, right)| Snailfish { left, right }),
        tag("]")
    )(input)
}

fn parse_snailfish_input(input: &str) -> IResult<&str, SnailfishItem> {
    alt((
        i32.map(|n| SnailfishItem::Number(n)),
        parse_snailfish.map(|f| SnailfishItem::Pair(Box::new(f))),
    ))(input)
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let (rem, sf) = parse_snailfish(s).unwrap();
            assert!(rem.is_empty());
            sf
        })
        .collect();

    // let mut test = parse_snailfish("[7,[6,[5,[4,[3,2]]]]]").unwrap().1;
    // test.explode(0, None, None);
    // println!("{}", test);

    // let mut test = parse_snailfish("[[6,[5,[4,[3,2]]]],1]").unwrap().1;
    // test.explode(0, None, None);
    // println!("{}", test);

    // let mut test = parse_snailfish("[1,[[[[2,3],4],5],6]]").unwrap().1;
    // test.explode(0, None, None);
    // println!("{}", test);

    // let mut test = parse_snailfish("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap().1;
    // test.explode(0, None, None);
    // println!("{}", test);

    // let mut test = parse_snailfish("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap().1;
    // test.explode(0, None, None);
    // println!("{}", test);

    // let mut test = parse_snailfish("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap().1;
    // test.reduce();
    // println!("{}", test);

    // Part 1

    let mut snailfish = input[0].clone();
    for sf in input.iter().skip(1).cloned() {
        snailfish = Snailfish::add(snailfish, sf);
    }
    println!("{}", snailfish);
    println!("Magnitude: {}", snailfish.magnitude());

    // Part 2

    let mut largest_magnitude = 0;
    for i in 0..input.len() {
        for j in 0..i {
            let mag1 = Snailfish::add(input[i].clone(), input[j].clone()).magnitude();
            let mag2 = Snailfish::add(input[j].clone(), input[i].clone()).magnitude();

            if mag1 > largest_magnitude {
                largest_magnitude = mag1;
            }
            if mag2 > largest_magnitude {
                largest_magnitude = mag2;
            }
        }
    }

    println!("Largest magnitude: {}", largest_magnitude);
}
