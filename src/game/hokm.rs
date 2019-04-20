
use super::*;
use crate::cards::*;

pub struct Hokm {
    deck: Deck,
    players: [PlayerState; 4],
    trump_suit: Option<Suit>,
    trump_caller: PlayerNumber,
    turn: PlayerNumber,
    trick: Option<Trick>,
    game_state: GameState,
}

impl Hokm {
    pub fn new<N: Into<PlayerNumber>>(trump_caller: N) -> Self {
        let trump_caller = trump_caller.into();
        Hokm {
            deck: Deck::new().shuffle(),
            players: [
                PlayerState::new(),
                PlayerState::new(),
                PlayerState::new(),
                PlayerState::new(),
            ],
            trump_suit: None,
            trump_caller,
            turn: trump_caller,
            trick: None,
            game_state: GameState::DealingInitialFiveCards,
        }
    }

    pub fn play(&mut self, players: [&Player; 4]) -> GameEvent {
        use GameState::*;
        match self.game_state {
            DealingInitialFiveCards => deal_initial_five_cards(self),
            SettingTrumpSuit => set_trump_suit(self, players),
            DealingRestOfCards => deal_rest_of_cards(self),
            SortHands => sort_hands(self),
            NormalPlay => normal_play(self, players),
            Finished => GameEvent::Won(self.determine_winner().expect("finished w/o winner?!")),
        }
    }

    pub fn determine_winner(&self) -> Option<Team> {
        match self.team_scores() {
            (t13, _) if t13 >= 7 => Some(Team::PlayersOneAndThree),
            (_, t24) if t24 >= 7 => Some(Team::PlayersTwoAndFour),
            _ => None
        }
    }

    pub fn player_state<N: Into<PlayerNumber>>(&self, p: N) -> &PlayerState {
        &self.players[p.into().as_index()]
    }

    fn caller(&self) -> &PlayerState  { self.player_state(self.trump_caller) }
    fn current(&self) -> &PlayerState { self.player_state(self.turn) }

    pub fn team_scores(&self) -> (u32, u32) {
        (
            self.players[0].score + self.players[2].score,
            self.players[1].score + self.players[3].score
        )
    }

    fn is_valid_play(&self, player: PlayerNumber, card: Card) -> bool {
        let hand = &self.players[player.as_index()].hand();
        if !hand.cards.contains(&card) {
            return false;
        }
        let trick = match self.trick {
            Some(ref trick) => trick,
            None => return false
        };
        let first_card = match trick.first_card() {
            Some(first_card) => first_card,
            None => return true
        };
        if hand.count_of_suit(first_card.suit()) > 0 {
            return card.suit() == first_card.suit()
        }
        true
    }

    pub fn game_state(&self) -> GameState {
        if let Some(_) = self.determine_winner() {
            return GameState::Finished;
        }
        self.game_state
    }

    pub fn deck_size(&self) -> usize           { self.deck.size() }
    pub fn trump_suit(&self) -> Option<Suit>   { self.trump_suit }
    pub fn trump_caller(&self) -> PlayerNumber { self.trump_caller }
    pub fn turn(&self) -> PlayerNumber         { self.turn }
    pub fn trick(&self) -> Option<&Trick>      { self.trick.as_ref() }
}

pub struct PlayerState {
    hand: Hand,
    score: u32,
}

impl PlayerState {
    pub fn new() -> Self {
        PlayerState {
            hand: Hand::new(),
            score: 0,
        }
    }
    pub fn deal_cards(&mut self, new_cards: Vec<Card>) {
        self.hand.combine(Hand { cards: new_cards });
    }
    pub fn sort_hand(&mut self) {
        self.hand.sort();
    }
    pub fn hand(&self) -> &Hand { &self.hand }
    pub fn score(&self) -> u32  { self.score }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameEvent {
    DealtCards(PlayerNumber, usize),
    SetTrumpSuit(Suit),
    Scored(PlayerNumber),
    InvalidPlay(PlayerNumber, Card),
    PlayedCard(PlayerNumber, Card),
    Won(Team),
    SortedHands,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    DealingInitialFiveCards,
    SettingTrumpSuit,
    DealingRestOfCards,
    SortHands,
    NormalPlay,
    Finished,
}

fn deal_initial_five_cards(hokm: &mut Hokm) -> GameEvent {
    let turn = hokm.turn();
    let cards = hokm.deck.draw_multiple_cards(5);
    assert_eq!(cards.len(), 5);
    hokm.players[turn.as_index()].deal_cards(cards);
    hokm.turn.increment();
    let total_cards_dealt: usize = hokm.players.iter()
        .map(|p| p.hand.cards.len())
        .sum();
    assert!(total_cards_dealt <= 20);
    if total_cards_dealt == 20 {
        hokm.game_state = GameState::SettingTrumpSuit;
        assert_eq!(hokm.turn, hokm.trump_caller);
    }
    GameEvent::DealtCards(turn, 5)
}

fn set_trump_suit(hokm: &mut Hokm, players: [&Player; 4]) -> GameEvent {
    assert!(hokm.trump_suit.is_none());
    let caller = hokm.caller();
    let trump_suit = players[hokm.turn.as_index()].call_trump_suit(&caller.hand);
    hokm.trump_suit = Some(trump_suit);
    hokm.game_state = GameState::DealingRestOfCards;
    GameEvent::SetTrumpSuit(trump_suit)
}

fn deal_rest_of_cards(hokm: &mut Hokm) -> GameEvent {
    let turn = hokm.turn();
    let cards = hokm.deck.draw_multiple_cards(4);
    assert_eq!(cards.len(), 4);
    hokm.players[turn.as_index()].deal_cards(cards);
    hokm.turn.increment();
    let total_cards_dealt: usize = hokm.players.iter()
        .map(|p| p.hand.cards.len())
        .sum();
    if total_cards_dealt == 52 {
        hokm.game_state = GameState::SortHands;
        hokm.trick = Some(Trick::new(hokm.turn()));
        assert_eq!(hokm.turn, hokm.trump_caller);
    }
    GameEvent::DealtCards(turn, 4)
}

fn sort_hands(hokm: &mut Hokm) -> GameEvent {
    for i in 0..4 {
        hokm.players[i].sort_hand();
    }
    hokm.game_state = GameState::NormalPlay;
    GameEvent::SortedHands
}

fn normal_play(hokm: &mut Hokm, players: [&Player; 4]) -> GameEvent {
    if let Some(team) = hokm.determine_winner() {
        hokm.game_state = GameState::Finished;
        return GameEvent::Won(team);
    }
    let trump_suit = hokm.trump_suit().unwrap();
    if hokm.trick().unwrap().have_all_played() {
        let winner = hokm.trick().unwrap().winner(trump_suit).expect("all played");
        for i in 0..4 {
            players[i].trick_end(hokm.trick().unwrap());
        }
        hokm.players[winner.as_index()].score += 1;
        hokm.turn = winner;
        hokm.trick = Some(Trick::new(hokm.turn()));
        return GameEvent::Scored(winner);
    }
    let current = hokm.current();
    let card = players[hokm.turn.as_index()].play(&current.hand, trump_suit, hokm.trick().unwrap());
    if !hokm.is_valid_play(hokm.turn(), card) {
        return GameEvent::InvalidPlay(hokm.turn(), card)
    }
    let turn = hokm.turn();
    hokm.players[turn.as_index()].hand.cards.retain(|c| *c != card);
    hokm.trick.as_mut().unwrap().played_cards[turn.as_index()] = Some(card);
    hokm.turn.increment();
    GameEvent::PlayedCard(turn, card)
}
