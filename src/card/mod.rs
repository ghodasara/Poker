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

    pub fn get_id(&self) -> i64 {
        ((self.suit as u32) * 13 + (self.rank as u32)) as i64
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
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
            Rank::Ace      =>  "A"
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
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
            Suit::Spades    =>  "♠"
        };
        write!(f, "{}", suit)
    }
}

impl Suit {
    pub fn get_suit_mask(suit: Suit) -> i64 {
        (0b1111111111111 << (suit as u32)) as i64
    }
}
