use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i128;
use nom::combinator::map;
use nom::sequence::preceded;
use num::{Integer, BigInt, ToPrimitive};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    DealNew,
    Cut(i128),
    DealIncrement(i128),
}

impl Instruction {
    pub fn execute<T: Default + Clone + Copy>(&self, deck: &mut [T]) {
        match self {
            Instruction::DealNew => {
                deck.reverse();
            },
            Instruction::Cut(n) => {
                if *n >= 0 {
                    deck.rotate_left(*n as usize);
                } else {
                    deck.rotate_right(n.unsigned_abs() as usize);
                }
            },
            Instruction::DealIncrement(n) => {
                let mut tmp_deck = vec![T::default(); deck.len()];
                let mut current_index = 0;

                for card in deck.iter() {
                    tmp_deck[current_index] = *card;

                    current_index += *n as usize;
                    current_index %= deck.len();
                }

                deck.copy_from_slice(&tmp_deck);
            },
        }
    }

    pub fn parse(line: &str) -> IResult<&str, Self> {
        alt((
            map(tag("deal into new stack"), |_| Instruction::DealNew),
            map(preceded(tag("cut "), i128), Instruction::Cut),
            map(preceded(tag("deal with increment "), i128), Instruction::DealIncrement),
        ))(line)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Deck {
    top_card: i128,
    step: i128,
    size: i128,
}

impl Deck {
    // Deck: (t + sx) % size
    // t := top card
    // s := step size
    //
    // DealNew:
    // t <- (t + s(size - 1)) % size
    // s <- (s * -1).rem_euclid(size)
    //
    // Cut n:
    // t <- (t + n).rem_euclid(size)
    //
    // Deal increment n
    // Item at pos 1 is now at pos: i where ni % size = 1
    // s <- (t + si) % size - t

    pub fn new(size: i128) -> Self {
        Self { top_card: 0, step: 1, size }
    }

    pub fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::DealNew => {
                self.top_card += self.step * (self.size - 1);
                self.top_card = self.top_card.rem_euclid(self.size);

                self.step = (-self.step).rem_euclid(self.size);
            },
            Instruction::Cut(n) => {
                self.top_card = (self.top_card + (n * self.step)).rem_euclid(self.size);
            },
            Instruction::DealIncrement(n) => {
                let pos = n.extended_gcd(&self.size).x.rem_euclid(self.size);

                let pos_card = (self.top_card + self.step * pos) % self.size;
                self.step = (pos_card - self.top_card).rem_euclid(self.size);
            },
        }
    }
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| Instruction::parse(s).unwrap().1)
        .collect();

    // Part 1

    let mut deck: Vec<_> = (0..10_007_u32).collect();
    for instruction in &input {
        instruction.execute(&mut deck);
    }

    let pos = deck.iter().position(|c| *c == 2019).unwrap();
    println!("[Part 1] Position: {}", pos);

    // Part 2

    const DECK_SIZE: i128 = 119_315_717_514_047;
    const SHUFFLES: i128  = 101_741_582_076_661;

    let mut deck = Deck::new(DECK_SIZE);
    for instruction in &input {
        deck.apply(*instruction);
    }

    let t = BigInt::from(deck.top_card);
    let s = BigInt::from(deck.step);
    let m = BigInt::from(deck.size);


    // new_t := t + t * s + t * s * s + ... + t * s^(SHUFFLES - 1)
    // Can be solved with geometric series and modulo arithmetic
    let inverse = (&s - 1_i32).extended_gcd(&m).x.to_i128().unwrap().rem_euclid(deck.size);
    let tmp = ((s.modpow(&BigInt::from(SHUFFLES), &m) - 1_i32) * inverse) % deck.size;

    let new_t = (&t * &tmp) % &m;

    // new_s := s^(SHUFFLES)
    let new_s = s.modpow(&BigInt::from(SHUFFLES), &m);

    let card = (new_t + new_s * 2020) % deck.size;
    println!("[Part 2] Card: {}", card);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example1() {
        let mut deck = Deck::new(10);
        deck.apply(Instruction::DealIncrement(7));
        deck.apply(Instruction::DealNew);
        deck.apply(Instruction::DealNew);

        assert_eq!(deck, Deck { top_card: 0, step: 3, size: 10 });
    }

    #[test]
    pub fn example2() {
        let mut deck = Deck::new(10);
        deck.apply(Instruction::Cut(6));
        deck.apply(Instruction::DealIncrement(7));
        deck.apply(Instruction::DealNew);

        assert_eq!(deck, Deck { top_card: 3, step: 7, size: 10 });
    }

    #[test]
    pub fn example3() {
        let mut deck = Deck::new(10);
        deck.apply(Instruction::DealIncrement(7));
        deck.apply(Instruction::DealIncrement(9));
        deck.apply(Instruction::Cut(-2));

        assert_eq!(deck, Deck { top_card: 6, step: 7, size: 10 });
    }

    #[test]
    pub fn example4() {
        let mut deck = Deck::new(10);
        deck.apply(Instruction::DealNew);
        deck.apply(Instruction::Cut(-2));
        deck.apply(Instruction::DealIncrement(7));
        deck.apply(Instruction::Cut(8));
        deck.apply(Instruction::Cut(-4));
        deck.apply(Instruction::DealIncrement(7));
        deck.apply(Instruction::Cut(3));
        deck.apply(Instruction::DealIncrement(9));
        deck.apply(Instruction::DealIncrement(3));
        deck.apply(Instruction::Cut(-1));

        assert_eq!(deck, Deck { top_card: 9, step: 3, size: 10 });
    }

}
