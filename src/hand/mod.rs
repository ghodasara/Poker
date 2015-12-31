use card;
use std::fmt;

mod tests;

pub struct Hand {
    pub cards: Vec<card::Card>
}

impl Hand {
    pub fn new(mut cards: Vec<card::Card>) -> Hand {
        cards.sort();
        Hand {
            cards: cards
        }
    }

    pub fn has_flush(&self) -> bool {
        let suit: card::Suit = self.cards.get(0).unwrap().suit.clone();
        for card in &self.cards {
            if card.suit != suit {
                return false
            }
        }
        true
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

#[derive(Debug, PartialEq)]
pub enum PokerHand {
    HighCard      = 0,
    OnePair       = 1,
    TwoPair       = 2,
    Trips         = 3,
    Straight      = 4,
    Flush         = 5,
    FullHouse     = 6,
    Quads         = 7,
    StraightFlush = 8,
}
