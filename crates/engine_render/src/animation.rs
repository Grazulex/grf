//! Sprite animation system
//!
//! Provides frame-based animations with timing and state management.

use std::collections::HashMap;

use crate::SpriteRegion;

/// A single animation frame
#[derive(Debug, Clone)]
pub struct AnimationFrame {
    /// The sprite region for this frame
    pub region: SpriteRegion,
    /// Duration of this frame in seconds
    pub duration: f32,
}

impl AnimationFrame {
    /// Create a new animation frame
    #[must_use]
    pub fn new(region: SpriteRegion, duration: f32) -> Self {
        Self { region, duration }
    }
}

/// An animation sequence
#[derive(Debug, Clone)]
pub struct Animation {
    /// Name of the animation
    pub name: String,
    /// Frames in the animation
    pub frames: Vec<AnimationFrame>,
    /// Whether the animation loops
    pub looping: bool,
}

impl Animation {
    /// Create a new animation
    #[must_use]
    pub fn new(name: &str, looping: bool) -> Self {
        Self {
            name: name.to_string(),
            frames: Vec::new(),
            looping,
        }
    }

    /// Add a frame to the animation
    pub fn add_frame(&mut self, region: SpriteRegion, duration: f32) -> &mut Self {
        self.frames.push(AnimationFrame::new(region, duration));
        self
    }

    /// Create animation with uniform frame duration
    #[must_use]
    pub fn from_regions(name: &str, regions: Vec<SpriteRegion>, frame_duration: f32, looping: bool) -> Self {
        let frames = regions
            .into_iter()
            .map(|r| AnimationFrame::new(r, frame_duration))
            .collect();
        Self {
            name: name.to_string(),
            frames,
            looping,
        }
    }

    /// Get total duration of the animation
    #[must_use]
    pub fn total_duration(&self) -> f32 {
        self.frames.iter().map(|f| f.duration).sum()
    }

    /// Get the frame at a given time
    #[must_use]
    pub fn frame_at(&self, time: f32) -> Option<&AnimationFrame> {
        if self.frames.is_empty() {
            return None;
        }

        let total = self.total_duration();
        if total <= 0.0 {
            return self.frames.first();
        }

        // Handle looping
        let effective_time = if self.looping {
            time % total
        } else {
            time.min(total - 0.001)
        };

        // Find the frame at this time
        let mut elapsed = 0.0;
        for frame in &self.frames {
            elapsed += frame.duration;
            if effective_time < elapsed {
                return Some(frame);
            }
        }

        self.frames.last()
    }

    /// Check if the animation has finished (non-looping only)
    #[must_use]
    pub fn is_finished(&self, time: f32) -> bool {
        !self.looping && time >= self.total_duration()
    }
}

/// Animation controller for managing multiple animations
#[derive(Debug)]
pub struct AnimationController {
    /// Available animations by name
    animations: HashMap<String, Animation>,
    /// Currently playing animation name
    current: Option<String>,
    /// Time elapsed in current animation
    time: f32,
    /// Playback speed multiplier
    pub speed: f32,
}

impl Default for AnimationController {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationController {
    /// Create a new animation controller
    #[must_use]
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            current: None,
            time: 0.0,
            speed: 1.0,
        }
    }

    /// Add an animation to the controller
    pub fn add(&mut self, animation: Animation) -> &mut Self {
        self.animations.insert(animation.name.clone(), animation);
        self
    }

    /// Play an animation by name
    pub fn play(&mut self, name: &str) {
        if self.current.as_deref() != Some(name) && self.animations.contains_key(name) {
            self.current = Some(name.to_string());
            self.time = 0.0;
        }
    }

    /// Play animation only if not already playing (doesn't reset)
    pub fn play_if_different(&mut self, name: &str) {
        if self.current.as_deref() != Some(name) {
            self.play(name);
        }
    }

    /// Update the animation (call each frame with delta time)
    pub fn update(&mut self, dt: f32) {
        self.time += dt * self.speed;
    }

    /// Get the current animation
    #[must_use]
    pub fn current_animation(&self) -> Option<&Animation> {
        self.current.as_ref().and_then(|name| self.animations.get(name))
    }

    /// Get the current frame's sprite region
    #[must_use]
    pub fn current_region(&self) -> Option<SpriteRegion> {
        self.current_animation()
            .and_then(|anim| anim.frame_at(self.time))
            .map(|frame| frame.region)
    }

    /// Check if current animation is finished
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.current_animation()
            .map(|anim| anim.is_finished(self.time))
            .unwrap_or(true)
    }

    /// Get current animation name
    #[must_use]
    pub fn current_name(&self) -> Option<&str> {
        self.current.as_deref()
    }

    /// Reset current animation to start
    pub fn reset(&mut self) {
        self.time = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_region(id: u32) -> SpriteRegion {
        SpriteRegion {
            u_min: id as f32 * 0.25,
            v_min: 0.0,
            u_max: (id + 1) as f32 * 0.25,
            v_max: 0.25,
        }
    }

    #[test]
    fn test_animation_frame_at() {
        let anim = Animation::from_regions(
            "walk",
            vec![test_region(0), test_region(1), test_region(2), test_region(3)],
            0.1,
            true,
        );

        // Frame 0 at t=0
        let frame = anim.frame_at(0.0).unwrap();
        assert!((frame.region.u_min - 0.0).abs() < 0.001);

        // Frame 1 at t=0.1
        let frame = anim.frame_at(0.1).unwrap();
        assert!((frame.region.u_min - 0.25).abs() < 0.001);

        // Frame 2 at t=0.25
        let frame = anim.frame_at(0.25).unwrap();
        assert!((frame.region.u_min - 0.5).abs() < 0.001);

        // Looping: t=0.4 should be frame 0 again
        let frame = anim.frame_at(0.4).unwrap();
        assert!((frame.region.u_min - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_animation_non_looping() {
        let anim = Animation::from_regions(
            "attack",
            vec![test_region(0), test_region(1)],
            0.2,
            false,
        );

        assert!(!anim.is_finished(0.0));
        assert!(!anim.is_finished(0.3));
        assert!(anim.is_finished(0.4));
        assert!(anim.is_finished(1.0));
    }

    #[test]
    fn test_controller_play() {
        let mut ctrl = AnimationController::new();
        ctrl.add(Animation::from_regions("idle", vec![test_region(0)], 1.0, true));
        ctrl.add(Animation::from_regions("walk", vec![test_region(1), test_region(2)], 0.1, true));

        ctrl.play("idle");
        assert_eq!(ctrl.current_name(), Some("idle"));

        ctrl.play("walk");
        assert_eq!(ctrl.current_name(), Some("walk"));

        // play_if_different shouldn't reset
        ctrl.update(0.05);
        ctrl.play_if_different("walk");
        // Time should still be ~0.05, not reset
    }

    #[test]
    fn test_controller_current_region() {
        let mut ctrl = AnimationController::new();
        ctrl.add(Animation::from_regions(
            "test",
            vec![test_region(0), test_region(1)],
            0.1,
            true,
        ));

        ctrl.play("test");

        // Frame 0
        let region = ctrl.current_region().unwrap();
        assert!((region.u_min - 0.0).abs() < 0.001);

        // After update, frame 1
        ctrl.update(0.1);
        let region = ctrl.current_region().unwrap();
        assert!((region.u_min - 0.25).abs() < 0.001);
    }
}
