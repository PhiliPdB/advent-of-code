use regex::Regex;

fn main() {
    let corrupt_instruction= include_str!("../input.txt");

    let part1_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let part1_mult_sum: i32 = part1_re.captures_iter(&corrupt_instruction)
        .map(|c| c.extract())
        .map(|(_, [x, y])| {
            x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap()
        })
        .sum();
    println!("[Part 1] Mul sum: {part1_mult_sum:9}");


    let mut enabled = true;
    let mut part2_mult_sum = 0;

    let part2_re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    for c in part2_re.captures_iter(&corrupt_instruction) {
        let m = c.get(0).map_or("", |m| m.as_str());
        if m == "do()" {
            enabled = true;
            continue;
        }
        if m == "don't()" {
            enabled = false;
            continue;
        }

        if enabled {
            let x = c.get(1).map_or("", |m| m.as_str());
            let y = c.get(2).map_or("", |m| m.as_str());

            part2_mult_sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap()
        }
    }
    println!("[Part 2] Mul sum: {part2_mult_sum:9}");
}
