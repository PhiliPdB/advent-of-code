mod bingo;

use bingo::Bingo;

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .filter(|s| !s.is_empty())
        .collect();


    let bingo_numbers: Vec<_> = input[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut bingo_boards = Vec::new();
    for i in (1..input.len()).step_by(5) {
        bingo_boards.push(Bingo::new(&input[i..i+5]));
    }

    // Start marking numbers
    for n in bingo_numbers {
        let bingos: Vec<_> = bingo_boards.iter_mut()
            // Modify the board to mark the number, and collect the bingo result
            .map(|board| board.mark_number(n))
            .collect();

        // Check if there is a bingo
        if let Some(board_index) = bingos.iter().position(|&b| b) {
            println!("Bingo with score: {}", n * bingo_boards[board_index].get_score());
            break;
        }
    }
}
