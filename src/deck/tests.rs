#[test]
fn num_cards() {
    let deck = super::Deck::new();
    assert_eq!(deck.cards.len(), 52);
}

#[test]
fn draw_card() {
    let mut deck = super::Deck::new();
    let top_card_value = deck.cards.get(51).unwrap().value.clone();
    let top_card_suit = deck.cards.get(51).unwrap().suit.clone();
    let card = deck.draw();
    assert_eq!(deck.cards.len(), 51);

    assert!(card.value != ::card::Value::None);
    assert!(card.suit != ::card::Suit::None);

    assert_eq!(card.value, top_card_value);
    assert_eq!(card.suit, top_card_suit);
}
