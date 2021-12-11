use std::collections::VecDeque;


const HEIGHT: usize = 10;
const WIDTH: usize = 10;

const STEPS: i32 = 100;


fn simulate_step(map: &mut [Vec<u8>]) -> u32 {
    // Go through the map to trigger flashes
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            flash(map, x, y);
        }
    }

    let mut flashes = 0;
    // Count flashes and reset energy levels
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if map[y][x] > 9 {
                flashes += 1;
                map[y][x] = 0;
            }
        }
    }

    flashes
}

fn flash(map: &mut [Vec<u8>], x: usize, y: usize) {
    let mut queue = VecDeque::from([(x, y)]);

    while let Some((cur_x, cur_y)) = queue.pop_front() {
        map[cur_y][cur_x] += 1;

        // Check if this triggered the first flash for a octopus,
        // if so add the neighbours to the queue
        if map[cur_y][cur_x] == 10 {
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let next_x = (cur_x as i32 + dx) as usize;
                    let next_y = (cur_y as i32 + dy) as usize;
                    if next_x < WIDTH && next_y < HEIGHT
                    {
                        queue.push_back((next_x, next_y))
                    }
                }
            }
        }
    }
}


fn main() {
    let mut input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>())
        .collect();

    let mut total_flashes = 0;
    for _ in 0..STEPS {
        total_flashes += simulate_step(&mut input);
    }
    println!("Total flashes: {}", total_flashes);
}
