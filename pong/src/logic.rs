//! All logic aspect of the pong game.
use engine::geometry::{AsRect, Rect};
use engine::physics::{Velocity, Solid, Position};
use engine::random::{rand, flip};
use crate::audio::Audio;

pub const RACKET_HEIGHT: f32 = 0.10;
pub const RACKET_WIDTH: f32 = 0.01;
pub const RACKET_SHIFT_X: f32 = 0.01;
const RACKET_SPEED: f32 = 0.75;

pub const BALL_DIM: f32 = 0.01;
pub const BALL_SPEED: f32 = 0.5;

pub const SCORE_MAX: u8 = 10;

/// Score of each of the player in a pong game.
///
/// Score in pong is one score per player.
/// Each player is represented by a side of the game, left or right.
/// The first to achieve [`max`] win the game.
/// Max score is 10 by default.
///
/// [`max`]: Score::max
///
pub struct Score {

    /// Left player score.
    left: u8,

    /// Right player score.
    right: u8,

    /// Score to achieve to win the game.
    max : u8,
}

impl Score {

    /// Create a new score, starting at 0 zero for each player.
    fn new() -> Score {
        Score {
            left: 0,
            right: 0,
            max: SCORE_MAX
        }
    }

    /// Get left player score.
    pub fn left(&self) -> u8 {
        self.left
    }

    /// Get right player score.
    pub fn right(&self) -> u8 {
        self.right
    }

    /// Add one point to left player.
    pub fn point_left(&mut self) { self.left += 1; }

    /// Add one point to right player.
    pub fn point_right(&mut self) {
        self.right += 1;
    }

    /// If a player reach the max score, the game is over.
    pub fn is_game_over(&self) -> bool {
        self.right >= self.max || self.left >= self.max
    }
}

/// A racket represents a player in pong game.
///
/// A racket is represented as a solid, meaning a rect with a speed.
/// It can be moved by the player.
/// It is used to reflect the ball to the opponent racket.
///
pub struct Racket {
    solid: Solid
}

impl Racket {

    /// Create a new racket.
    ///
    /// At creation the racket is not moving.
    /// It can't get out from the screen.
    fn new(x: f32, y: f32) -> Racket {
        let pos = Position::new(x,y);
        let vel = Velocity::new(0.,0.);
        let limit = Rect::new(0.,0.,1.,1.);
        Racket {
            solid: Solid::new(pos, vel, RACKET_WIDTH, RACKET_HEIGHT, limit)
        }
    }

    /// Racket update is just the solid physics updating.
    fn update(&mut self, dt: f32) {
        self.solid.update(dt);
    }

    /// To make the racket start moving.
    pub fn accelerate(&mut self) {
        self.solid.add_v(&Velocity::new(0., RACKET_SPEED));
    }

    /// To make the racket stop moving.
    pub fn decelerate(&mut self) {
        self.solid.add_v(&Velocity::new(0., -RACKET_SPEED));
    }
}

/// Converting the racket to a Rect make it more easy for collision and drawing.
impl AsRect for Racket {
    fn as_rect(&self) -> Rect {
        self.solid.as_rect()
    }
}

/// The ball which move across the board, between rackets.
///
/// A ball is represented as a solid.
/// It is reflected on rackets.
///
pub struct Ball {
    solid: Solid
}

impl Ball {

    /// Create a new ball with a random direction
    fn new(x: f32, y: f32) -> Ball {

        let mut random_angle: i32 = rand(-35,35);
        if flip() {
            random_angle += 180;
        }
        let random_angle = random_angle as f32 * std::f32::consts::PI / 180.;

        let pos = Position::new(x,y);
        let vel = Velocity::new(random_angle.cos() * BALL_SPEED,random_angle.sin() * BALL_SPEED);
        let limit = Rect::new(-1.,-1.,2.,2.);

        Ball {
            solid: Solid::new(pos, vel, BALL_DIM, BALL_DIM, limit)
        }
    }

    /// Ball update is just the solid physics updating.
    fn update(&mut self, dt: f32) {
        self.solid.update(dt);
    }

    pub fn solid(&self)-> & Solid{
        &self.solid
    }

    pub fn m_solid(&mut self)-> &mut Solid{
        &mut self.solid
    }

    pub fn set_y(&mut self, y : f32){
        self.solid.m_pos().set_y(y);
    }

    pub fn reflect(&mut self) {
        let vy = self.solid.m_vel().vy();
        self.solid.m_vel().set_vy(-vy);
    }
}

/// Converting the ball to a Rect make it more easy for collision and drawing.
impl AsRect for Ball {
    fn as_rect(&self) -> Rect {
        self.solid.as_rect()
    }
}

/// Logic is a structure that contains all entities from a pong game.
///
/// It contains the two rackets and the ball.
/// There is also the current score.
/// Access is done directly through the fields, there are public.
///
pub struct Logic<'a> {
    pub left_racket: Racket,
    pub right_racket: Racket,
    pub ball: Ball,
    pub score: Score,

    audio : &'a Audio,

    is_over: bool,
}

impl Logic<'_> {

    /// Create a new game logic with default values for game settings
    pub fn new(audio: &Audio) -> Logic {
        Logic {
            left_racket: Racket::new(RACKET_SHIFT_X, 0.5 - (RACKET_HEIGHT / 2.)),
            right_racket: Racket::new(1. - RACKET_WIDTH - RACKET_SHIFT_X, 0.5 - (RACKET_HEIGHT / 2.)),
            ball: Ball::new(0.5 - (BALL_DIM / 2.), 0.5 - (BALL_DIM / 2.)),
            score: Score::new(),

            audio,
            is_over: false
        }
    }

    /// Update each entity of a delta of time and check if the game is over.
    pub fn update(&mut self, dt: f32) {
        self.left_racket.update(dt);
        self.right_racket.update(dt);
        self.ball.update(dt);
        self.update_score();
    }

    /// Check if the ball is out of board, meaning there is a goal.
    /// Add a point to the player who scores and thrown another ball
    fn update_score(&mut self){
        let ball_rect = self.ball.as_rect();
        let x = ball_rect.xc();
        if x < 0. || x > 1. {
            self.audio.play_lose();

            if x < 0. {
                self.score.point_right();
            } else {
                self.score.point_left()
            }
            if self.score.is_game_over() {
                self.is_over = true
            }
            self.ball = Ball::new(0.5 - (BALL_DIM / 2.), 0.5 - (BALL_DIM / 2.));
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
