use sdl2::pixels::Color;
use sdl2::rect::{Point};
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;

use engine::geometry::AsRect;
use engine::graphics::{RectSprite, Sprite, Window};

use crate::logic;
use crate::logic::Logic;
use engine::random::rand;
use engine::geometry;

pub const SPACESHIP_SPRITE_PATH: &str = "res/spaceship.bmp";
pub const SPACESHIP_ACCELERATING_SPRITE_PATH: &str = "res/accelerating_spaceship.bmp";
pub const ASTEROID_SPRITE_PATHS: [&str;4] = ["res/asteroid_0.bmp","res/asteroid_1.bmp","res/asteroid_2.bmp","res/asteroid_3.bmp"];

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
    sprite: Sprite<'a>,
    logic_id : u32
}

impl  Asteroid<'_> {
    pub  fn new<'a> (logic_id : u32)-> Asteroid<'a> {
        let sprite_index = rand(0, ASTEROID_SPRITE_PATHS.len() as i32);
        Asteroid {
            sprite: Sprite::from_bmp(ASTEROID_SPRITE_PATHS[sprite_index as usize]),
            logic_id
        }
    }
}

pub struct Asteroids<'a>{
    asteroid_vec : Vec<Asteroid<'a>>
}

impl Asteroids<'_>{
    pub  fn new<'a> ()-> Asteroids<'a> {
         Asteroids {
            asteroid_vec: Vec::new()
        }
    }

    pub fn is_existing(&self, logic_id : u32) -> bool {
        for asteroid in &self.asteroid_vec{
            if logic_id == asteroid.logic_id{
                return true
            }
        }
        return false
    }

    pub fn create(&mut self, logic_id : u32) {
        let asteroid = Asteroid::new(logic_id);
        self.asteroid_vec.push(asteroid);
    }

    pub fn update(&mut self, logic_id : u32, logic_rect : geometry::Rect, w : u32, h : u32){
        for asteroid in &mut self.asteroid_vec{
            if logic_id == asteroid.logic_id{
                asteroid.sprite.update(logic_rect, 0., w, h);
                return
            }
        }
    }

    pub fn draw(&self, canvas : &mut WindowCanvas){
        for asteroid in &self.asteroid_vec {
            asteroid.sprite.draw(canvas);
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
    asteroids: Asteroids<'a>,
}

impl Graphics<'_> {
    /// Init the dynamic elements required to draw the game
    pub fn new(_canvas_width: u32, _canvas_height: u32, _ttf_context: &Sdl2TtfContext) -> Graphics {
        Graphics {
            spaceship: Spaceship::new(),
            asteroids: Asteroids::new(),
        }
    }

    /// Update the dynamic elements accordingly to the state of the game.
    pub fn update(&mut self, logic: &Logic, window: &Window, _ttf_context: &Sdl2TtfContext) {
        let w = window.width();
        let h = window.height();
        self.spaceship.update(&logic.spaceship, w, h);

        for logic_asteroid in logic.asteroids() {
            let id = logic_asteroid.get_id();

            if !self.asteroids.is_existing(id){
                self.asteroids.create(id);
            }
            self.asteroids.update(logic_asteroid.get_id(), logic_asteroid.as_rect(), w, h);
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
        self.asteroids.draw(canvas);

        canvas.present();
    }
}