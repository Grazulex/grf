//! Main input state management

use glam::Vec2;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::PhysicalKey;

use crate::keyboard::{KeyCode, KeyboardState};

/// Main input state that tracks all input devices
#[derive(Debug, Default)]
pub struct Input {
    keyboard: KeyboardState,
}

impl Input {
    /// Create a new input state
    pub fn new() -> Self {
        Self::default()
    }

    /// Handle a keyboard event from winit
    pub fn on_keyboard_event(&mut self, event: &KeyEvent) {
        if let PhysicalKey::Code(key_code) = event.physical_key {
            let key = KeyCode::from(key_code);
            match event.state {
                ElementState::Pressed => self.keyboard.on_key_pressed(key),
                ElementState::Released => self.keyboard.on_key_released(key),
            }
        }
    }

    /// Update at the end of each frame
    pub fn end_frame(&mut self) {
        self.keyboard.end_frame();
    }

    /// Check if a key is currently pressed
    #[must_use]
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_pressed(key)
    }

    /// Check if a key was just pressed this frame
    #[must_use]
    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_just_pressed(key)
    }

    /// Check if a key was just released this frame
    #[must_use]
    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.keyboard.is_just_released(key)
    }

    /// Get movement direction from arrow keys or WASD
    #[must_use]
    pub fn get_movement_direction(&self) -> Vec2 {
        let mut dir = Vec2::ZERO;

        if self.is_key_pressed(KeyCode::W) || self.is_key_pressed(KeyCode::Up) {
            dir.y -= 1.0;
        }
        if self.is_key_pressed(KeyCode::S) || self.is_key_pressed(KeyCode::Down) {
            dir.y += 1.0;
        }
        if self.is_key_pressed(KeyCode::A) || self.is_key_pressed(KeyCode::Left) {
            dir.x -= 1.0;
        }
        if self.is_key_pressed(KeyCode::D) || self.is_key_pressed(KeyCode::Right) {
            dir.x += 1.0;
        }

        // Normalize diagonal movement
        if dir.length_squared() > 0.0 {
            dir = dir.normalize();
        }

        dir
    }

    /// Get a reference to the keyboard state
    #[must_use]
    pub fn keyboard(&self) -> &KeyboardState {
        &self.keyboard
    }
}
