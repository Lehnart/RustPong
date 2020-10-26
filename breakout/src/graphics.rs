use engine::graphics::{Sprite, Window};
use crate::logic::{Logic, BOARD_LEFT_LIMIT_X, BOARD_RIGHT_LIMIT_X, BLOCK_ROW_N, BLOCK_COL_N, BOARD_TOP_LIMIT_Y};
use engine::geometry::{AsRect, Rect};
use sdl2::pixels::Color;

pub const RACKET_COLOR :Color = Color{ r:62,g:117,b:207,a:0 };
pub const LIMIT_COLOR :Color = Color::WHITE;
pub const BLOCK_COLORS : [Color;8] =[Color::RED, Color::RED, Color::BLUE, Color::BLUE, Color::GREEN, Color::GREEN, Color::YELLOW, Color::YELLOW];
pub const BALL_COLOR : Color = Color::WHITE;

/// Struct containing all basic dynamic elements required to draw the game.
///
pub struct Graphics {
    racket: Sprite,
    left_limit: Sprite,
    right_limit: Sprite,
    top_limit: Sprite,
    blocks : Vec<Sprite>,
    ball : Sprite,
}

impl Graphics {

    /// Init the dynamic elements required to draw the game
    pub fn new() -> Graphics {
        let mut blocks: Vec<Sprite> = Vec::new();
        for i in 0..BLOCK_ROW_N {
            for _j in 0..BLOCK_COL_N{
                blocks.push( Sprite::default(BLOCK_COLORS[i as usize]));
            }
        }

        Graphics {
            racket: Sprite::default(RACKET_COLOR),
            left_limit: Sprite::default(LIMIT_COLOR),
            right_limit: Sprite::default(LIMIT_COLOR),
            top_limit: Sprite::default(LIMIT_COLOR),
            ball : Sprite::default(BALL_COLOR),
            blocks
        }
    }

    /// Update the dynamic elements accordingly to the state of the game.
    pub fn update(&mut self, logic: &Logic, window: &Window) {
        let w = window.width();
        let h = window.height();

        self.racket.update(logic.racket.as_rect(), w, h);
        self.ball.update(logic.ball.as_rect(), w, h);
        self.left_limit.update(Rect::from_2_points(0.,0., BOARD_LEFT_LIMIT_X, 1.), w, h);
        self.right_limit.update(Rect::from_2_points(BOARD_RIGHT_LIMIT_X,0., 1.01, 1.), w, h);
        self.top_limit.update(Rect::from_2_points(0.,0., 1.0, BOARD_TOP_LIMIT_Y), w, h);

        for i in 0..self.blocks.len(){
            let block_logic = logic.blocks.get(i);
            let block_graphics = &mut self.blocks[i];
            block_graphics.update(block_logic.as_rect(), w, h);
        }
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
        self.ball.draw(canvas);
        self.left_limit.draw(canvas);
        self.right_limit.draw(canvas);
        self.top_limit.draw(canvas);
        for b in &self.blocks{
            b.draw(canvas);
        }
        canvas.present();
    }
}