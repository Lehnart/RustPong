use crate::logic::{Ball, BALL_DIM, Logic, Racket, RACKET_HEIGHT, RACKET_SHIFT_X, RACKET_WIDTH};

pub fn collide(logic: &mut Logic, dt: f32) {
    let ball: &Ball = logic.ball();
    let left_racket: &Racket = logic.left_racket();
    let right_racket: &Racket = logic.right_racket();

    let px = ball.x() - (ball.vx() * dt);
    let x = ball.x();
    let y = ball.y();

    let mut invert = false;

    if x <= RACKET_SHIFT_X + RACKET_WIDTH
        && px >= RACKET_SHIFT_X + RACKET_WIDTH
        && y < left_racket.y() + RACKET_HEIGHT
        && y + BALL_DIM > left_racket.y()
    {
        invert = true;
    }

    if x + BALL_DIM >= 1. - RACKET_SHIFT_X - RACKET_WIDTH
        && px + BALL_DIM <= 1. - RACKET_SHIFT_X - RACKET_WIDTH
        && y < right_racket.y() + RACKET_HEIGHT
        && y + BALL_DIM > right_racket.y()
    {
        invert = true;
    }

    let ball = logic.m_ball();
    if invert {
        ball.set_vx( - ball.vx());
    }
}