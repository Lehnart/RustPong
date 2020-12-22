use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;

use engine::geometry;
use engine::geometry::AsRect;
use engine::graphics::{RectSprite, RenderedString, Sprite, Window};

use crate::logic::Logic;
use crate::logic;

pub struct Spaceship {
    sprite: RectSprite,
    orientation_point: Point,
    speed_point: Point,
}

impl Spaceship {
    pub fn new() -> Spaceship {
        Spaceship {
            sprite: RectSprite::default(Color::WHITE),
            orientation_point: Point::new(0, 0),
            speed_point: Point::new(0, 0),
        }
    }

    pub fn update(&mut self, logic_spaceship: &logic::Spaceship, w: u32, h: u32) {

        self.sprite.update(logic_spaceship.as_rect(), w, h);

        let xc = self.sprite.rect.center().x();
        let yc = self.sprite.rect.center().y();
        self.orientation_point = Point::new(
            xc + (self.sprite.rect.width() as f32 *logic_spaceship.orientation.cos()) as i32 ,
            yc + (self.sprite.rect.width() as f32 *logic_spaceship.orientation.sin())  as i32
        );

        let speed_line_length = logic_spaceship.solid.vel.mag()*(w as f32);
        let speed_line_angle = logic_spaceship.solid.vel.angle();
        self.speed_point = Point::new(
            xc + (speed_line_length as f32 *speed_line_angle.cos()) as i32 ,
            yc + (speed_line_length as f32 *speed_line_angle.sin())  as i32
        )
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {

        self.sprite.draw(canvas);
        let start =self.sprite.rect.center();

        canvas.set_draw_color(Color::RED);
        canvas.draw_line(start,self.orientation_point);

        canvas.set_draw_color(Color::BLUE);
        canvas.draw_line(start,self.speed_point);
    }
}

pub struct Graphics<> {
    spaceship: Spaceship
}

impl Graphics {
    /// Init the dynamic elements required to draw the game
    pub fn new(canvas_width: u32, canvas_height: u32, ttf_context: &Sdl2TtfContext) -> Graphics {
        Graphics {
            spaceship: Spaceship::new()
        }
    }

    /// Update the dynamic elements accordingly to the state of the game.
    pub fn update(&mut self, logic: &Logic, window: &Window, ttf_context: &Sdl2TtfContext) {
        self.spaceship.update(&logic.spaceship, window.width(), window.height());
    }

    /// Draw the game.
    ///
    /// Start by clearing the all board.
    /// It draws each dynamic element and show the canvas
    pub fn draw(&self, window: &mut Window) {
        window.clear();

        let canvas = &mut window.canvas;
        self.spaceship.draw(canvas);

        canvas.present();
    }
}