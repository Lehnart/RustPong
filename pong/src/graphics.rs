use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;

use engine::geometry::AsRect;
use engine::graphics::{RectSprite, RenderedString, Window};

use crate::logic::Logic;

pub const MID_LINE_N: u32 = 30;
pub const MID_LINE_WIDTH: u32 = 5;
pub const MID_LINE_RELATIVE_LENGTH: f32 = 0.6;

pub const SCORE_POINT_SIZE: u16 = 48;
pub const SCORE_POSITION_Y: i32 = 100;
pub const LEFT_SCORE_POSITION_X: i32 = 150;
pub const RIGHT_SCORE_POSITION_X: i32 = 450;


pub const FONT_PATH: &str = "res/atari.ttf";

/// Struct containing all basic dynamic elements required to draw the game.
///
/// The graphics part contains the 2 rackets, the ball, and the score.
pub struct Graphics {
    left_racket: RectSprite,
    right_racket: RectSprite,
    ball: RectSprite,
    score: Score,
}

impl Graphics {
    /// Init the dynamic elements required to draw the game
    pub fn new() -> Graphics {
        Graphics {
            left_racket: RectSprite::default(Color::WHITE),
            right_racket: RectSprite::default(Color::WHITE),
            ball: RectSprite::default(Color::WHITE),
            score: Score::new(),
        }
    }

    /// Update the dynamic elements accordingly to the state of the game.
    pub fn update(&mut self, logic: &Logic, window: &Window) {
        let w = window.width();
        let h = window.height();

        self.left_racket.update(logic.left_racket.as_rect(), w, h);
        self.right_racket.update(logic.right_racket.as_rect(), w, h);
        self.ball.update(logic.ball.as_rect(), w, h);
        self.score.update(logic);
    }

    /// Draw the game.
    ///
    /// Start by clearing the all board.
    /// Then, it draws the static element : the mid line for instance.
    /// Finally, it draws each dynamic element and show the canvas
    pub fn draw(&self, window: &mut Window, ttf_context: &Sdl2TtfContext) {
        window.clear();

        self.draw_mid_line(window);

        let canvas = &mut window.canvas;
        canvas.fill_rect(self.left_racket.rect).unwrap();
        canvas.fill_rect(self.right_racket.rect).unwrap();
        canvas.fill_rect(self.ball.rect).unwrap();

        self.score.draw(canvas, ttf_context);

        canvas.present();
    }


    /// Private function to draw the static mid line.
    fn draw_mid_line(&self, window: &mut Window) {
        let n_lines = MID_LINE_N;
        let len_line = window.height() / n_lines;
        let line_width = MID_LINE_WIDTH;
        let line_height = (MID_LINE_RELATIVE_LENGTH * len_line as f32) as u32;
        let line_x = (window.width() - line_width) / 2;

        let canvas = &mut window.canvas;
        canvas.set_draw_color(Color::WHITE);
        for i in 0..n_lines {
            let rect = Rect::new(line_x as i32, (i * len_line) as i32, line_width, line_height);
            canvas.fill_rect(rect).unwrap();
        }
    }
}

/// Used to draw the current score, using digit sprites.
struct Score {
    left: u8,
    right: u8,
}

impl Score {
    /// Create an empty score
    pub fn new() -> Score {
        let left: u8 = 0;
        let right: u8 = 0;
        Score {
            left,
            right,
        }
    }

    /// Get the current score from the game state
    pub fn update(&mut self, logic: &Logic) {
        self.left = logic.score.left();
        self.right = logic.score.right();
    }

    /// Draw the score on the scree, first selecting the right sprites from the digit, then showing it.
    pub fn draw(&self, canvas: &mut WindowCanvas, ttf_context: &Sdl2TtfContext) {
        let left_str = RenderedString::new(
            &self.left.to_string(),
            LEFT_SCORE_POSITION_X,
            SCORE_POSITION_Y, ttf_context,
            FONT_PATH,
            SCORE_POINT_SIZE,
        );

        let right_str = RenderedString::new(
            &self.right.to_string(),
            RIGHT_SCORE_POSITION_X,
            SCORE_POSITION_Y, ttf_context,
            FONT_PATH,
            SCORE_POINT_SIZE,
        );

        left_str.draw(canvas);
        right_str.draw(canvas);
    }
}