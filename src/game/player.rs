
use super::*;
use crate::cards::*;

pub trait Player {
    fn name(&self) -> String;
    fn call_trump_suit(&self, hand: &Hand) -> Suit;
    fn play(&self, hand: &Hand, trump_suit: Suit, trick: &Trick) -> Card;
    fn trick_end(&self, _trick: &Trick) {
        // ... so that player can keep track of played cards
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerNumber {
    One = 1,
    Two,
    Three,
    Four,
}

impl PlayerNumber {
    pub fn increment(&mut self) {
        *self = match self {
            PlayerNumber::One   => PlayerNumber::Two,
            PlayerNumber::Two   => PlayerNumber::Three,
            PlayerNumber::Three => PlayerNumber::Four,
            PlayerNumber::Four  => PlayerNumber::One,
        };
    }

    pub fn as_index(&self) -> usize {
        *self as usize - 1
    }
    pub fn from_index(idx: usize) -> Self {
        (idx + 1).into()
    }
}

impl From<usize> for PlayerNumber {
    fn from(x: usize) -> PlayerNumber {
        match x {
            1 => PlayerNumber::One,
            2 => PlayerNumber::Two,
            3 => PlayerNumber::Three,
            4 => PlayerNumber::Four,
            _ => panic!("Invalid value for player number: {}", x),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Team {
    PlayersOneAndThree,
    PlayersTwoAndFour,
}
