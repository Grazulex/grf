//! ECS Components for the game
//!
//! Components are pure data structs that can be attached to entities.

use engine_render::glam::Vec2;

/// Position component with previous position for interpolation
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub current: Vec2,
    pub previous: Vec2,
}

impl Position {
    /// Create a new position at the given coordinates
    #[must_use]
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32) -> Self {
        let pos = Vec2::new(x, y);
        Self {
            current: pos,
            previous: pos,
        }
    }

    /// Create from a Vec2
    #[must_use]
    pub fn from_vec2(pos: Vec2) -> Self {
        Self {
            current: pos,
            previous: pos,
        }
    }

    /// Get interpolated position for smooth rendering
    #[must_use]
    pub fn interpolated(&self, alpha: f32) -> Vec2 {
        self.previous.lerp(self.current, alpha)
    }

    /// Save current position as previous (call before movement)
    pub fn save_previous(&mut self) {
        self.previous = self.current;
    }
}

/// Velocity component for movement
#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    /// Create a new velocity
    #[must_use]
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Get as Vec2
    #[must_use]
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

/// Marker component for player-controlled entities
#[derive(Debug, Clone, Copy, Default)]
pub struct PlayerControlled {
    /// Movement speed in pixels per second
    pub speed: f32,
}

impl PlayerControlled {
    /// Create with given speed
    #[must_use]
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

/// Marker component for entities the camera should follow
#[derive(Debug, Clone, Copy, Default)]
pub struct CameraTarget;

/// Sprite rendering component
#[derive(Debug, Clone, Copy)]
pub struct SpriteRender {
    pub width: f32,
    pub height: f32,
}

impl SpriteRender {
    /// Create a new sprite with given size
    #[must_use]
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Get size as Vec2
    #[must_use]
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

/// Collider component for collision detection
#[derive(Debug, Clone, Copy)]
pub struct Collider {
    /// Half-size of the collision box
    pub half_width: f32,
    pub half_height: f32,
}

impl Collider {
    /// Create a collider with given full size
    #[must_use]
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            half_width: width * 0.5,
            half_height: height * 0.5,
        }
    }

    /// Get half-size as Vec2
    #[must_use]
    pub fn half_size(&self) -> Vec2 {
        Vec2::new(self.half_width, self.half_height)
    }
}
