use std::time::SystemTime;

use engine::graphics::Window;

use crate::audio::Audio;
use crate::collide::check_collision;
use crate::event::handle_event;
use crate::graphics::Graphics;
use crate::input::handle_input;
use crate::logic::Logic;

mod logic;
mod graphics;
mod event;
mod input;
mod collide;
mod audio;

pub const WINDOW_WIDTH: u32 = 600;
pub const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut audio = Audio::new();
    let mut logic = Logic::new();
    let mut window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut graphics = Graphics::new(WINDOW_WIDTH, WINDOW_HEIGHT, &ttf_context);

    let mut previous = SystemTime::now();

    'game_loop: loop {
        let next = SystemTime::now();
        let dt = next.duration_since(previous).unwrap().as_secs_f32();
        previous = next;

        let event_pump = &mut window.event_pump;
        for event in event_pump.poll_iter() {
            handle_event(event, &mut logic, &audio);
        }
        handle_input(event_pump, &mut logic);

        logic.update(dt);
        if logic.is_over() {
            break 'game_loop;
        }

        audio.update(&logic);
        check_collision(&mut logic, dt, &audio);

        graphics.update(&logic, &window, &ttf_context);
        graphics.draw(&mut window);
    }
}
