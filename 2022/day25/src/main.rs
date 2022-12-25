use std::fmt::{Display, Formatter, Result};


#[derive(Debug, Clone, Copy)]
enum Digits {
    Zero, One, Two, Minus, DoubleMinus,
}

impl Digits {
    fn value(&self) -> i64 {
        match self {
            Digits::Zero => 0,
            Digits::One => 1,
            Digits::Two => 2,
            Digits::Minus => -1,
            Digits::DoubleMinus => -2,
        }
    }
}

impl Display for Digits {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Digits::Zero => write!(f, "0"),
            Digits::One => write!(f, "1"),
            Digits::Two => write!(f, "2"),
            Digits::Minus => write!(f, "-"),
            Digits::DoubleMinus => write!(f, "="),
        }
    }
}

fn main() {
    let fuel_requirement: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    match c {
                        '0' => Digits::Zero,
                        '1' => Digits::One,
                        '2' => Digits::Two,
                        '-' => Digits::Minus,
                        '=' => Digits::DoubleMinus,
                        _ => unimplemented!(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let fuel_sum: i64 = fuel_requirement.iter()
        .map(|snafu| {
            let len = snafu.len() as u32;
            let mut value = 0;
            for (i, d) in snafu.iter().enumerate() {
                value += d.value() * 5_i64.pow(len - 1 - i as u32);
            }
            value
        })
        .sum();

    println!("Fuel sum (decimal): {fuel_sum}");

    let mut i = 0;
    let mut fuel_sum_snafu = Vec::new();
    let mut fuel_left = fuel_sum;
    loop {
        if fuel_left == 0 {
            break;
        }

        for d in [Digits::Zero, Digits::One, Digits::Two, Digits::Minus, Digits::DoubleMinus] {
            let digit_value = d.value() * 5_i64.pow(i);
            if (fuel_left - digit_value) % 5_i64.pow(i + 1) == 0 {
                fuel_sum_snafu.push(d);
                fuel_left -= digit_value;
                break;
            }
        }

        i += 1;
    }
    fuel_sum_snafu.reverse();

    print!("[Part 1] Fuel sum (SNAFU): ");
    for d in fuel_sum_snafu {
        print!("{}", d);
    }
    println!();
}
