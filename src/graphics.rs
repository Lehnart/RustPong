use sdl2::EventPump;
use sdl2::pixels::{Color};
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas, TextureCreator};
use sdl2::surface::Surface;

use crate::logic::{AsRect, Logic};
use crate::logic;
use sdl2::video::WindowContext;

pub const SCORE_SPRITE_PATHS: [&str; 10] = ["0.bmp", "1.bmp", "2.bmp", "3.bmp", "4.bmp", "5.bmp", "6.bmp", "7.bmp", "8.bmp", "9.bmp"];


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

    left_racket: Sprite,
    right_racket: Sprite,
    ball: Sprite,

    score: Score,
}

impl Graphics{
    pub fn new(canvas: &WindowCanvas) -> Graphics {
        let dim = canvas.output_size().unwrap();

        Graphics {
            width: dim.0,
            height: dim.1,
            left_racket: Sprite::new(0, 0, 1, 1),
            right_racket: Sprite::new(0, 0, 1, 1),
            ball: Sprite::new(0, 0, 1, 1),
            score: Score::new(canvas),
        }
    }

    pub fn update(&mut self, logic: &Logic) {
        self.left_racket.update(logic.left_racket().as_rect(), self.width, self.height);
        self.right_racket.update(logic.right_racket().as_rect(), self.width, self.height);
        self.ball.update(logic.ball().as_rect(), self.width, self.height);
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();


        canvas.set_draw_color(Color::WHITE);
        &self.draw_mid_line(canvas);
        canvas.fill_rect(self.left_racket.rect).unwrap();
        canvas.fill_rect(self.right_racket.rect).unwrap();
        canvas.fill_rect(self.ball.rect).unwrap();
        canvas.present();
    }

    fn draw_mid_line(&self, canvas: &mut WindowCanvas) {
        let n_lines = 30;
        let len_line = self.height / n_lines;
        let line_width = 5;
        let line_height = (2. / 3. * len_line as f32) as u32;
        let line_x = (self.width - line_width) / 2;
        for i in 0..n_lines {
            let rect = Rect::new(line_x as i32, (i * len_line) as i32, line_width, line_height);
            canvas.fill_rect(rect).unwrap();
        }
    }
}

struct Sprite {
    rect: Rect,
}

impl Sprite {
    fn new(x: i32, y: i32, w: u32, h: u32) -> Sprite {
        Sprite {
            rect: Rect::new(x, y, w, h)
        }
    }

    fn update(&mut self, logic_rect: logic::Rect, canvas_width: u32, canvas_height: u32) {
        self.rect.y = (logic_rect.y * canvas_height as f32) as i32;
        self.rect.x = (logic_rect.x * canvas_width as f32) as i32;
        self.rect.set_width((logic_rect.w * canvas_width as f32) as u32);
        self.rect.set_height((logic_rect.h * canvas_height as f32) as u32);
    }
}

struct Score {
    left: u8,
    right: u8,
    //digits: Vec<Surface>
}

impl Score {
    pub fn new(canvas: &WindowCanvas) -> Score {

        // let mut digits: Vec<Surface> = Vec::new();
        // for path in SCORE_SPRITE_PATHS.iter() {
        //     let surface = Surface::load_bmp(path).unwrap();
        //     digits.push(surface);
        // }
        let left: u8 = 0;
        let right: u8 = 0;
        Score {
            left,
            right,
            //digits
        }
    }

    pub fn update(&mut self, logic: &Logic) {
        self.left = logic.score().left();
        self.right = logic.score().right();
    }

}