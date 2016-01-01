#[cfg(test)]
use ::{card, deck, hand};

#[test]
fn create_hand() {
    let mut deck = deck::Deck::new();
    let mut cards: Vec<card::Card> = vec![];
    for _i in 0..5 {
        cards.push(deck.draw().unwrap());
    }
    let hand = hand::Hand::new(cards);

    assert_eq!(hand.cards.len(), 5);
}

#[test]
fn has_flush() {
    let a = card::Card::new(card::Value::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Value::Six, card::Suit::Hearts);
    let c = card::Card::new(card::Value::Ten, card::Suit::Hearts);
    let d = card::Card::new(card::Value::Queen, card::Suit::Hearts);
    let e = card::Card::new(card::Value::Two, card::Suit::Hearts);
    let cards: Vec<card::Card> = vec![a, b, c, d, e];
    let hand: hand::Hand = hand::Hand::new(cards);

    assert_eq!(hand.has_flush(), true);
}

#[test]
fn no_flush() {
    let a = card::Card::new(card::Value::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Value::Six, card::Suit::Spades);
    let c = card::Card::new(card::Value::Ten, card::Suit::Hearts);
    let d = card::Card::new(card::Value::Queen, card::Suit::Hearts);
    let e = card::Card::new(card::Value::Two, card::Suit::Hearts);
    let cards: Vec<card::Card> = vec![a, b, c, d, e];
    let hand: hand::Hand = hand::Hand::new(cards);

    assert_eq!(hand.has_flush(), false);
}

#[test]
fn high_card_raw_value() {
    let a = card::Card::new(card::Value::Three, card::Suit::Hearts);
    let poker_hand = hand::PokerHand::HighCard(a);

    assert_eq!(poker_hand.raw_value(), 0);
}

#[test]
fn one_pair_raw_value() {
    let a = card::Card::new(card::Value::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Value::Three, card::Suit::Diamonds);
    let poker_hand = hand::PokerHand::OnePair((a, b));

    assert_eq!(poker_hand.raw_value(), 1);
}

#[test]
fn two_pair_raw_value() {
    let a = card::Card::new(card::Value::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Value::Three, card::Suit::Diamonds);
    let c = card::Card::new(card::Value::King, card::Suit::Spades);
    let d = card::Card::new(card::Value::King, card::Suit::Hearts);
    let poker_hand = hand::PokerHand::TwoPair((a, b), (c, d));

    assert_eq!(poker_hand.raw_value(), 2);
}

#[test]
fn trips_raw_value() {
    let a = card::Card::new(card::Value::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Value::Three, card::Suit::Diamonds);
    let c = card::Card::new(card::Value::Three, card::Suit::Spades);
    let poker_hand = hand::PokerHand::Trips((a, b, c));

    assert_eq!(poker_hand.raw_value(), 3);
}

#[test]
fn straight_raw_value() {
    let a = card::Card::new(card::Value::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Value::Four, card::Suit::Diamonds);
    let c = card::Card::new(card::Value::Five, card::Suit::Spades);
    let d = card::Card::new(card::Value::Six, card::Suit::Hearts);
    let e = card::Card::new(card::Value::Seven, card::Suit::Hearts);
    let poker_hand = hand::PokerHand::Straight((a, b, c, d, e));

    assert_eq!(poker_hand.raw_value(), 4);
}

#[test]
fn flush_raw_value() {
    let a = card::Card::new(card::Value::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Value::King, card::Suit::Hearts);
    let c = card::Card::new(card::Value::Five, card::Suit::Hearts);
    let d = card::Card::new(card::Value::Jack, card::Suit::Hearts);
    let e = card::Card::new(card::Value::Ten, card::Suit::Hearts);
    let poker_hand = hand::PokerHand::Flush((a, b, c, d, e));

    assert_eq!(poker_hand.raw_value(), 5);
}

#[test]
fn full_house_raw_value() {
    let a = card::Card::new(card::Value::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Value::Three, card::Suit::Spades);
    let c = card::Card::new(card::Value::Three, card::Suit::Clubs);
    let d = card::Card::new(card::Value::Ten, card::Suit::Hearts);
    let e = card::Card::new(card::Value::Ten, card::Suit::Diamonds);
    let poker_hand = hand::PokerHand::FullHouse((a, b, c), (d, e));

    assert_eq!(poker_hand.raw_value(), 6);
}

#[test]
fn quads_raw_value() {
    let a = card::Card::new(card::Value::Three, card::Suit::Diamonds);
    let b = card::Card::new(card::Value::Three, card::Suit::Spades);
    let c = card::Card::new(card::Value::Three, card::Suit::Clubs);
    let d = card::Card::new(card::Value::Three, card::Suit::Hearts);
    let poker_hand = hand::PokerHand::Quads((a, b, c, d));

    assert_eq!(poker_hand.raw_value(), 7);
}

#[test]
fn straight_flush_raw_value() {
    let a = card::Card::new(card::Value::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Value::Four, card::Suit::Hearts);
    let c = card::Card::new(card::Value::Five, card::Suit::Hearts);
    let d = card::Card::new(card::Value::Six, card::Suit::Hearts);
    let e = card::Card::new(card::Value::Seven, card::Suit::Hearts);
    let poker_hand = hand::PokerHand::StraightFlush((a, b, c, d, e));

    assert_eq!(poker_hand.raw_value(), 8);
}
