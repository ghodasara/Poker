#[test]
fn num_cards() {
    let deck = super::Deck::new();
    assert_eq!(deck.cards.len(), 52);
}

#[test]
fn draw_card() {
    let mut deck = super::Deck::new();
    let _card = deck.draw();
    assert_eq!(deck.cards.len(), 51);
}
