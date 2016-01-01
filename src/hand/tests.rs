#[cfg(test)]
use ::card::*;

#[test]
fn create_hand() {
    let mut deck = ::deck::Deck::new();
    let mut cards: Vec<::card::Card> = vec![];
    for _i in 0..5 {
        cards.push(deck.draw().unwrap());
    }
    let hand = ::hand::Hand::new(cards);

    assert_eq!(hand.cards.len(), 5);
}

#[test]
fn has_flush() {
    let a = Card::new(Value::Three, Suit::Hearts);
    let b = Card::new(Value::Six, Suit::Hearts);
    let c = Card::new(Value::Ten, Suit::Hearts);
    let d = Card::new(Value::Queen, Suit::Hearts);
    let e = Card::new(Value::Two, Suit::Hearts);
    let cards: Vec<Card> = vec![a, b, c, d, e];
    let hand: ::hand::Hand = ::hand::Hand::new(cards);

    assert_eq!(hand.has_flush(), true);
}

#[test]
fn no_flush() {
    let a = Card::new(Value::Three, Suit::Hearts);
    let b = Card::new(Value::Six, Suit::Spades);
    let c = Card::new(Value::Ten, Suit::Hearts);
    let d = Card::new(Value::Queen, Suit::Hearts);
    let e = Card::new(Value::Two, Suit::Hearts);
    let cards: Vec<Card> = vec![a, b, c, d, e];
    let hand: ::hand::Hand = ::hand::Hand::new(cards);

    assert_eq!(hand.has_flush(), false);
}

#[test]
fn high_card_raw_value() {

}
