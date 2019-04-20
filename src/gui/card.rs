
use super::*;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use std::f64;
use std::fmt;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionState {
    None,
    Candidate,
    Selected,
}

#[derive(Clone)]
pub struct GuiCard {
    pub card: Card,
    pub face_up: bool,
    x: i32,
    y: i32,
    angle: f64,
    pub scale: f64,
    pub state: SelectionState,
    pub alt_back: bool,
    pub on_click: Option<Arc<Fn(&mut GuiCard) -> Option<Card>>>,
}

impl fmt::Debug for GuiCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GuiCard {{ {:?} @({},{}) }}", self.card, self.x, self.y)
    }
}

impl GuiCard {
    pub fn new(card: Card) -> GuiCard {
        GuiCard {
            card,
            face_up: true,
            x: 0,
            y: 0,
            angle: 0.0,
            scale: DEFAULT_SCALE,
            state: SelectionState::None,
            alt_back: false,
            on_click: None,
        }
    }
    pub fn width(&self) -> u32 {
        (CARD_WIDTH as f64 * self.scale) as u32
    }
    pub fn height(&self) -> u32 {
        (CARD_HEIGHT as f64 * self.scale) as u32
    }

    fn covers_point(&self, x: i32, y: i32) -> bool {
        let center = Point::new(self.width() as i32 / 2, self.height() as i32 / 2);
        let (dx, dy) = (
            (self.x + center.x - x) as f64,
            (y - self.y - center.y) as f64,
        );
        let theta = self.angle.to_radians();
        let tx = center.x + (theta.cos() * dx - theta.sin() * dy) as i32;
        let ty = center.y + (theta.sin() * dx + theta.cos() * dy) as i32;
        let dest = Rect::new(0, 0, self.width(), self.height());
        dest.contains_point(Point::new(tx, ty))
    }

    fn move_around_self_axis(&mut self, delta_x: i32, delta_y: i32) -> MoveAnimation {
        let (dx, dy) = (delta_x as f64, delta_y as f64);
        let theta = self.angle.to_radians();
        let tx = (theta.cos() * dx - theta.sin() * dy) as i32;
        let ty = (theta.sin() * dx + theta.cos() * dy) as i32;
        MoveAnimation::new(
            self.get_position(),
            Point::new(self.x + tx, self.y + ty),
            3
        )
    }

    pub fn raise(&mut self) -> MoveAnimation {
        self.state = SelectionState::Candidate;
        self.move_around_self_axis(0, -20)
    }
    pub fn unraise(&mut self) -> MoveAnimation {
        self.state = SelectionState::None;
        self.move_around_self_axis(0, 20)
    }
}

impl Paintable for GuiCard {
    fn process(&mut self) -> bool {
        false
    }

    fn paint(&mut self, textures: &Textures, canvas: &mut WindowCanvas) -> Result<(), String> {
        let dest = Rect::new(self.x, self.y, self.width(), self.height());
        let center = Point::new(self.width() as i32 / 2, self.height() as i32 / 2);
        let (texture, src) = match (self.face_up, self.alt_back) {
            (true, _) => textures.card_front(self.card),
            (false, false) => textures.card_back(),
            (false, true) => textures.alt_back(),
        };
        canvas.copy_ex(texture, src, dest, self.angle, center, false, false)?;
        Ok(())
    }
}

impl Clickable for GuiCard {
    fn click(&mut self, x: i32, y: i32) -> (bool, Option<Card>) {
        if self.covers_point(x, y) {
            let mut yielded_card = None;
            if let Some(func) = self.on_click.as_ref().map(|a| Arc::clone(a)) {
                yielded_card = func(self);
            }
            return (true, yielded_card);
        }
        (false, None)
    }
}

impl Positioned for GuiCard {
    fn get_position(&self) -> Point {
        Point::new(self.x, self.y)
    }
    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

impl Rotatable for GuiCard {
    fn get_rotation(&self) -> f64 {
        self.angle
    }
    fn set_rotation(&mut self, angle: f64) {
        self.angle = angle;
    }
}
