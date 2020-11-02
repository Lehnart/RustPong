use engine::graphics::{Window, Sprite, RectSprite};
use sdl2::pixels::Color;
use crate::logic::{Logic, BOARD_LEFT_LIMIT, BOARD_TOP_LIMIT, BOARD_RIGHT_LIMIT, BOARD_BOTTOM_LIMIT, TANK_WIDTH, TANK_HEIGHT, BOARD_TOP_LIMIT_HEIGHT};
use engine::geometry::AsRect;
use sdl2::rect::{Rect};
use engine::geometry;

pub const TANK_SPRITE_PATH : &str = "res/tank.bmp";
pub const LEFT_TANK_COLOR : Color = Color::RGB(255,0,0);
pub const RIGHT_TANK_COLOR : Color = Color::RGB(0,0,255);

pub const LIMIT_COLOR: Color = Color::WHITE;

pub struct Graphics<'a> {
    left_tank: Sprite<'a>,
    right_tank: Sprite<'a>,
    left_limit: RectSprite,
    right_limit: RectSprite,
    top_limit: RectSprite,
    bottom_limit : RectSprite,
}

impl Graphics<'_> {
    /// Init the dynamic elements required to draw the game
    pub  fn new<'a>(canvas_width: u32, canvas_height: u32) -> Graphics<'a> {
        let y_shift = (canvas_height-canvas_width) as i32;
        Graphics {
            left_tank: Sprite::new(
                0,
                y_shift,
                TANK_SPRITE_PATH,
                Rect::new(0,0,(TANK_WIDTH*canvas_width as f32) as u32,(TANK_HEIGHT*canvas_width as f32) as u32),
                LEFT_TANK_COLOR),
            right_tank: Sprite::new(
                0,
                y_shift,
                TANK_SPRITE_PATH,
                Rect::new(0,0,(TANK_WIDTH*canvas_width as f32) as u32,(TANK_HEIGHT*canvas_width as f32) as u32),
                RIGHT_TANK_COLOR),
            left_limit: RectSprite::new(
                0,
                y_shift,
                LIMIT_COLOR
            ),
            right_limit: RectSprite::new(
                0,
                y_shift,
                LIMIT_COLOR
            ),
            top_limit: RectSprite::new(
                0,
                y_shift,
                LIMIT_COLOR
            ),
            bottom_limit : RectSprite::new(
                0,
                y_shift,
                LIMIT_COLOR
            ),
        }
    }

    /// Update the dynamic elements accordingly to the state of the game.
    pub fn update(&mut self, logic: &Logic, window: &Window) {
        let w = window.width();

        self.left_tank.update(logic.left_tank.as_rect(), logic.left_tank.get_orientation().to_degrees() as f64, w, w);
        self.right_tank.update(logic.right_tank.as_rect(), logic.right_tank.get_orientation().to_degrees() as f64, w, w);

        self.left_limit.update(geometry::Rect::from_2_points(0.,BOARD_TOP_LIMIT-BOARD_TOP_LIMIT_HEIGHT,BOARD_LEFT_LIMIT,1.01),w,w);
        self.top_limit.update(geometry::Rect::from_2_points(0.,BOARD_TOP_LIMIT-BOARD_TOP_LIMIT_HEIGHT,1.01,BOARD_TOP_LIMIT),w,w);
        self.right_limit.update(geometry::Rect::from_2_points(BOARD_RIGHT_LIMIT,BOARD_TOP_LIMIT-BOARD_TOP_LIMIT_HEIGHT,1.01,1.01),w,w);
        self.bottom_limit.update(geometry::Rect::from_2_points(0.,BOARD_BOTTOM_LIMIT,1.01,1.01),w,w);
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

        self.left_limit.draw(canvas);
        self.top_limit.draw(canvas);
        self.right_limit.draw(canvas);
        self.bottom_limit.draw(canvas);

        canvas.present();


    }
}