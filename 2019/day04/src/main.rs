fn main() {
    // Transformed from 245182
    const MIN: [i32; 6] = [2, 4, 5, 5, 5, 5];
    // Transformed from 790572
    const MAX: [i32; 6] = [7, 9, 9, 9, 9, 9];

    let mut current = MIN;
    let mut part1_valid_passwords = 0;
    let mut part2_valid_passwords = 0;
    loop {
        // Create vector with the amount of items in each number group.
        let mut counts = Vec::with_capacity(5);
        let mut index = 0;
        while index < 6 {
            let mut c = 0;
            let current_number = current[index];
            while index < 6 && current[index] == current_number {
                c += 1;
                index += 1;
            }
            counts.push(c);
        }

        // Check conditions for parts 1 & 2 validity.

        if counts.len() < 6 {
            part1_valid_passwords += 1;
        }

        if counts.contains(&2) {
            part2_valid_passwords += 1;
        }


        // Check from the left where to start incrementing
        let mut start_i = 5;
        while current[start_i] == 9 {
            start_i -= 1;
        }

        // Increment from left to right
        for i in start_i..6 {
            if current[i] == 9 {
                current[i] = current[i - 1];
            } else {
                current[i] += 1;
            }
        }

        if current == MAX {
            break;
        }
    }

    println!("[Part 1] Valid passwords: {:#4}", part1_valid_passwords);
    println!("[Part 2] Valid passwords: {:#4}", part2_valid_passwords);
}
