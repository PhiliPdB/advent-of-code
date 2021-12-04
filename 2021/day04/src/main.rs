mod bingo;

use bingo::Bingo;

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .filter(|s| !s.is_empty())
        .collect();

    // Parse the bingo numbers
    let bingo_numbers: Vec<_> = input[0]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Get all the bingo boards
    let mut bingo_boards = Vec::new();
    for i in (1..input.len()).step_by(5) {
        bingo_boards.push(Bingo::new(&input[i..i+5]));
    }

    // Keep track of the bingo scores
    let mut bingo_scores = Vec::with_capacity(bingo_boards.len());

    // Start marking numbers
    for n in bingo_numbers {
        let bingos: Vec<_> = bingo_boards.iter_mut()
            // Modify the board to mark the number, and collect the bingo result
            .map(|board| board.mark_number(n))
            .collect();

        // Filter out the non-bingos, while keeping track of the board indeces with a bingo.
        let bingo_indices: Vec<_> = bingos.iter()
            .enumerate()
            .filter_map(|(i, &b)| b.then(|| i))
            .collect();

        // Print score for each board that has a bingo
        for board_index in bingo_indices {
            bingo_scores.push(n * bingo_boards[board_index].get_score());
        }

        // Retain the boards that don't have a bingo
        bingo_boards.retain(|board| !board.check_bingo());
    }


    println!("Best board score:  {}", bingo_scores[0]);
    println!("Worst board score: {}", bingo_scores[bingo_scores.len() - 1]);
}
