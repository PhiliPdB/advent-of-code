use std::fmt::{Display, Formatter, Result};

use nom::{IResult, Parser};
use nom::branch::alt;
use nom::character::complete::i32;
use nom::bytes::complete::tag;
use nom::sequence::{delimited, pair, preceded};


#[derive(Debug,Clone)]
pub struct Snailfish {
    left: SnailfishItem,
    right: SnailfishItem,
}

impl Snailfish {

    pub fn parse(input: &str) -> IResult<&str, Self> {
        delimited(
            tag("["),
            pair(SnailfishItem::parse, preceded(tag(","), SnailfishItem::parse))
                .map(|(left, right)| Self { left, right }),
            tag("]")
        )(input)
    }

    pub fn add(sf1: Self, sf2: Self) -> Self {
        let mut snailfish = Self {
            left: SnailfishItem::Pair(Box::new(sf1)),
            right: SnailfishItem::Pair(Box::new(sf2)),
        };
        snailfish.reduce();

        snailfish
    }

    #[inline]
    pub fn magnitude(&self) -> i32 {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }

    #[inline]
    pub fn get_left_most_val(&mut self) -> &mut i32 {
        self.left.get_left_most_val()
    }

    #[inline]
    pub fn get_right_most_val(&mut self) -> &mut i32 {
        self.right.get_right_most_val()
    }

    pub fn reduce(&mut self) {
        let mut did_reduce = true;
        while did_reduce {
            did_reduce = self.explode(0, None, None) || self.split();
        }
    }

    #[inline]
    fn explode(&mut self, depth: i32, left_val: Option<&mut SnailfishItem>, right_val: Option<&mut SnailfishItem>) -> bool {
        self.left.explode(depth + 1, left_val, Some(&mut self.right))
            || self.right.explode(depth + 1, Some(&mut self.left), right_val)
    }

    #[inline]
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
enum SnailfishItem {
    Number(i32),
    Pair(Box<Snailfish>),
}

impl SnailfishItem {

    #[inline]
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            i32.map(|n| Self::Number(n)),
            Snailfish::parse.map(|f| Self::Pair(Box::new(f))),
        ))(input)
    }

    #[inline]
    pub fn magnitude(&self) -> i32 {
        match self {
            SnailfishItem::Number(n) => *n,
            SnailfishItem::Pair(sf) => sf.magnitude(),
        }
    }

    #[inline]
    pub fn get_left_most_val(&mut self) -> &mut i32 {
        match self {
            SnailfishItem::Number(n) => n,
            SnailfishItem::Pair(sf) => sf.get_left_most_val(),
        }
    }

    #[inline]
    pub fn get_right_most_val(&mut self) -> &mut i32 {
        match self {
            SnailfishItem::Number(n) => n,
            SnailfishItem::Pair(sf) => sf.get_right_most_val(),
        }
    }

    #[inline]
    pub fn is_pair(&self) -> bool {
        match self {
            SnailfishItem::Number(_) => false,
            SnailfishItem::Pair(_) => true,
        }
    }

    #[inline]
    pub fn is_deepest_pair(&self) -> bool {
        match self {
            SnailfishItem::Number(_) => false,
            SnailfishItem::Pair(sf) => {
                !sf.left.is_pair() && !sf.right.is_pair()
            },
        }
    }

    #[inline]
    pub fn get_value(&self) -> i32 {
        match self {
            SnailfishItem::Number(n) => *n,
            SnailfishItem::Pair(_) => panic!("Not a value"),
        }
    }

    #[inline]
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
                    }
                    *left = SnailfishItem::Number(0);

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
                    }
                    *right = SnailfishItem::Number(0);

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
