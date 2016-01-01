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
    HighCard((card::Card)),
    OnePair((card::Card, card::Card)),
    TwoPair((card::Card, card::Card), (card::Card)),
    Trips((card::Card, card::Card, card::Card)),
    Straight((card::Card, card::Card, card::Card, card::Card, card::Card)),
    Flush((card::Card, card::Card, card::Card, card::Card, card::Card)),
    FullHouse((card::Card, card::Card, card::Card), (card::Card, card::Card)),
    Quads((card::Card, card::Card, card::Card, card::Card)),
    StraightFlush((card::Card, card::Card, card::Card, card::Card, card::Card)),
}

impl PokerHand {
    fn raw_value(&self) -> u32 {
        match *self {
            PokerHand::HighCard(_) => 0,
            PokerHand::OnePair(_) => 1,
            PokerHand::TwoPair(_, _) => 2,
            PokerHand::Trips(_) => 3,
            PokerHand::Straight(_) => 4,
            PokerHand::Flush(_) => 5,
            PokerHand::FullHouse(_, _) => 6,
            PokerHand::Quads(_) => 7,
            PokerHand::StraightFlush(_) => 8,
        }
    }
}
