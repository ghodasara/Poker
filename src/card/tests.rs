#[cfg(test)]
use std::cmp;
#[cfg(test)]
use ::card;

#[test]
fn create_card() {
    let rank: card::Rank = card::Rank::Queen;
    let suit: card::Suit = card::Suit::Clubs;
    let card = card::Card::new(rank, suit);

    assert_eq!(card.rank, rank);
    assert_eq!(card.suit, suit);
}

#[test]
fn card_cmp_less() {
    let lesser = card::Card {
        rank: card::Rank::Five,
        suit: card::Suit::Hearts
    };
    let greater = card::Card {
        rank: card::Rank::Six,
        suit: card::Suit::Spades
    };

    assert_eq!(lesser.partial_cmp(&greater).unwrap(), cmp::Ordering::Less);
}

#[test]
fn card_cmp_greater() {
    let lesser = card::Card {
        rank: card::Rank::Five,
        suit: card::Suit::Hearts
    };
    let greater = card::Card {
        rank: card::Rank::Six,
        suit: card::Suit::Spades
    };

    assert_eq!(greater.partial_cmp(&lesser).unwrap(), cmp::Ordering::Greater);
}

#[test]
fn card_cmp_equal() {
    let heart_card = card::Card {
        rank: card::Rank::Five,
        suit: card::Suit::Hearts
    };
    let spade_card = card::Card {
        rank: card::Rank::Five,
        suit: card::Suit::Spades
    };

    assert_eq!(heart_card.partial_cmp(&spade_card).unwrap(), cmp::Ordering::Equal);
}

#[test]
fn card_id() {
    let two_spades = card::Card {
        rank: card::Rank::Two,
        suit: card::Suit::Spades
    };
    let id: i64 = 0b0000000000001000000000000000000000000000000000000000;

    assert_eq!(two_spades.get_id(), id);
}

#[test]
fn suit_mask() {
    let diamonds_mask: i64 = 0b1111111111111;
    let clubs_mask: i64 = 0b11111111111110000000000000;
    let hearts_mask: i64 = 0b111111111111100000000000000000000000000;
    let spades_mask: i64 = 0b1111111111111000000000000000000000000000000000000000;

    assert_eq!(card::Suit::get_suit_mask(card::Suit::Diamonds), diamonds_mask);
    assert_eq!(card::Suit::get_suit_mask(card::Suit::Clubs), clubs_mask);
    assert_eq!(card::Suit::get_suit_mask(card::Suit::Hearts), hearts_mask);
    assert_eq!(card::Suit::get_suit_mask(card::Suit::Spades), spades_mask);
}
