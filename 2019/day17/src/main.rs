use std::iter;

use intcode::Program;

mod intcode;

fn main() {
    let mut program = Program::new(
        include_str!("../input.txt")
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    );

    let mut part1_program = program.clone();
    part1_program.run(iter::empty());

    for row in &part1_program.image {
        for c in row {
            print!("{}", c);
        }
        println!();
    }

    // Part 1: Find intersections
    let width = part1_program.image[0].len();
    let height = part1_program.image.len();

    let mut intersections = Vec::new();
    for (y, row) in part1_program.image.iter().enumerate().skip(1).take(height - 2) {
        for (x, c) in row.iter().enumerate().skip(1).take(width - 2) {
            // Check if this is an intersection
            if *c == '#' && part1_program.image[y - 1][x] == '#' && part1_program.image[y][x - 1] == '#'
                && part1_program.image[y][x + 1] == '#' && part1_program.image[y + 1][x] == '#'
            {
                intersections.push((x, y));
            }
        }
    }

    let alignment_sum: usize = intersections.iter()
        .map(|(x, y)| x * y)
        .sum();

    println!();
    println!("Alignment sum: {}", alignment_sum);

    // Part 2

    // Found manually
    let _path = [
        "R", "8", "R", "10", "R", "10", // A
        "R", "4", "R", "8", "R", "10", "R", "12",  // B
        "R", "8", "R", "10", "R", "10", // A
        "R", "12", "R", "4", "L", "12", "L", "12", // C
        "R", "8", "R", "10", "R", "10",  // A
        "R", "4", "R", "8", "R", "10", "R", "12",  // B
        "R", "12", "R", "4", "L", "12", "L", "12", // C
        "R", "8", "R", "10", "R", "10", // A
        "R", "4", "R", "8", "R", "10", "R", "12", // B
        "R", "12", "R", "4", "L", "12", "L", "12" // C
    ];

    let main = ["A", "B", "A", "C", "A", "B", "C", "A", "B", "C"];
    let a = ["R", "8", "R", "10", "R", "10"];
    let b = ["R", "4", "R", "8", "R", "10", "R", "12"];
    let c = ["R", "12", "R", "4", "L", "12", "L", "12"];


    let input_string = format!("{main}\n{a}\n{b}\n{c}\nn\n", main=main.join(","), a=a.join(","), b=b.join(","), c=c.join(","));
    let input = input_string.chars()
        .map(|c| c as u8);

    program[0] = 2;
    let collected_dust = program.run(input);

    println!("Collected dust: {}", collected_dust);
}
