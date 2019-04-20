
use super::*;
use crate::cards::Hand;
use std::cmp::min;
use std::sync::Arc;
use std::{f32, f64};

#[derive(Clone)]
pub struct GuiHand {
    hand: Hand,
    animated_cards: Stacked<Animated<GuiCard>>,
    tx: i32,
    ty: i32,
    arranged: bool,
    pub on_click: Option<Arc<Fn(&Hand, Card) -> Option<Hand>>>,
}

impl GuiHand {
    pub fn new(hand: Hand) -> Self {
        GuiHand {
            hand,
            animated_cards: Stacked::new(),
            tx: 0,
            ty: 0,
            arranged: false,
            on_click: None,
        }
    }

    fn arrange_cards(&mut self) {
        self.arranged = true;
        let mut z = 0;
        self.animated_cards = Stacked::new();
        let arr = self.card_arrangements(self.hand.cards.len());
        for (c, (pos, angle)) in self.hand.cards.iter().zip(arr) {
            let mut gc = GuiCard::new(*c);
            gc.set_position(pos.x, pos.y);
            gc.set_rotation(angle);
            gc.on_click = Some(Arc::new(|c: &mut GuiCard| {
                match c.state {
                    SelectionState::None => c.state = SelectionState::Candidate,
                    SelectionState::Candidate => c.state = SelectionState::Selected,
                    SelectionState::Selected => return Some(c.card)
                }
                None
            }));
            self.animated_cards.add(z, Animated::new(gc));
            z += 1;
        }
    }

    pub fn card_arrangements(&self, count: usize) -> Vec<(Point, f64)> {
        let circle = Circle {
            cx: (SCENE_WIDTH / 2) as i32 - (CARD_WIDTH as f64 * DEFAULT_SCALE / 2.0) as i32,
            cy: (SCENE_HEIGHT / 2) as i32,
            radius: min(5 * SCENE_WIDTH / 12, SCENE_HEIGHT / 4),
        };
        let mut res = Vec::new();
        let mut angle = 4.0 * f32::consts::PI / 6.0;
        angle -= (13.0 - count as f32) * f32::consts::PI / 72.0;
        for _ in 0..count {
            let (x, y) = circle.find_point(angle);
            let x = x + self.tx;
            let y = y + self.ty;
            let a = -(angle as f64).to_degrees() + 90.0;
            res.push((Point::new(x, y), a));
            angle -= 4.0 * f32::consts::PI / 12.0 / 12.0;
        }
        res
    }

    pub fn pop_card(&mut self, card: Card) -> Option<GuiCard> {
        if !self.arranged {
            self.arrange_cards();
        }
        self.hand.cards.retain(|c| *c != card);
        self.arranged = false;
        self.animated_cards.iter()
            .find(|ac| ac.object.card == card)
            .map(|ac| ac.object.clone())
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.cards.push(card);
        self.arranged = false;        
    }
}

impl Paintable for GuiHand {
    fn process(&mut self) -> bool {
        self.animated_cards.process()
    }
    fn paint(&mut self, textures: &Textures, canvas: &mut WindowCanvas) -> Result<(), String> {
        if !self.arranged {
            self.arrange_cards();
        }
        self.animated_cards.paint(textures, canvas)
    }
}

impl Clickable for GuiHand {
    fn click(&mut self, x: i32, y: i32) -> (bool, Option<Card>) {
        let mut clicked_card = None;
        for ac in self.animated_cards.iter_mut_rev() {
            let (handled, _) = ac.click(x, y);
            if handled {
                clicked_card = Some(ac);
                break;
            }
        }
        let (card, state) = match clicked_card {
            Some(ac) => (ac.object.card, ac.object.state),
            None => return (false, None)
        };
        for ac in self.animated_cards.iter_mut() {
            if ac.object.card != card && ac.object.state != SelectionState::None {
                let animation = ac.object.unraise();
                ac.add_animation(Box::new(animation));
            }
            if ac.object.card == card {
                let animation = ac.object.raise();
                ac.add_animation(Box::new(animation));
            }
        }
        let mut to_yield = None;
        if state == SelectionState::Selected {
            if let Some(func) = self.on_click.as_ref() {
                match func(&self.hand, card) {
                    Some(new_hand) => {
                        self.hand = new_hand;
                        self.arranged = false;
                    }
                    None => {}
                }
            }
            to_yield = Some(card);
        }
        (true, to_yield)
    }
}

impl Positioned for GuiHand {
    fn set_position(&mut self, x: i32, y: i32) {
        self.tx = x; // FIXME: this needs to be translated from center
        self.ty = y;
        self.arranged = false;
    }

    fn move_by(&mut self, delta_x: i32, delta_y: i32) {
        self.tx += delta_x;
        self.ty += delta_y;
        self.arranged = false;
    }
}
