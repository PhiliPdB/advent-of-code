struct Move {
    amount: i32,
    from: usize,
    to: usize,
}

impl Move {
    pub fn new(amount: i32, from: usize, to: usize) -> Self {
        Self { amount, from, to }
    }
}

fn main() {
    let stacks = [
        vec!['N', 'R', 'G', 'P'],
        vec!['J', 'T', 'B', 'L', 'F', 'G', 'D', 'C'],
        vec!['M', 'S', 'V'],
        vec!['L', 'S', 'R', 'C', 'Z', 'P'],
        vec!['P', 'S', 'L', 'V', 'C', 'W', 'D', 'Q'],
        vec!['C', 'T', 'N', 'W', 'D', 'M', 'S'],
        vec!['H', 'D', 'G', 'W', 'P'],
        vec!['Z', 'L', 'P', 'H', 'S', 'C', 'M', 'V'],
        vec!['R', 'P', 'F', 'L', 'W', 'G', 'Z']
    ];

    let moves: Vec<_> = include_str!("../moves.txt")
        .lines()
        .map(|l| {
            let splited: Vec<_> = l.split(' ').collect();
            Move::new(
                splited[1].parse().unwrap(),
                splited[3].parse::<usize>().unwrap() - 1,
                splited[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect();


    let mut part1_stacks = stacks.clone();
    for m in &moves {
        for _ in 0..m.amount {
            if let Some(pop) = part1_stacks[m.from].pop() {
                part1_stacks[m.to].push(pop);
            }
        }
    }

    print!("[Part 1] Answer: ");
    for s in part1_stacks {
        print!("{}", s[s.len() - 1]);
    }
    println!();


    let mut part2_stacks = stacks;
    for m in &moves {
        let mut crates = vec![char::default(); m.amount as usize];
        for i in 0..m.amount {
            if let Some(pop) = part2_stacks[m.from].pop() {
                crates[(m.amount - i - 1) as usize] = pop;
            }
        }
        part2_stacks[m.to].extend(crates);
    }

    print!("[Part 2] Answer: ");
    for s in part2_stacks {
        print!("{}", s[s.len() - 1]);
    }
    println!();
}
