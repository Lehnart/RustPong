use engine::physics::{Solid, Position, Velocity};
use engine::geometry::{Rect, AsRect};

pub const BOARD_LEFT_LIMIT : f32 = 0.02;
pub const BOARD_RIGHT_LIMIT : f32 = 0.98;
pub const BOARD_TOP_LIMIT : f32 = 0.10;
pub const BOARD_BOTTOM_LIMIT : f32 = 0.98;

pub const TANK_WIDTH : f32 =  0.05;
pub const TANK_HEIGHT : f32 =  0.05;
pub const TANK_VELOCITY : f32 = 0.10;
pub const TANK_ROTATION_ANGLE : f32 = std::f32::consts::PI / 8. ;

pub const LEFT_TANK_X0 : f32 =  0.20;
pub const LEFT_TANK_Y0 : f32 =  0.50;
pub const RIGHT_TANK_X0 : f32 =  0.80;
pub const RIGHT_TANK_Y0 : f32 =  0.50;

pub struct Tank {
    solid: Solid,
    orientation: f32
}

impl Tank {
    pub fn new(x0 : f32, y0 :f32) -> Tank {
        let pos = Position::new(x0, y0);
        let vel = Velocity::new(0., 0.);
        let w = TANK_WIDTH;
        let h = TANK_HEIGHT;
        let limit = Rect::from_2_points(BOARD_LEFT_LIMIT, BOARD_TOP_LIMIT, BOARD_RIGHT_LIMIT, BOARD_BOTTOM_LIMIT);

        let solid = Solid::new(pos, vel, w, h, limit);

        Tank {
            solid,
            orientation : 0.
        }
    }

    pub fn accelerate(&mut self) {
        self.solid.vel.set_vx(TANK_VELOCITY*self.orientation.cos());
        self.solid.vel.set_vy(TANK_VELOCITY*self.orientation.sin());
    }

    pub fn decelerate(&mut self) {
        self.solid.vel.set_vx(0.);
        self.solid.vel.set_vy(0.);
    }

    pub fn turn_left(&mut self) {
        self.orientation -= TANK_ROTATION_ANGLE;
        let v = self.solid.vel.mag();
        self.solid.vel.set_vx(v*self.orientation.cos());
        self.solid.vel.set_vy(v*self.orientation.sin());
    }

    pub fn turn_right(&mut self) {
        self.orientation += TANK_ROTATION_ANGLE;
        let v = self.solid.vel.mag();
        self.solid.vel.set_vx(v*self.orientation.cos());
        self.solid.vel.set_vy(v*self.orientation.sin());
    }

    fn update(&mut self, dt: f32) {
        self.solid.update(dt);
    }
}

impl AsRect for Tank {
    fn as_rect(&self) -> Rect {
        self.solid.as_rect()
    }
}


/// Logic is a structure that contains all entities from the game.
///
pub struct Logic {
    pub left_tank : Tank,
    is_over: bool,
}

impl Logic {
    /// Create a new game logic with default values for game settings
    pub fn new() -> Logic {
        Logic {
            left_tank : Tank::new(LEFT_TANK_X0,LEFT_TANK_Y0),
            is_over: false,
        }
    }

    /// Update each entity of a delta of time and check if the game is over.
    pub fn update(&mut self, dt: f32) {
        self.left_tank.update(dt);
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