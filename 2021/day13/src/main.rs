use std::collections::HashSet;

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .collect();

    let mut dots: HashSet<_> = input[0].lines()
        .map(|s| {
            let nums: Vec<_> = s.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            (nums[0], nums[1])
        })
        .collect();

    let instructions: Vec<_> = input[1].lines()
        .map(|s| {
            let axis = s.chars().nth(11).unwrap();
            let n = s.split_at(13).1.parse::<i32>().unwrap();

            (axis, n)
        })
        .collect();

    for (fold, (axis, n)) in instructions.iter().enumerate() {
        match axis {
            'x' => {
                dots = dots.into_iter()
                    .map(|(x, y)| if x > *n {
                        (n - (x - n), y)
                    } else {
                        (x, y)
                    })
                    .collect();
            },
            'y' => {
                dots = dots.into_iter()
                    .map(|(x, y)| if y > *n {
                        (x, n - (y - n))
                    } else {
                        (x, y)
                    })
                    .collect()
            },
            _ => panic!("Invalid axis"),
        }

        if fold == 0 {
            println!("[Part 1] Dots: {}", dots.len());
        }
    }

    // Print the code

    let max_x = dots.iter().max_by(|(x1, _), (x2, _)| x1.cmp(x2)).unwrap().0 as usize;
    let max_y = dots.iter().max_by(|(_, y1), (_, y2)| y1.cmp(y2)).unwrap().1 as usize;
    let mut grid = vec![vec![false; max_x + 1]; max_y + 1];
    for (x, y) in dots {
        grid[y as usize][x as usize] = true;
    }

    for row in grid {
        for item in row {
            if item {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
