#[test]
fn num_cards() {
    let deck = deck::Deck::new();
    assert_eq!(deck.cards.len(), 52);
}

#[test]
fn draw_one_card() {
    let mut deck = deck::Deck::new();
    let top_card_value = deck.cards.get(51).unwrap().value.clone();
    let top_card_suit = deck.cards.get(51).unwrap().suit.clone();
    let card = deck.draw().unwrap();
    assert_eq!(deck.cards.len(), 51);

    assert!(card.value != ::card::Value::None);
    assert!(card.suit != ::card::Suit::None);

    assert_eq!(card.value, top_card_value);
    assert_eq!(card.suit, top_card_suit);
}

#[test]
fn draw_too_many_cards() {
    let mut deck = deck::Deck::new();
    for i in 0..53 {
        let exists = match deck.draw() {
            None    => false,
            Some(_) => true
        };
        if i < 52 {
            assert!(exists);
        } else {
            assert!(!exists);
        }
    }
}
