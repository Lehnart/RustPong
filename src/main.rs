use std::time::SystemTime;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use graphics::Graphics;
use graphics::Window;
use logic::Logic;
use logic::Racket;

mod graphics;
mod logic;

fn main() {
    let mut window = Window::new(800, 800);
    let mut graphics = Graphics::new(&window.canvas);
    let mut logic = Logic::new();

    let mut start = SystemTime::now();

    'game_loop: loop {
        let next = SystemTime::now();
        let dt = next.duration_since(start).unwrap().as_secs_f32();
        start = next;

        let event_pump = &mut window.event_pump;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop;
                }

                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    logic.decelerate(Racket::Left);
                }
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    logic.accelerate(Racket::Left);
                }
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    logic.accelerate(Racket::Left);
                }
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    logic.decelerate(Racket::Left);
                }

                Event::KeyDown { keycode: Some(Keycode::Z), repeat: false, .. } => {
                    logic.decelerate(Racket::Right);
                }
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
                    logic.accelerate(Racket::Right);
                }
                Event::KeyUp { keycode: Some(Keycode::Z), repeat: false, .. } => {
                    logic.accelerate(Racket::Right);
                }
                Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => {
                    logic.decelerate(Racket::Right);
                }
                _ => {}
            }
        }
        logic.update(dt);
        graphics.update(&logic);
        graphics.draw(&mut window.canvas);
    }
}

