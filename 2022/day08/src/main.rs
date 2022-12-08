fn main() {
    let heights: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect();
    println!("Grid: {}x{}", heights.len(), heights[0].len());

    let width = heights[0].len();
    let height = heights.len();

    let mut visible_trees = 0;
    for y in 0..height {
        for x in 0..width {
            let h = heights[y][x];

            // Check left
            let mut visible_left = true;
            for i in 0..x {
                if heights[y][i] >= h {
                    visible_left = false;
                    break;
                }
            }

            // Check right
            let mut visible_right = true;
            for i in (x + 1)..width {
                if heights[y][i] >= h {
                    visible_right = false;
                    break;
                }
            }

            // Check top
            let mut visible_top = true;
            for i in 0..y {
                if heights[i][x] >= h {
                    visible_top = false;
                    break;
                }
            }

            // Check bottom
            let mut visible_bottom = true;
            for i in (y + 1)..height {
                if heights[i][x] >= h {
                    visible_bottom = false;
                    break;
                }
            }

            if visible_left || visible_right || visible_top || visible_bottom {
                visible_trees += 1;
            }
        }
    }
    println!("[Part 1] Visible trees: {visible_trees}");


    let mut max_scenic_score = 0;
    for y in 0..height {
        for x in 0..width {
            let h = heights[y][x];

            // Check left
            let mut distance_left = 0;
            for i in (0..x).rev() {
                distance_left += 1;
                if heights[y][i] >= h {
                    break;
                }
            }

            // Check right
            let mut distance_right = 0;
            for i in (x + 1)..width {
                distance_right += 1;
                if heights[y][i] >= h {
                    break;
                }
            }

            // Check top
            let mut distance_top = 0;
            for i in (0..y).rev() {
                distance_top += 1;
                if heights[i][x] >= h {
                    break;
                }
            }

            // Check bottom
            let mut distance_bottom = 0;
            for i in (y + 1)..height {
                distance_bottom += 1;
                if heights[i][x] >= h {
                    break;
                }
            }

            let scenic_score = distance_left * distance_right * distance_top * distance_bottom;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    println!("[Part 2] Scenic score: {max_scenic_score}");
}
