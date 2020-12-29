use engine::collide::collide;
use engine::geometry::AsRect;

use crate::audio::Audio;
use crate::logic::{BLOCK_COL_COUNT, BLOCK_ROW_COUNT, BOARD_BOTTOM_LIMIT, BOARD_LEFT_LIMIT, BOARD_RIGHT_LIMIT, BOARD_TOP_LIMIT, Logic, Map, Shell, Tank};

fn collide_shell_and_map(shell: &mut Shell, map: &Map) {
    let shell_rect = shell.as_rect();
    for i in 0..BLOCK_COL_COUNT {
        for j in 0..BLOCK_ROW_COUNT {
            let block = map.get_block(i as u32, j as u32);
            match block {
                None => continue,
                Some(block_rect) => {
                    match collide(&shell_rect, &block_rect) {
                        None => continue,
                        Some(_) => {
                            shell.destroy();
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn collide_shell_and_limits(shell: &mut Shell) {
    if shell.is_destroyed() {
        return;
    }
    let shell_rect = shell.as_rect();
    if shell_rect.xc() > BOARD_RIGHT_LIMIT || shell_rect.xc() < BOARD_LEFT_LIMIT {
        shell.destroy();
    }
    if shell_rect.yc() > BOARD_BOTTOM_LIMIT || shell_rect.yc() < BOARD_TOP_LIMIT {
        shell.destroy();
    }
}

fn collide_tank_and_map(tank: &mut Tank, map: &Map, dt: f32) {
    let tank_rect = tank.as_rect();
    for i in 0..BLOCK_COL_COUNT {
        for j in 0..BLOCK_ROW_COUNT {
            let block = map.get_block(i as u32, j as u32);
            match block {
                None => continue,
                Some(block_rect) => {
                    match collide(&tank_rect, &block_rect) {
                        None => continue,
                        Some(_) => {
                            tank.move_back(dt);
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn collide_tanks(tank1: &mut Tank, tank2: &mut Tank, dt: f32) {
    let tank1_rect = tank1.as_rect();
    let tank2_rect = tank2.as_rect();
    match collide(&tank1_rect, &tank2_rect) {
        None => (),
        Some(_) => {
            tank1.move_back(dt);
            tank2.move_back(dt);
        }
    }
}


fn collide_shell_and_tank(shell: &mut Shell, tank: &mut Tank) -> bool {
    let tank_rect = tank.as_rect();
    let shell_rect = shell.as_rect();
    match collide(&tank_rect, &shell_rect) {
        None => false,
        Some(_) => {
            if !tank.is_impacted() {
                let angle = shell.get_orientation();
                tank.impact(angle);
                shell.destroy();
                return true;
            }
            return false;
        }
    }
}

pub fn check_collision(logic: &mut Logic, dt: f32, audio: &Audio) {
    collide_shell_and_limits(&mut logic.left_tank.shell);
    collide_shell_and_limits(&mut logic.right_tank.shell);
    collide_shell_and_map(&mut logic.left_tank.shell, &logic.map);
    collide_shell_and_map(&mut logic.right_tank.shell, &logic.map);
    collide_tank_and_map(&mut logic.left_tank, &logic.map, dt);
    collide_tank_and_map(&mut logic.right_tank, &logic.map, dt);
    collide_tanks(&mut logic.left_tank, &mut logic.right_tank, dt);
    if collide_shell_and_tank(&mut logic.left_tank.shell, &mut logic.right_tank)
    {
        logic.score.point_left();
        audio.play_right_explosion();
    }
    if collide_shell_and_tank(&mut logic.right_tank.shell, &mut logic.left_tank) {
        logic.score.point_right();
        audio.play_left_explosion();
    }
}