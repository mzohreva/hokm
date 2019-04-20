
use super::*;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use std::cell::RefCell;

const HUMAN: PlayerNumber = PlayerNumber::Four;
const MARGIN: u32 = 25;

pub struct Game {
    game: Hokm,
    gui_trick: Option<GuiTrick>,
    gui_hand: Option<GuiHand>,
    played_card: Option<(Animated<GuiCard>, usize)>,
    dealt_cards: Vec<(Animated<GuiCard>, usize)>,
    total_cards_dealt: usize,
    accept_click: bool,
    pausing_cycles: u32,

    deck_pile: Option<GuiPile>,
    player_piles: [Option<GuiPile>; 3],
    player_scores: [Option<GuiPlayerScore>; 4],
    arranged: bool,
    game_over: bool,

    players: [Box<Player>; 3],
    human_player: GuiPlayer,
}

impl Game {
    pub fn new() -> Self {
        let game = Hokm::new(PlayerNumber::One);
        Game {
            game,
            gui_trick: None,
            gui_hand: None,
            played_card: None,
            dealt_cards: Vec::new(),
            total_cards_dealt: 0,
            accept_click: true,
            pausing_cycles: 0,
            deck_pile: None,
            player_piles: [None, None, None],
            player_scores: [None, None, None, None],
            arranged: false,
            game_over: false,
            players: [
                Box::new(SensiblePlayer::new()),
                Box::new(SensiblePlayer::new()),
                Box::new(SensiblePlayer::new()),
            ],
            human_player: GuiPlayer { card: RefCell::new(None) },
        }
    }

    fn set_gui_hand(&mut self, hand: Hand) {
        let mut gui_hand = GuiHand::new(hand);
        gui_hand.move_by(0, 8 * SCENE_HEIGHT as i32 / 17);
        self.gui_hand = Some(gui_hand);
    }

    fn arrange_objects(&mut self) {
        self.arranged = true;
        let psh = self.game.player_state(HUMAN);
        if psh.hand().cards.len() > 0 {
            self.set_gui_hand(psh.hand().to_owned());
        }

        if let Some(trick) = self.game.trick() {
            let mut gui_trick = GuiTrick::new(trick.to_owned());
            gui_trick.move_by(0, -(SCENE_HEIGHT as i32) / 40);
            self.gui_trick = Some(gui_trick);
        }

        if self.game.deck_size() > 0 {
            let mut dp = GuiPile::new(self.game.deck_size(), PileSpread::Deck);
            dp.set_position(MARGIN as i32, (SCENE_HEIGHT - MARGIN - GuiPile::height()) as i32);
            self.deck_pile = Some(dp);
        }

        let pyc = (SCENE_HEIGHT - GuiPile::height()) as i32 / 2;
        let pxc = (SCENE_WIDTH - GuiPile::width()) as i32 / 2;
        for i in 0..3 {
            let ps = self.game.player_state(PlayerNumber::from_index(i));
            let spread = match i % 2 {
                0 => PileSpread::Vertical,
                _ => PileSpread::Horizontal,
            };
            let (x, y) = match i {
                0 => (MARGIN as i32, pyc),
                1 => (pxc, MARGIN as i32),
                2 => ((SCENE_WIDTH - GuiPile::width() - MARGIN) as i32, pyc),
                _ => unreachable!(),
            };
            let mut pp = GuiPile::new(ps.hand().cards.len(), spread);
            pp.set_position(x, y);
            pp.name = Some(self.players[i].name());
            self.player_piles[i] = Some(pp);
        }
        if [GameState::NormalPlay, GameState::Finished].contains(&self.game.game_state()) {
            for i in 0..4 {
                let ps = self.game.player_state(PlayerNumber::from_index(i));
                let (orientation, x, y) = match i {
                    0 => (Orientation::Vertical, (MARGIN + GuiPile::width() + 10) as i32, pyc + 10),
                    1 => (Orientation::Horizontal, pxc + 10, (MARGIN + GuiPile::height() + 10) as i32),
                    2 => (Orientation::Vertical, (SCENE_WIDTH - GuiPile::width() - MARGIN - 10) as i32, pyc + 10),
                    3 => (Orientation::Horizontal, pxc + 10, (SCENE_HEIGHT - MARGIN) as i32),
                    _ => unreachable!(),
                };
                let mut psc = GuiPlayerScore::new(orientation);
                psc.x = x;
                psc.y = y;
                psc.score = ps.score();
                self.player_scores[i] = Some(psc);
            }
        }
    }

    fn process_animations(&mut self) -> bool {
        let mut repaint = false;
        if let Some(ref mut gui_hand) = self.gui_hand {
            repaint |= gui_hand.process();
        }
        if let Some(ref mut gui_trick) = self.gui_trick {
            repaint |= gui_trick.process();
        }
        if let Some((ref mut ac, pi)) = self.played_card {
            ac.process();
            if ac.animations.is_empty() {
                if pi == HUMAN.as_index() {
                    let c = ac.object.card;
                    self.human_player.card.replace(Some(c));
                }
                self.arranged = false;
                self.played_card = None;
            }
            return true;
        }
        self.dealt_cards.retain(|(dc, _)| !dc.animations.is_empty());
        for (ac, pi) in self.dealt_cards.iter_mut().rev() {
            let done = !ac.process();
            if done {
                if *pi == HUMAN.as_index() {
                    self.gui_hand.as_mut().unwrap().add_card(ac.object.card);
                } else {
                    self.player_piles[*pi].as_mut().unwrap().add_card();
                }
                if self.dealt_cards.len() == 1 {
                    // this is the last one
                    self.arranged = false;
                }
            }
            return true;
        }
        if self.total_cards_dealt == 52 && self.deck_pile.is_some() {
            self.deck_pile = None;
            self.arranged = false;
            repaint = true;
        }
        repaint
    }
}

impl Paintable for Game {
    fn process(&mut self) -> bool {
        if self.pausing_cycles > 0 {
            self.pausing_cycles -= 1;
            if self.pausing_cycles == 0 {
                self.arranged = false;
            }
            return true;
        }
        let repaint = self.process_animations();
        if repaint {
            return true;
        }
        if self.game_over {
            return false;
        }
        if self.game.game_state() == GameState::Finished {
            self.game_over = true;
            self.arranged = false;
            println!("Game over!");
            return true;
        }
        let waiting_for_user = self.game.turn() == HUMAN &&
            self.game.game_state() == GameState::NormalPlay &&
            self.game.trick().map_or(false, |t| !t.have_all_played()) &&
            self.human_player.card.borrow().is_none();

        if waiting_for_user {
            return false;
        }
        let players: [&Player; 4] = [
            self.players[0].as_ref(),
            self.players[1].as_ref(),
            self.players[2].as_ref(),
            &self.human_player,
        ];
        let event = self.game.play(players);
        println!("Event: {:?}", event);
        match event {
            GameEvent::DealtCards(p, n) => {
                self.total_cards_dealt += n;
                if p == HUMAN {
                    if self.gui_hand.is_none() {
                        self.set_gui_hand(Hand::new());
                    }
                    let dp = self.deck_pile.as_mut().unwrap();
                    let gh = self.gui_hand.as_ref().unwrap();
                    let ps = self.game.player_state(p);
                    let ca = gh.card_arrangements(ps.hand().cards.len());
                    let s = ps.hand().cards.len() - n;
                    for (c, (pos, a)) in ps.hand().cards.iter().zip(ca).skip(s).rev() {
                        let mut gc = dp.pop_card().unwrap();
                        gc.card = *c;
                        let mut ac = Animated::new(gc);
                        ac.move_to(pos, 10);
                        ac.rotate_to(a, 10);
                        ac.flip_card(10);
                        ac.scale_card(DEFAULT_SCALE, 10);
                        self.dealt_cards.push((ac, p.as_index()));
                    }
                    return true;
                }
                let pp = self.player_piles[p.as_index()].as_ref().unwrap();
                let dp = self.deck_pile.as_mut().unwrap();
                for i in (0..n).rev() {
                    let gc = dp.pop_card().unwrap();
                    let mut ac = Animated::new(gc);
                    ac.move_to(pp.get_position(pp.size() + i), 10);
                    self.dealt_cards.push((ac, p.as_index()));
                }
                return true;
            },
            GameEvent::SortedHands => {
                self.arranged = false;
                return true;
            },
            GameEvent::SetTrumpSuit(_) => {
                self.arranged = false;
                return true;
            },
            GameEvent::Scored(_) => {
                self.pausing_cycles = 60;
                return true;
            },
            GameEvent::InvalidPlay(_, _) => {
                self.arranged = false;
                return true;
            },
            GameEvent::PlayedCard(p, card) => {
                if p == HUMAN {
                    self.arranged = false;
                    return true;
                }
                let gui_trick = self.gui_trick.as_mut().unwrap();
                let pp = self.player_piles[p.as_index()].as_mut().unwrap();
                let mut gc = pp.pop_card().unwrap();
                gc.card = card;
                let mut ac = Animated::new(gc);
                let steps = 15;
                ac.move_to(gui_trick.position_of(p.as_index()).unwrap(), steps);
                ac.rotate_to(180.0, steps);
                ac.flip_card(steps);
                self.played_card = Some((ac, p.as_index()));
                return true;
            },
            GameEvent::Won(_) => {
                self.accept_click = false;
                return false;
            },
        }
    }

    fn paint(&mut self, textures: &Textures, canvas: &mut WindowCanvas) -> Result<(), String> {
        if !self.arranged {
            self.arrange_objects();
        }
        if let Some(ref mut deck_pile) = self.deck_pile {
            deck_pile.paint(textures, canvas)?;
        }
        for i in 0..3 {
            if let Some(ref mut pp) = self.player_piles[i] {
                pp.paint(textures, canvas)?;
            }
        }
        for i in 0..4 {
            if let Some(ref mut psc) = self.player_scores[i] {
                psc.paint(textures, canvas)?;
            }
        }
        if let Some(ref mut gui_trick) = self.gui_trick {
            gui_trick.paint(textures, canvas)?;
        }
        if let Some(ref mut gui_hand) = self.gui_hand {
            gui_hand.paint(textures, canvas)?;
        }
        if let Some((ref mut ac, _)) = self.played_card {
            ac.paint(textures, canvas)?;
        }
        for (ac, _) in self.dealt_cards.iter_mut() {
            ac.paint(textures, canvas)?;
        }
        if let Some(ts) = self.game.trump_suit() {
            let (t, src) = textures.suit(ts);
            canvas.copy(t, src, Rect::new(10, 10, 30, 30))?;
        }
        if [GameState::NormalPlay, GameState::Finished].contains(&self.game.game_state()) {
            let scores = self.game.team_scores();
            let score_board = format!("{} - {}", scores.0, scores.1);
            canvas.string(60, 25, &score_board, Color::RGB(255, 255, 255))?;
        }
        Ok(())
    }
}

impl Clickable for Game {
    fn click(&mut self, x: i32, y: i32) -> (bool, Option<Card>) {
        if !self.accept_click || self.pausing_cycles > 0 || self.played_card.is_some() {
            return (false, None);
        }
        if self.gui_hand.is_none() || self.gui_trick.is_none() {
            return (false, None);
        }
        let gui_hand = self.gui_hand.as_mut().unwrap();
        let gui_trick = self.gui_trick.as_mut().unwrap();
        let (handled, yielded_card) = gui_hand.click(x, y);
        if let Some(card) = yielded_card {
            let gc = gui_hand.pop_card(card).unwrap();
            let mut ac = Animated::new(gc);
            let steps = 15;
            ac.move_to(gui_trick.position_of(self.game.turn().as_index()).unwrap(), steps);
            ac.rotate_to(180.0, steps);
            ac.scale_card(SMALLER_CARDS, steps);
            self.played_card = Some((ac, HUMAN.as_index()));
        }
        return (handled, yielded_card);
    }
}

struct GuiPlayer {
    card: RefCell<Option<Card>>,
}

impl Player for GuiPlayer {
    fn name(&self) -> String {
        "Human".to_owned()
    }

    fn call_trump_suit(&self, _hand: &Hand) -> Suit {
        Suit::Spades // TODO
    }

    fn play(&self, _hand: &Hand, _trump_suit: Suit, _trick: &Trick) -> Card {
        self.card.replace(None).unwrap()
    }
}
