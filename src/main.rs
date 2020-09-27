use std::time::SystemTime;

use graphics::Graphics;
use graphics::Window;
use logic::Logic;

use event::handle_event;

mod graphics;
mod logic;
mod event;

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
            handle_event(event, &mut logic);
        }

        logic.update(dt);
        if logic.is_over(){
            break 'game_loop
        }

        graphics.update(&logic);
        graphics.draw(&mut window.canvas);


    }
}

