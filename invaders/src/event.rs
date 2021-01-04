use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::audio::Audio;
use crate::logic::Logic;

/// Call the logic command accordingly to the input
pub fn handle_event(event: Event, logic: &mut Logic, _audio: &Audio) {
    match event {
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            logic.over();
        }
        Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
            logic.spaceship.decelerate();
        }
        Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
            logic.spaceship.accelerate();
        }
        Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
            logic.spaceship.accelerate();
        }
        Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
            logic.spaceship.decelerate();
        }
        Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
            logic.spaceship.fire();
        }
        _ => {}
    }
}