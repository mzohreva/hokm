
use super::*;
use crate::cards::*;

#[derive(Debug, Clone)]
pub struct Trick {
    pub played_cards: [Option<Card>; 4],
    pub first_player: PlayerNumber,
}

impl Trick {
    pub fn new(first_player: PlayerNumber) -> Self {
        Trick {
            played_cards: [None, None, None, None],
            first_player,
        }
    }

    pub fn first_card(&self) -> Option<Card> {
        self.played_cards[self.first_player.as_index()]
    }

    pub fn played_cards_in_order(&self) -> [Option<Card>; 4] {
        let first = self.first_player.as_index();
        [
            self.played_cards[(first + 0) % 4],
            self.played_cards[(first + 1) % 4],
            self.played_cards[(first + 2) % 4],
            self.played_cards[(first + 3) % 4],
        ]
    }

    pub fn have_all_played(&self) -> bool {
        !self.played_cards.iter().any(|pc| pc.is_none())
    }

    pub fn winner(&self, trump_suit: Suit) -> Option<PlayerNumber> {
        if !self.have_all_played() {
            return None;
        }
        let cards = [
            self.played_cards[0].unwrap(),
            self.played_cards[1].unwrap(),
            self.played_cards[2].unwrap(),
            self.played_cards[3].unwrap(),
        ];
        let first = self.first_player.as_index();
        let mut w = first;
        for i in 1..4 {
            let p = (first + i) % 4;
            if cards[p].suit() != cards[w].suit() && cards[p].suit() == trump_suit {
                w = p;
            }
            if cards[p].suit() == cards[w].suit() && cards[p].rank() > cards[w].rank() {
                w = p;
            }
        }
        Some(PlayerNumber::from_index(w))
    }
}
