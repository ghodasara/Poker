use std::{fmt, cmp};

#[derive(Debug, PartialEq)]
pub struct Card {
    pub value: Value,
    pub suit: Suit
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

impl cmp::PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Value {
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            Value::Two      =>  "2",
            Value::Three    =>  "3",
            Value::Four     =>  "4",
            Value::Five     =>  "5",
            Value::Six      =>  "6",
            Value::Seven    =>  "7",
            Value::Eight    =>  "8",
            Value::Nine     =>  "9",
            Value::Ten      =>  "T",
            Value::Jack     =>  "J",
            Value::Queen    =>  "Q",
            Value::King     =>  "K",
            Value::Ace      =>  "A",
            Value::None     =>  "-"
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
