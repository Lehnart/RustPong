use crate::logic::{BALL_DIM, Logic, RACKET_HEIGHT, RACKET_SHIFT_X, RACKET_WIDTH};
use std::f64::consts::PI;
use engine::geometry::{AsRect, Rect};
use engine::collide::collide;
use crate::audio::Audio;

enum Dir{
    Left,
    Right,
    None,
}

pub struct Collide<'a>{
    is_collide : bool,
    audio : &'a Audio
}

impl Collide<'_>{

    pub fn new(audio: &Audio)->Collide {
        Collide{
            is_collide:false,
            audio
        }
    }

    pub fn is_collide(&self) -> bool {
        self.is_collide
    }

    pub fn collide_ball_and_wall(&mut self, logic: &mut Logic){

        let ball_rect = logic.ball.as_rect();
        let bottom_wall_rect = Rect::from_2_points(0.,0.,1.,0.);
        match collide(&ball_rect,&bottom_wall_rect){
            Some(_rect) => {
                logic.ball.set_y(0.);
                logic.ball.reflect();
                self.audio.play_wall_bounce();
            }
            None => () ,
        };

        let top_wall_rect = Rect::from_2_points(0.,1.,1.,1.);
        match collide(&ball_rect,&top_wall_rect){
            Some(_rect) => {
                logic.ball.set_y(1.-BALL_DIM);
                logic.ball.reflect();
                self.audio.play_wall_bounce();
            },
            None => () ,
        };
    }

    pub fn collide(&mut self, logic: &mut Logic, dt: f32) {

        self.is_collide = false;

        let ball = logic.ball.solid();
        let left_racket: Rect = logic.left_racket.as_rect();
        let right_racket: Rect = logic.right_racket.as_rect();

        let px = ball.pos().x() - (ball.vel().vx() * dt);
        let x = ball.pos().x();
        let y = ball.pos().y();

        let mut dir : Dir = Dir::None;
        let mut angle = 0.;
        if x <= RACKET_SHIFT_X + RACKET_WIDTH
            && px >= RACKET_SHIFT_X + RACKET_WIDTH
            && y < left_racket.y0() + RACKET_HEIGHT
            && y + BALL_DIM > left_racket.y0()
            && ball.vel().vx() < 0.
        {
            dir = Dir::Right;
            angle = (y - left_racket.y0() - (RACKET_HEIGHT/2.))/RACKET_HEIGHT*2.*70./180.*(PI as f32);
        }

        if x + BALL_DIM >= 1. - RACKET_SHIFT_X - RACKET_WIDTH
            && px + BALL_DIM <= 1. - RACKET_SHIFT_X - RACKET_WIDTH
            && y < right_racket.y0() + RACKET_HEIGHT
            && y + BALL_DIM > right_racket.y0()
            && ball.vel().vx() > 0.
        {
            dir = Dir::Left;
            angle = (y - right_racket.y0() - (RACKET_HEIGHT/2.))/RACKET_HEIGHT*2.*70./180.*(PI as f32);
        }

        match dir {
            Dir::Left | Dir::Right =>{

                self.is_collide = true;

                let ball = logic.ball.m_solid();
                let vx = ball.vel().mag()*angle.cos();
                let vy = ball.vel().mag()*angle.sin();

                ball.m_vel().set_vx(vx);
                ball.m_vel().set_vy(vy);
                match dir {
                    Dir::Left => {
                        let vx = ball.vel().vx();
                        ball.m_vel().set_vx(-vx)
                    },
                    _ => {}
                }

            },
            Dir::None =>{}
        }
    }
}
