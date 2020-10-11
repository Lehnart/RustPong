pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub trait AsRect {
    fn as_rect(&self) -> Rect;
}
