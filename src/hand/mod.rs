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
        let suit_mask: i64 = card::Suit::get_suit_mask(self.cards.first().unwrap().suit);
        for card in &self.cards[1..] {
            if (-suit_mask) & card.get_mask() != 0 {
                return false;
            }
        }
        true
    }

    fn has_straight(&self) -> bool {
        let mut ranks: i64 = 0;
        for card in &self.cards {
            ranks |= 1 << card.rank as i64;
        }

        let straight_matcher: i64 = 0b11111;
        let ace_low_matcher: i64 = 0b1000000001111;
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

    fn has_pair(&self) -> bool {
        let mut ranks: i64 = 1 << self.cards.first().unwrap().rank as i64;
        for card in &self.cards[1..] {
            if ranks | (1 << card.rank as i64) == ranks {
                return true
            }
            ranks |= (1 << card.rank as i64);
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
