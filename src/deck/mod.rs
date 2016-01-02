extern crate rand;

use self::rand::{thread_rng, Rng};
use card;

mod tests;

pub struct Deck {
    pub cards: Vec<card::Card>
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<card::Card> = vec![];

        for rank in 0..13 {
            for suit in 0..4 {
                let rank = match rank {
                    0   =>  card::Rank::Two,
                    1   =>  card::Rank::Three,
                    2   =>  card::Rank::Four,
                    3   =>  card::Rank::Five,
                    4   =>  card::Rank::Six,
                    5   =>  card::Rank::Seven,
                    6   =>  card::Rank::Eight,
                    7   =>  card::Rank::Nine,
                    8   =>  card::Rank::Ten,
                    9   =>  card::Rank::Jack,
                    10  =>  card::Rank::Queen,
                    11  =>  card::Rank::King,
                    12  =>  card::Rank::Ace,
                    _   =>  card::Rank::None
                };
                let suit = match suit {
                    0   =>  card::Suit::Diamonds,
                    1   =>  card::Suit::Clubs,
                    2   =>  card::Suit::Hearts,
                    3   =>  card::Suit::Spades,
                    _   =>  card::Suit::None
                };
                cards.push(card::Card {
                    rank: rank,
                    suit: suit
                });
            }
        }

        thread_rng().shuffle(&mut cards);

        Deck {
            cards: cards
        }
    }

    pub fn draw(&mut self) -> Option<card::Card> {
        let len = self.cards.len();
        if len == 0 {
            None
        } else {
            Some(self.cards.remove(len - 1))
        }
    }
}
