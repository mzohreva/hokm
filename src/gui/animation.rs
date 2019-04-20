
use super::*;
use sdl2::rect::Point;

const ANIMATION_SPEED_FACTOR: u32 = 1;

#[derive(Clone)]
pub struct MoveAnimation {
    start: Point,
    end: Point,
    pos: Point,
    steps: u32,
    s: u32,
}

impl MoveAnimation {
    pub fn new(start: Point, end: Point, steps: u32) -> Self {
        MoveAnimation {
            start, end,
            pos: start,
            steps: steps * ANIMATION_SPEED_FACTOR,
            s: 0,
        }
    }
}

impl<T: Positioned> Animation<T> for MoveAnimation {
    fn animation_step(&mut self, object: &mut T) -> bool {
        if self.s >= self.steps {
            return false;
        }
        let rem = self.steps as i32 - self.s as i32;
        let dx = self.end.x() - self.pos.x();
        let dy = self.end.y() - self.pos.y();
        self.pos = self.pos.offset(dx / rem, dy / rem);
        object.set_position(self.pos.x(), self.pos.y());
        self.s += 1;
        true
    }

    fn reverse(&mut self) {
        let (start, end) = (self.start, self.end);
        self.start = end;
        self.end = start;
        self.s = 0;
    }
}

#[derive(Clone)]
pub struct RotateAnimation {
    start: f64,
    end: f64,
    angle: f64,
    steps: u32,
    s: u32,
}

impl RotateAnimation {
    pub fn new(start: f64, end: f64, steps: u32) -> Self {
        RotateAnimation {
            start, end,
            angle: start,
            steps: steps * ANIMATION_SPEED_FACTOR,
            s: 0,
        }
    }
}

impl<T: Rotatable> Animation<T> for RotateAnimation {
    fn animation_step(&mut self, object: &mut T) -> bool {
        if self.s >= self.steps {
            return false;
        }
        let rem = self.steps as i32 - self.s as i32;
        let da = self.end - self.angle;
        self.angle = self.angle + da / rem as f64;
        object.set_rotation(self.angle);
        self.s += 1;
        true
    }

    fn reverse(&mut self) {
        let (start, end) = (self.start, self.end);
        self.start = end;
        self.end = start;
        self.s = 0;
    }
}

#[derive(Clone)]
pub struct FlipCard {
    steps: u32,
    s: u32,
}

impl FlipCard {
    pub fn new(steps: u32) -> Self {
        FlipCard { steps: steps * ANIMATION_SPEED_FACTOR, s: 0 }
    }
}

impl Animation<GuiCard> for FlipCard {
    fn animation_step(&mut self, object: &mut GuiCard) -> bool {
        if self.s >= self.steps {
            return false;
        }
        if self.s == self.steps / 2 {
            object.face_up = !object.face_up;
        }
        self.s += 1;
        true
    }
    fn reverse(&mut self) {
        self.s = 0;
    }
}

#[derive(Clone)]
pub struct ScaleCard {
    start: f64,
    end: f64,
    steps: u32,
    s: u32,
}

impl ScaleCard {
    pub fn new(start: f64, end: f64, steps: u32) -> Self {
        ScaleCard {
            start, end,
            steps: steps * ANIMATION_SPEED_FACTOR,
            s: 0,
        }
    }
}

impl Animation<GuiCard> for ScaleCard {
    fn animation_step(&mut self, object: &mut GuiCard) -> bool {
        if self.s >= self.steps {
            return false;
        }
        let rem = self.steps as i32 - self.s as i32;
        let ds = self.end - object.scale;
        object.scale = object.scale + ds / rem as f64;
        self.s += 1;
        true
    }
    fn reverse(&mut self) {
        let (start, end) = (self.start, self.end);
        self.start = end;
        self.end = start;
        self.s = 0;
    }
}
