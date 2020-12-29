use sdl2::surface::Surface;

use engine::geometry::{AsRect, Rect, Circle};
use engine::physics::{CircleSolid, Position, Velocity, RectSolid};

pub const SPACESHIP_RADIUS: f32 = 0.04;
pub const SPACESHIP_STARTING_POSITION_X0: f32 = 0.5;
pub const SPACESHIP_STARTING_POSITION_Y0: f32 = 0.5;
pub const SPACESHIP_ACCELERATION: f32 = 0.2;
pub const SPACESHIP_ROTATION_SPEED: f32 = 5.;
pub const SPACESHIP_MAX_SPEED: f32 = 1.;
pub const SPACESHIP_FIRING_DELAY: f32 = 0.3;

pub const BULLET_SPEED : f32 = 0.5;
pub const BULLET_RADIUS : f32 = 0.005;

#[derive(Debug)]
pub enum Turning {
    NONE,
    LEFT,
    RIGHT,
}

pub struct Bullet{
    pub solid: CircleSolid
}

impl Bullet{
    pub fn new(x : f32, y: f32, orientation : f32) -> Bullet{
        let position = Position::new(x, y);
        let velocity = Velocity::new(BULLET_SPEED*orientation.cos(), BULLET_SPEED*orientation.sin());
        let limit = Rect::new(-1., -1., 3., 3.);
        let circle_solid = CircleSolid::new(position, velocity, BULLET_RADIUS, limit);
        Bullet{
            solid : circle_solid
        }
    }

    pub fn is_out_of_limit(&self) -> bool {
        let x = self.solid.pos.x();
        let y = self.solid.pos.y();
        if x < 0. || x > 1.{
            return false;
        }
        if y < 0. || y > 1.{
            return false;
        }

        return true;
    }

    pub fn update(&mut self, dt : f32){
        self.solid.update(dt);
    }

}

impl AsRect for Bullet {
    fn as_rect(&self) -> Rect {
        let x = self.solid.pos.x();
        let y = self.solid.pos.y();
        let r = self.solid.r;
        Rect::new(x - r, y - r, r, r)
    }
}

pub struct Spaceship {
    pub solid: CircleSolid,
    pub orientation: f32,
    pub accelerating: bool,
    pub firing: bool,
    firing_delay: f32,
    turning: Turning,
    bullets : Vec<Bullet>,
}

impl Spaceship {
    pub fn new() -> Spaceship {
        let position = Position::new(SPACESHIP_STARTING_POSITION_X0, SPACESHIP_STARTING_POSITION_Y0);
        let velocity = Velocity::new(0., 0.);
        let limit = Rect::new(-1., -1., 3., 3.);

        Spaceship {
            solid: CircleSolid::new(position, velocity, SPACESHIP_RADIUS, limit),
            orientation: 0.,
            accelerating: false,
            firing: false,
            firing_delay: SPACESHIP_FIRING_DELAY,
            turning: Turning::NONE,
            bullets : Vec::new()
        }
    }

    pub fn accelerate(&mut self) {
        self.accelerating = true;
    }

    pub fn decelerate(&mut self) {
        self.accelerating = false;
    }

    pub fn turn(&mut self, turn: Turning) {
        self.turning = turn;
    }

    pub fn update(&mut self, dt: f32) {
        self.update_orientation(dt);
        self.update_speed(dt);
        self.update_position(dt);
        self.update_firing(dt);
        self.update_bullets(dt);
    }

    pub fn bullets(&self) -> &Vec<Bullet>{
        &self.bullets
    }

    fn update_orientation(&mut self, dt: f32) {
        match &self.turning {
            Turning::LEFT => self.orientation -= SPACESHIP_ROTATION_SPEED * dt,
            Turning::RIGHT => self.orientation += SPACESHIP_ROTATION_SPEED * dt,
            Turning::NONE => (),
        }
    }

    fn update_speed(&mut self, dt: f32) {
        if self.accelerating {
            let mut vx = self.solid.vel.vx();
            let mut vy = self.solid.vel.vy();

            vx += SPACESHIP_ACCELERATION * dt * self.orientation.cos();
            vy += SPACESHIP_ACCELERATION * dt * self.orientation.sin();

            if vx > SPACESHIP_MAX_SPEED {
                vx = SPACESHIP_MAX_SPEED
            }
            if vx < -SPACESHIP_MAX_SPEED {
                vx = -SPACESHIP_MAX_SPEED
            }
            if vy > SPACESHIP_MAX_SPEED {
                vy = SPACESHIP_MAX_SPEED
            }
            if vy < - SPACESHIP_MAX_SPEED {
                vy = -SPACESHIP_MAX_SPEED
            }

            self.solid.vel.set_vx(vx);
            self.solid.vel.set_vy(vy);
        }
    }

    fn update_position(&mut self, dt: f32) {
        self.handle_out_of_limit();
        self.solid.update(dt);
    }

    fn update_firing(&mut self, dt :f32){
        if self.firing && self.firing_delay > SPACESHIP_FIRING_DELAY {
            self.firing_delay = 0.;
            let rect = self.as_rect();
            let bullet = Bullet::new(rect.xc(), rect.yc(),self.orientation);
            self.bullets.push(bullet);
        }
        self.firing_delay += dt;
    }

    fn update_bullets(&mut self, dt :f32){
        self.bullets.retain(|bullet| { return bullet.is_out_of_limit(); });
        for mut bullet in &mut self.bullets { bullet.update(dt); }
    }

    fn handle_out_of_limit(&mut self) {
        let pos_x = self.solid.pos.x();
        let pos_y = self.solid.pos.y();
        if pos_x < 0. {
            self.solid.pos.set_x(1. - pos_x);
        }
        if pos_x > 1. {
            self.solid.pos.set_x(pos_x - 1.);
        }
        if pos_y < 0. {
            self.solid.pos.set_y(1. - pos_y);
        }
        if pos_y > 1. {
            self.solid.pos.set_y(pos_y - 1.);
        }
    }
}

impl AsRect for Spaceship {
    fn as_rect(&self) -> Rect {
        let x = self.solid.pos.x();
        let y = self.solid.pos.y();
        let r = self.solid.r;
        Rect::new(x - r, y - r, r, r)
    }
}

/// Logic is a structure that contains all entities from the game.
pub struct Logic {
    is_over: bool,
    pub spaceship: Spaceship,
}

impl Logic {
    /// Create a new game logic with default values for game settings
    pub fn new() -> Logic {
        Logic {
            is_over: false,
            spaceship: Spaceship::new(),
        }
    }

    /// Update each entity of a delta of time and check if the game is over.
    pub fn update(&mut self, dt: f32) {
        self.spaceship.update(dt);
    }

    /// Set the game over
    pub fn over(&mut self) {
        self.is_over = true;
    }

    /// Is the game over?
    pub fn is_over(&self) -> bool {
        self.is_over
    }
}