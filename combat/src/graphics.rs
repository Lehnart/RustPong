use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;

use engine::geometry;
use engine::geometry::AsRect;
use engine::graphics::{RectSprite, RenderedString, Sprite, Window};

use crate::logic::{BLOCK_COL_COUNT, BLOCK_ROW_COUNT, BOARD_BOTTOM_LIMIT, BOARD_LEFT_LIMIT, BOARD_RIGHT_LIMIT, BOARD_TOP_LIMIT, BOARD_TOP_LIMIT_HEIGHT, Logic, Map, TANK_HEIGHT, TANK_WIDTH};
use crate::logic;

pub const FONT_PATH: &str = "res/atari.ttf";
pub const FONT_SIZE: u16 = 96;

pub const TANK_SPRITE_PATH: &str = "res/tank.bmp";
pub const LEFT_TANK_COLOR: Color = Color::RGB(255, 0, 0);
pub const RIGHT_TANK_COLOR: Color = Color::RGB(0, 0, 255);

pub const LIMIT_COLOR: Color = Color::WHITE;
pub const DECOR_COLOR: Color = Color::WHITE;

pub const BLOCK_SIZE: i32 = 18;

pub const LEFT_SCORE_POSITION_X: i32 = 100;
pub const LEFT_SCORE_POSITION_Y: i32 = 50;
pub const LEFT_SCORE_COLOR: Color = LEFT_TANK_COLOR;

pub const RIGHT_SCORE_POSITION_X: i32 = 500;
pub const RIGHT_SCORE_POSITION_Y: i32 = 50;
pub const RIGHT_SCORE_COLOR: Color = RIGHT_TANK_COLOR;

/// A decor is the graphics counterpart of the map in the game.
/// It is a grid of rectangle figures.
pub struct Decor {
    blocks: [[RectSprite; BLOCK_COL_COUNT]; BLOCK_ROW_COUNT],
    x_shift: i32,
    y_shift: i32,
}

impl Decor {
    /// Create a grid of rect that will be updated accordingly to a map
    pub fn new(x_shift: i32, y_shift: i32, canvas_width: u32) -> Decor {
        let ys = y_shift as f32 + (canvas_width as f32 * BOARD_TOP_LIMIT);
        let xs = x_shift as f32 + (canvas_width as f32 * BOARD_LEFT_LIMIT);

        let blocks = [[RectSprite::new(0, 0, DECOR_COLOR); BLOCK_COL_COUNT]; BLOCK_ROW_COUNT];
        Decor {
            blocks,
            x_shift: xs as i32,
            y_shift: ys as i32,
        }
    }

    /// Update the grid of rectangle
    fn update(&mut self, map: &Map) {
        for j in 0..BLOCK_ROW_COUNT {
            for i in 0..BLOCK_COL_COUNT {
                let block = &mut self.blocks[j][i];
                let map_block = map.get_block(i as u32, j as u32);
                match map_block {
                    None => block.hide(),
                    Some(_) => {
                        block.show();
                        block.rect.set_width(BLOCK_SIZE as u32);
                        block.rect.set_height(BLOCK_SIZE as u32);
                        block.rect.set_x(self.x_shift + (BLOCK_SIZE * i as i32));
                        block.rect.set_y(self.y_shift + (BLOCK_SIZE * j as i32));
                    }
                }
            }
        }
    }

    /// Draw to the screen the decor
    fn draw(&self, canvas: &mut WindowCanvas) {
        for j in 0..BLOCK_ROW_COUNT {
            for i in 0..BLOCK_COL_COUNT {
                let block = &self.blocks[j][i];
                block.draw(canvas);
            }
        }
    }
}

/// Limit of the game board as 4 rectangles.
pub struct Limit {
    left_limit: RectSprite,
    right_limit: RectSprite,
    top_limit: RectSprite,
    bottom_limit: RectSprite,
}

impl Limit {
    pub fn new(y_shift: i32) -> Limit {
        let left_limit = RectSprite::new(0, y_shift, LIMIT_COLOR);
        let right_limit = RectSprite::new(0, y_shift, LIMIT_COLOR);
        let top_limit = RectSprite::new(0, y_shift, LIMIT_COLOR);
        let bottom_limit = RectSprite::new(0, y_shift, LIMIT_COLOR);

        Limit {
            left_limit,
            right_limit,
            top_limit,
            bottom_limit,
        }
    }

    pub fn update(&mut self, w: u32) {
        self.left_limit.update(
            geometry::Rect::from_2_points(0., BOARD_TOP_LIMIT - BOARD_TOP_LIMIT_HEIGHT, BOARD_LEFT_LIMIT, 1.01),
            w,
            w,
        );
        self.top_limit.update(
            geometry::Rect::from_2_points(0., 0., 1.01, BOARD_TOP_LIMIT),
            w,
            w,
        );
        self.right_limit.update(
            geometry::Rect::from_2_points(BOARD_RIGHT_LIMIT, BOARD_TOP_LIMIT - BOARD_TOP_LIMIT_HEIGHT, 1.01, 1.01),
            w,
            w,
        );
        self.bottom_limit.update(
            geometry::Rect::from_2_points(0., BOARD_BOTTOM_LIMIT, 1.01, 1.01),
            w,
            w,
        );
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        self.left_limit.draw(canvas);
        self.right_limit.draw(canvas);
        self.top_limit.draw(canvas);
        self.bottom_limit.draw(canvas);
    }
}

pub struct Score<'a> {
    left_score: String,
    right_score: String,
    rendered_left_score: RenderedString<'a>,
    rendered_right_score: RenderedString<'a>,
}

impl Score<'_> {
    pub fn new(ttf_context: &Sdl2TtfContext) -> Score {
        let left_score = "0".parse().unwrap();
        let right_score = "0".parse().unwrap();

        Score {
            left_score,
            right_score,
            rendered_left_score: RenderedString::new_colored(
                &"0".parse().unwrap(),
                LEFT_SCORE_POSITION_X,
                LEFT_SCORE_POSITION_Y,
                ttf_context,
                FONT_PATH,
                FONT_SIZE,
                LEFT_SCORE_COLOR,
            ),
            rendered_right_score: RenderedString::new_colored(
                &"0".parse().unwrap(),
                RIGHT_SCORE_POSITION_X,
                RIGHT_SCORE_POSITION_Y,
                ttf_context,
                FONT_PATH,
                FONT_SIZE,
                RIGHT_SCORE_COLOR,
            ),
        }
    }

    pub fn update(&mut self, logic: &Logic, ttf_context: &Sdl2TtfContext) {
        if !self.left_score.eq(&logic.score.get_left_score().to_string()) {
            self.left_score = logic.score.get_left_score().to_string();
            self.rendered_left_score = RenderedString::new_colored(
                &self.left_score,
                LEFT_SCORE_POSITION_X,
                LEFT_SCORE_POSITION_Y,
                ttf_context,
                FONT_PATH,
                FONT_SIZE,
                LEFT_SCORE_COLOR,
            )
        }

        if !self.right_score.eq(&logic.score.get_right_score().to_string()) {
            self.right_score = logic.score.get_right_score().to_string();
            self.rendered_right_score = RenderedString::new_colored(
                &self.right_score,
                RIGHT_SCORE_POSITION_X,
                RIGHT_SCORE_POSITION_Y,
                ttf_context,
                FONT_PATH,
                FONT_SIZE,
                RIGHT_SCORE_COLOR,
            )
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        self.rendered_right_score.draw(canvas);
        self.rendered_left_score.draw(canvas);
    }
}

/// Graphics representation of the tank and of his bullet
pub struct Tank<'a> {
    tank: Sprite<'a>,
    shell: RectSprite,
}

impl Tank<'_> {
    pub fn new(y_shift: i32, color: Color, canvas_width: u32) -> Tank<'static> {
        let tank_rect = Rect::new(0, 0, (TANK_WIDTH * canvas_width as f32) as u32, (TANK_HEIGHT * canvas_width as f32) as u32);
        let tank = Sprite::new(0, y_shift, TANK_SPRITE_PATH, tank_rect, color);
        let shell = RectSprite::new(0, y_shift, color);
        Tank {
            tank,
            shell,
        }
    }

    pub fn update(&mut self, logic_tank: &logic::Tank, w: u32) {
        let mut tank_angle = logic_tank.get_orientation().to_degrees() as f64;
        if logic_tank.is_impacted() {
            tank_angle = self.tank.angle + 45.;
        }
        self.tank.update(logic_tank.as_rect(), tank_angle, w, w);
        let logic_shell = logic_tank.get_shell();

        if logic_shell.is_destroyed() {
            self.shell.hide();
        } else {
            self.shell.show();
            self.shell.update(logic_shell.as_rect(), w, w);
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        self.shell.draw(canvas);
        self.tank.draw(canvas);
    }
}

pub struct Graphics<'a> {
    left_tank: Tank<'a>,
    right_tank: Tank<'a>,
    limit: Limit,
    decor: Decor,
    score: Score<'a>,
}

impl Graphics<'_> {
    /// Init the dynamic elements required to draw the game
    pub fn new(canvas_width: u32, canvas_height: u32, ttf_context: &Sdl2TtfContext) -> Graphics {
        let y_shift = (canvas_height - canvas_width) as i32;

        let left_tank = Tank::new(y_shift, LEFT_TANK_COLOR, canvas_width);
        let right_tank = Tank::new(y_shift, RIGHT_TANK_COLOR, canvas_width);
        let limit = Limit::new(y_shift);
        let decor = Decor::new(0, y_shift, canvas_width);
        let score = Score::new(&ttf_context);
        Graphics {
            left_tank,
            right_tank,
            limit,
            decor,
            score,
        }
    }

    /// Update the dynamic elements accordingly to the state of the game.
    pub fn update(&mut self, logic: &Logic, window: &Window, ttf_context: &Sdl2TtfContext) {
        let w = window.width();

        self.left_tank.update(&logic.left_tank, w);
        self.right_tank.update(&logic.right_tank, w);

        self.limit.update(w);
        self.decor.update(&logic.map);

        self.score.update(logic, ttf_context);
    }

    /// Draw the game.
    ///
    /// Start by clearing the all board.
    /// It draws each dynamic element and show the canvas
    pub fn draw(&self, window: &mut Window) {
        window.clear();

        let canvas = &mut window.canvas;

        self.left_tank.draw(canvas);
        self.right_tank.draw(canvas);
        self.limit.draw(canvas);
        self.decor.draw(canvas);
        self.score.draw(canvas);

        canvas.present();
    }
}