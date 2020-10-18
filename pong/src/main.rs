mod graphics;
mod event;
mod collide;
mod audio;
mod logic;

use std::time::SystemTime;

use logic::Logic;
use audio::Audio;
use event::handle_event;
use collide::Collide;

use engine::audio::init_audio;
use crate::graphics::Graphics;
use engine::graphics::Window;

fn main() {

    init_audio();
    let audio = Audio::new();
    let mut logic = Logic::new(&audio);
    let mut collide = Collide::new(&audio);

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
        if logic.is_over(){
            break 'game_loop
        }

        collide.collide_ball_and_wall(&mut logic);
        collide.collide(&mut logic, dt);

        audio.update(&collide);

        graphics.update(&logic,&window);
        graphics.draw(&mut window);
    }
}

