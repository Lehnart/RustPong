use engine::geometry::{AsRect, Rect};
use engine::physics::{Position, RectSolid, Velocity};
use engine::random::rand;

pub const RACKET_WIDTH: f32 = 0.08;
pub const RACKET_HEIGHT: f32 = 0.02;
pub const RACKET_Y0: f32 = 0.9;
pub const RACKET_SPEED: f32 = 0.75;

pub const BOARD_LEFT_LIMIT_X: f32 = 0.015;
pub const BOARD_RIGHT_LIMIT_X: f32 = 0.985;
pub const BOARD_TOP_LIMIT_Y: f32 = 0.015;

pub const BLOCK_WIDTH: f32 = 0.06;
pub const BLOCK_HEIGHT: f32 = 0.015;
pub const BLOCK_STEP_X: f32 = 0.01;
pub const BLOCK_STEP_Y: f32 = 0.01;
pub const BLOCK_ROW_N: u8 = 8;
pub const BLOCK_COL_N: u8 = 14;

pub const BLOCKS_X0: f32 = 0.015;
pub const BLOCKS_Y0: f32 = 0.2;

pub const BALL_X0: f32 = 0.5;
pub const BALL_Y0: f32 = 0.5;
pub const BALL_SPEED: f32 = 0.5;
pub const BALL_DIM: f32 = 0.01;

pub const BOUNCE_ANGLE_MAX: f32 = 270. + 60.;
pub const BOUNCE_ANGLE_MIN: f32 = 270. - 60.;

pub const LIFE_STARTING_COUNT: u32 = 3;

/// The Racket represents the player.
///
/// A racket is a rectangle that can be moved from left to right, trying to reach the ball
/// in order to not let it pass.
pub struct Racket {
    solid: RectSolid
}

impl Racket {
    /// Create a new racket in the center of the board.
    pub fn new() -> Racket {
        let pos = Position::new(0.5 - (RACKET_WIDTH / 2.), RACKET_Y0);
        let vel = Velocity::new(0., 0.);
        let w = RACKET_WIDTH;
        let h = RACKET_HEIGHT;
        let limit = Rect::from_2_points(BOARD_LEFT_LIMIT_X, 0., BOARD_RIGHT_LIMIT_X, 1.);

        let solid = RectSolid::new(pos, vel, w, h, limit);

        Racket {
            solid
        }
    }

    /// To make the racket start moving.
    pub fn accelerate(&mut self) {
        let pv = self.solid.vel.copy();
        self.solid.vel.set_vx(pv.vx() + RACKET_SPEED);
    }

    /// To make the racket stop moving.
    pub fn decelerate(&mut self) {
        let pv = self.solid.vel.copy();
        self.solid.vel.set_vx(pv.vx() - RACKET_SPEED);
    }

    /// Racket update is just the solid physics updating.
    fn update(&mut self, dt: f32) {
        self.solid.update(dt);
    }

    /// Compute the bounce angle of the ball on the racket
    pub fn get_bounce_angle(&self, x: f32, _y: f32) -> f32 {
        let rect = self.as_rect();
        let rel_x = (x - rect.x0()) / RACKET_WIDTH;
        let angle = ((rel_x * (BOUNCE_ANGLE_MAX - BOUNCE_ANGLE_MIN)) + BOUNCE_ANGLE_MIN).to_radians();
        angle
    }
}

/// Converting the racket to a Rect make it more easy for collision and drawing.
impl AsRect for Racket {
    fn as_rect(&self) -> Rect {
        self.solid.as_rect()
    }
}

/// Represent a block which can be destroyed on collision with ball.
///
/// The block can be destroyed when collided by the ball.
/// Each block has a value, which determines the score earned when it is destroyed.
pub struct Block {
    solid: RectSolid,
    value: u8,
    is_destroyed: bool,
}

impl Block {
    /// Create a new block add the given position with a given value.
    pub fn new(x: f32, y: f32, value: u8) -> Block {
        Block {
            solid: RectSolid::fixed(x, y, BLOCK_WIDTH, BLOCK_HEIGHT),
            value,
            is_destroyed: false,
        }
    }

    pub fn get_value(&self) -> u8 { self.value }
    pub fn destroy(&mut self) { self.is_destroyed = true }
    pub fn is_destroyed(&self) -> bool { self.is_destroyed }
}

impl AsRect for Block {
    fn as_rect(&self) -> Rect {
        self.solid.as_rect()
    }
}

/// Represents all the blocks in one struct to handle drawing and collision more easily
///
/// Blocks are represented as a vector of blocks
pub struct Blocks {
    pub block_vec: Vec<Block>
}

impl Blocks {
    /// Create all the blocks of the game
    pub fn new() -> Blocks {
        let mut blocks: Vec<Block> = Vec::new();
        for i in 0..BLOCK_ROW_N {
            for j in 0..BLOCK_COL_N {
                blocks.push(
                    Block::new(
                        j as f32 * (BLOCK_WIDTH + BLOCK_STEP_X) + BLOCKS_X0,
                        i as f32 * (BLOCK_HEIGHT + BLOCK_STEP_Y) + BLOCKS_Y0,
                        (BLOCK_ROW_N - i - 1) / 2 as u8,
                    )
                );
            }
        }
        Blocks { block_vec: blocks }
    }

    /// Get a block at a given index.
    pub fn get(&self, i: usize) -> &Block {
        &self.block_vec[i]
    }
}

/// The ball which move across the board, between rackets.
///
/// A ball is represented as a solid.
/// It is reflected on rackets.
///
pub struct Ball {
    solid: RectSolid
}

impl Ball {
    /// Create a new ball with a random direction
    fn new() -> Ball {
        let random_angle = (rand(90 - 45, 90 + 45) as f32).to_radians();
        let pos = Position::new(BALL_X0, BALL_Y0);
        let vel = Velocity::new(random_angle.cos() * BALL_SPEED, random_angle.sin() * BALL_SPEED);
        let limit = Rect::from_2_points(0., 0., 1., 2.);

        Ball {
            solid: RectSolid::new(pos, vel, BALL_DIM, BALL_DIM, limit)
        }
    }

    /// Ball update is just the solid physics updating.
    fn update(&mut self, dt: f32) {
        self.solid.update(dt);
    }

    /// Reflect ball on x direction
    pub fn reflect_x(&mut self, x_shift: f32) {
        let x = self.solid.pos.x();
        self.solid.pos.set_x(x + x_shift);
        let vx = self.solid.vel.vx();
        self.solid.vel.set_vx(-vx);
    }

    /// Reflect ball on y direction
    pub fn reflect_y(&mut self, y_shift: f32) {
        let y = self.solid.pos.y();
        self.solid.pos.set_y(y + y_shift);
        let vy = self.solid.vel.vy();
        self.solid.vel.set_vy(-vy);
    }

    /// Bounce at a given angle.
    pub fn bounce(&mut self, angle: f32, y_shift: f32) {

        // Shift the ball outside the collision
        let y = self.solid.pos.y();
        self.solid.pos.set_y(y + y_shift);

        // Set the new speed
        let vx = self.solid.vel.mag() * angle.cos();
        let vy = self.solid.vel.mag() * angle.sin();
        self.solid.vel.set_vx(vx);
        self.solid.vel.set_vy(vy);
    }

    /// Reset ball position at the center of the board
    pub fn reset(&mut self) {
        let random_angle = (rand(90 - 45, 90 + 45) as f32).to_radians();
        let pos = Position::new(BALL_X0, BALL_Y0);
        let vel = Velocity::new(random_angle.cos() * BALL_SPEED, random_angle.sin() * BALL_SPEED);
        self.solid.vel = vel;
        self.solid.pos = pos;
    }
}

/// Converting the ball to a Rect make it more easy for collision and drawing.
impl AsRect for Ball {
    fn as_rect(&self) -> Rect {
        self.solid.as_rect()
    }
}

/// The score of the current game.
///
/// Score depends on how many blocks have been destroyed and depending on their value.
pub struct Score {
    current: u32,
}

impl Score {
    /// Create a new score, starting at 0 zero.
    fn new() -> Score {
        Score {
            current: 0,
        }
    }

    /// Get player score.
    pub fn get(&self) -> u32 {
        self.current
    }

    /// Add points to the score.
    pub fn add(&mut self, points: u32) {
        self.current += points
    }
}

/// The current number of life.
///
/// When it reachs 0, the game is over.
pub struct Life {
    count: u32,
}

impl Life {
    /// Creating the life counter
    fn new() -> Life {
        Life {
            count: LIFE_STARTING_COUNT,
        }
    }

    pub fn get(&self) -> u32 {
        self.count
    }

    /// Remove one life from the current count
    pub fn remove(&mut self) {
        self.count -= 1;
    }
}

/// Logic is a structure that contains all entities from the game.
///
pub struct Logic {
    pub racket: Racket,
    pub blocks: Blocks,
    pub ball: Ball,
    pub score: Score,
    pub life: Life,
    is_over: bool,
}

impl Logic {
    /// Create a new game logic with default values for game settings
    pub fn new() -> Logic {
        Logic {
            racket: Racket::new(),
            blocks: Blocks::new(),
            ball: Ball::new(),
            score: Score::new(),
            life: Life::new(),
            is_over: false,
        }
    }

    /// Update each entity of a delta of time and check if the game is over.
    pub fn update(&mut self, dt: f32) {
        self.racket.update(dt);
        self.ball.update(dt);

        if self.ball.solid.pos.y() > 1. {
            self.life.remove();
            self.ball.reset();
        }

        if self.life.count <= 0 {
            self.over();
        }
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