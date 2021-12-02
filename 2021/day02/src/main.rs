use std::str::FromStr;


enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();

        if let Ok(magnitude) = parts[1].parse::<i32>() {
            match parts[0] {
                "forward" => Ok(Direction::Forward(magnitude)),
                "down"    => Ok(Direction::Down(magnitude)),
                "up"      => Ok(Direction::Up(magnitude)),
                _         => Err("Invalid direction")
            }
        } else {
            Err("Can't parse magnitude")
        }
    }
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| Direction::from_str(s).unwrap())
        .collect();

    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;

    for direction in input {
        match direction {
            Direction::Forward(x) => {
                forward += x;
                depth   += aim * x;
            },
            Direction::Down(x)    => aim += x,
            Direction::Up(x)      => aim -= x,
        }
    }

    println!("Answer part 1: {}", forward * aim);
    println!("Answer part 2: {}", forward * depth);
}
