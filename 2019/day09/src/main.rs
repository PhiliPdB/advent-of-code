use intcode::Program;

mod intcode;


fn main() {
    let program = Program::new(
        include_str!("../input.txt")
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    );


    println!("BOOST keycode: {}", program.clone().run(1));

    println!("Coordinates of distress signal: {}", program.clone().run(2));
}
