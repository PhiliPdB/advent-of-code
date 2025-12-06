fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .collect();

    // Get all numbers
    let numbers: Vec<Vec<u32>> = input
        .iter()
        .take(input.len() - 1)
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    // Get operands
    let operands: Vec<_> = input
        .iter()
        .map(|l| l.split_whitespace().collect())
        .next_back()
        .unwrap();

    let mut part1_answer_sum = 0;
    for (i, op) in operands.iter().enumerate() {
        part1_answer_sum += match *op {
            "+" => numbers
                .iter()
                .map(|ns| ns[i] as u64)
                .sum::<u64>(),
            "*" => numbers
                .iter()
                .map(|ns| ns[i] as u64)
                .product(),
            _ => panic!("Unknown operand"),
        }
    }
    println!("[Part 1] Answer sum: {part1_answer_sum}");

    // Part 2

    // Convert input to chars for easier indexing
    let input_chars: Vec<Vec<_>> = input
        .iter()
        .map(|l| l.chars().collect())
        .collect();
    let mut cephalopod_numbers = vec![vec![]];
    let mut number_index = 0;
    // Read numbers from the columns
    for i in 0..input[0].len() {
        let column: String = input_chars
            .iter()
            .map(|cs| cs[i])
            .collect();
        if column.trim().is_empty() {
            // Start with new list of numbers to operate on
            number_index += 1;
            cephalopod_numbers.push(vec![]);
        } else {
            // Otherwise skip last char (the operand), trim, and parse as number
            let n: u64 = column[..column.len() - 1]
                .trim()
                .parse()
                .unwrap();
            cephalopod_numbers[number_index].push(n);
        }
    }

    let mut part2_answer_sum = 0;
    for (op, ns) in operands.iter().zip(&cephalopod_numbers) {
        part2_answer_sum += match *op {
            "+" => ns.iter().sum::<u64>(),
            "*" => ns.iter().product(),
            _ => panic!("Unknown operand"),
        }
    }
    println!("[Part 2] Answer sum: {part2_answer_sum}");
}
