use crate::logic::Logic;
use engine::graphics::Window;
use crate::graphics::Graphics;
use std::time::SystemTime;
use crate::event::handle_event;
use crate::input::handle_input;
use crate::collide::{collide_shell_and_map, collide_tank_and_map, collide_tanks, collide_shell_and_tank};

mod logic;
mod graphics;
mod event;
mod input;
mod collide;

pub const WINDOW_WIDTH : u32 = 600;
pub const WINDOW_HEIGHT : u32 = 700;

fn main() {
    let mut logic = Logic::new();

    let mut window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut graphics = Graphics::new(WINDOW_WIDTH,WINDOW_HEIGHT);
    let ttf_context = sdl2::ttf::init().unwrap();

    let mut previous = SystemTime::now();
    'game_loop: loop {
        let next = SystemTime::now();
        let dt = next.duration_since(previous).unwrap().as_secs_f32();
        previous = next;

        let event_pump = &mut window.event_pump;
        for event in event_pump.poll_iter() {
            handle_event(event, &mut logic);
        }
        handle_input(event_pump, &mut logic);


        logic.update(dt);
        if logic.is_over() {
            break 'game_loop;
        }

        collide_shell_and_map(&mut logic.left_tank.shell, &logic.map);
        collide_shell_and_map(&mut logic.right_tank.shell, &logic.map);
        collide_tank_and_map(&mut logic.left_tank, &logic.map, dt);
        collide_tank_and_map(&mut logic.right_tank, &logic.map, dt);
        collide_tanks(&mut logic.left_tank, &mut logic.right_tank, dt);
        if collide_shell_and_tank(&mut logic.left_tank.shell, &mut logic.right_tank )
        {
            logic.score.point_left();
        }
        if collide_shell_and_tank(&mut logic.right_tank.shell, &mut logic.left_tank ){
            logic.score.point_right();
        }

        graphics.update(&logic, &window);
        graphics.draw(&mut window, &ttf_context);
    }
}
