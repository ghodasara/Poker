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
fn compare_lesser_poker_hand() {
    assert_eq!(hand::PokerRanking::HighCard < hand::PokerRanking::OnePair, true);
    assert_eq!(hand::PokerRanking::HighCard == hand::PokerRanking::OnePair, false);
    assert_eq!(hand::PokerRanking::HighCard > hand::PokerRanking::OnePair, false);
}

#[test]
fn compare_greater_poker_hand() {
    assert_eq!(hand::PokerRanking::Flush < hand::PokerRanking::TwoPair, false);
    assert_eq!(hand::PokerRanking::Flush == hand::PokerRanking::TwoPair, false);
    assert_eq!(hand::PokerRanking::Flush > hand::PokerRanking::TwoPair, true);
}

#[test]
fn compare_equal_poker_hands() {
    assert_eq!(hand::PokerRanking::Trips < hand::PokerRanking::Trips, false);
    assert_eq!(hand::PokerRanking::Trips == hand::PokerRanking::Trips, true);
    assert_eq!(hand::PokerRanking::Trips > hand::PokerRanking::Trips, false);
}
