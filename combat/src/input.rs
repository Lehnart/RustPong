use sdl2::keyboard::Scancode;

use crate::logic::Logic;

pub fn handle_input(event_pump: &sdl2::EventPump, logic: &mut Logic) {
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Left) {
        logic.left_tank.turn_left();
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::Right) {
        logic.left_tank.turn_right();
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::A) {
        logic.right_tank.turn_left();
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::D) {
        logic.right_tank.turn_right();
    }
}