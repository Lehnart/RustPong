use engine::physics::{Solid, Position, Velocity};
use engine::geometry::{Rect, AsRect};

pub const RACKET_WIDTH :f32 = 0.08;
pub const RACKET_HEIGHT :f32 = 0.02;
pub const RACKET_Y0 : f32 = 0.9;
pub const RACKET_SPEED : f32 = 0.75;

pub const BOARD_LEFT_LIMIT_X : f32 = 0.015;
pub const BOARD_RIGHT_LIMIT_X : f32 = 0.985;

pub struct Racket {
    solid: Solid
}

impl Racket {
    pub fn new() -> Racket {
        let pos = Position::new(0.5-(RACKET_WIDTH/2.), RACKET_Y0);
        let vel = Velocity::new(0.,0.);
        let w = RACKET_WIDTH;
        let h = RACKET_HEIGHT;
        let limit = Rect::from_2_points(BOARD_LEFT_LIMIT_X,0.,BOARD_RIGHT_LIMIT_X,1.);

        let solid = Solid::new(pos,vel,w,h,limit);

        Racket {
            solid
        }
    }

    /// To make the racket start moving.
    pub fn accelerate(&mut self) {
        let pv = self.solid.vel.copy();
        self.solid.vel.set_vx(pv.vx()+RACKET_SPEED);
    }

    /// To make the racket stop moving.
    pub fn decelerate(&mut self) {
        let pv = self.solid.vel.copy();
        self.solid.vel.set_vx(pv.vx()-RACKET_SPEED);
    }

    /// Racket update is just the solid physics updating.
    fn update(&mut self, dt: f32) {
        self.solid.update(dt);
    }

}

/// Converting the racket to a Rect make it more easy for collision and drawing.
impl AsRect for Racket {
    fn as_rect(&self) -> Rect {
        self.solid.as_rect()
    }
}

/// Logic is a structure that contains all entities from the game.
///
///
pub struct Logic {
    pub racket: Racket,

    is_over: bool,
}

impl Logic {
    /// Create a new game logic with default values for game settings
    pub fn new() -> Logic {
        Logic {
            racket: Racket::new(),
            is_over: false
        }
    }

    /// Update each entity of a delta of time and check if the game is over.
    pub fn update(&mut self, dt: f32) {
        self.racket.update(dt);
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