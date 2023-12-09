use std::str::FromStr;
use std::collections::HashSet;


#[derive(Debug)]
struct ScratchCard {
    _id: u32,
    winning_numbers: HashSet<u32>,
    my_numbers: Vec<u32>,
}

impl ScratchCard {
    fn points(&self) -> u32 {
        let mut points = 0;
        for n in &self.my_numbers {
            if self.winning_numbers.contains(n) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        points
    }

    fn matching_numbers(&self) -> usize {
        self.my_numbers.iter()
            .filter(|n| self.winning_numbers.contains(*n))
            .count()
    }
}

impl FromStr for ScratchCard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [card_id, numbers] = s.split(':').collect::<Vec<_>>().try_into()
            .map_err(|_| "Invalid card format")?;
        let card_id: u32 = card_id[5..].trim().parse()
            .map_err(|_| "Unable to parse card id")?;

        let [winning, selected] = numbers.split(" | ").collect::<Vec<_>>().try_into()
            .map_err(|_| "Unable to parse numbers")?;
        let winning: HashSet<u32> = winning.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let selected: Vec<u32> = selected.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(ScratchCard { _id: card_id, winning_numbers: winning, my_numbers: selected })
    }
}

fn main() {
    let cards: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| ScratchCard::from_str(l).unwrap())
        .collect();

    let sum_of_points: u32 = cards.iter()
        .map(|c| c.points())
        .sum();
    println!("[Part 1] Total point worth: {sum_of_points}");

    let mut card_count = vec![1; cards.len()];
    for i in 0..cards.len() {
        let card = &cards[i];
        let total_cards = card_count[i];

        let matches = card.matching_numbers();
        for count in card_count.iter_mut().skip(i+1).take(matches) {
            *count += total_cards;
        }
    }
    println!("[Part 2] Total cards: {}", card_count.iter().sum::<u32>());
}
