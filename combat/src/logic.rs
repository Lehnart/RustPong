use engine::geometry::{AsRect, Rect};
use engine::physics::{Position, Solid, Velocity};

pub const BOARD_LEFT_LIMIT: f32 = 0.02;
pub const BOARD_RIGHT_LIMIT: f32 = 0.98;
pub const BOARD_TOP_LIMIT: f32 = 0.02;
pub const BOARD_TOP_LIMIT_HEIGHT: f32 = 0.02;
pub const BOARD_BOTTOM_LIMIT: f32 = 0.98;

pub const TANK_WIDTH: f32 = 0.04;
pub const TANK_HEIGHT: f32 = 0.04;
pub const TANK_VELOCITY: f32 = 0.10;
pub const TANK_ROTATION_ANGLE: f32 = std::f32::consts::PI / 8.;
pub const TANK_ROTATION_DELAY: f32 = 0.25;

pub const SHELL_WIDTH: f32 = 0.005;
pub const SHELL_HEIGHT: f32 = 0.005;
pub const SHELL_VELOCITY: f32 = 0.30;

pub const LEFT_TANK_X0: f32 = 0.06;
pub const LEFT_TANK_Y0: f32 = 0.50;
pub const RIGHT_TANK_X0: f32 = 0.90;
pub const RIGHT_TANK_Y0: f32 = 0.50;

pub struct Shell {
    solid: Solid,
    is_destroyed: bool,
}

impl Shell {
    pub fn new() -> Shell {
        let pos = Position::new(0., 0.);
        let vel = Velocity::new(0., 0.);
        let w = SHELL_WIDTH;
        let h = SHELL_HEIGHT;
        let limit = Rect::from_2_points(-1., -1.0, 2.0, 2.0);

        let solid = Solid::new(pos, vel, w, h, limit);
        Shell {
            solid,
            is_destroyed: true,
        }
    }

    pub fn is_destroyed(&self) -> bool {
        self.is_destroyed
    }

    pub fn destroy(&mut self) {
        self.is_destroyed = true;
    }

    fn launch(&mut self, x0: f32, y0: f32, angle: f32) {
        self.is_destroyed = false;
        let pos = Position::new(x0, y0);
        let vel = Velocity::new(SHELL_VELOCITY * angle.cos(), SHELL_VELOCITY * angle.sin());
        self.solid.pos = pos;
        self.solid.vel = vel;
    }

    fn update(&mut self, dt: f32) {
        self.solid.update(dt);
    }
}

impl AsRect for Shell {
    fn as_rect(&self) -> Rect {
        self.solid.as_rect()
    }
}


pub struct Tank {
    solid: Solid,
    shell: Shell,
    orientation: f32,
    rotation_delay: f32,
}

impl Tank {
    pub fn new(x0: f32, y0: f32, orientation: f32) -> Tank {
        let pos = Position::new(x0, y0);
        let vel = Velocity::new(0., 0.);
        let w = TANK_WIDTH;
        let h = TANK_HEIGHT;
        let limit = Rect::from_2_points(BOARD_LEFT_LIMIT, BOARD_TOP_LIMIT, BOARD_RIGHT_LIMIT, BOARD_BOTTOM_LIMIT);

        let solid = Solid::new(pos, vel, w, h, limit);

        Tank {
            solid,
            shell: Shell::new(),
            orientation,
            rotation_delay: 0.,
        }
    }

    pub fn get_shell(&self) -> &Shell {
        &self.shell
    }

    pub fn get_orientation(&self) -> f32 {
        self.orientation
    }

    pub fn accelerate(&mut self) {
        self.solid.vel.set_vx(TANK_VELOCITY * self.orientation.cos());
        self.solid.vel.set_vy(TANK_VELOCITY * self.orientation.sin());
    }

    pub fn decelerate(&mut self) {
        self.solid.vel.set_vx(0.);
        self.solid.vel.set_vy(0.);
    }

    pub fn turn_left(&mut self) {
        if self.rotation_delay < TANK_ROTATION_DELAY {
            return;
        }
        self.rotation_delay = 0.;

        self.orientation -= TANK_ROTATION_ANGLE;
        let v = self.solid.vel.mag();
        self.solid.vel.set_vx(v * self.orientation.cos());
        self.solid.vel.set_vy(v * self.orientation.sin());
    }

    pub fn turn_right(&mut self) {
        if self.rotation_delay < TANK_ROTATION_DELAY {
            return;
        }
        self.rotation_delay = 0.;

        self.orientation += TANK_ROTATION_ANGLE;
        let v = self.solid.vel.mag();
        self.solid.vel.set_vx(v * self.orientation.cos());
        self.solid.vel.set_vy(v * self.orientation.sin());
    }

    pub fn fire(&mut self) {
        if self.shell.is_destroyed {
            let rect = self.solid.as_rect();
            self.shell.launch(rect.xc(), rect.yc(), self.orientation);
        }
    }

    fn update(&mut self, dt: f32) {
        self.rotation_delay += dt;
        self.solid.update(dt);

        self.shell.update(dt);

        // TODO : a déplacer dans la gestion des collisions
        let shell_rect = self.shell.as_rect();
        if shell_rect.xc() > BOARD_RIGHT_LIMIT || shell_rect.xc() < BOARD_LEFT_LIMIT {
            self.shell.destroy();
        }
        if shell_rect.yc() > BOARD_BOTTOM_LIMIT || shell_rect.yc() < BOARD_TOP_LIMIT {
            self.shell.destroy();
        }
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
    pub left_tank: Tank,
    pub right_tank: Tank,
    is_over: bool,
}

impl Logic {
    /// Create a new game logic with default values for game settings
    pub fn new() -> Logic {
        Logic {
            left_tank: Tank::new(LEFT_TANK_X0, LEFT_TANK_Y0, 0.),
            right_tank: Tank::new(RIGHT_TANK_X0, RIGHT_TANK_Y0, std::f32::consts::PI),
            is_over: false,
        }
    }

    /// Update each entity of a delta of time and check if the game is over.
    pub fn update(&mut self, dt: f32) {
        self.left_tank.update(dt);
        self.right_tank.update(dt);
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