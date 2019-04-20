
use crate::game::*;
use crate::cards::*;
use rand::{thread_rng, RngCore};

pub struct RandomPlayer;

impl Player for RandomPlayer {
    fn name(&self) -> String {
        "Random".to_owned()
    }

    fn call_trump_suit(&self, _hand: &Hand) -> Suit {
        Suit::Hearts
    }

    fn play(&self, hand: &Hand, trump_suit: Suit, trick: &Trick) -> Card {
        let first_card = match trick.played_cards[trick.first_player.as_index()] {
            Some(card) => card,
            None => {
                // I'm the first to play!
                let r = thread_rng().next_u32() as usize % hand.cards.len();
                return hand.cards[r];
            }
        };
        // Let's see if I have a card with the same suit as the first card
        if let Some(card) = hand.highest_rank_card(first_card.suit()) {
            return card;
        }
        // Maybe I have a trump card?
        if hand.count_of_suit(trump_suit) > 0 {
            let candidates = hand.cards_of_suit(trump_suit);
            let r = thread_rng().next_u32() as usize % candidates.len();
            return candidates[r];
        }
        // Otherwise, choose a card at random
        let r = thread_rng().next_u32() as usize % hand.cards.len();
        return hand.cards[r];
    }
}
