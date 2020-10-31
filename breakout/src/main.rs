use std::time::SystemTime;

use engine::audio::init_audio;
use engine::graphics::Window;

use crate::audio::Audio;
use crate::collide::{collide_ball_and_blocks, collide_ball_and_racket, collide_ball_and_wall};
use crate::event::handle_event;
use crate::graphics::Graphics;
use crate::logic::Logic;

mod logic;
mod event;
mod graphics;
mod collide;
mod audio;

fn main() {
    init_audio();
    let audio = Audio::new();
    let mut logic = Logic::new();

    let ttf_context = sdl2::ttf::init().unwrap();
    let mut window = Window::new(600, 600);
    let mut graphics = Graphics::new();

    let mut previous = SystemTime::now();
    'game_loop: loop {
        let next = SystemTime::now();
        let dt = next.duration_since(previous).unwrap().as_secs_f32();
        previous = next;

        let event_pump = &mut window.event_pump;
        for event in event_pump.poll_iter() {
            handle_event(event, &mut logic);
        }

        logic.update(dt);
        if logic.is_over() {
            break 'game_loop;
        }

        collide_ball_and_racket(&mut logic, &audio);
        collide_ball_and_wall(&mut logic, &audio);
        collide_ball_and_blocks(&mut logic, &audio);

        graphics.update(&logic, &window);
        graphics.draw(&mut window, &ttf_context);
    }
}