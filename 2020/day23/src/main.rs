
const PART2: bool = true;
const TOTAL_NUMBERS: u32 = if PART2 {
    1_000_000
} else {
    9
};

const ITERATIONS: u32 = if PART2 {
    10_000_000
} else {
    100
};


fn main() {
    // Test input
    // let input: [u32; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7]
    //    .map(|i| i - 1);
    // Real input
    let input: [u32; 9] = [9, 1, 6, 4, 3, 8, 2, 7, 5]
        .map(|i| i - 1);

    // Create list where the index number points to the next number
    let mut next_number = vec![0; TOTAL_NUMBERS as usize];
    for i in input.windows(2) {
        next_number[i[0] as usize] = i[1];
    }

    if PART2 {
        next_number[input[input.len() - 1] as usize] = 9;
        for i in 9..(TOTAL_NUMBERS - 1) {
            next_number[i as usize] = i + 1;
        }
        next_number[TOTAL_NUMBERS as usize - 1] = input[0];
    } else {
        // Link last number to the first
        next_number[input[input.len() - 1] as usize] = input[0];
    }


    let mut current_cup = input[0];
    for _move in 0..ITERATIONS {
        let next1 = next_number[current_cup as usize];
        let next2 = next_number[next1 as usize];
        let next3 = next_number[next2 as usize];

        let target = {
            let mut target = current_cup - 1;
            if target > TOTAL_NUMBERS {
                target = TOTAL_NUMBERS - 1;
            }
            while target == next1 || target == next2 || target == next3 {
                target -= 1;

                if target > TOTAL_NUMBERS {
                    target = TOTAL_NUMBERS - 1;
                }
            }

            target
        };

        next_number[current_cup as usize] = next_number[next3 as usize];
        next_number[next3 as usize] = next_number[target as usize];
        next_number[target as usize] = next1;


        current_cup = next_number[current_cup as usize];
    }


    if PART2 {
        let next1 = next_number[0] as u64;
        let next2 = next_number[next1 as usize] as u64;

        println!("Result: {}", (next1 + 1) * (next2 + 1));
    } else {
        let mut result = Vec::new();
        let mut current = 0;
        for _ in 0..(TOTAL_NUMBERS - 1) {
            result.push(next_number[current]);

            current = next_number[current] as usize;
        }

        println!("Result: {}", result.iter().map(|i| i +  1).fold(0, |acc, n| acc * 10 + n));
    }
}
