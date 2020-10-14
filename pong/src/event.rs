use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::logic::Logic;

pub fn handle_event(event:Event, logic: &mut Logic){
    match event {
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            logic.over();
        }

        Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
            logic.left_racket.decelerate();
        }
        Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
            logic.left_racket.accelerate();
        }
        Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
            logic.left_racket.accelerate();
        }
        Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
            logic.left_racket.decelerate();
        }

        Event::KeyDown { keycode: Some(Keycode::Z), repeat: false, .. } => {
            logic.right_racket.decelerate();
        }
        Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
            logic.right_racket.accelerate();
        }
        Event::KeyUp { keycode: Some(Keycode::Z), repeat: false, .. } => {
            logic.right_racket.accelerate();
        }
        Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => {
            logic.right_racket.decelerate();
        }
        _ => {}
    }
}