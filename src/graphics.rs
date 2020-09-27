use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::logic::{Logic, AsRect};
use crate::logic;

pub struct Window {
    pub canvas: WindowCanvas,
    pub event_pump: EventPump,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        let sdl_context = sdl2::init().unwrap();

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Pong in Rust", width, height)
            .position_centered()
            .build()
            .unwrap();

        let window = Window {
            canvas: window.into_canvas().build().unwrap(),
            event_pump: sdl_context.event_pump().unwrap(),
        };

        window
    }
}

pub struct Graphics {
    width: u32,
    height: u32,

    left_racket: Rect,
    right_racket: Rect,
    ball: Rect,
}

impl Graphics {
    pub fn new(canvas: &WindowCanvas) -> Graphics {
        let dim = canvas.output_size().unwrap();

        Graphics {
            width: dim.0,
            height: dim.1,
            left_racket: Rect::new(0,0,1,1,),
            right_racket: Rect::new(0,0,1,1,),
            ball: Rect::new(0,0,1,1,),
        }
    }

    pub fn update(&mut self, logic: &Logic) {
        Graphics::update_rect(&mut self.left_racket, &logic.left_racket().as_rect(), self.width, self.height);
        Graphics::update_rect(&mut self.right_racket, &logic.right_racket().as_rect(), self.width, self.height);
        Graphics::update_rect(&mut self.ball, &logic.ball().as_rect(), self.width, self.height);
    }

    fn update_rect(graphic_rect : &mut Rect, logic_rect: &logic::Rect, w : u32, h: u32){
        graphic_rect.y = (logic_rect.y * h as f32) as i32;
        graphic_rect.x = (logic_rect.x * w as f32) as i32;
        graphic_rect.set_width( (logic_rect.w * w as f32) as u32);
        graphic_rect.set_height( (logic_rect.h * h  as f32) as u32);
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rect(self.left_racket).unwrap();
        canvas.fill_rect(self.right_racket).unwrap();
        canvas.fill_rect(self.ball).unwrap();
        canvas.present();
    }
}