use card;
use std::fmt;

mod tests;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    pub cards: Vec<card::Card>,
}

impl Hand {
    pub fn new(mut cards: Vec<card::Card>) -> Hand {
        cards.sort();
        Hand {
            cards: cards,

        }
    }

    fn has_flush(&self) -> bool {
        let mut suits = 0;
        for card in &self.cards {
            suits = suits | (1 << card.suit as u32);
        }
        match suits {
            0b0001 => true,
            0b0010 => true,
            0b0100 => true,
            0b1000 => true,
            _ => false
        }
    }

    fn has_straight(&self) -> bool {
        let mut ranks = 0;
        for card in &self.cards {
            ranks = ranks | (1 << card.rank as u32);
        }

        let straight_matcher = 0b11111;
        let ace_low_matcher = 0b1000000001111;
        if ranks ^ ace_low_matcher == 0 {
            return true;
        }
        for i in 0..8 {
            if ranks ^ (straight_matcher << i) == 0 {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} {}", self.cards.get(0).unwrap(),
                                    self.cards.get(1).unwrap(),
                                    self.cards.get(2).unwrap(),
                                    self.cards.get(3).unwrap(),
                                    self.cards.get(4).unwrap())
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum PokerRanking {
    HighCard,
    OnePair,
    TwoPair,
    Trips,
    Straight,
    Flush,
    FullHouse,
    Quads,
    StraightFlush,
}
