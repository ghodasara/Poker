#[cfg(test)]
use std::cmp;
#[cfg(test)]
use ::card;

#[test]
fn create_card() {
    let value: card::Value = card::Value::Queen;
    let suit: card::Suit = card::Suit::Clubs;
    let card = card::Card::new(value, suit);

    assert_eq!(card.value, value);
    assert_eq!(card.suit, suit);
}

#[test]
fn card_cmp_less() {
    let lesser = card::Card {
        value: card::Value::Five,
        suit: card::Suit::Hearts
    };
    let greater = card::Card {
        value: card::Value::Six,
        suit: card::Suit::Spades
    };

    assert_eq!(lesser.partial_cmp(&greater).unwrap(), cmp::Ordering::Less);
}

#[test]
fn card_cmp_greater() {
    let lesser = card::Card {
        value: card::Value::Five,
        suit: card::Suit::Hearts
    };
    let greater = card::Card {
        value: card::Value::Six,
        suit: card::Suit::Spades
    };

    assert_eq!(greater.partial_cmp(&lesser).unwrap(), cmp::Ordering::Greater);
}

#[test]
fn card_cmp_equal() {
    let heart_card = card::Card {
        value: card::Value::Five,
        suit: card::Suit::Hearts
    };
    let spade_card = card::Card {
        value: card::Value::Five,
        suit: card::Suit::Spades
    };

    assert_eq!(heart_card.partial_cmp(&spade_card).unwrap(), cmp::Ordering::Equal);
}
