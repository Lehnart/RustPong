use sdl2::surface::Surface;

use engine::geometry::{AsRect, Rect};
use engine::physics::{CircleSolid, Position, Velocity};

pub const SPACESHIP_RADIUS: f32 = 0.04;
pub const SPACESHIP_STARTING_POSITION_X0: f32 = 0.5;
pub const SPACESHIP_STARTING_POSITION_Y0: f32 = 0.5;

pub struct Spaceship {
    solid: CircleSolid,
    orientation: f32,
}

impl Spaceship {
    pub fn new() -> Spaceship {
        let position = Position::new(SPACESHIP_STARTING_POSITION_X0, SPACESHIP_STARTING_POSITION_Y0);
        let velocity = Velocity::new(0., 0.);
        let limit = Rect::new(-1., -1., 3., 3.);

        Spaceship {
            solid: CircleSolid::new(position, velocity, SPACESHIP_RADIUS, limit),
            orientation: 0.,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.solid.update(dt);
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