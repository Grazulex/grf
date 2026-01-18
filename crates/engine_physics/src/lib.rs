//! Engine Physics - 2D Collision Detection
//!
//! This crate provides AABB collision detection with spatial
//! partitioning for efficient broad-phase collision detection.

use glam::Vec2;

/// Axis-Aligned Bounding Box for collision detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    /// Minimum corner (bottom-left)
    pub min: Vec2,
    /// Maximum corner (top-right)
    pub max: Vec2,
}

impl AABB {
    /// Create a new AABB from position and size
    #[must_use]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            min: Vec2::new(x, y),
            max: Vec2::new(x + width, y + height),
        }
    }

    /// Check if this AABB intersects with another
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
    }

    /// Check if this AABB contains a point
    #[must_use]
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    /// Get the width of this AABB
    #[must_use]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    /// Get the height of this AABB
    #[must_use]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// Get the center point of this AABB
    #[must_use]
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }
}

/// Default spatial grid cell size in pixels
pub const DEFAULT_CELL_SIZE: f32 = 64.0;
