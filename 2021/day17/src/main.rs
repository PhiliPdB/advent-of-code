use std::cmp;

// Example input
// const TARGET_X_MIN: i32 =  20;
// const TARGET_X_MAX: i32 =  30;
// const TARGET_Y_MIN: i32 = -10;
// const TARGET_Y_MAX: i32 =  -5;
// Real input
const TARGET_X_MIN: i32 =   48;
const TARGET_X_MAX: i32 =   70;
const TARGET_Y_MIN: i32 = -189;
const TARGET_Y_MAX: i32 = -148;


fn simulate(start_x: i32, start_y: i32) -> (bool, i32) {
    let mut current_x = 0;
    let mut vx = start_x;
    let mut current_y = 0;
    let mut vy = start_y;

    let mut max_y = 0;
    let mut hit_target = false;

    while current_y + vy >= TARGET_Y_MIN && current_x + vx <= TARGET_X_MAX {
        current_x += vx;
        current_y += vy;

        vx = cmp::max(vx - 1, 0);
        vy -= 1;

        if current_y > max_y {
            max_y = current_y;
        }

        if (TARGET_X_MIN..=TARGET_X_MAX).contains(&current_x)
            && (TARGET_Y_MIN..=TARGET_Y_MAX).contains(&current_y)
        {
            hit_target = true;
            break;
        }
    }

    (hit_target, max_y)
}

fn main() {
    let mut total_hits = 0;
    let mut y_max = i32::MIN;
    for x in 1..(TARGET_X_MAX + 1) {
        for y in TARGET_Y_MIN..(-TARGET_Y_MIN + 1) {
            let (hit_target, my) = simulate(x, y);
            if hit_target {
                total_hits += 1;

                if my > y_max {
                    y_max = my;
                }
            }
        }
    }

    println!("Max y: {}", y_max);
    println!("Total hits: {}", total_hits);
}
