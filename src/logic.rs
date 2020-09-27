pub const RACKET_HEIGHT: f32 = 0.10;
pub const RACKET_WIDTH: f32 = 0.01;
pub const BALL_DIM: f32 = 0.01;
const RACKET_SPEED: f32 = 0.50;

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

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }

    fn update(&mut self, dt: f32) {
        self.y += self.vy * dt;
        if self.y < 0. {
            self.y = 0.;
        } else if self.y + RACKET_HEIGHT > 1. {
            self.y = 1. - RACKET_HEIGHT;
        }
    }


    pub fn accelerate(&mut self) {
        self.vy += RACKET_SPEED
    }

    pub fn decelerate(&mut self) {
        self.vy -= RACKET_SPEED
    }
}

pub struct Ball {
    x: f32,
    y: f32,
}

impl Ball {
    fn new(x: f32, y: f32) -> Ball {
        Ball { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
}

pub struct Logic {
    left_racket: Racket,
    right_racket: Racket,
    ball: Ball,
    is_over: bool
}

impl Logic {
    pub fn new() -> Logic {
        Logic {
            left_racket: Racket::new(0.01, 0.5 - (RACKET_HEIGHT / 2.)),
            right_racket: Racket::new(1. - RACKET_WIDTH - 0.01, 0.5 - (RACKET_HEIGHT / 2.)),
            ball: Ball::new(0.5 - (BALL_DIM / 2.), 0.5 - (BALL_DIM / 2.)),
            is_over : false
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.left_racket.update(dt);
        self.right_racket.update(dt);
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

    pub fn over(&mut self) {
        self.is_over = true;
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }
}
