use std::fmt;

pub struct Card {
    pub value: Value,
    pub suit: Suit
}

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
            Suit::Diamonds  =>  "D",
            Suit::Clubs     =>  "C",
            Suit::Hearts    =>  "H",
            Suit::Spades    =>  "S",
            Suit::None      =>  "-"
        };
        write!(f, "{}", suit)
    }
}
