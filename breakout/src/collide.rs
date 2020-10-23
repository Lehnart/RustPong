use crate::logic::{Logic, BOARD_LEFT_LIMIT_X, BOARD_RIGHT_LIMIT_X, BALL_DIM, BOARD_TOP_LIMIT_Y};
use engine::geometry::{AsRect, Rect};
use engine::collide::collide;

/// Handle the collision between the rackets and the ball
pub fn collide_ball_and_racket(logic: &mut Logic) {
    let ball = logic.ball.as_rect();
    let racket = logic.racket.as_rect();

    match collide(&ball, &racket) {
        Some(rect) => {
            if rect.yc() > racket.yc() {
                ()
            }
            let angle = logic.racket.get_bounce_angle(rect.xc(), rect.yc());
            logic.ball.bounce(angle, -rect.h());
        }
        None => ()
    }
}
pub fn collide_ball_and_wall(logic: &mut Logic) {
    let ball = logic.ball.as_rect();
    let left_wall = Rect::from_2_points(0., 0., BOARD_LEFT_LIMIT_X, 1.);
    match collide(&ball, &left_wall) {
        Some(_rect) => {
            logic.ball.reflect_x(BOARD_LEFT_LIMIT_X);
        }
        None => (),
    };

    let right_wall = Rect::from_2_points(BOARD_RIGHT_LIMIT_X, 0., 1., 1.);
    match collide(&ball, &right_wall) {
        Some(_rect) => {
            logic.ball.reflect_x(BOARD_RIGHT_LIMIT_X - BALL_DIM);
        }
        None => (),
    };

    let top_wall = Rect::from_2_points(0., 0., 1., BOARD_TOP_LIMIT_Y);
    match collide(&ball, &top_wall) {
        Some(_rect) => {
            logic.ball.reflect_y(BOARD_TOP_LIMIT_Y);
        }
        None => (),
    };
}