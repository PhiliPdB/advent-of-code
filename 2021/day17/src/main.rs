use std::cmp;


fn simulate(start_x: i32, start_y: i32) -> (i32, i32, i32) {
    let mut current_x = 0;
    let mut vx = start_x;
    let mut current_y = 0;
    let mut vy = start_y;
    let mut max_y = start_y;

    while current_y >= 0 {
        current_x += vx;
        current_y += vy;

        vx = cmp::max(vx - 1, 0);
        vy -= 1;

        if current_y > max_y {
            max_y = current_y;
        }
    }

    (current_x, current_y, max_y)
}

fn main() {
    // Example input
    // let target_x_min =  20;
    // let target_x_max =  30;
    // let target_y_min = -10;
    // let target_y_max =  -5;
    // Real input
    let target_x_min =   48;
    let target_x_max =   70;
    let target_y_min = -189;
    let target_y_max = -148;

    let mut y_max = i32::MIN;
    let mut pos = (0, 0);
    for x in 0..target_x_min {
        let mut last_successful = false;
        for y in 0..-target_y_min {
            let (tx, ty, my) = simulate(x, y);
            if target_x_min <= tx && tx <= target_x_max && target_y_min <= ty && ty <= target_y_max {
                last_successful = true;
                if my > y_max {
                    y_max = my;
                    pos = (x, y);
                }
            } else if last_successful {
                // Starting to overshoot
                break;
            }
        }
    }

    println!("Max y: {}, {:?}", y_max, pos);
}
