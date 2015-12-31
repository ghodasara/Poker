use card;

mod tests;

pub struct Hand {
    pub cards: Vec<card::Card>
}

impl Hand {
    pub fn new(cards: Vec<card::Card>) -> Hand {
        Hand {
            cards: cards
        }
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
