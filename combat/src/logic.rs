use sdl2::surface::Surface;

use engine::geometry::{AsRect, Rect};
use engine::physics::{Position, Solid, Velocity};

pub const BOARD_LEFT_LIMIT: f32 = 0.05;
pub const BOARD_RIGHT_LIMIT: f32 = 0.95;
pub const BOARD_TOP_LIMIT: f32 = 0.05;
pub const BOARD_TOP_LIMIT_HEIGHT: f32 = 0.05;
pub const BOARD_BOTTOM_LIMIT: f32 = 0.95;

pub const TANK_WIDTH: f32 = 0.04;
pub const TANK_HEIGHT: f32 = 0.04;
pub const TANK_VELOCITY: f32 = 0.10;
pub const TANK_ROTATION_ANGLE: f32 = std::f32::consts::PI / 8.;
pub const TANK_ROTATION_DELAY: f32 = 0.25;
pub const TANK_IMPACT_DELAY: f32 = 0.5;

pub const SHELL_WIDTH: f32 = 0.005;
pub const SHELL_HEIGHT: f32 = 0.005;
pub const SHELL_VELOCITY: f32 = 0.30;

pub const LEFT_TANK_X0: f32 = 0.06;
pub const LEFT_TANK_Y0: f32 = 0.50;
pub const RIGHT_TANK_X0: f32 = 0.90;
pub const RIGHT_TANK_Y0: f32 = 0.50;

pub const BLOCK_ROW_COUNT: usize = 30;
pub const BLOCK_COL_COUNT: usize = 30;
pub const LEVELS: [&str; 1] = ["res/level_1.bmp"];

pub struct Score{
    left_score : u32,
    right_score : u32,
}

impl Score{

    pub fn new()-> Score{
        Score{
            left_score:0,
            right_score:0
        }
    }

    pub fn point_left(&mut self){
        self.left_score +=1;
    }

    pub fn point_right(&mut self){
        self.right_score +=1;
    }

    pub fn get_left_score(&self) -> u32{
        return self.left_score;
    }
    pub fn get_right_score(&self)-> u32{
        return self.right_score;
    }

}

pub struct Map {
    blocks: [[bool; BLOCK_COL_COUNT]; BLOCK_ROW_COUNT]
}

impl Map {
    pub fn load(map_index: u32) -> Map {
        let mut blocks = [[false; BLOCK_COL_COUNT]; BLOCK_ROW_COUNT];
        let surface = Surface::load_bmp(LEVELS[map_index as usize]).unwrap();
        let pixels = surface.without_lock().unwrap();
        for j in 0..surface.height() {
            for i in 0..surface.width() {
                let index = j * surface.pitch() + i;
                let pixel = pixels[index as usize];
                let mut block: bool = false;
                if pixel == 0 {
                    block = true;
                }
                blocks[j as usize][i as usize] = block;
            }
        }
        Map { blocks }
    }

    fn width() -> f32 {
        (BOARD_RIGHT_LIMIT - BOARD_LEFT_LIMIT) as f32 / BLOCK_COL_COUNT as f32
    }
    fn height() -> f32 {
        (BOARD_BOTTOM_LIMIT - BOARD_TOP_LIMIT) as f32 / BLOCK_ROW_COUNT as f32
    }

    pub fn get_block(&self, i: u32, j: u32) -> Option<Rect> {
        let block = self.blocks[j as usize][i as usize];
        let w = Map::width();
        let h = Map::height();

        match block {
            false => None,
            true => Some(
                Rect::new(
                    BOARD_LEFT_LIMIT + (i as f32 * w),
                    BOARD_TOP_LIMIT + (j as f32 * h),
                    w,
                    h,
                )
            )
        }
    }
}

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
        let pos = Position::new(x0 - (SHELL_WIDTH as f32 / 2.), y0 - (SHELL_HEIGHT as f32 / 2.));
        let vel = Velocity::new(SHELL_VELOCITY * angle.cos(), SHELL_VELOCITY * angle.sin());
        self.solid.pos = pos;
        self.solid.vel = vel;
    }

    fn update(&mut self, dt: f32) {
        self.solid.update(dt);
        Map::load(0);
    }

    pub fn get_orientation(&self) -> f32 {
        self.solid.vel.angle()
    }
}

impl AsRect for Shell {
    fn as_rect(&self) -> Rect {
        self.solid.as_rect()
    }
}


pub struct Tank {
    solid: Solid,
    pub shell: Shell,
    orientation: f32,
    rotation_delay: f32,
    is_impacted: bool,
    impact_delay: f32,
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
            is_impacted: false,
            impact_delay: 0.,
        }
    }

    pub fn get_shell(&self) -> &Shell {
        &self.shell
    }

    pub fn get_orientation(&self) -> f32 {
        self.orientation
    }

    pub fn is_impacted(&self) -> bool { self.is_impacted }

    pub fn impact(&mut self, angle: f32) {
        if self.is_impacted { () }

        self.is_impacted = true;
        self.orientation = angle;
        self.accelerate();
    }

    pub fn accelerate(&mut self) {
        if self.is_impacted { () }

        self.solid.vel.set_vx(TANK_VELOCITY * self.orientation.cos());
        self.solid.vel.set_vy(TANK_VELOCITY * self.orientation.sin());
    }

    pub fn decelerate(&mut self) {
        if self.is_impacted { () }

        self.solid.vel.set_vx(0.);
        self.solid.vel.set_vy(0.);
    }

    fn turn(&mut self, dir: bool) {
        if dir { self.orientation += TANK_ROTATION_ANGLE; } else { self.orientation -= TANK_ROTATION_ANGLE; }
        let v = self.solid.vel.mag();
        self.solid.vel.set_vx(v * self.orientation.cos());
        self.solid.vel.set_vy(v * self.orientation.sin());
    }

    pub fn turn_left(&mut self) {
        if self.is_impacted { () }

        if self.rotation_delay < TANK_ROTATION_DELAY {
            return;
        }
        self.rotation_delay = 0.;
        self.turn(false)
    }

    pub fn turn_right(&mut self) {
        if self.is_impacted { () }

        if self.rotation_delay < TANK_ROTATION_DELAY {
            return;
        }
        self.rotation_delay = 0.;

        self.turn(true)
    }

    pub fn fire(&mut self) {
        if self.is_impacted { () }

        if self.shell.is_destroyed {
            let rect = self.solid.as_rect();
            self.shell.launch(rect.xc(), rect.yc(), self.orientation);
        }
    }

    fn update(&mut self, dt: f32) {
        self.rotation_delay += dt;

        if self.is_impacted {
            self.impact_delay += dt;

            if self.impact_delay > TANK_IMPACT_DELAY {
                self.impact_delay = 0.;
                self.is_impacted = false;
                self.solid.vel.set_vx(0.);
                self.solid.vel.set_vy(0.);
            }
        }

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

    pub fn move_back(&mut self, dt: f32) {
        self.solid.update(-dt);
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
    pub score : Score,
    pub left_tank: Tank,
    pub right_tank: Tank,
    pub map: Map,
    is_over: bool,
}

impl Logic {
    /// Create a new game logic with default values for game settings
    pub fn new() -> Logic {
        Logic {
            score : Score::new(),
            left_tank: Tank::new(LEFT_TANK_X0, LEFT_TANK_Y0, 0.),
            right_tank: Tank::new(RIGHT_TANK_X0, RIGHT_TANK_Y0, std::f32::consts::PI),
            map: Map::load(0),
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