use sdl2::pixels::Color;
use sdl2::rect::Rect;

use engine::geometry;
use engine::geometry::AsRect;
use engine::graphics::{RectSprite, Sprite, Window};

use crate::logic::{BLOCK_COL_COUNT, BLOCK_ROW_COUNT, BOARD_BOTTOM_LIMIT, BOARD_LEFT_LIMIT, BOARD_RIGHT_LIMIT, BOARD_TOP_LIMIT, BOARD_TOP_LIMIT_HEIGHT, Logic, Map, TANK_HEIGHT, TANK_WIDTH};
use sdl2::render::WindowCanvas;

pub const TANK_SPRITE_PATH: &str = "res/tank.bmp";
pub const LEFT_TANK_COLOR: Color = Color::RGB(255, 0, 0);
pub const RIGHT_TANK_COLOR: Color = Color::RGB(0, 0, 255);

pub const LIMIT_COLOR: Color = Color::WHITE;
pub const DECOR_COLOR: Color = Color::WHITE;

pub const BLOCK_SIZE : i32 = 18;

pub struct Decor {
    blocks: [[RectSprite; BLOCK_COL_COUNT]; BLOCK_ROW_COUNT],
    x_shift : i32,
    y_shift : i32,
}

impl Decor {
    pub fn new(x_shift: i32, y_shift: i32, canvas_width:u32) -> Decor {
        let ys = y_shift as f32 + (canvas_width as f32 * BOARD_TOP_LIMIT);
        let xs = x_shift as f32 + (canvas_width as f32 * BOARD_LEFT_LIMIT);

        let blocks = [[RectSprite::new(0, 0, DECOR_COLOR); BLOCK_COL_COUNT]; BLOCK_ROW_COUNT];
        Decor {
            blocks,
            x_shift : xs as i32,
            y_shift : ys as i32
        }
    }

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
                        block.rect.set_x(self.x_shift + (BLOCK_SIZE*i as i32));
                        block.rect.set_y(self.y_shift + (BLOCK_SIZE*j as i32));
                    }
                }
            }
        }
    }

    fn draw(&self, canvas : &mut WindowCanvas){
        for j in 0..BLOCK_ROW_COUNT {
            for i in 0..BLOCK_COL_COUNT {
                let block = &self.blocks[j][i];
                block.draw(canvas);
            }
        }
    }
}

pub struct Graphics<'a> {
    left_tank: Sprite<'a>,
    left_shell: RectSprite,
    right_tank: Sprite<'a>,
    right_shell: RectSprite,
    left_limit: RectSprite,
    right_limit: RectSprite,
    top_limit: RectSprite,
    bottom_limit: RectSprite,
    decor: Decor,
}

impl Graphics<'_> {
    /// Init the dynamic elements required to draw the game
    pub fn new<'a>(canvas_width: u32, canvas_height: u32) -> Graphics<'a> {
        let y_shift = (canvas_height - canvas_width) as i32;
        Graphics {
            left_tank: Sprite::new(
                0,
                y_shift,
                TANK_SPRITE_PATH,
                Rect::new(0, 0, (TANK_WIDTH * canvas_width as f32) as u32, (TANK_HEIGHT * canvas_width as f32) as u32),
                LEFT_TANK_COLOR),
            left_shell: RectSprite::new(
                0,
                y_shift,
                LEFT_TANK_COLOR,
            ),
            right_tank: Sprite::new(
                0,
                y_shift,
                TANK_SPRITE_PATH,
                Rect::new(0, 0, (TANK_WIDTH * canvas_width as f32) as u32, (TANK_HEIGHT * canvas_width as f32) as u32),
                RIGHT_TANK_COLOR),
            right_shell: RectSprite::new(
                0,
                y_shift,
                RIGHT_TANK_COLOR,
            ),
            left_limit: RectSprite::new(
                0,
                y_shift,
                LIMIT_COLOR,
            ),
            right_limit: RectSprite::new(
                0,
                y_shift,
                LIMIT_COLOR,
            ),
            top_limit: RectSprite::new(
                0,
                y_shift,
                LIMIT_COLOR,
            ),
            bottom_limit: RectSprite::new(
                0,
                y_shift,
                LIMIT_COLOR,
            ),
            decor: Decor::new(0,y_shift,canvas_width),
        }
    }

    /// Update the dynamic elements accordingly to the state of the game.
    pub fn update(&mut self, logic: &Logic, window: &Window) {
        let w = window.width();

        let mut left_tank_angle = logic.left_tank.get_orientation().to_degrees() as f64;
        if logic.left_tank.is_impacted() {
            left_tank_angle = self.left_tank.angle + 45.;
        }
        self.left_tank.update(logic.left_tank.as_rect(), left_tank_angle, w, w);

        let mut right_tank_angle = logic.right_tank.get_orientation().to_degrees() as f64;
        if logic.right_tank.is_impacted() {
            right_tank_angle = self.right_tank.angle + 45.;
        }
        self.right_tank.update(logic.right_tank.as_rect(), right_tank_angle, w, w);
        let left_shell = logic.left_tank.get_shell();
        let right_shell = logic.right_tank.get_shell();


        if left_shell.is_destroyed() {
            self.left_shell.hide();
        } else {
            self.left_shell.show();
            self.left_shell.update(left_shell.as_rect(), w, w);
        }
        if right_shell.is_destroyed() {
            self.right_shell.hide();
        } else {
            self.right_shell.show();
            self.right_shell.update(right_shell.as_rect(), w, w);
        }

        self.left_limit.update(geometry::Rect::from_2_points(0., BOARD_TOP_LIMIT - BOARD_TOP_LIMIT_HEIGHT, BOARD_LEFT_LIMIT, 1.01), w, w);
        self.top_limit.update(geometry::Rect::from_2_points(0., 0., 1.01, BOARD_TOP_LIMIT), w, w);

        self.right_limit.update(geometry::Rect::from_2_points(BOARD_RIGHT_LIMIT, BOARD_TOP_LIMIT - BOARD_TOP_LIMIT_HEIGHT, 1.01, 1.01), w, w);
        self.bottom_limit.update(geometry::Rect::from_2_points(0., BOARD_BOTTOM_LIMIT, 1.01, 1.01), w, w);

        self.decor.update(&logic.map);
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
        self.left_shell.draw(canvas);
        self.right_shell.draw(canvas);
        self.left_limit.draw(canvas);
        self.top_limit.draw(canvas);
        self.right_limit.draw(canvas);
        self.bottom_limit.draw(canvas);
        self.decor.draw(canvas);

        canvas.present();
    }
}