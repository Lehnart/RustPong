use sdl2::pixels::Color;
use sdl2::rect::{Point};
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;

use engine::geometry::AsRect;
use engine::graphics::{RectSprite, Sprite, Window};

use crate::logic;
use crate::logic::Logic;

pub const SPACESHIP_SPRITE_PATH: &str = "res/spaceship.bmp";
pub const SPACESHIP_ACCELERATING_SPRITE_PATH: &str = "res/accelerating_spaceship.bmp";
pub const ASTEROID_SPRITE_PATH: &str = "res/asteroid.bmp";

pub struct Bullet {
    sprite: RectSprite
}

impl Bullet {
    pub fn new() -> Bullet {
        Bullet {
            sprite: RectSprite::default(Color::WHITE)
        }
    }
}

pub struct Asteroid<'a> {
    sprite: Sprite<'a>
}

impl  Asteroid<'_> {
    pub  fn new<'a> ()-> Asteroid<'a> {
        Asteroid {
            sprite: Sprite::from_bmp(ASTEROID_SPRITE_PATH)
        }
    }
}

pub struct Spaceship<'a> {
    sprite: Sprite<'a>,
    accelerating_sprite: Sprite<'a>,
    speed_point: Point,
    accelerating: bool,
    bullets: Vec<Bullet>,
}


impl Spaceship<'_> {
    pub fn new() -> Spaceship<'static> {
        Spaceship {
            sprite: Sprite::from_bmp(SPACESHIP_SPRITE_PATH),
            accelerating_sprite: Sprite::from_bmp(SPACESHIP_ACCELERATING_SPRITE_PATH),
            speed_point: Point::new(0, 0),
            accelerating: false,
            bullets: Vec::new(),
        }
    }

    pub fn update(&mut self, logic_spaceship: &logic::Spaceship, w: u32, h: u32) {
        self.sprite.update(logic_spaceship.as_rect(), logic_spaceship.orientation.to_degrees() as f64, w, h);
        self.accelerating_sprite.update(logic_spaceship.as_rect(), logic_spaceship.orientation.to_degrees() as f64, w, h);
        self.accelerating = logic_spaceship.accelerating;

        let xc = self.sprite.dest_rect.center().x();
        let yc = self.sprite.dest_rect.center().y();
        let speed_line_length = logic_spaceship.solid.vel.mag() * (w as f32);
        let speed_line_angle = logic_spaceship.solid.vel.angle();
        self.speed_point = Point::new(
            xc + (speed_line_length as f32 * speed_line_angle.cos()) as i32,
            yc + (speed_line_length as f32 * speed_line_angle.sin()) as i32,
        );

        self.bullets.clear();
        for logic_bullet in logic_spaceship.bullets() {
            let mut bullet = Bullet::new();
            bullet.sprite.update(logic_bullet.as_rect(), w, h);
            self.bullets.push(bullet);
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        match self.accelerating {
            true => self.accelerating_sprite.draw(canvas),
            false => self.sprite.draw(canvas)
        }

        for bullet in &self.bullets {
            bullet.sprite.draw(canvas);
        }

        let start = self.sprite.dest_rect.center();
        canvas.set_draw_color(Color::BLUE);
        canvas.draw_line(start, self.speed_point).unwrap();
    }
}

pub struct Graphics<'a> {
    spaceship: Spaceship<'a>,
    asteroids: Vec<Asteroid<'a>>,
}

impl Graphics<'_> {
    /// Init the dynamic elements required to draw the game
    pub fn new(_canvas_width: u32, _canvas_height: u32, _ttf_context: &Sdl2TtfContext) -> Graphics {
        Graphics {
            spaceship: Spaceship::new(),
            asteroids: Vec::new(),
        }
    }

    /// Update the dynamic elements accordingly to the state of the game.
    pub fn update(&mut self, logic: &Logic, window: &Window, _ttf_context: &Sdl2TtfContext) {
        let w = window.width();
        let h = window.height();
        self.spaceship.update(&logic.spaceship, w, h);

        self.asteroids.clear();
        for logic_asteroid in logic.asteroids() {
            let mut asteroid = Asteroid::new();
            asteroid.sprite.update(logic_asteroid.as_rect(), 0.,w, h);
            self.asteroids.push(asteroid);
        }
    }

    /// Draw the game.
    ///
    /// Start by clearing the all board.
    /// It draws each dynamic element and show the canvas
    pub fn draw(&self, window: &mut Window) {
        window.clear();

        let canvas = &mut window.canvas;
        self.spaceship.draw(canvas);

        for asteroid in &self.asteroids {
            asteroid.sprite.draw(canvas);
        }

        canvas.present();
    }
}