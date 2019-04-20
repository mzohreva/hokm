
use super::*;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

pub struct Circle {
    pub cx: i32,
    pub cy: i32,
    pub radius: u32,
}

impl Circle {
    pub fn find_point(&self, angle: f32) -> (i32, i32) {
        let x = (self.radius as f32 * angle.cos()) as i32;
        let y = (self.radius as f32 * angle.sin()) as i32;
        (self.cx + x, self.cy - y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
pub struct GuiPlayerScore {
    pub score: u32,
    pub x: i32,
    pub y: i32,
    pub orientation: Orientation,
}

impl GuiPlayerScore {
    pub fn new(orientation: Orientation) -> Self {
        GuiPlayerScore {
            score: 0,
            x: 0,
            y: 0,
            orientation,
        }
    }
}

impl Paintable for GuiPlayerScore {
    fn process(&mut self) -> bool {
        false
    }

    fn paint(&mut self, _textures: &Textures, canvas: &mut WindowCanvas) -> Result<(), String> {
        let color = Color::RGB(255, 255, 255);
        for i in 0..7 {
            let (x, y) = match self.orientation {
                Orientation::Horizontal => (self.x as i16 + i * 10, self.y as i16),
                Orientation::Vertical => (self.x as i16, self.y as i16 + i * 10),
            };
            let radius = 3;
            if (i as u32) < self.score {
                canvas.filled_circle(x, y, radius, color)?;
            } else {
                canvas.circle(x, y, radius, color)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct GuiCardPlaceHolder {
    x: i32,
    y: i32,
    pub scale: f64,
}

impl GuiCardPlaceHolder {
    pub fn new() -> Self {
        GuiCardPlaceHolder {
            x: 0,
            y: 0,
            scale: DEFAULT_SCALE,
        }
    }
    pub fn width(&self) -> u32 {
        (CARD_WIDTH as f64 * self.scale - 1.0) as u32
    }
    pub fn height(&self) -> u32 {
        (CARD_HEIGHT as f64 * self.scale - 1.0) as u32
    }
}

impl Paintable for GuiCardPlaceHolder {
    fn process(&mut self) -> bool { false }

    fn paint(&mut self, _textures: &Textures, canvas: &mut WindowCanvas) -> Result<(), String> {
        let dest = Rect::new(self.x, self.y, self.width(), self.height());
        let p1 = dest.top_left();
        let p2 = dest.bottom_right();
        let (x1, y1) = (p1.x() as i16, p1.y() as i16);
        let (x2, y2) = (p2.x() as i16, p2.y() as i16);
        let rad = (10.0 * self.scale) as i16;
        canvas.rounded_rectangle(x1, y1, x2, y2, rad, Color::RGB(0, 0, 0))?;
        Ok(())
    }
}

impl Positioned for GuiCardPlaceHolder {
    fn get_position(&self) -> Point {
        Point::new(self.x, self.y)
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Clone, Debug)]
pub enum GuiCardOrPlaceHolder {
    Card(GuiCard),
    PlaceHolder(GuiCardPlaceHolder),
}

impl Paintable for GuiCardOrPlaceHolder {
    fn process(&mut self) -> bool {
        match self {
            GuiCardOrPlaceHolder::Card(c) => c.process(),
            GuiCardOrPlaceHolder::PlaceHolder(p) => p.process(),
        }
    }

    fn paint(&mut self, textures: &Textures, canvas: &mut WindowCanvas) -> Result<(), String> {
        match self {
            GuiCardOrPlaceHolder::Card(c) => c.paint(textures, canvas),
            GuiCardOrPlaceHolder::PlaceHolder(p) => p.paint(textures, canvas),
        }
    }
}
