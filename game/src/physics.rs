//! Physics primitives, bounding boxes, and kinematics constants for the running game.

/// Axis-Aligned Bounding Box (AABB) for 2D collision detection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    /// Horizontal position of the top-left corner in pixels.
    pub x: f64,
    /// Vertical position of the top-left corner in pixels.
    pub y: f64,
    /// Width of the bounding box in pixels.
    pub width: f64,
    /// Height of the bounding box in pixels.
    pub height: f64,
}

impl Aabb {
    /// Creates a new axis-aligned bounding box.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns `true` if this bounding box intersects with `other`.
    pub fn intersects(&self, other: &Aabb) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

/// Downward gravitational acceleration in pixels / second^2.
pub const GRAVITY: f64 = 1100.0;

/// Upward velocity applied when starting a jump in pixels / second (negative is upwards).
pub const JUMP_VELOCITY: f64 = -620.0;

/// Baseline vertical coordinate (pixels from top of viewport) where the ground level sits.
pub const GROUND_Y: f64 = 420.0;

/// Visual and physical height of the runners in pixels.
pub const RUNNER_HEIGHT: f64 = 70.0;

/// Effective collision hitbox width of the runners in pixels.
pub const RUNNER_WIDTH: f64 = 34.0;
