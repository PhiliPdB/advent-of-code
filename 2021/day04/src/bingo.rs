use std::collections::HashMap;

#[derive(Debug)]
pub struct Bingo {
    board: [[i32; 5]; 5],
    marked_fields: [[bool; 5]; 5],
    unmarked_numbers: HashMap<i32, (usize, usize)>,
}

impl Bingo {
    pub fn new(numbers: &[&str]) -> Self {
        assert_eq!(numbers.len(), 5);

        let mut board = [[0; 5]; 5];
        for i in 0..5 {
            board[i] = numbers[i]
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        }
        let board = board;

        let mut unmarked_numbers = HashMap::with_capacity(25);
        for i in 0..5 {
            for j in 0..5 {
                unmarked_numbers.insert(board[i][j], (i, j));
            }
        }

        Self {
            board, unmarked_numbers,
            marked_fields: [[false; 5]; 5],
        }
    }

    pub fn mark_number(&mut self, number: i32) -> bool {
        if !self.unmarked_numbers.contains_key(&number) {
            // Number is not on this board
            return false;
        }

        // Remove from the unmarked number and get it's coordinates
        let (x, y) = self.unmarked_numbers.remove(&number).unwrap();
        self.marked_fields[x][y] = true;

        self.check_bingo()
    }

    pub fn get_score(&self) -> i32 {
        self.unmarked_numbers.keys().sum()
    }


    fn check_bingo(&self) -> bool {
        // Check rows
        if self.marked_fields.iter().any(|row| row.iter().all(|&n| n)) {
            true
        } else {
            // Check columns
            let mut has_column_bingo = false;
            for i in 0..5 {
                has_column_bingo = has_column_bingo || self.marked_fields.iter()
                    .all(|row| row[i]);
            }

            has_column_bingo
        }
    }
}
