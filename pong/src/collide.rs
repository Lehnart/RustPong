use crate::logic::{Ball, BALL_DIM, Logic, Racket, RACKET_HEIGHT, RACKET_SHIFT_X, RACKET_WIDTH};
use std::f64::consts::PI;

enum Dir{
    Left,
    Right,
    None,
}

pub(crate) struct Collide{
    is_collide : bool,
}

impl Collide {
    pub fn is_collide(&self) -> bool {
        self.is_collide
    }
}

impl Collide{

    pub fn new()->Collide {
        Collide{
            is_collide:false
        }
    }

    pub fn collide(&mut self, logic: &mut Logic, dt: f32) {

        self.is_collide = false;

        let ball: &Ball = logic.ball();
        let left_racket: &Racket = logic.left_racket();
        let right_racket: &Racket = logic.right_racket();

        let px = ball.x() - (ball.vx() * dt);
        let x = ball.x();
        let y = ball.y();


        let mut dir : Dir = Dir::None;
        let mut angle = 0.;
        if x <= RACKET_SHIFT_X + RACKET_WIDTH
            && px >= RACKET_SHIFT_X + RACKET_WIDTH
            && y < left_racket.y() + RACKET_HEIGHT
            && y + BALL_DIM > left_racket.y()
            && ball.vx() < 0.
        {
            dir = Dir::Right;
            angle = (y - left_racket.y() - (RACKET_HEIGHT/2.))/RACKET_HEIGHT*2.*70./180.*(PI as f32);
        }

        if x + BALL_DIM >= 1. - RACKET_SHIFT_X - RACKET_WIDTH
            && px + BALL_DIM <= 1. - RACKET_SHIFT_X - RACKET_WIDTH
            && y < right_racket.y() + RACKET_HEIGHT
            && y + BALL_DIM > right_racket.y()
            && ball.vx() > 0.
        {
            dir = Dir::Left;
            angle = (y - right_racket.y() - (RACKET_HEIGHT/2.))/RACKET_HEIGHT*2.*70./180.*(PI as f32);
        }

        match dir {
            Dir::Left | Dir::Right =>{

                self.is_collide = true;

                let ball = logic.m_ball();
                let vx = ball.v()*angle.cos();
                let vy = ball.v()*angle.sin();

                ball.set_vx(vx);
                ball.set_vy(vy);
                match dir {
                    Dir::Left => ball.set_vx(-ball.vx()),
                    _ => {}
                }

            },
            Dir::None =>{}
        }

    }
}
