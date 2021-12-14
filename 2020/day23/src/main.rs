
const NUMBERS: u32 = 9;


// const INPUT: [u32; 9] = [9, 1, 6, 4, 3, 8, 2, 7, 5];

fn main() {
    // let mut input: [u32; NUMBERS as usize] = [3, 8, 9, 1, 2, 5, 4, 6, 7]
    let mut input: Vec<_> = [9, 1, 6, 4, 3, 8, 2, 7, 5].iter()
        .map(|i| i - 1)
        .collect();


    for _move in 0..100 {
        let target = {
            let mut target = input[0] - 1;
            if target > NUMBERS {
                target = NUMBERS - 1;
            }

            while input[1..4].contains(&target) {
                target -= 1;
                if target > NUMBERS {
                    target = NUMBERS - 1;
                }
            }
            target
        };

        // println!("Target: {}", target);
        let target_location = input.iter().position(|i| *i == target).unwrap() - 1;
        input.rotate_left(1);
        // println!("Location: {} in {:?}", target_location, input);

        let sub = target_location - 2;
        let picked_up = &input[0..3].to_owned();
        for i in 0..sub {
            input[i] = input[i + 3];
        }
        // println!("{:?}", input);
        for (i, cup) in picked_up.iter().enumerate() {
            input[sub + i] = *cup;
        }

        // println!("Input: {:?}", &input);

        // TODO

        // Update current cup
        // current_cup += 1;
        // current_cup %= NUMBERS;
    }

    while input[0] != 0 {
        input.rotate_left(1);
    }

    println!("Result: {}", input.iter().skip(1).map(|i| i +  1).fold(0, |acc, n| acc * 10 + n));
}
