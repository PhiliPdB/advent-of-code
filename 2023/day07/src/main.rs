use std::cmp::Ordering;
use std::collections::HashMap;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Ace, King, Queen, Jack,
    Ten, Nine, Eight, Seven, Six, Five, Four, Three, Two,
    Joker,
}

impl Card {
    fn from_char(c: char, with_jokers: bool) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => if with_jokers {
                Self::Joker
            } else {
                Self::Jack
            },
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Could not match card: {c}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind, FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair, OnePair,
    HighCard
}

impl HandType {
    fn from_cards(cards: &[Card; 5]) -> Self {
        let mut count: HashMap<Card, u32> = HashMap::new();

        for card in cards {
            *count.entry(*card).or_default() += 1;
        }

        let jokers = count.remove(&Card::Joker).unwrap_or(0);

        let mut count: Vec<_> = count.into_values()
            .collect();
        count.sort_unstable_by(|a, b| b.cmp(a));

        if count.is_empty() {
            count.push(0);
        }

        if count[0] + jokers == 5 {
            Self::FiveOfAKind
        } else if count[0] + jokers == 4 {
            Self::FourOfAKind
        } else if count[0] + jokers == 3 && count[1] == 2 {
            Self::FullHouse
        } else if count[0] + jokers == 3 {
            Self::ThreeOfAKind
        } else if count[0] + jokers == 2 && count[1] == 2 {
            Self::TwoPair
        } else if count[0] + jokers == 2 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: [Card; 5], bid: u32) -> Self {
        let hand_type = HandType::from_cards(&cards);

        Self { cards, bid, hand_type }
    }

    fn from_str(s: &str, with_jokers: bool) -> Result<Self, &'static str> {
        let [cards, bid] = s.split(' ').collect::<Vec<_>>().try_into()
            .map_err(|_| "Invalid format")?;

        let cards = cards.chars()
            .map(|c| Card::from_char(c, with_jokers))
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "Error parsing cards")?;
        let bid = bid.parse().map_err(|_| "Bid is not a number")?;

        Ok(Self::new(cards, bid))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type)
            .then(self.cards[0].cmp(&other.cards[0]))
            .then(self.cards[1].cmp(&other.cards[1]))
            .then(self.cards[2].cmp(&other.cards[2]))
            .then(self.cards[3].cmp(&other.cards[3]))
            .then(self.cards[4].cmp(&other.cards[4]))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand_type == other.hand_type
    }
}

fn main() {
    let hands: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| (Hand::from_str(l, false).unwrap(), Hand::from_str(l, true).unwrap()))
        .collect();

    let mut part1_hands: Vec<_> = hands.iter()
        .map(|(h, _)| *h)
        .collect();
    part1_hands.sort_unstable_by(|a, b| b.cmp(a));

    let part1_winnings: u32 = part1_hands.into_iter().enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .sum();
    println!("[Part 1] Winnings: {part1_winnings}");


    let mut part2_hands: Vec<_> = hands.iter()
        .map(|(_, h)| *h)
        .collect();
    part2_hands.sort_unstable_by(|a, b| b.cmp(a));

    let part2_winnings: u32 = part2_hands.into_iter().enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .sum();
    println!("[Part 2] Winnings: {part2_winnings}");
}
