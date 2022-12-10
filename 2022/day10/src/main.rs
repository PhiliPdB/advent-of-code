
#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    pub fn get_addx_value(&self) -> i32 {
        match self {
            Instruction::Noop => panic!(),
            Instruction::AddX(v) => *v,
        }
    }

    pub fn is_addx(&self) -> bool {
        match self {
            Instruction::Noop => false,
            Instruction::AddX(_) => true,
        }
    }
}

fn draw(cycle: i32, sprite_loc: i32, amount: i32) {
    if cycle % 40 == 0 && cycle != 0 {
        println!();
    }

    let mut cycle = cycle % 40;
    for i in 0..amount {
        if (sprite_loc-1..=sprite_loc+1).contains(&cycle) {
            print!("\u{2588}")
        } else {
            print!(" ");
        }

        cycle += 1;
        if cycle >= 40 && i < amount - 1  {
            println!();
            cycle = 0;
        }
    }
}

fn main() {
    let instructions: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            if l == "noop" {
                Instruction::Noop
            } else {
                let (_, v) = l.split_once(' ').unwrap();
                Instruction::AddX(v.parse().unwrap())
            }
        })
        .collect();

    let mut x = 1;
    let mut cycle = 1;
    let mut sum = 0;
    for instruction in &instructions {
        match instruction {
            Instruction::Noop => {
                cycle += 1;
            },
            Instruction::AddX(v) => {
                cycle += 2;
                x += v;
            },
        }

        if (cycle - 20) % 40 == 0 {
            sum += x * cycle;
        } else if (cycle - 20) % 40 == 1 && instruction.is_addx() {
            sum += (x - instruction.get_addx_value()) * (cycle - 1);
        }
    }
    println!("[Part 1] Sum: {sum}");

    println!("Part 2:");
    x = 1;
    cycle = 0;
    for instruction in &instructions {
        match instruction {
            Instruction::Noop => {
                draw(cycle, x, 1);

                cycle += 1;
            },
            Instruction::AddX(v) => {
                draw(cycle, x, 2);

                cycle += 2;
                x += v;
            },
        }
    }
}
