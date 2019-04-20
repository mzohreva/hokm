
use super::*;

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: Vec::new()
        }
    }

    pub fn draw_from_deck(deck: &mut Deck, size: usize) -> Hand {
        Hand {
            cards: deck.draw_multiple_cards(size),
        }
    }

    pub fn combine(&mut self, mut other: Hand) {
        self.cards.append(&mut other.cards);
    }

    pub fn sort(&mut self) {
        self.cards.sort();
    }

    pub fn count_of_suit(&self, suit: Suit) -> usize {
        self.cards.iter()
            .filter(|c| c.suit() == suit)
            .count()
    }

    pub fn cards_of_suit(&self, suit: Suit) -> Vec<Card> {
        self.cards.iter()
            .filter(|c| c.suit() == suit)
            .map(|c| *c)
            .collect()
    }

    pub fn highest_rank_card(&self, suit: Suit) -> Option<Card> {
        self.cards.iter()
            .filter(|c| c.suit() == suit)
            .max_by(|c1, c2| c1.rank().cmp(&c2.rank()))
            .map(|c| *c)
    }
}
