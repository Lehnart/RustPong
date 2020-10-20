use engine::graphics::{Sprite, Window};
use crate::logic::{Logic, BOARD_LEFT_LIMIT_X, BOARD_RIGHT_LIMIT_X};
use engine::geometry::{AsRect, Rect};
use sdl2::pixels::Color;

pub const RACKET_COLOR :Color = Color{ r:62,g:117,b:207,a:0 };
pub const LIMIT_COLOR :Color = Color::WHITE;

/// Struct containing all basic dynamic elements required to draw the game.
///
pub struct Graphics {
    racket: Sprite,
    left_limit: Sprite,
    right_limit: Sprite,
}

impl Graphics {

    /// Init the dynamic elements required to draw the game
    pub fn new() -> Graphics {
        Graphics {
            racket: Sprite::default(RACKET_COLOR),
            left_limit: Sprite::default(LIMIT_COLOR),
            right_limit: Sprite::default(LIMIT_COLOR),
        }
    }

    /// Update the dynamic elements accordingly to the state of the game.
    pub fn update(&mut self, logic: &Logic, window: &Window) {
        let w = window.width();
        let h = window.height();

        self.racket.update(logic.racket.as_rect(), w, h);
        self.left_limit.update(Rect::from_2_points(0.,0., BOARD_LEFT_LIMIT_X, 1.), w, h);
        self.right_limit.update(Rect::from_2_points(BOARD_RIGHT_LIMIT_X,0., 1.01, 1.), w, h);
    }

    /// Draw the game.
    ///
    /// Start by clearing the all board.
    /// Then, it draws the static element : the mid line for instance.
    /// Finally, it draws each dynamic element and show the canvas
    pub fn draw(&self, window: &mut Window) {

        window.clear();


        let canvas = &mut window.canvas;
        self.racket.draw(canvas);
        self.left_limit.draw(canvas);
        self.right_limit.draw(canvas);

        canvas.present();
    }
}