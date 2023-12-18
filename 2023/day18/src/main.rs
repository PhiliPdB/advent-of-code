use std::str::FromStr;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up, Down, Left, Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction: {c}"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i64,
}

impl Instruction {
    fn parse_from_color(s: &str) -> Self {
        let parts: Vec<_> = s.split_whitespace().collect();

        let distance = parts[2][2..7].chars()
            .map(|c| c.to_digit(16).unwrap() as i64)
            .fold(0, |acc, c| 16 * acc + c);

        let direction = match parts[2][7..].chars().next().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            d => panic!("Invalid direction: {d}"),
        };

        Self { direction, distance }
    }
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        let direction = Direction::from_char(parts[0].chars().next().unwrap());

        let distance = parts[1].parse().unwrap();

        Ok(Self { direction, distance })
    }
}

fn calculate_volume(instructions: &[Instruction]) -> i64 {
    let (mut current_x, mut current_y) = (0, 0);
    // Save the vertices of the polygon we create
    let mut vertices = Vec::with_capacity(instructions.len());
    vertices.push((current_x, current_y));

    let mut perimeter_size = 0;
    for Instruction { direction, distance } in instructions {
        match direction {
            Direction::Up => {
                current_y -= distance;
            },
            Direction::Down => {
                current_y += distance;
            },
            Direction::Left => {
                current_x -= distance;
            },
            Direction::Right => {
                current_x += distance;
            },
        }

        vertices.push((current_x, current_y));
        perimeter_size += distance;
    }
    vertices.push(vertices[0]);

    // Calculate volume using the Shoelace Formula
    let volume = vertices.windows(2)
        .fold(0, |acc, p| {
            let (p1_x, p1_y) = p[0];
            let (p2_x, p2_y) = p[1];

            acc + (p1_x * p2_y) - (p2_x * p1_y)
        }) / 2;

    // Using Pick's Theorem to calculate the final lava volume
    volume + perimeter_size / 2 + 1
}


fn main() {
    let part1_instructions: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect();
    println!("[Part 1] Volume: {:14}", calculate_volume(&part1_instructions));


    let part2_instructions: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| Instruction::parse_from_color(l))
        .collect();
    println!("[Part 2] Volume: {:14}", calculate_volume(&part2_instructions));
}
