use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::logic::Logic;
use crate::audio::Audio;

/// Call the logic command accordingly to the input
pub fn handle_event(event: Event, logic: &mut Logic, audio: &Audio) {
    match event {
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            logic.over();
        }
        _ => {}
    }
}