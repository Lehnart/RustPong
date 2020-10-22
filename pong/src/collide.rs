use engine::collide::collide;
use engine::geometry::{AsRect, Rect};

use crate::audio::Audio;
use crate::logic::{BALL_DIM, Logic};

pub struct Collide<'a> {
    audio: &'a Audio
}

impl Collide<'_> {
    pub fn new(audio: &Audio) -> Collide {
        Collide {
            audio
        }
    }

    pub fn collide_ball_and_wall(&mut self, logic: &mut Logic) {
        let ball_rect = logic.ball.as_rect();
        let bottom_wall_rect = Rect::from_2_points(0., 0., 1., 0.);
        match collide(&ball_rect, &bottom_wall_rect) {
            Some(_rect) => {
                logic.ball.reflect(0.);
                self.audio.play_wall_bounce();
            }
            None => (),
        };

        let top_wall_rect = Rect::from_2_points(0., 1., 1., 1.);
        match collide(&ball_rect, &top_wall_rect) {
            Some(_rect) => {
                logic.ball.reflect(1. - BALL_DIM);
                self.audio.play_wall_bounce();
            }
            None => (),
        };
    }

    /// Handle the collision between the rackets and the ball
    pub fn collide_ball_and_racket(&mut self, logic: &mut Logic) {
        let ball_rect = logic.ball.as_rect();
        let left_racket: Rect = logic.left_racket.as_rect();
        let right_racket: Rect = logic.right_racket.as_rect();

        match collide(&ball_rect, &left_racket) {
            Some(rect) => {
                if rect.xc() < left_racket.xc() {
                    ()
                }
                let angle = logic.left_racket.get_bounce_angle(rect.xc(), rect.yc());
                logic.ball.bounce(angle, rect.w());
                self.audio.play_racket_bounce();
            }
            None => ()
        }

        match collide(&ball_rect, &right_racket) {
            Some(rect) => {
                if rect.xc() < right_racket.xc() {
                    ()
                }
                let angle = logic.right_racket.get_bounce_angle(rect.xc(), rect.yc());
                logic.ball.bounce(angle, -rect.w());
                self.audio.play_racket_bounce();
            }
            None => ()
        }
    }
}
