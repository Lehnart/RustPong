use std::time::{SystemTime, UNIX_EPOCH};

pub const RACKET_HEIGHT: f32 = 0.10;
pub const RACKET_WIDTH: f32 = 0.01;
pub const RACKET_SHIFT_X: f32 = 0.01;
pub const BALL_DIM: f32 = 0.01;
pub const BALL_SPEED : f32 = 0.5;
const RACKET_SPEED: f32 = 0.75;

pub struct Score{
    left : u8,
    right : u8
}

impl Score{
    fn new() -> Score{
        Score{
            left :0,
            right :0,
        }
    }

    pub fn left(&self) -> u8 {
        self.left
    }
    pub fn right(&self) -> u8 {
        self.right
    }

    pub fn point_left(&mut self){
        self.left += 1;
    }

    pub fn point_right(&mut self){
        self.right += 1;
    }

}

pub struct Rect{
    pub x : f32,
    pub y : f32,
    pub w : f32,
    pub h : f32
}

pub trait AsRect {
    fn as_rect(&self) -> Rect;
}


pub struct Racket {
    x: f32,
    y: f32,
    vy: f32,

}

impl Racket {
    fn new(x: f32, y: f32) -> Racket {
        Racket {
            x,
            y,
            vy: 0.,
        }
    }

    fn update(&mut self, dt: f32) {
        self.y += self.vy * dt;
        if self.y < 0. {
            self.y = 0.;
        } else if self.y + RACKET_HEIGHT > 1. {
            self.y = 1. - RACKET_HEIGHT;
        }
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn accelerate(&mut self) {
        self.vy += RACKET_SPEED
    }

    pub fn decelerate(&mut self) {
        self.vy -= RACKET_SPEED
    }
}

impl AsRect for Racket{
    fn as_rect(&self) -> Rect {
        Rect{
            x: self.x,
            y: self.y,
            w: RACKET_WIDTH,
            h: RACKET_HEIGHT
        }
    }
}

pub struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32
}

impl Ball {
    fn new(x: f32, y: f32) -> Ball {

        let mut random_angle : u128 = 90 ;
        while ( random_angle > 70 && random_angle < 110)
        || (random_angle > 250 && random_angle < 290 ){
            random_angle = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() % 360;
        }
        let random_angle = random_angle as f32;
        let random_angle = random_angle * std::f64::consts::PI as f32 / 180.;
        Ball {
            x,
            y,
            vx: random_angle.cos()*BALL_SPEED,
            vy: random_angle.sin()*BALL_SPEED
        }

    }

    fn update(&mut self, dt: f32){
        self.y += self.vy * dt;
        self.x += self.vx * dt;

        if self.y < 0. || self.y + BALL_DIM > 1. {
            self.vy = - self.vy;
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn vx(&self) -> f32 {
        self.vx
    }

    pub fn set_vx(&mut self, vx: f32) {
        self.vx = vx;
    }
}

impl AsRect for Ball{
    fn as_rect(&self) -> Rect {
        Rect{
            x: self.x,
            y: self.y,
            w: BALL_DIM,
            h: BALL_DIM
        }
    }
}

pub struct Logic {
    left_racket: Racket,
    right_racket: Racket,
    ball: Ball,
    score: Score,
    is_over: bool
}

impl Logic {
    pub fn new() -> Logic {
        Logic {
            left_racket: Racket::new(RACKET_SHIFT_X, 0.5 - (RACKET_HEIGHT / 2.)),
            right_racket: Racket::new(1. - RACKET_WIDTH - RACKET_SHIFT_X, 0.5 - (RACKET_HEIGHT / 2.)),
            ball: Ball::new(0.5 - (BALL_DIM / 2.), 0.5 - (BALL_DIM / 2.)),
            score: Score::new(),
            is_over : false
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.left_racket.update(dt);
        self.right_racket.update(dt);
        self.ball.update(dt);

        if self.ball.x() < 0. || self.ball.x() > 1. {

            if self.ball.x() < 0.{
                self.score.point_right();
            }

            else{
                self.score.point_left()
            }

            self.ball = Ball::new(0.5 - (BALL_DIM / 2.), 0.5 - (BALL_DIM / 2.));
        }

    }

    pub fn left_racket(&self) -> &Racket {
        &self.left_racket
    }

    pub fn right_racket(&self) -> &Racket {
        &self.right_racket
    }

    pub fn m_left_racket(&mut self) -> &mut Racket {
        &mut self.left_racket
    }

    pub fn m_right_racket(&mut self) -> &mut Racket {
        &mut self.right_racket
    }

    pub fn ball(&self) -> &Ball {
        &self.ball
    }

    pub fn m_ball(&mut self) -> &mut Ball {
        &mut self.ball
    }

    pub fn over(&mut self) {
        self.is_over = true;
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }

    pub fn score(&self) -> &Score {
        &self.score
    }
}
