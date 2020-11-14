use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::logic::Logic;

/// Call the logic command accordingly to the input
pub fn handle_event(event: Event, logic: &mut Logic) {
    match event {
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            logic.over();
        }
        Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
            logic.left_tank.accelerate();
        }
        Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
            logic.left_tank.decelerate();
        }
        Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
            logic.left_tank.fire();
        }
        Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
            logic.right_tank.fire();
        }
        Event::KeyDown { keycode: Some(Keycode::Z), repeat: false, .. } => {
            logic.right_tank.accelerate();
        }
        Event::KeyUp { keycode: Some(Keycode::Z), repeat: false, .. } => {
            logic.right_tank.decelerate();
        }
        Event::KeyDown { keycode: Some(Keycode::R), repeat: false, .. } => {
            logic.change_map();
        }
        _ => {}
    }
}