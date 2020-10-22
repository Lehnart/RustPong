use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::surface::Surface;

use engine::geometry::AsRect;
use engine::graphics::{Sprite, Window};

use crate::logic::Logic;

pub const SCORE_SPRITE_PATHS: [&str; 10] = [
    "res/0.bmp",
    "res/1.bmp",
    "res/2.bmp",
    "res/3.bmp",
    "res/4.bmp",
    "res/5.bmp",
    "res/6.bmp",
    "res/7.bmp",
    "res/8.bmp",
    "res/9.bmp"
];

pub const MID_LINE_N: u32 = 30;
pub const MID_LINE_WIDTH: u32 = 5;
pub const MID_LINE_RELATIVE_LENGTH : f32 = 0.6;

/// Struct containing all basic dynamic elements required to draw the game.
///
/// The graphics part contains the 2 rackets, the ball, and the score.
pub struct Graphics<'a> {
    left_racket: Sprite,
    right_racket: Sprite,
    ball: Sprite,
    score: Score<'a>,
}

impl Graphics<'_> {

    /// Init the dynamic elements required to draw the game
    pub fn new<'a>() -> Graphics<'a> {
        Graphics {
            left_racket: Sprite::default(Color::WHITE),
            right_racket: Sprite::default(Color::WHITE),
            ball: Sprite::default(Color::WHITE),
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
    pub fn draw(&self, window: &mut Window) {

        window.clear();

        self.draw_mid_line(window);

        let canvas = &mut window.canvas;
        canvas.fill_rect(self.left_racket.rect).unwrap();
        canvas.fill_rect(self.right_racket.rect).unwrap();
        canvas.fill_rect(self.ball.rect).unwrap();

        self.score.draw(canvas);

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
struct Score<'a> {
    left: u8,
    right: u8,
    digits: Vec<Surface<'a>>,
}

impl Score<'_> {

    /// Create an empty score
    pub fn new<'a>() -> Score<'a> {
        let mut digits: Vec<Surface> = Vec::new();
        for path in SCORE_SPRITE_PATHS.iter() {
            let surface = Surface::load_bmp(path).unwrap();
            digits.push(surface);
        }
        let left: u8 = 0;
        let right: u8 = 0;
        Score {
            left,
            right,
            digits,
        }
    }

    /// Get the current score from the game state
    pub fn update(&mut self, logic: &Logic) {
        self.left = logic.score.left();
        self.right = logic.score.right();
    }

    /// Draw the score on the scree, first selecting the right sprites from the digit, then showing it.
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        let left_digit = &self.digits[(self.left % 10) as usize];
        let right_digit = &self.digits[(self.right % 10) as usize];

        let texture_creator = canvas.texture_creator();
        let left_texture = &texture_creator.create_texture_from_surface(left_digit).unwrap();
        let right_texture = &texture_creator.create_texture_from_surface(right_digit).unwrap();

        let w = left_digit.width();
        let h = left_digit.height();

        let c_dim = canvas.output_size().unwrap();
        let cw = c_dim.0;

        canvas.copy(
            left_texture,
            Rect::new(0, 0, w, h),
            Rect::new(((cw / 4) - (w / 2)) as i32, 50, w, h),
        ).unwrap();

        canvas.copy(
            right_texture,
            Rect::new(0, 0, w, h),
            Rect::new(((3 * cw / 4) - (w / 2)) as i32, 50, w, h),
        ).unwrap();
    }
}