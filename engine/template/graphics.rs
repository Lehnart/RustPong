use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;

use engine::geometry;
use engine::geometry::AsRect;
use engine::graphics::{RectSprite, RenderedString, Sprite, Window};

use crate::logic::Logic;
use crate::logic;


pub struct Graphics<> {

}

impl Graphics {
    /// Init the dynamic elements required to draw the game
    pub fn new(canvas_width: u32, canvas_height: u32, ttf_context: &Sdl2TtfContext) -> Graphics {
        Graphics {
        }
    }

    /// Update the dynamic elements accordingly to the state of the game.
    pub fn update(&mut self, logic: &Logic, window: &Window, ttf_context: &Sdl2TtfContext) {
    }

    /// Draw the game.
    ///
    /// Start by clearing the all board.
    /// It draws each dynamic element and show the canvas
    pub fn draw(&self, window: &mut Window) {
        window.clear();

        let canvas = &mut window.canvas;


        canvas.present();
    }
}