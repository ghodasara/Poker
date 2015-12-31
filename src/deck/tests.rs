#[test]
fn num_cards() {
    let deck = super::Deck::new();
    assert_eq!(deck.cards.len(), 52);
}
