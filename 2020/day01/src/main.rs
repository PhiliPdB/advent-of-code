const SUM_TO: i32 = 2020;

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    for x in 0..input.len() {
        for y in x..input.len() {
            if input[x] + input[y] == SUM_TO {
                println!("Part 1: {:#9}", input[x] * input[y]);
                break;
            }
        }
    }

    for x in 0..input.len() {
        for y in x..input.len() {
            for z in y..input.len() {
                if input[x] + input[y] + input[z] == SUM_TO {
                    println!("Part 2: {:#9}", input[x] * input[y] * input[z]);
                    break;
                }
            }
        }
    }
}
