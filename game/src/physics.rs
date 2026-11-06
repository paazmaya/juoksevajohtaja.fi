#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Aabb {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn intersects(&self, other: &Aabb) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

pub const GRAVITY: f64 = 1400.0; // pixels / second^2
pub const JUMP_VELOCITY: f64 = -620.0; // upwards
pub const GROUND_Y: f64 = 420.0; // baseline ground level
pub const RUNNER_HEIGHT: f64 = 70.0;
pub const RUNNER_WIDTH: f64 = 40.0;
