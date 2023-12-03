use std::collections::{HashMap, hash_map::Entry};

fn main() {
    let map: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let mut part1_sum = 0;
    for y in 0..map.len() {
        let mut x = 0;
        while x < map[y].len() {
            if !map[y][x].is_digit(10) {
                x += 1;
                continue;
            }

            let mut number = 0;
            let mut length = 0;
            for tmp_x in x..map[y].len() {
                if let Some(digit) = map[y][tmp_x].to_digit(10) {
                    number = 10 * number + digit;
                    length += 1;
                } else {
                    break;
                }
            }
            let number = number;
            let length = length;

            // Check if there is a symbol beside it
            let mut has_symbol = false;
            for y in [y.overflowing_sub(1).0, y, y + 1] {
                for x in (x as isize - 1)..=(x as isize + length as isize) {
                    if y < map.len() && x > 0 && (x as usize) < map[y].len()
                        && map[y][x as usize] != '.' && !map[y][x as usize].is_digit(10)
                    {
                        if map[y][x as usize] == '*' {
                            match gear_map.entry((y, x as usize)) {
                                Entry::Occupied(mut e) => {
                                    e.get_mut().push(number);
                                },
                                Entry::Vacant(e) => {
                                    e.insert(vec![number]);
                                },
                            }
                        }

                        has_symbol = true;
                        break;
                    }
                }
            }

            x += length;
            if has_symbol {
                part1_sum += number;
            }
        }
    }
    println!("[Part 1] Sum of part numbers: {part1_sum}");

    let gear_sum: u32 = gear_map.into_iter()
        .filter_map(|(_, v)| {
            if v.len() != 2 {
                None
            } else {
                Some(v[0] * v[1])
            }
        })
        .sum();
    println!("[Part 2] Sum of gear ratios: {gear_sum}");
}
