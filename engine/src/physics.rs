use crate::geometry::{AsRect, Rect, AsCircle, Circle};

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
    pub fn default() -> Velocity {
        Velocity {
            vx: 0.,
            vy: 0.,
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
    pub fn angle(&self) -> f32 {self.vy.atan2(self.vx)}

    pub fn set_vx(&mut self, vx: f32) { self.vx = vx; }
    pub fn set_vy(&mut self, vy: f32) {
        self.vy = vy;
    }
}

pub struct RectSolid {
    pub pos: Position,
    pub vel: Velocity,
    w: f32,
    h: f32,
    limit: Rect,
}

impl RectSolid {
    pub fn new(pos: Position, vel: Velocity, w: f32, h: f32, limit: Rect) -> RectSolid
    {
        RectSolid {
            pos,
            vel,
            w,
            h,
            limit,
        }
    }

    pub fn fixed(x: f32, y: f32, w: f32, h: f32) -> RectSolid
    {
        let pos = Position::new(x, y);
        let vel = Velocity::default();

        RectSolid {
            pos,
            vel,
            w,
            h,
            limit: Rect::default(),
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

        if self.pos.x() < self.limit.x0() {
            self.pos.set_x(self.limit.x0());
        } else if self.pos.x() + self.w > self.limit.x1() {
            self.pos.set_x(self.limit.x1() - self.w);
        }
    }
}

impl AsRect for RectSolid {
    fn as_rect(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, self.w, self.h)
    }
}

pub struct CircleSolid {
    pub pos: Position,
    pub vel: Velocity,
    pub r: f32,
    limit: Rect,
}

impl CircleSolid {
    pub fn new(pos: Position, vel: Velocity, r: f32, limit: Rect) -> CircleSolid
    {
        CircleSolid {
            pos,
            vel,
            r,
            limit,
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.pos.set_x(self.pos.x + (self.vel.vx * dt));
        self.pos.set_y(self.pos.y + (self.vel.vy * dt));

        if self.pos.y() < self.limit.y0() {
            self.pos.set_y(self.limit.y0());
        } else if self.pos.y() > self.limit.y1() {
            self.pos.set_y(self.limit.y1());
        }

        if self.pos.x() < self.limit.x0() {
            self.pos.set_x(self.limit.x0());
        } else if self.pos.x() > self.limit.x1() {
            self.pos.set_x(self.limit.x1());
        }
    }
}

impl AsCircle for CircleSolid {
    fn as_circle(&self) -> Circle {
        Circle::new(self.pos.x, self.pos.y, self.r)
    }
}
