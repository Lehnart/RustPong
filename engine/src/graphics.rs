use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::rect::Rect;
use crate::geometry;
use sdl2::pixels::Color;

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

    pub fn width(&self)->u32{
        let dim = self.canvas.output_size().unwrap();
        dim.0
    }

    pub fn height(&self)->u32{
        let dim = self.canvas.output_size().unwrap();
        dim.1
    }

    pub fn clear(&mut self){
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

    }
}


pub struct Sprite {
    pub rect: Rect,
}

impl Sprite {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Sprite {
        Sprite {
            rect: Rect::new(x, y, w, h)
        }
    }

    pub fn default() -> Sprite{
        Sprite::new(0,0,1,1)
    }

    pub fn update(&mut self, logic_rect: geometry::Rect, canvas_width: u32, canvas_height: u32) {
        self.rect.y = (logic_rect.y0() * canvas_height as f32) as i32;
        self.rect.x = (logic_rect.x0() * canvas_width as f32) as i32;
        self.rect.set_width((logic_rect.w() * canvas_width as f32) as u32);
        self.rect.set_height((logic_rect.h() * canvas_height as f32) as u32);
    }
}
