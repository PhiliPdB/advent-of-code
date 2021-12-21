
// Test input
// const PLAYER1_STARTING: i32 = 4;
// const PLAYER2_STARTING: i32 = 8;
// Real input
const PLAYER1_STARTING: i32 = 5;
const PLAYER2_STARTING: i32 = 6;

struct Die(i32);

impl Die {
    fn roll(&mut self) -> i32 {
        let value = self.0;
        self.0 += 1;
        if self.0 > 100 {
            self.0 = 1;
        }
        value
    }
}


fn main() {
    let mut player_positions = [PLAYER1_STARTING - 1, PLAYER2_STARTING - 1];
    let mut player_scores = [0, 0];


    let mut round = 0;
    let mut die = Die(1);
    while player_scores[0] < 1000 && player_scores[1] < 1000 {
        let player = round % 2;

        // Update player position
        player_positions[player] += die.roll() + die.roll() + die.roll();
        player_positions[player] %= 10;
        // Update player score
        player_scores[player] += player_positions[player] + 1;

        round += 1;
    }

    let score = (round as i32) * 3 * player_scores[0].min(player_scores[1]);
    println!("{} {}", round * 3, player_scores[0].min(player_scores[1]));
    println!("Part 1: {}", score);
}
