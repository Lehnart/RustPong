use engine::graphics::{Window, Sprite};
use sdl2::pixels::Color;
use crate::logic::Logic;
use engine::geometry::AsRect;
use sdl2::rect::{Rect};

pub const TANK_SPRITE_PATH : &str = "res/tank.bmp";
pub const TANK_SPRITE_WIDTH : u32 = 25;
pub const TANK_SPRITE_HEIGHT : u32 = 25;
pub const LEFT_TANK_COLOR : Color = Color::RGB(255,0,0);
pub const RIGHT_TANK_COLOR : Color = Color::RGB(0,0,255);


pub struct Graphics<'a> {
    left_tank: Sprite<'a>,
    right_tank: Sprite<'a>,
}

impl Graphics<'_> {
    /// Init the dynamic elements required to draw the game
    pub  fn new<'a>() -> Graphics<'a> {
        Graphics {
            left_tank: Sprite::new(
                TANK_SPRITE_PATH,
                Rect::new(0,0,TANK_SPRITE_WIDTH,TANK_SPRITE_HEIGHT),
                LEFT_TANK_COLOR),
            right_tank: Sprite::new(
                TANK_SPRITE_PATH,
                Rect::new(0,0,TANK_SPRITE_WIDTH,TANK_SPRITE_HEIGHT),
                RIGHT_TANK_COLOR),
        }
    }

    /// Update the dynamic elements accordingly to the state of the game.
    pub fn update(&mut self, logic: &Logic, window: &Window) {
        let w = window.width();
        let h = window.height();

        self.left_tank.update(logic.left_tank.as_rect(), logic.left_tank.get_orientation().to_degrees() as f64, w, h);
        self.right_tank.update(logic.right_tank.as_rect(), logic.right_tank.get_orientation().to_degrees() as f64, w, h);
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
        canvas.present();


    }
}