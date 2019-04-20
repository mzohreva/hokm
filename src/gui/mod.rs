
use crate::cards::*;
use crate::game::*;
use crate::players::*;
use sdl2::image::LoadTexture;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;

mod animation;
mod card;
mod game;
mod hand;
mod main;
mod misc;
mod pile;
mod trick;

use animation::*;
use card::*;
use game::*;
use hand::*;
pub use main::gui_main;
use misc::*;
use pile::*;
use trick::*;

const SCENE_WIDTH: u32 = 800;
const SCENE_HEIGHT: u32 = 800;
const CARD_WIDTH: u32 = 226;
const CARD_HEIGHT: u32 = 316;
const DEFAULT_SCALE: f64 = 0.5;
const SMALLER_CARDS: f64 = DEFAULT_SCALE * 0.9;

pub trait Paintable {
    // if process() returns true, the object will be re-painted
    fn process(&mut self) -> bool;
    fn paint(&mut self, textures: &Textures, canvas: &mut WindowCanvas) -> Result<(), String>;
}

pub trait Clickable {
    fn click(&mut self, x: i32, y: i32) -> (bool, Option<Card>);
}

pub trait Positioned {
    fn get_position(&self) -> Point { Point::new(0,0) } // FIXME
    fn set_position(&mut self, x: i32, y: i32);
    fn move_by(&mut self, delta_x: i32, delta_y: i32) {
        let pos = self.get_position();
        self.set_position(pos.x + delta_x, pos.y + delta_y);
    }
}

pub trait Rotatable {
    fn get_rotation(&self) -> f64;
    fn set_rotation(&mut self, angle: f64);
    fn rotate_by(&mut self, degrees: f64) {
        self.set_rotation(self.get_rotation() + degrees);
    }
}

pub trait Animation<T>: AnimationClone<T> {
    fn animation_step(&mut self, object: &mut T) -> bool;
    fn reverse(&mut self);
}

impl<T> Clone for Box<Animation<T>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait AnimationClone<T> {
    fn clone_box(&self) -> Box<Animation<T>>;
}

impl<T, U> AnimationClone<T> for U where U: Animation<T> + 'static + Clone {
    fn clone_box(&self) -> Box<Animation<T>> {
        Box::new(self.clone())
    }
}

pub struct Textures<'a> {
    pub card_fronts: Texture<'a>,
    pub card_back: Texture<'a>,
    pub alt_back: Texture<'a>,
    pub suits: Texture<'a>,
    pub background: Texture<'a>,
}

impl<'a> Textures<'a> {
    pub fn load<T>(texture_creator: &'a TextureCreator<T>) -> Result<Self, String> {
        Ok(Textures {
            card_fronts: texture_creator.load_texture("res/cards.png")?,
            card_back: texture_creator.load_texture("res/back-3.png")?,
            alt_back: texture_creator.load_texture("res/back-2.png")?,
            suits: texture_creator.load_texture("res/suits.png")?,
            background: texture_creator.load_texture("res/background-2.png")?,
        })
    }

    pub fn card_front(&self, card: Card) -> (&Texture<'a>, Option<Rect>) {
        let rx = match card.rank().as_i32() {
            14 => 0,
            x => (x - 1) * 265,
        };
        let ry = match card.suit() {
            Suit::Clubs => 0,
            Suit::Hearts => 354,
            Suit::Spades => 354 * 2,
            Suit::Diamonds => 354 * 3,
        };
        (&self.card_fronts, Some(Rect::new(rx, ry, CARD_WIDTH, CARD_HEIGHT)))
    }
    pub fn card_back(&self) -> (&Texture<'a>, Option<Rect>) {
        (&self.card_back, None)
    }
    pub fn alt_back(&self) -> (&Texture<'a>, Option<Rect>) {
        (&self.alt_back, None)
    }
    pub fn suit(&self, suit: Suit) -> (&Texture<'a>, Option<Rect>) {
        let rx = match suit {
            Suit::Hearts => 0,
            Suit::Clubs => 60,
            Suit::Diamonds => 120,
            Suit::Spades => 180,
        };
        (&self.suits, Some(Rect::new(rx, 0, 60, 60)))
    }
}

#[derive(Clone, Debug)]
pub struct Stacked<T>(Vec<Vec<T>>);

impl<T: Clone> Stacked<T> {
    pub fn new() -> Self {
        Stacked(Vec::new())
    }
    pub fn add(&mut self, z: u8, obj: T) {
        let z = z as usize;
        if z >= self.0.len() {
            self.0.resize(z + 1, Vec::new());
        }
        self.0[z].push(obj);
    }
    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = &T> + 'a> {
        Box::new(self.0.iter().flatten())
    }
    pub fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &mut T> + 'a> {
        Box::new(self.0.iter_mut().flatten())
    }
    pub fn iter_mut_rev<'a>(&'a mut self) -> Box<Iterator<Item = &mut T> + 'a> {
        Box::new(self.0.iter_mut().flatten().rev())
    }
}

impl<T: Paintable + Clone> Paintable for Stacked<T> {
    fn process(&mut self) -> bool {
        let mut repaint = false;
        for o in self.iter_mut() {
            repaint |= o.process();
        }
        repaint
    }

    fn paint(&mut self, textures: &Textures, canvas: &mut WindowCanvas) -> Result<(), String> {
        for o in self.iter_mut() {
            o.paint(textures, canvas)?;
        }
        Ok(())
    }
}

impl<T: Clickable + Clone> Clickable for Stacked<T> {
    fn click(&mut self, x: i32, y: i32) -> (bool, Option<Card>) {
        for o in self.iter_mut_rev() {
            let (handled, yielded_card) = o.click(x, y);
            if handled {
                return (true, yielded_card);
            }
        }
        (false, None)
    }
}

#[derive(Clone)]
pub struct Animated<T> {
    pub object: T,
    animations: Vec<Box<Animation<T>>>,
}

impl<T> Animated<T> {
    pub fn new(object: T) -> Self {
        Animated {
            object,
            animations: Vec::new(),
        }
    }
    pub fn add_animation(&mut self, animation: Box<Animation<T>>) {
        self.animations.push(animation);
    }
}

impl<T: Positioned> Animated<T> {
    pub fn move_to(&mut self, dest: Point, steps: u32) {
        self.add_animation(Box::new(MoveAnimation::new(
            self.object.get_position(),
            dest,
            steps
        )));
    }
}

impl<T: Rotatable> Animated<T> {
    pub fn rotate_to(&mut self, dest: f64, steps: u32) {
        self.add_animation(Box::new(RotateAnimation::new(
            self.object.get_rotation(),
            dest,
            steps
        )));
    }
}

impl Animated<GuiCard> {
    pub fn flip_card(&mut self, steps: u32) {
        self.add_animation(Box::new(FlipCard::new(steps)));
    }

    pub fn scale_card(&mut self, dest_scale: f64, steps: u32) {
        self.add_animation(Box::new(ScaleCard::new(
            self.object.scale,
            dest_scale,
            steps
        )));
    }
}

impl<T: Paintable> Paintable for Animated<T> {
    fn process(&mut self) -> bool {
        let mut repaint = false;
        let mut keep = Vec::with_capacity(self.animations.len());
        for a in self.animations.iter_mut() {
            let r = a.animation_step(&mut self.object);
            keep.push(r);
            repaint |= r;
        }
        // Remove animations that are finished
        for i in (0..keep.len()).rev() {
            if !keep[i] {
                self.animations.swap_remove(i);
            }
        }
        repaint
    }

    fn paint(&mut self, textures: &Textures, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.object.paint(textures, canvas)
    }
}

impl<T: Clickable> Clickable for Animated<T> {
    fn click(&mut self, x: i32, y: i32) -> (bool, Option<Card>) {
        self.object.click(x, y)
    }
}

pub struct Scene {
    game: Game,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { game: Game::new() }
    }
}

impl Paintable for Scene {
    fn process(&mut self) -> bool {
        self.game.process()
    }

    fn paint(&mut self, textures: &Textures, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.clear();
        canvas.copy(&textures.background, None, None)?;
        self.game.paint(textures, canvas)?;
        canvas.present();
        Ok(())
    }
}

impl Clickable for Scene {
    fn click(&mut self, x: i32, y: i32) -> (bool, Option<Card>) {
        self.game.click(x, y)
    }
}
