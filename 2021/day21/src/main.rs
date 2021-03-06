use cached::proc_macro::cached;

// Test input
// const PLAYER1_STARTING: i32 = 4;
// const PLAYER2_STARTING: i32 = 8;
// Real input
const PLAYER1_STARTING: i32 = 5;
const PLAYER2_STARTING: i32 = 6;

pub struct DeterministicDie(i32);

impl DeterministicDie {
    pub fn new() -> Self {
        Self(1)
    }

    pub fn roll(&mut self) -> i32 {
        let value = self.0;
        self.0 += 1;
        if self.0 > 100 {
            self.0 = 1;
        }
        value
    }
}

impl Default for DeterministicDie {
    fn default() -> Self {
        Self::new()
    }
}


fn play_deterministic() -> i32 {
    let mut player_positions = [PLAYER1_STARTING - 1, PLAYER2_STARTING - 1];
    let mut player_scores = [0, 0];


    let mut round = 0;
    let mut die = DeterministicDie::new();
    while player_scores[0] < 1000 && player_scores[1] < 1000 {
        let player = round % 2;

        // Update player position
        player_positions[player] += die.roll() + die.roll() + die.roll();
        player_positions[player] %= 10;
        // Update player score
        player_scores[player] += player_positions[player] + 1;

        round += 1;
    }

    (round as i32) * 3 * player_scores[0].min(player_scores[1])
}

#[cached]
fn play_dirac(positions: [i32; 2], scores: [i32; 2], player1_turn: bool) -> [i64; 2] {
    if scores[0] >= 21 {
        return [1, 0];
    } else if scores[1] >= 21 {
        return [0, 1];
    }

    let player =
        if player1_turn {
            0
        } else {
            1
        };
    let mut p1_wins = 0;
    let mut p2_wins = 0;

    for (die_outcome, amount) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let mut new_position = positions;
        let mut new_score = scores;
        new_position[player] += die_outcome;
        new_position[player] %= 10;
        new_score[player] += new_position[player] + 1;

        let [p1, p2] = play_dirac(new_position, new_score, !player1_turn);
        p1_wins += p1 * amount;
        p2_wins += p2 * amount;
    }

    [p1_wins, p2_wins]
}


fn main() {
    // Part 1
    println!("Deterministic game score: {}", play_deterministic());
    // Part 2
    println!("Dirac game most wins: {:?}", play_dirac([PLAYER1_STARTING - 1, PLAYER2_STARTING - 1], [0, 0], true).iter().max().unwrap())
}
