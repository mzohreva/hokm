
use super::*;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use std::cmp::max;

const SHOW_PLAYER_NAME: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PileSpread {
    Deck,
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct GuiPile {
    size: usize,
    spread: PileSpread,
    gui_cards: Stacked<GuiCard>,
    place_holder: GuiCardPlaceHolder,
    x: i32,
    y: i32,
    pub name: Option<String>,
    arranged: bool,
}

impl GuiPile {
    pub fn new(size: usize, spread: PileSpread) -> Self {
        GuiPile {
            size,
            spread,
            gui_cards: Stacked::new(),
            place_holder: GuiCardPlaceHolder::new(),
            x: 0,
            y: 0,
            name: None,
            arranged: false,
        }
    }

    fn scale() -> f64 {
        SMALLER_CARDS
    }
    pub fn width() -> u32 {
        (CARD_WIDTH as f64 * GuiPile::scale()) as u32
    }
    pub fn height() -> u32 {
        (CARD_HEIGHT as f64 * GuiPile::scale()) as u32
    }

    fn arrange_cards(&mut self) {
        self.arranged = true;
        self.gui_cards = Stacked::new();
        self.place_holder.scale = GuiPile::scale();
        self.place_holder.set_position(self.x, self.y);
        let (size, sx, sy) = self.pos_info(self.size);
        for i in 0..size {
            let mut gc = GuiCard::new(Card::new(Rank::Ace, Suit::Spades));
            gc.face_up = false;
            gc.set_position(self.x + sx * i as i32, self.y + sy * i as i32);
            gc.scale = GuiPile::scale();
            self.gui_cards.add(i as u8, gc);
        }
    }

    fn pos_info(&self, x: usize) -> (usize, i32, i32) {
        match self.spread {
            PileSpread::Deck => {
                let y = match x {
                    0 => 0,
                    _ => max(7, (x as f32 / 4.0).ceil() as usize)
                };
                (y, 1, 1)
            },
            PileSpread::Horizontal => (x, 3, 0),
            PileSpread::Vertical => (x, 0, 3),
        }
    }

    pub fn add_card(&mut self) {
        self.size += 1;
        self.arranged = false;
    }

    pub fn pop_card(&mut self) -> Option<GuiCard> {
        let len = self.gui_cards.iter().count();
        if len > 0 {
            self.size -= 1;
            return self.gui_cards.iter_mut_rev().next().map(|gc| gc.clone());
        }
        None
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get_position(&self, card_index: usize) -> Point {
        let (i, sx, sy) = self.pos_info(card_index);
        Point::new(self.x + sx * i as i32, self.y + sy * i as i32)
    }
}

impl Paintable for GuiPile {
    fn process(&mut self) -> bool {
        false
    }

    fn paint(&mut self, textures: &Textures, canvas: &mut WindowCanvas) -> Result<(), String> {
        if !self.arranged {
            self.arrange_cards();
        }
        if SHOW_PLAYER_NAME {
            if let Some(ref name) = self.name {
                canvas.string(self.x as i16, self.y as i16 - 10, name, Color::RGB(255, 255, 255))?;
            }
        }
        self.place_holder.paint(textures, canvas)?;
        self.gui_cards.paint(textures, canvas)
    }
}

impl Positioned for GuiPile {
    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.arranged = false;
    }

    fn move_by(&mut self, delta_x: i32, delta_y: i32) {
        self.x += delta_x;
        self.y += delta_y;
        self.arranged = false;
    }
}
