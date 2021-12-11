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

    // Count flashes and reset energy levels
    let mut flashes = 0;
    for row in map.iter_mut() {
        for item in row.iter_mut() {
            if *item > 9 {
                flashes += 1;
                *item = 0;
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
            for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                let next_x = (cur_x as i32 + dx) as usize;
                let next_y = (cur_y as i32 + dy) as usize;
                // NOTE: If next_x (or next_y) has an overflow due to the subtraction,
                //       it will also result in a value bigger than the WIDTH (or HEIGHT).
                if next_x < WIDTH && next_y < HEIGHT {
                    queue.push_back((next_x, next_y))
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
    let mut current_step = 0;
    loop {
        let flashes = simulate_step(&mut input);

        // Only count the first few steps for the amount of flashes
        if current_step < STEPS {
            total_flashes += flashes;
        }
        // Check if the whole grid flashed to see if we have synchronisation
        if flashes == (WIDTH * HEIGHT) as u32 {
            break;
        }
        current_step += 1;
    }
    println!("Total flashes (in first {} steps): {}", STEPS, total_flashes);

    println!("Synchronisation in step: {}", current_step + 1);
}
