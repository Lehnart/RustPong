use sdl2::render::{WindowCanvas};
use sdl2::EventPump;
use sdl2::rect::Rect;
use crate::geometry;
use sdl2::pixels::Color;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::surface::Surface;

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
    pub color: Color,
    is_visible : bool
}

impl Sprite {
    pub fn new(x: i32, y: i32, w: u32, h: u32, color : Color) -> Sprite {
        Sprite {
            rect: Rect::new(x, y, w, h),
            color,
            is_visible : true
        }
    }

    pub fn default(color : Color) -> Sprite{
        Sprite::new(0,0,1,1, color)
    }

    pub fn update(&mut self, logic_rect: geometry::Rect, canvas_width: u32, canvas_height: u32) {
        self.rect.y = (logic_rect.y0() * canvas_height as f32) as i32;
        self.rect.x = (logic_rect.x0() * canvas_width as f32) as i32;
        self.rect.set_width((logic_rect.w() * canvas_width as f32) as u32);
        self.rect.set_height((logic_rect.h() * canvas_height as f32) as u32);
    }

    pub fn draw(&self, canvas : &mut WindowCanvas){
        if self.is_visible {
            canvas.set_draw_color(self.color);
            canvas.fill_rect(self.rect).unwrap();
        }
    }

    pub fn hide(&mut self){
        self.is_visible = false;
    }

    pub fn show(&mut self){
        self.is_visible = true;
    }
}

pub struct RenderedString<'a> {
    surface :  Surface<'a>,
    xc : i32,
    yc : i32,
}

impl RenderedString<'_> {
    pub fn new<'a>( str : &String, xc : i32, yc : i32 ,ttf_context: &Sdl2TtfContext, font_path : &str, font_size : u16) -> RenderedString<'a> {
        let font = ttf_context.load_font(font_path, font_size).unwrap();
        let surface = font.render(&*str).solid(Color::WHITE).unwrap();
        RenderedString{ surface, xc , yc }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas){

        let texture_creator = canvas.texture_creator();
        let texture = &texture_creator.create_texture_from_surface(&self.surface).unwrap();
        let texture_query = texture.query();
        let w = texture_query.width;
        let h = texture_query.height;
        canvas.copy(
            texture,
            Rect::new(0, 0, w, h),
            Rect::new(self.xc - (w/2) as i32, self.yc - (h/2)  as i32, w, h),
        ).unwrap();

    }
}