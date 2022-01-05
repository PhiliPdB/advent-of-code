use intcode::Program;


mod intcode;

fn main() {
    let mut program = Program::new(
        include_str!("../input.txt")
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    );

    // Part 1

    let mut part1_program = program.clone();
    // Run the program
    part1_program.run();

    println!("Painted panels: {}", part1_program.canvas.len());


    // Part 2
    println!("Part 2:");
    // Make sure to start on a white square
    program.canvas.insert((0, 0), 1);
    // Run the program
    program.run();

    // Get the dimensions of the canvas
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for (x, y) in program.canvas.keys() {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    // Draw the canvas
    for y in min_y..(max_y+1) {
        for x in min_x..(max_x+1) {
            match program.canvas.get(&(x, y)).unwrap_or(&0) {
                0 => print!(" "),
                1 => print!("\u{2588}"),
                _ => unreachable!(),
            }
        }
        println!()
    }
}
