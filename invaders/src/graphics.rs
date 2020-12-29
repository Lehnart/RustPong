use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;

use engine::geometry;
use engine::geometry::AsRect;
use engine::graphics::{RectSprite, RenderedString, Sprite, Window};

use crate::logic::{Logic, SPACESHIP_HEIGHT, SPACESHIP_WIDTH};
use crate::logic;

pub const SPACESHIP_SPRITE_PATH: &str = "res/sprite/spaceship.bmp";
pub const MISSILE_COLOR: Color = Color::GREEN;

pub struct Spaceship<'a> {
    spaceship: Sprite<'a>,
    missile: RectSprite,
}

impl Spaceship<'_> {
    pub fn new(cw: u32, ch: u32) -> Spaceship<'static> {
        let sprite_rect = Rect::new(0, 0, (SPACESHIP_WIDTH * cw as f32) as u32, (SPACESHIP_HEIGHT * ch as f32) as u32);
        let sprite = Sprite::simple_new(SPACESHIP_SPRITE_PATH, sprite_rect);
        Spaceship {
            spaceship: sprite,
            missile: RectSprite::default(MISSILE_COLOR),
        }
    }

    pub fn update(&mut self, logic_spaceship: &logic::Spaceship, cw: u32, ch: u32) {
        self.spaceship.update(logic_spaceship.as_rect(), 0., cw, ch);
        if logic_spaceship.missile.is_destroyed() {
            self.missile.hide();
        } else {
            self.missile.show();
            self.missile.update(logic_spaceship.missile.as_rect(), cw, ch);
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        self.spaceship.draw(canvas);
        self.missile.draw(canvas);
    }
}


pub struct Graphics<'a> {
    spaceship: Spaceship<'a>,
}

impl Graphics<'_> {
    /// Init the dynamic elements required to draw the game
    pub fn new(cw: u32, ch: u32, ttf_context: &Sdl2TtfContext) -> Graphics {
        Graphics {
            spaceship: Spaceship::new(cw, ch),
        }
    }

    /// Update the dynamic elements accordingly to the state of the game.
    pub fn update(&mut self, logic: &Logic, window: &Window, ttf_context: &Sdl2TtfContext) {
        let w = window.width();
        let h = window.height();
        self.spaceship.update(&logic.spaceship, w, h)
    }

    /// Draw the game.
    ///
    /// Start by clearing the all board.
    /// It draws each dynamic element and show the canvas
    pub fn draw(&self, window: &mut Window) {
        window.clear();
        let canvas = &mut window.canvas;
        self.spaceship.draw(canvas);
        canvas.present();
    }
}