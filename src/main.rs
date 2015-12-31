extern crate poker;

use poker::{deck, hand, card};

fn main() {
    let mut deck = deck::Deck::new();
    let mut cards: Vec<card::Card> = vec![];
    for _i in 0..5 {
        cards.push(deck.draw().unwrap());
    }
    let hand = hand::Hand::new(cards);
    println!("{}", hand);
}
