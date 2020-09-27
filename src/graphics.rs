use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::logic::{BALL_DIM, Logic, RACKET_HEIGHT, RACKET_WIDTH};

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
        self.left_racket.y = (logic.left_racket().y() * self.height as f32) as i32;
        self.left_racket.x = (logic.left_racket().x() * self.width as f32) as i32;
        self.left_racket.set_width((RACKET_WIDTH * self.width as f32) as u32);
        self.left_racket.set_height((RACKET_HEIGHT * self.height as f32) as u32);

        self.right_racket.y = (logic.right_racket().y() * self.height as f32) as i32;
        self.right_racket.x = (logic.right_racket().x() * self.width as f32) as i32;
        self.right_racket.set_width((RACKET_WIDTH * self.width as f32) as u32);
        self.right_racket.set_height((RACKET_HEIGHT * self.height as f32) as u32);

        self.ball.x = (logic.ball().x() * self.width as f32) as i32;
        self.ball.y = (logic.ball().y() * self.height as f32) as i32;
        self.ball.set_width((BALL_DIM * self.width as f32) as u32);
        self.ball.set_height((BALL_DIM * self.height as f32) as u32);
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