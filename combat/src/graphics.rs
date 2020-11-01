use engine::graphics::{Sprite, Window};
use sdl2::pixels::Color;
use crate::logic::Logic;
use engine::geometry::AsRect;


pub struct Graphics {
    left_tank: Sprite,
}

impl Graphics {
    /// Init the dynamic elements required to draw the game
    pub fn new() -> Graphics {
        Graphics {
            left_tank: Sprite::default(Color::WHITE),
        }
    }

    /// Update the dynamic elements accordingly to the state of the game.
    pub fn update(&mut self, logic: &Logic, window: &Window) {
        let w = window.width();
        let h = window.height();

        self.left_tank.update(logic.left_tank.as_rect(), w, h);
    }

    /// Draw the game.
    ///
    /// Start by clearing the all board.
    /// It draws each dynamic element and show the canvas
    pub fn draw(&self, window: &mut Window) {
        window.clear();

        let canvas = &mut window.canvas;
        self.left_tank.draw(canvas);
        canvas.present();
    }
}