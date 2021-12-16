use std::collections::{VecDeque, HashSet};


fn game_value(player1_cards: &VecDeque<u16>, player2_cards: &VecDeque<u16>) -> u64 {
    let player1_value = player1_cards.iter().fold(0, |acc, n| acc * 10 + *n as u32);
    let player2_value = player2_cards.iter().fold(0, |acc, n| acc * 10 + *n as u32);

    ((player1_value as u64) << 32) | player2_value as u64
}


fn combat(mut player1_cards: VecDeque<u16>, mut player2_cards: VecDeque<u16>, recursive: bool) -> (bool, VecDeque<u16>) {
    let mut games = HashSet::new();

    while !player1_cards.is_empty() && !player2_cards.is_empty() {
        let game_value = game_value(&player1_cards, &player2_cards);
        if recursive && !games.insert(game_value) {
            // Game immediately ends in a win for player 1
            return (true, player1_cards);
        }

        // Let the players draw their cards
        let player1_value = player1_cards.pop_front().unwrap();
        let player2_value = player2_cards.pop_front().unwrap();

        let player1_won =
            if recursive && player1_value <= player1_cards.len() as u16 && player2_value <= player2_cards.len() as u16 {
                // Recursion to determine winning player
                let new_p1_cards = player1_cards.iter().take(player1_value as usize).cloned().collect();
                let new_p2_cards = player2_cards.iter().take(player2_value as usize).cloned().collect();

                combat(new_p1_cards, new_p2_cards, recursive).0
            } else {
                player1_value > player2_value
            };


        if player1_won {
            player1_cards.push_back(player1_value);
            player1_cards.push_back(player2_value);
        } else {
            player2_cards.push_back(player2_value);
            player2_cards.push_back(player1_value);
        }
    }

    let player1_won = !player1_cards.is_empty();
    let winning_cards =
        if player1_won {
            player1_cards
        } else {
            player2_cards
        };

    (player1_won, winning_cards)
}

fn get_score(winning_cards: &VecDeque<u16>) -> u32 {
    winning_cards.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, n)| acc + (i as u32 + 1) * *n as u32)
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .collect();

    // Fill players hands

    let mut player1_cards = VecDeque::with_capacity(input[0].lines().count() * 2);
    let mut player2_cards = VecDeque::with_capacity(input[0].lines().count() * 2);

    for card in input[0].lines().skip(1) {
        player1_cards.push_back(card.parse::<u16>().unwrap());
    }
    for card in input[1].lines().skip(1) {
        player2_cards.push_back(card.parse::<u16>().unwrap());
    }


    // Calculate the score for the winning cards

    let part1_winning_cards = combat(player1_cards.clone(), player2_cards.clone(), false).1;
    let part2_winning_cards = combat(player1_cards.clone(), player2_cards.clone(), true).1;

    println!("[Part 1] Winning score: {}", get_score(&part1_winning_cards));
    println!("[Part 2] Winning score: {}", get_score(&part2_winning_cards));
}
