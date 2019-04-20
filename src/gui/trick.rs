
use super::*;

#[derive(Clone, Debug)]
pub struct GuiTrick {
    trick: Trick,
    cards: Stacked<GuiCardOrPlaceHolder>,
    tx: i32,
    ty: i32,
    arranged: bool,
}

impl GuiTrick {
    pub fn new(trick: Trick) -> Self {
        GuiTrick {
            trick,
            cards: Stacked::new(),
            tx: 0,
            ty: 0,
            arranged: false,
        }
    }

    fn arrange_cards(&mut self) {
        self.arranged = true;
        self.cards = Stacked::new();
        let scale = SMALLER_CARDS;
        let scale_spaced = scale * 1.1;
        for i in 0..4 {
            let ii = i as isize;
            let xf = ((ii % 2 - 1) * (ii - 1) * 2) as i32 + 1;
            let yf = ((ii % 2) * (1 - 2 * (ii/3))) as i32 + 1;
            let x = (SCENE_WIDTH as i32 - xf * (CARD_WIDTH as f64 * scale_spaced) as i32) / 2;
            let y = (SCENE_HEIGHT as i32 - yf * (CARD_HEIGHT as f64 * scale_spaced) as i32) / 2;
            let gop = match self.trick.played_cards[i] {
                Some(card) => {
                    let mut gc = GuiCard::new(card);
                    gc.set_position(x + self.tx, y + self.ty);
                    gc.scale = scale;
                    GuiCardOrPlaceHolder::Card(gc)
                },
                None => {
                    let mut gcp = GuiCardPlaceHolder::new();
                    gcp.set_position(x + self.tx, y + self.ty);
                    gcp.scale = scale;
                    GuiCardOrPlaceHolder::PlaceHolder(gcp)
                },
            };
            self.cards.add(0, gop);
        }
    }

    pub fn position_of(&mut self, slot: usize) -> Option<Point> {
        if !self.arranged {
            self.arrange_cards();
        }
        self.cards.iter().enumerate()
            .find(|(i, _)| *i == slot)
            .map(|(_, gcp)| match gcp {
                GuiCardOrPlaceHolder::Card(c) => c.get_position(),
                GuiCardOrPlaceHolder::PlaceHolder(p) => p.get_position(),
            })
    }
}

impl Paintable for GuiTrick {
    fn process(&mut self) -> bool {
        false
    }

    fn paint(&mut self, textures: &Textures, canvas: &mut WindowCanvas) -> Result<(), String> {
        if !self.arranged {
            self.arrange_cards();
        }
        self.cards.paint(textures, canvas)
    }
}

impl Positioned for GuiTrick {
    fn set_position(&mut self, x: i32, y: i32) {
        self.tx = x; // FIXME: this needs to be translated from center
        self.ty = y;
        self.arranged = false;
    }
}
