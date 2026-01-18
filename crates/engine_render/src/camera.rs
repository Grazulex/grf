//! 2D Camera with transformations and smooth follow

use glam::{Mat4, Vec2};

/// 2D Camera for world-to-screen transformations
#[derive(Debug, Clone)]
pub struct Camera2D {
    /// Camera position in world coordinates (center of view)
    position: Vec2,
    /// Zoom level (1.0 = 1:1 pixels, 2.0 = 2x zoom in)
    zoom: f32,
    /// Viewport size in pixels
    viewport: Vec2,
    /// Smooth follow target (if any)
    follow_target: Option<Vec2>,
    /// Smooth follow speed (0.0 = instant, higher = slower)
    follow_smoothness: f32,
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            viewport: Vec2::new(1280.0, 720.0),
            follow_target: None,
            follow_smoothness: 5.0,
        }
    }
}

impl Camera2D {
    /// Create a new camera centered at origin
    #[must_use]
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            viewport: Vec2::new(viewport_width, viewport_height),
            ..Default::default()
        }
    }

    /// Get camera position
    #[must_use]
    pub fn position(&self) -> Vec2 {
        self.position
    }

    /// Set camera position directly
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.follow_target = None;
    }

    /// Get current zoom level
    #[must_use]
    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    /// Set zoom level (clamped to 0.5..4.0)
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.5, 4.0);
    }

    /// Get viewport size
    #[must_use]
    pub fn viewport(&self) -> Vec2 {
        self.viewport
    }

    /// Update viewport size (call on window resize)
    pub fn set_viewport(&mut self, width: f32, height: f32) {
        self.viewport = Vec2::new(width, height);
    }

    /// Set a target to follow smoothly
    pub fn follow(&mut self, target: Vec2, smoothness: f32) {
        self.follow_target = Some(target);
        self.follow_smoothness = smoothness.max(0.1);
    }

    /// Stop following target
    pub fn stop_follow(&mut self) {
        self.follow_target = None;
    }

    /// Update camera (call each frame for smooth follow)
    pub fn update(&mut self, dt: f32) {
        if let Some(target) = self.follow_target {
            // Exponential smoothing: position = lerp(position, target, 1 - e^(-speed * dt))
            let t = 1.0 - (-self.follow_smoothness * dt).exp();
            self.position = self.position.lerp(target, t);
        }
    }

    /// Transform world coordinates to screen coordinates
    #[must_use]
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        // 1. Translate relative to camera
        let relative = world_pos - self.position;
        // 2. Apply zoom
        let zoomed = relative * self.zoom;
        // 3. Offset to screen center
        zoomed + self.viewport * 0.5
    }

    /// Transform screen coordinates to world coordinates
    #[must_use]
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        // Inverse of world_to_screen
        // 1. Offset from screen center
        let centered = screen_pos - self.viewport * 0.5;
        // 2. Remove zoom
        let unzoomed = centered / self.zoom;
        // 3. Translate to world
        unzoomed + self.position
    }

    /// Get the visible world bounds (for culling)
    /// Returns (min, max) corners in world coordinates
    #[must_use]
    pub fn visible_bounds(&self) -> (Vec2, Vec2) {
        let half_size = self.viewport * 0.5 / self.zoom;
        let min = self.position - half_size;
        let max = self.position + half_size;
        (min, max)
    }

    /// Check if a point is visible
    #[must_use]
    pub fn is_point_visible(&self, world_pos: Vec2) -> bool {
        let (min, max) = self.visible_bounds();
        world_pos.x >= min.x && world_pos.x <= max.x
            && world_pos.y >= min.y && world_pos.y <= max.y
    }

    /// Check if a rectangle is visible (AABB intersection)
    #[must_use]
    pub fn is_rect_visible(&self, rect_min: Vec2, rect_max: Vec2) -> bool {
        let (cam_min, cam_max) = self.visible_bounds();
        rect_max.x >= cam_min.x && rect_min.x <= cam_max.x
            && rect_max.y >= cam_min.y && rect_min.y <= cam_max.y
    }

    /// Get the view matrix for GPU rendering
    /// This transforms world coordinates to normalized device coordinates (-1 to 1)
    #[must_use]
    pub fn view_matrix(&self) -> Mat4 {
        // Scale by zoom and normalize to NDC
        let scale_x = 2.0 * self.zoom / self.viewport.x;
        let scale_y = 2.0 * self.zoom / self.viewport.y;

        // Translate camera position
        let translate_x = -self.position.x * scale_x;
        let translate_y = -self.position.y * scale_y;

        Mat4::from_cols_array(&[
            scale_x, 0.0, 0.0, 0.0,
            0.0, -scale_y, 0.0, 0.0, // Negative Y for screen coordinates (Y down)
            0.0, 0.0, 1.0, 0.0,
            translate_x, -translate_y, 0.0, 1.0,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_to_screen_center() {
        let camera = Camera2D::new(800.0, 600.0);
        // Camera at origin, zoom 1.0
        // World origin should be at screen center
        let screen = camera.world_to_screen(Vec2::ZERO);
        assert_eq!(screen, Vec2::new(400.0, 300.0));
    }

    #[test]
    fn test_screen_to_world_inverse() {
        let mut camera = Camera2D::new(800.0, 600.0);
        camera.set_position(Vec2::new(100.0, 50.0));
        camera.set_zoom(2.0);

        let world_pos = Vec2::new(150.0, 75.0);
        let screen_pos = camera.world_to_screen(world_pos);
        let back_to_world = camera.screen_to_world(screen_pos);

        assert!((world_pos - back_to_world).length() < 0.001);
    }

    #[test]
    fn test_visible_bounds_zoom() {
        let mut camera = Camera2D::new(800.0, 600.0);

        // Zoom 1.0: visible area = 800x600
        let (min, max) = camera.visible_bounds();
        assert_eq!(min, Vec2::new(-400.0, -300.0));
        assert_eq!(max, Vec2::new(400.0, 300.0));

        // Zoom 2.0: visible area = 400x300
        camera.set_zoom(2.0);
        let (min, max) = camera.visible_bounds();
        assert_eq!(min, Vec2::new(-200.0, -150.0));
        assert_eq!(max, Vec2::new(200.0, 150.0));
    }

    #[test]
    fn test_zoom_clamp() {
        let mut camera = Camera2D::new(800.0, 600.0);

        camera.set_zoom(0.1);
        assert_eq!(camera.zoom(), 0.5);

        camera.set_zoom(10.0);
        assert_eq!(camera.zoom(), 4.0);
    }
}
