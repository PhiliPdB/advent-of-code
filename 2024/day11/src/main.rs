use cached::proc_macro::cached;


fn blink(stones: &[u64], num_times: u32) -> Vec<u64> {
    let mut stones = stones.to_vec();

    for _ in 0..num_times {
        let old_len = stones.len();
        for i in 0..old_len {
            let s = &mut stones[i];

            if *s == 0 {
                *s = 1;
            } else if (s.ilog10() + 1) % 2 == 0 {
                // Split the number into two halves
                let num_digits = s.ilog10() + 1;
                let left_half = *s / 10_u64.pow(num_digits / 2);
                let right_half = *s % 10_u64.pow(num_digits / 2);

                *s = left_half;
                stones.push(right_half);
            } else {
                *s *= 2024;
            }
        }
    }

    stones
}


#[cached]
fn blink_count(stone: u64, blinks_left: u32) -> u64 {
    if blinks_left == 0 {
        return 1;
    }

    if stone == 0 {
        return blink_count(1, blinks_left - 1);
    } else if (stone.ilog10() + 1) % 2 == 0 {
        let num_digits = stone.ilog10() + 1;
        let left_half = stone / 10_u64.pow(num_digits / 2);
        let right_half = stone % 10_u64.pow(num_digits / 2);

        return blink_count(left_half, blinks_left - 1)
            + blink_count(right_half, blinks_left - 1);
    } else {
        return blink_count(stone * 2024, blinks_left - 1);
    }
}


fn main() {
    let stones: Vec<_> = include_str!("../input.txt")
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();


    let part1_stones = blink(&stones, 25).len();
    println!("[Part 1] Number of stones: {part1_stones:15}");


    let part2_stones: u64 = stones.iter()
        .map(|&stone| blink_count(stone, 75))
        .sum();
    println!("[Part 2] Number of stones: {part2_stones:15}");
}
