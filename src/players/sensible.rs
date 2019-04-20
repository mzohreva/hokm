
use crate::game::*;
use crate::cards::*;
use std::cell::RefCell;
use std::collections::HashSet;

pub struct SensiblePlayer {
    played_cards: RefCell<HashSet<Card>>,
}

impl SensiblePlayer {
    pub fn new() -> Self {
        SensiblePlayer {
            played_cards: RefCell::default(),
        }
    }
    fn restart(&self) {
        self.played_cards.borrow_mut().clear();
    }
    fn mark_as_played(&self, card: Card) {
        self.played_cards.borrow_mut().insert(card);
    }

    fn play_first(&self, hand: &Hand, _trump_suit: Suit) -> Card {
        Suit::all_suits()
            .into_iter()
            .flat_map(|&suit| hand.highest_rank_card(suit))
            .max_by(compare_rank)
            .expect("non-empty hand")
    }

    fn play_second(&self, hand: &Hand, trump_suit: Suit, right: Card) -> Card {
        let (first_card, first_suit) = (right, right.suit());
        let (options, any_card) = legal_plays(hand, first_card);
        if !any_card {
            let highest = *options.iter().max_by(compare_rank_ref).unwrap();
            if beats(highest, right, trump_suit, first_suit) {
                return highest;
            }
            return options.into_iter().min_by(compare_rank).unwrap();
        }
        // Decide if I want to play a trump card or not
        let trump_cards = hand.cards_of_suit(trump_suit);
        if trump_cards.is_empty() {
            // TODO: choose the throw away card wisely
            return options.into_iter().min().unwrap();
        }
        trump_cards.into_iter().min_by(compare_rank).unwrap()
    }

    fn play_third(&self, hand: &Hand, trump_suit: Suit, across: Card, right: Card) -> Card {
        let (first_card, first_suit) = (across, across.suit());
        let (options, any_card) = legal_plays(hand, first_card);
        let teammate_beats_right = beats(across, right, trump_suit, first_suit);
        if !any_card {
            if teammate_beats_right {
                // my teammate beats the right opponent
                return options.into_iter().min_by(compare_rank).unwrap();
            }
            let highest = *options.iter().max_by(compare_rank_ref).unwrap();
            if beats(highest, right, trump_suit, first_suit) {
                return highest;
            }
            return options.into_iter().min_by(compare_rank).unwrap();
        }
        if teammate_beats_right {
            let c1 = options.iter().filter(|c| c.suit() != trump_suit).min_by(compare_rank_ref);
            let c2 = options.iter().min_by(compare_rank_ref);
            return *c1.or(c2).unwrap();
        }
        let mut trump_cards = hand.cards_of_suit(trump_suit);
        if trump_cards.is_empty() {
            // TODO: choose the throw away card wisely
            return options.into_iter().min_by(compare_rank).unwrap();
        }
        // find the minimum trump card that beats right
        trump_cards.sort();
        for c in trump_cards {
            if beats(c, right, trump_suit, first_suit) {
                return c;
            }
        }
        let c1 = options.iter().filter(|c| c.suit() != trump_suit).min_by(compare_rank_ref);
        let c2 = options.iter().min_by(compare_rank_ref);
        return *c1.or(c2).unwrap();
    }

    fn play_last(&self, hand: &Hand, trump_suit: Suit, left: Card, across: Card, right: Card) -> Card {
        let (first_card, first_suit) = (left, left.suit());
        let (options, any_card) = legal_plays(hand, first_card);
        let teammate_beats_both = beats_all(across, &[left, right], trump_suit, first_suit);
        if !any_card {
            if teammate_beats_both {
                return options.into_iter().min_by(compare_rank).unwrap();
            }
            let highest = *options.iter().max_by(compare_rank_ref).unwrap();
            if beats_all(highest, &[left, right], trump_suit, first_suit) {
                return highest;
            }
            return options.into_iter().min_by(compare_rank).unwrap();
        }
        if teammate_beats_both {
            let c1 = options.iter().filter(|c| c.suit() != trump_suit).min_by(compare_rank_ref);
            let c2 = options.iter().min_by(compare_rank_ref);
            return *c1.or(c2).unwrap();
        }
        let mut trump_cards = hand.cards_of_suit(trump_suit);
        if trump_cards.is_empty() {
            // TODO: choose the throw away card wisely
            return options.into_iter().min_by(compare_rank).unwrap();
        }
        // find the minimum trump card that beats both
        trump_cards.sort();
        for c in trump_cards {
            if beats_all(c, &[left, right], trump_suit, first_suit) {
                return c;
            }
        }
        let c1 = options.iter().filter(|c| c.suit() != trump_suit).min_by(compare_rank_ref);
        let c2 = options.iter().min_by(compare_rank_ref);
        return *c1.or(c2).unwrap();
    }
}

impl Player for SensiblePlayer {
    fn name(&self) -> String {
        "Sensible".to_owned()
    }

    fn call_trump_suit(&self, hand: &Hand) -> Suit {
        self.restart();
        let mut best_by_count = None;
        let mut best_by_highest = None;
        for &suit in Suit::all_suits() {
            let count = hand.count_of_suit(suit);
            match best_by_count {
                Some((_, c)) if count <= c => {},
                _ => best_by_count = Some((suit, count)),
            }
            if let Some(highest) = hand.highest_rank_card(suit) {
                match best_by_highest {
                    Some((_, r)) if highest.rank() <= r => {},
                    _ => best_by_highest = Some((suit, highest.rank())),
                }
            }
        }
        match (best_by_count, best_by_highest) {
            (Some((sc, count)), _) if count >= 3 => sc,
            (_, Some((sh, rank))) if rank >= Rank::Ten => sh,
            (Some((sc, _)), _) => sc,
            _ => Suit::Hearts
        }
    }

    fn play(&self, hand: &Hand, trump_suit: Suit, trick: &Trick) -> Card {
        match trick.played_cards_in_order() {
            [None, None, None, None] => self.play_first(hand, trump_suit),
            [Some(c1), None, None, None] => self.play_second(hand, trump_suit, c1),
            [Some(c1), Some(c2), None, None] => self.play_third(hand, trump_suit, c1, c2),
            [Some(c1), Some(c2), Some(c3), None] => self.play_last(hand, trump_suit, c1, c2, c3),
            _ => panic!("everyone has played"),
        }
    }

    fn trick_end(&self, trick: &Trick) {
        for c in trick.played_cards.iter() {
            let c = c.as_ref().unwrap();
            self.mark_as_played(*c);
        }
    }
}

fn legal_plays(hand: &Hand, first_card: Card) -> (Vec<Card>, bool) {
    let same_suit = hand.cards_of_suit(first_card.suit());
    if !same_suit.is_empty() {
        return (same_suit, false);
    }
    (hand.cards.clone(), true)
}

// Does c1 beat c2?
fn beats(c1: Card, c2: Card, trump_suit: Suit, first_suit: Suit) -> bool {
    if c1.suit() == c2.suit() {
        return c1.rank() > c2.rank()
    }
    if c1.suit() == trump_suit {
        return true;
    }
    if c2.suit() == trump_suit {
        return false;
    }
    c1.suit() == first_suit
}

fn beats_all(c: Card, cards: &[Card], trump_suit: Suit, first_suit: Suit) -> bool {
    let mut b = true;
    for card in cards {
        b &= beats(c, *card, trump_suit, first_suit);
    }
    b
}

fn compare_rank(c1: &Card, c2: &Card) -> std::cmp::Ordering {
    c1.rank().cmp(&c2.rank())
}
fn compare_rank_ref(c1: &&Card, c2: &&Card) -> std::cmp::Ordering {
    c1.rank().cmp(&c2.rank())
}
