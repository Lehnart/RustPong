use crate::logic::{Logic, BOARD_LEFT_LIMIT_X, BOARD_RIGHT_LIMIT_X, BOARD_TOP_LIMIT_Y};
use engine::geometry::{AsRect, Rect};
use engine::collide::collide;
use crate::audio::Audio;

/// Handle the collision between the rackets and the ball
pub fn collide_ball_and_racket(logic: &mut Logic, audio : &Audio) {
    let ball = logic.ball.as_rect();
    let racket = logic.racket.as_rect();

    match collide(&ball, &racket) {
        Some(rect) => {
            audio.play_racket_bounce();
            if rect.yc() > racket.yc() {
                ()
            }
            let angle = logic.racket.get_bounce_angle(rect.xc(), rect.yc());
            logic.ball.bounce(angle, -rect.h());
        }
        None => ()
    }
}
pub fn collide_ball_and_wall(logic: &mut Logic, audio : &Audio) {
    let ball = logic.ball.as_rect();
    let left_wall = Rect::from_2_points(0., 0., BOARD_LEFT_LIMIT_X, 1.);
    match collide(&ball, &left_wall) {
        Some(rect) => {
            audio.play_wall_bounce();
            logic.ball.reflect_x(rect.w());
        }
        None => (),
    };

    let right_wall = Rect::from_2_points(BOARD_RIGHT_LIMIT_X, 0., 1., 1.);
    match collide(&ball, &right_wall) {
        Some(rect) => {
            audio.play_wall_bounce();
            logic.ball.reflect_x(-rect.w());
        }
        None => (),
    };

    let top_wall = Rect::from_2_points(0., 0., 1., BOARD_TOP_LIMIT_Y);
    match collide(&ball, &top_wall) {
        Some(_rect) => {
            audio.play_wall_bounce();
            logic.ball.reflect_y(BOARD_TOP_LIMIT_Y);
        }
        None => (),
    };
}

pub fn collide_ball_and_blocks(logic: &mut Logic, audio : &Audio) {
    let ball = logic.ball.as_rect();

    for block in &mut  logic.blocks.block_vec {

        if block.is_destroyed() { continue }

        let block_rec = block.as_rect();
        match collide(&ball, &block_rec) {
            Some(rect) => {
                logic.score.add((block.get_value()+1) as u32);
                block.destroy();
                audio.play_block_bounce(block.get_value() as i32);

                if rect.w() > rect.h() {
                    let mut y_shift = rect.h();
                    if rect.yc() < block_rec.yc(){
                        y_shift = - y_shift;
                    }
                    logic.ball.reflect_y(y_shift);

                }

               else {
                    let mut x_shift = rect.w();
                    if rect.xc() < block_rec.xc(){
                        x_shift = - x_shift;
                    }
                    logic.ball.reflect_x(x_shift);
                }
            }
            None => (),
        };
    }



}