use std::cmp::Ordering;

use nom::combinator::map;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Int(i32),
}

impl Packet {
    fn parse(line: &str) -> IResult<&str, Self> {
        alt((
            map(delimited(tag("["), separated_list0(tag(","), Packet::parse), tag("]")), Packet::List),
            map(i32, Packet::Int)
        ))(line)
    }

    fn is_right_order(left: &Packet, right: &Packet) -> Option<bool> {
        match (left, right) {
            (Packet::Int(l), Packet::Int(r)) => {
                if l == r {
                    None
                } else {
                    Some(l < r)
                }
            },
            (Packet::List(l), Packet::List(r)) => {
                let min_length = usize::min(l.len(), r.len());

                for i in 0..min_length {
                    if let Some(c) = Packet::is_right_order(&l[i], &r[i]) {
                        return Some(c);
                    }
                }

                if l.len() == r.len() {
                    None
                } else {
                    Some(l.len() < r.len())
                }
            },
            (l, Packet::Int(n)) => Packet::is_right_order(l, &Packet::List(vec![Packet::Int(*n)])),
            (Packet::Int(n), r) => Packet::is_right_order(&Packet::List(vec![Packet::Int(*n)]), r),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Packet::is_right_order(self, other)
            .map(|b| {
                if b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
    }
}


fn main() {
    let pairs: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .map(|pair| {
            let (p1, p2) = pair.split_once('\n').unwrap();
            (Packet::parse(p1).unwrap().1, Packet::parse(p2).unwrap().1)
        })
        .collect();

    let part1_result: usize = pairs.iter().enumerate()
        .filter_map(|(i, (l, r))| {
            if let Some(b) = Packet::is_right_order(l, r) {
                if b {
                    Some(i + 1)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum();
    println!("[Part 1] Result: {part1_result}");

    let mut all_packets: Vec<_> = pairs.into_iter()
        .flat_map(|p| [p.0, p.1])
        .collect();

    let divider_packets = [
        Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Int(6)])]),
    ];

    all_packets.push(divider_packets[0].clone());
    all_packets.push(divider_packets[1].clone());

    all_packets.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let decoder_key: usize = all_packets.iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if *p == divider_packets[0] || *p == divider_packets[1] {
                Some(i + 1)
            } else {
                None
            }
        })
        .product();
    println!("[Part 2] Decoder key: {decoder_key}");
}
