use crate::logic::{Shell, Map, BLOCK_COL_COUNT, BLOCK_ROW_COUNT, Tank};
use engine::geometry::AsRect;
use engine::collide::collide;

pub fn collide_shell_and_map(shell: &mut Shell, map: &Map) {
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
                            break
                        }
                    }
                }
            }
        }
    }
}

pub fn collide_tank_and_map(tank: &mut Tank, map: &Map, dt: f32){
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
                            break
                        }
                    }
                }
            }
        }
    }
}

pub fn collide_tanks(tank1: &mut Tank,tank2: &mut Tank, dt:f32){
    let tank1_rect = tank1.as_rect();
    let tank2_rect = tank2.as_rect();
    match collide(&tank1_rect, &tank2_rect){
        None => (),
        Some(_) => {
            tank1.move_back(dt);
            tank2.move_back(dt);
        }
    }
}


pub fn collide_shell_and_tank(shell: &mut Shell, tank: &mut Tank){
    let tank_rect = tank.as_rect();
    let shell_rect = shell.as_rect();
    match collide(&tank_rect, &shell_rect){
        None => (),
        Some(_) => {
            if !tank.is_impacted() {
                let angle = shell.get_orientation();
                tank.impact(angle);
                shell.destroy();
            }
        }
    }
}