
use rand::seq::SliceRandom;
use rand::thread_rng;
use super::*;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        for suit in Suit::all_suits() {
            for rank in Rank::all_ranks() {
                cards.push(Card::new(*rank, *suit));
            }
        }
        Deck { cards }
    }

    pub fn shuffle(mut self) -> Self {
        let mut rng = thread_rng();
        let v = &mut self.cards;
        v.shuffle(&mut rng);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn draw_multiple_cards(&mut self, size: usize) -> Vec<Card> {
        let index = self.cards.len().checked_sub(size).unwrap_or(0);
        self.cards.split_off(index)
    }
}
