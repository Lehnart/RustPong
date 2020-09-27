pub(crate) enum Racket {
    Left,
    Right,
}

const RACKET_SPEED: f32 = 0.50;
pub(crate) const RACKET_HEIGHT: f32 = 0.10;
pub(crate) const RACKET_WIDTH: f32 = 0.01;
pub(crate) const BALL_DIM: f32 = 0.01;

pub(crate) struct Logic {
    pub(crate) left_racket_x: f32,
    pub(crate) left_racket_y: f32,
    left_racket_vy: f32,
    pub(crate) right_racket_x: f32,
    pub(crate) right_racket_y: f32,
    right_racket_vy: f32,
    pub(crate) ball_x: f32,
    pub(crate) ball_y: f32,
}

impl Logic {
    pub(crate) fn new() -> Logic {
        Logic {
            left_racket_x: 0.01,
            left_racket_y: 0.5,
            left_racket_vy: 0.,
            right_racket_x: 1. - RACKET_WIDTH - 0.01,
            right_racket_y: 0.5,
            right_racket_vy: 0.,
            ball_x: 0.5 - (BALL_DIM / 2.),
            ball_y: 0.5 - (BALL_DIM / 2.),
        }
    }

    pub(crate) fn update(&mut self, dt: f32) {
        self.left_racket_y += self.left_racket_vy * dt;
        self.right_racket_y += self.right_racket_vy * dt;

        if self.left_racket_y < 0. {
            self.left_racket_y = 0.;
        }

        if self.left_racket_y + RACKET_HEIGHT > 1. {
            self.left_racket_y = 1. - RACKET_HEIGHT;
        }

        if self.right_racket_y < 0. {
            self.right_racket_y = 0.;
        }

        if self.right_racket_y + RACKET_HEIGHT > 1. {
            self.right_racket_y = 1. - RACKET_HEIGHT;
        }
    }

    pub(crate) fn accelerate(&mut self, racket: Racket) {
        match racket {
            Racket::Left => self.left_racket_vy += RACKET_SPEED,
            Racket::Right => self.right_racket_vy += RACKET_SPEED,
        }
    }

    pub(crate) fn decelerate(&mut self, racket: Racket) {
        match racket {
            Racket::Left => self.left_racket_vy -= RACKET_SPEED,
            Racket::Right => self.right_racket_vy -= RACKET_SPEED,
        }
    }
}
