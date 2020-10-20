use crate::geometry::{AsRect, Rect};

pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position {
            x,
            y,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }


    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}

pub struct Velocity {
    vx: f32,
    vy: f32,
}

impl Velocity {
    pub fn new(vx: f32, vy: f32) -> Velocity {
        Velocity {
            vx,
            vy,
        }
    }

    pub fn copy(&self) -> Velocity {
        Velocity {
            vx: self.vx,
            vy: self.vy,
        }
    }

    pub fn vx(&self) -> f32 {
        self.vx
    }
    pub fn vy(&self) -> f32 {
        self.vy
    }

    pub fn mag(&self) -> f32 {
        ((self.vx * self.vx) + (self.vy * self.vy)).sqrt()
    }

    pub fn set_vx(&mut self, vx: f32) { self.vx = vx; }
    pub fn set_vy(&mut self, vy: f32) {
        self.vy = vy;
    }
}

pub struct Solid {
    pos: Position,
    vel: Velocity,
    w: f32,
    h: f32,
    limit: Rect,
}

impl Solid {
    pub fn new(pos: Position, vel: Velocity, w: f32, h: f32, limit: Rect) -> Solid
    {
        Solid {
            pos,
            vel,
            w,
            h,
            limit,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos.set_x(self.pos.x + (self.vel.vx * dt));
        self.pos.set_y(self.pos.y + (self.vel.vy * dt));

        if self.pos.y() < self.limit.y0() {
            self.pos.set_y(self.limit.y0());
        } else if self.pos.y() + self.h > self.limit.y1() {
            self.pos.set_y(self.limit.y1() - self.h);
        }
    }

    pub fn vel(&self) -> &Velocity {
        &self.vel
    }

    pub fn m_vel(&mut self) -> &mut Velocity {
        &mut self.vel
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn m_pos(&mut self) -> &mut Position {
        &mut self.pos
    }
}

impl AsRect for Solid {
    fn as_rect(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, self.w, self.h)
    }
}