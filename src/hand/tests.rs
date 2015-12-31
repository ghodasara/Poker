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
    let a = Card {
        value: Value::Three,
        suit: Suit::Hearts
    };
    let b = Card {
        value: Value::Six,
        suit: Suit::Hearts
    };
    let c = Card {
        value: Value::Ten,
        suit: Suit::Hearts
    };
    let d = Card {
        value: Value::Queen,
        suit: Suit::Hearts
    };
    let e = Card {
        value: Value::Two,
        suit: Suit::Hearts
    };
    let cards: Vec<Card> = vec![a, b, c, d, e];
    let hand: ::hand::Hand = ::hand::Hand::new(cards);

    assert_eq!(hand.has_flush(), true);
}

#[test]
fn no_flush() {
    let a = Card {
        value: Value::Three,
        suit: Suit::Hearts
    };
    let b = Card {
        value: Value::Six,
        suit: Suit::Spades
    };
    let c = Card {
        value: Value::Ten,
        suit: Suit::Hearts
    };
    let d = Card {
        value: Value::Queen,
        suit: Suit::Hearts
    };
    let e = Card {
        value: Value::Two,
        suit: Suit::Hearts
    };
    let cards: Vec<Card> = vec![a, b, c, d, e];
    let hand: ::hand::Hand = ::hand::Hand::new(cards);

    assert_eq!(hand.has_flush(), false);
}
