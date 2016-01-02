use std::{fmt, cmp};

mod tests;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl cmp::Ord for Card {
    fn cmp(&self, other: &Card) -> cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl cmp::Eq for Card { }

impl cmp::PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<cmp::Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card {
            rank: rank,
            suit: suit
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Rank {
    None    =   0,
    Two     =   2,
    Three   =   3,
    Four    =   4,
    Five    =   5,
    Six     =   6,
    Seven   =   7,
    Eight   =   8,
    Nine    =   9,
    Ten     =   10,
    Jack    =   11,
    Queen   =   12,
    King    =   13,
    Ace     =   14
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            Rank::Two      =>  "2",
            Rank::Three    =>  "3",
            Rank::Four     =>  "4",
            Rank::Five     =>  "5",
            Rank::Six      =>  "6",
            Rank::Seven    =>  "7",
            Rank::Eight    =>  "8",
            Rank::Nine     =>  "9",
            Rank::Ten      =>  "T",
            Rank::Jack     =>  "J",
            Rank::Queen    =>  "Q",
            Rank::King     =>  "K",
            Rank::Ace      =>  "A",
            Rank::None     =>  "-"
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    None,
    Diamonds,
    Clubs,
    Hearts,
    Spades
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit = match *self {
            Suit::Diamonds  =>  "♢",
            Suit::Clubs     =>  "♣",
            Suit::Hearts    =>  "♡",
            Suit::Spades    =>  "♠",
            Suit::None      =>  "-"
        };
        write!(f, "{}", suit)
    }
}
