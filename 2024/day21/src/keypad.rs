use itertools::Itertools;


pub trait Keypad {
    fn button_position(button: char) -> (usize, usize);

    fn forbidden_position() -> (usize, usize);


    fn move_sequences(start: char, end: char) -> Vec<Vec<char>> {
        let (start_x, start_y) = Self::button_position(start);
        let (end_x, end_y) = Self::button_position(end);

        let delta_x = end_x as i32 - start_x as i32;
        let delta_y = end_y as i32 - start_y as i32;

        let mut sequence = vec![];
        if delta_x > 0 {
            sequence.extend(vec!['>'; delta_x as usize]);
        } else {
            sequence.extend(vec!['<'; delta_x.abs() as usize]);
        }
        if delta_y > 0 {
            sequence.extend(vec!['v'; delta_y as usize]);
        } else {
            sequence.extend(vec!['^'; delta_y.abs() as usize]);
        }

        let mut valid_sequences = Vec::new();
        'iter_moves: for mut p in sequence.iter().cloned().permutations(sequence.len()) {
            let mut current_position = (start_x as i32, start_y as i32);

            for &step in &p {
                match step {
                    '>' => current_position.0 += 1,
                    '<' => current_position.0 -= 1,
                    'v' => current_position.1 += 1,
                    '^' => current_position.1 -= 1,
                    _ => panic!("Invalid step: {}", step),
                }

                if (current_position.0 as usize, current_position.1 as usize) == Self::forbidden_position() {
                    continue 'iter_moves;
                }
            }

            // Add the confirmation button
            p.push('A');
            // And collect the valid sequence
            valid_sequences.push(p);
        }

        valid_sequences
    }
}

pub struct NumericKeypad;

impl Keypad for NumericKeypad {
    fn button_position(button: char) -> (usize, usize) {
        match button {
            '7' => (0, 0),
            '8' => (1, 0),
            '9' => (2, 0),
            '4' => (0, 1),
            '5' => (1, 1),
            '6' => (2, 1),
            '1' => (0, 2),
            '2' => (1, 2),
            '3' => (2, 2),
            '0' => (1, 3),
            'A' => (2, 3),
            _ => panic!("Invalid button: {}", button),
        }
    }

    fn forbidden_position() -> (usize, usize) {
        (0, 3)
    }
}

pub struct DirectionalKeypad;

impl Keypad for DirectionalKeypad {
    fn button_position(button: char) -> (usize, usize) {
        match button {
            '^' => (1, 0),
            'A' => (2, 0),
            '<' => (0, 1),
            'v' => (1, 1),
            '>' => (2, 1),
            _ => panic!("Invalid button: {}", button),
        }
    }

    fn forbidden_position() -> (usize, usize) {
        (0, 0)
    }
}
