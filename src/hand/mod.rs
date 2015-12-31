use card;

mod tests;

pub struct Hand {
    pub cards: Vec<card::Card>
}

impl Hand {
    pub fn new(cards: Vec<card::Card>) -> Hand {
        Hand {
            cards: cards
        }
    }
}
