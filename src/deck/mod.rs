extern crate rand;

use self::rand::{thread_rng, Rng};
use card;

mod tests;

#[allow(dead_code)]
pub struct Deck {
    pub cards: Vec<card::Card>
}

#[allow(dead_code)]
impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<card::Card> = vec![];

        for value in 0..13 {
            for suit in 0..4 {
                let value = match value {
                    0   =>  card::Value::Two,
                    1   =>  card::Value::Three,
                    2   =>  card::Value::Four,
                    3   =>  card::Value::Five,
                    4   =>  card::Value::Six,
                    5   =>  card::Value::Seven,
                    6   =>  card::Value::Eight,
                    7   =>  card::Value::Nine,
                    8   =>  card::Value::Ten,
                    9   =>  card::Value::Jack,
                    10  =>  card::Value::Queen,
                    11  =>  card::Value::King,
                    12  =>  card::Value::Ace,
                    _   =>  card::Value::None
                };
                let suit = match suit {
                    0   =>  card::Suit::Diamonds,
                    1   =>  card::Suit::Clubs,
                    2   =>  card::Suit::Hearts,
                    3   =>  card::Suit::Spades,
                    _   =>  card::Suit::None
                };
                cards.push(card::Card {
                    value: value,
                    suit: suit
                });
            }
        }

        thread_rng().shuffle(&mut cards);

        Deck {
            cards: cards
        }
    }
}
