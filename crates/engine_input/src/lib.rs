//! Engine Input - Input abstraction layer
//!
//! This crate provides abstract input handling with support for
//! keyboard, mouse, and gamepad with configurable mappings.

mod input;
mod keyboard;

pub use glam::Vec2;
pub use input::Input;
pub use keyboard::{KeyCode, KeyboardState};

/// Button state for input tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonState {
    /// Button is not pressed
    #[default]
    Released,
    /// Button was just pressed this frame
    JustPressed,
    /// Button is being held down
    Held,
    /// Button was just released this frame
    JustReleased,
}

impl ButtonState {
    /// Check if the button is currently pressed (JustPressed or Held)
    #[must_use]
    pub fn is_pressed(self) -> bool {
        matches!(self, Self::JustPressed | Self::Held)
    }

    /// Check if the button was just pressed this frame
    #[must_use]
    pub fn is_just_pressed(self) -> bool {
        matches!(self, Self::JustPressed)
    }

    /// Check if the button was just released this frame
    #[must_use]
    pub fn is_just_released(self) -> bool {
        matches!(self, Self::JustReleased)
    }
}
