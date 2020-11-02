use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::render::WindowCanvas;
use sdl2::surface::Surface;
use sdl2::ttf::Sdl2TtfContext;

use crate::geometry;

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

    pub fn width(&self) -> u32 {
        let dim = self.canvas.output_size().unwrap();
        dim.0
    }

    pub fn height(&self) -> u32 {
        let dim = self.canvas.output_size().unwrap();
        dim.1
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }
}


pub struct Sprite<'a> {
    pub surface : Surface<'a>,
    pub dest_rect: Rect,
    pub angle: f64,
    is_visible: bool,
}

impl Sprite<'_>{
    pub fn new( surf_path : &str, dest_rect : Rect, color: Color) -> Sprite {
        let mut surface = Surface::load_bmp(surf_path).unwrap();
        surface.set_color_mod(color);
        Sprite {
            surface,
            dest_rect,
            angle : 0.,
            is_visible: true,
        }
    }

    pub fn update(&mut self, logic_rect: geometry::Rect, angle: f64, canvas_width: u32, canvas_height: u32) {
        self.dest_rect.y = (logic_rect.y0()*(canvas_height as f32)) as i32;
        self.dest_rect.x = (logic_rect.x0()*(canvas_width as f32)) as i32;
        self.angle = angle;
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        if self.is_visible {

            let texture_creator = canvas.texture_creator();
            let texture = &texture_creator.create_texture_from_surface(&self.surface).unwrap();

            canvas.copy_ex(
                texture,
                Rect::new(0,0,self.surface.width(),self.surface.height()),
                self.dest_rect,
                self.angle,
                Point::new((self.dest_rect.width()/2) as i32,(self.dest_rect.height()/2) as i32),
                false,
                false
            ).unwrap();
        }
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }
}

pub struct RectSprite {
    pub rect: Rect,
    pub color: Color,
    is_visible: bool,
}

impl RectSprite {
    pub fn new(x: i32, y: i32, w: u32, h: u32, color: Color) -> RectSprite {
        RectSprite {
            rect: Rect::new(x, y, w, h),
            color,
            is_visible: true,
        }
    }

    pub fn default(color: Color) -> RectSprite {
        RectSprite::new(0, 0, 1, 1, color)
    }

    pub fn update(&mut self, logic_rect: geometry::Rect, canvas_width: u32, canvas_height: u32) {
        self.rect.y = (logic_rect.y0() * canvas_height as f32) as i32;
        self.rect.x = (logic_rect.x0() * canvas_width as f32) as i32;
        self.rect.set_width((logic_rect.w() * canvas_width as f32) as u32);
        self.rect.set_height((logic_rect.h() * canvas_height as f32) as u32);
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        if self.is_visible {
            canvas.set_draw_color(self.color);
            canvas.fill_rect(self.rect).unwrap();
        }
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }
}

pub struct RenderedString<'a> {
    surface: Surface<'a>,
    xc: i32,
    yc: i32,
}

impl RenderedString<'_> {
    pub fn new<'a>(str: &String, xc: i32, yc: i32, ttf_context: &Sdl2TtfContext, font_path: &str, font_size: u16) -> RenderedString<'a> {
        let font = ttf_context.load_font(font_path, font_size).unwrap();
        let surface = font.render(&*str).solid(Color::WHITE).unwrap();
        RenderedString { surface, xc, yc }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        let texture_creator = canvas.texture_creator();
        let texture = &texture_creator.create_texture_from_surface(&self.surface).unwrap();
        let texture_query = texture.query();
        let w = texture_query.width;
        let h = texture_query.height;
        canvas.copy(
            texture,
            Rect::new(0, 0, w, h),
            Rect::new(self.xc - (w / 2) as i32, self.yc - (h / 2) as i32, w, h),
        ).unwrap();
    }
}