use itertools::Itertools;


fn expand<const N: usize>(mut galaxies: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut height = galaxies.iter().map(|(y, _)| *y).max().unwrap() + 1;
    let mut width = galaxies.iter().map(|(_, x)| *x).max().unwrap() + 1;

    // Expand the universe
    let mut y = 0;
    while y < height {
        if galaxies.iter().filter(|(gy, _)| *gy == y).count() == 0 {
            for (gy, _) in galaxies.iter_mut() {
                if *gy > y {
                    *gy += N - 1;
                }
            }

            height += N - 1;
            y += N;
        } else {
            y += 1;
        }
    }

    let mut x = 0;
    while x < width {
        if galaxies.iter().filter(|(_, gx)| *gx == x).count() == 0 {
            for (_, gx) in galaxies.iter_mut() {
                if *gx > x {
                    *gx += N - 1;
                }
            }

            width += N - 1;
            x += N;
        } else {
            x += 1;
        }
    }

    galaxies
}

fn distance((y1, x1): (usize, usize), (y2, x2): (usize, usize)) -> usize {
    y1.abs_diff(y2) + x1.abs_diff(x2)
}

fn distance_sum(galaxies: &[(usize, usize)]) -> usize {
    galaxies.iter()
        .combinations(2)
        .map(|comb| distance(*comb[0], *comb[1]))
        .sum()
}

fn main() {
    let galaxies: Vec<_> = include_str!("../input.txt")
        .lines().enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate()
                .filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((y, x))
                    } else {
                        None
                    }
                })
        })
        .collect();


    let part1_expansion = expand::<2>(galaxies.clone());
    println!("[Part 1] Galaxy distances: {:12}", distance_sum(&part1_expansion));

    let part2_expansion = expand::<1_000_000>(galaxies.clone());
    println!("[Part 2] Galaxy distances: {:12}", distance_sum(&part2_expansion));
}
