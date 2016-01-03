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
    let a = card::Card::new(card::Rank::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Rank::Six, card::Suit::Hearts);
    let c = card::Card::new(card::Rank::Ten, card::Suit::Hearts);
    let d = card::Card::new(card::Rank::Queen, card::Suit::Hearts);
    let e = card::Card::new(card::Rank::Two, card::Suit::Hearts);
    let cards: Vec<card::Card> = vec![a, b, c, d, e];
    let hand: hand::Hand = hand::Hand::new(cards);

    assert_eq!(hand.has_flush(), true);
}

#[test]
fn no_flush() {
    let a = card::Card::new(card::Rank::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Rank::Six, card::Suit::Spades);
    let c = card::Card::new(card::Rank::Ten, card::Suit::Hearts);
    let d = card::Card::new(card::Rank::Queen, card::Suit::Hearts);
    let e = card::Card::new(card::Rank::Two, card::Suit::Hearts);
    let cards: Vec<card::Card> = vec![a, b, c, d, e];
    let hand: hand::Hand = hand::Hand::new(cards);

    assert_eq!(hand.has_flush(), false);
}

#[test]
fn has_straight() {
    let a = card::Card::new(card::Rank::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Rank::Six, card::Suit::Diamonds);
    let c = card::Card::new(card::Rank::Four, card::Suit::Hearts);
    let d = card::Card::new(card::Rank::Seven, card::Suit::Clubs);
    let e = card::Card::new(card::Rank::Five, card::Suit::Spades);
    let cards: Vec<card::Card> = vec![a, b, c, d, e];
    let hand: hand::Hand = hand::Hand::new(cards);

    assert_eq!(hand.has_straight(), true);
}

#[test]
fn has_ace_low_straight() {
    let a = card::Card::new(card::Rank::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Rank::Two, card::Suit::Diamonds);
    let c = card::Card::new(card::Rank::Four, card::Suit::Hearts);
    let d = card::Card::new(card::Rank::Ace, card::Suit::Clubs);
    let e = card::Card::new(card::Rank::Five, card::Suit::Spades);
    let cards: Vec<card::Card> = vec![a, b, c, d, e];
    let hand: hand::Hand = hand::Hand::new(cards);

    assert_eq!(hand.has_straight(), true);
}

#[test]
fn no_straight() {
    let a = card::Card::new(card::Rank::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Rank::Six, card::Suit::Diamonds);
    let c = card::Card::new(card::Rank::King, card::Suit::Hearts);
    let d = card::Card::new(card::Rank::Seven, card::Suit::Clubs);
    let e = card::Card::new(card::Rank::Five, card::Suit::Spades);
    let cards: Vec<card::Card> = vec![a, b, c, d, e];
    let hand: hand::Hand = hand::Hand::new(cards);

    assert_eq!(hand.has_straight(), false);
}

#[test]
fn has_pair() {
    let a = card::Card::new(card::Rank::Seven, card::Suit::Hearts);
    let b = card::Card::new(card::Rank::Six, card::Suit::Diamonds);
    let c = card::Card::new(card::Rank::King, card::Suit::Hearts);
    let d = card::Card::new(card::Rank::Seven, card::Suit::Clubs);
    let e = card::Card::new(card::Rank::Five, card::Suit::Spades);
    let cards: Vec<card::Card> = vec![a, b, c, d, e];
    let hand: hand::Hand = hand::Hand::new(cards);

    assert_eq!(hand.has_pair(), true);
}

#[test]
fn has_pair_in_set() {
    let a = card::Card::new(card::Rank::Seven, card::Suit::Hearts);
    let b = card::Card::new(card::Rank::Six, card::Suit::Diamonds);
    let c = card::Card::new(card::Rank::King, card::Suit::Hearts);
    let d = card::Card::new(card::Rank::Seven, card::Suit::Clubs);
    let e = card::Card::new(card::Rank::Seven, card::Suit::Spades);
    let cards: Vec<card::Card> = vec![a, b, c, d, e];
    let hand: hand::Hand = hand::Hand::new(cards);

    assert_eq!(hand.has_pair(), true);
}

#[test]
fn no_pair() {
    let a = card::Card::new(card::Rank::Three, card::Suit::Hearts);
    let b = card::Card::new(card::Rank::Six, card::Suit::Diamonds);
    let c = card::Card::new(card::Rank::King, card::Suit::Hearts);
    let d = card::Card::new(card::Rank::Seven, card::Suit::Clubs);
    let e = card::Card::new(card::Rank::Five, card::Suit::Spades);
    let cards: Vec<card::Card> = vec![a, b, c, d, e];
    let hand: hand::Hand = hand::Hand::new(cards);

    assert_eq!(hand.has_pair(), false);
}

#[test]
fn high_card_raw_value() {
    assert_eq!(hand::PokerRanking::HighCard as u32, 0);
}

#[test]
fn one_pair_raw_value() {
    assert_eq!(hand::PokerRanking::OnePair as u32, 1);
}

#[test]
fn two_pair_raw_value() {
    assert_eq!(hand::PokerRanking::TwoPair as u32, 2);
}

#[test]
fn trips_raw_value() {
    assert_eq!(hand::PokerRanking::Trips as u32, 3);
}

#[test]
fn straight_raw_value() {
    assert_eq!(hand::PokerRanking::Straight as u32, 4);
}

#[test]
fn flush_raw_value() {
    assert_eq!(hand::PokerRanking::Flush as u32, 5);
}

#[test]
fn full_house_raw_value() {
    assert_eq!(hand::PokerRanking::FullHouse as u32, 6);
}

#[test]
fn quads_raw_value() {
    assert_eq!(hand::PokerRanking::Quads as u32, 7);
}

#[test]
fn straight_flush_raw_value() {
    assert_eq!(hand::PokerRanking::StraightFlush as u32, 8);
}
