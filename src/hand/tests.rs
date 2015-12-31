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
