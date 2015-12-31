extern crate poker;

use poker::deck;

fn main() {
    let deck = deck::Deck::new();
    println!("{}", deck.cards[0].suit);
}
