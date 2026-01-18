//! Keyboard input handling

use std::collections::HashMap;
use winit::keyboard::KeyCode as WinitKeyCode;

use crate::ButtonState;

/// Keyboard key codes (subset of common keys)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // Numbers
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,

    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    // Arrow keys
    Up, Down, Left, Right,

    // Modifiers
    LShift, RShift, LCtrl, RCtrl, LAlt, RAlt,

    // Common keys
    Space, Enter, Escape, Tab, Backspace,

    // Unknown/unsupported
    Unknown,
}

impl From<WinitKeyCode> for KeyCode {
    fn from(key: WinitKeyCode) -> Self {
        match key {
            // Letters
            WinitKeyCode::KeyA => Self::A,
            WinitKeyCode::KeyB => Self::B,
            WinitKeyCode::KeyC => Self::C,
            WinitKeyCode::KeyD => Self::D,
            WinitKeyCode::KeyE => Self::E,
            WinitKeyCode::KeyF => Self::F,
            WinitKeyCode::KeyG => Self::G,
            WinitKeyCode::KeyH => Self::H,
            WinitKeyCode::KeyI => Self::I,
            WinitKeyCode::KeyJ => Self::J,
            WinitKeyCode::KeyK => Self::K,
            WinitKeyCode::KeyL => Self::L,
            WinitKeyCode::KeyM => Self::M,
            WinitKeyCode::KeyN => Self::N,
            WinitKeyCode::KeyO => Self::O,
            WinitKeyCode::KeyP => Self::P,
            WinitKeyCode::KeyQ => Self::Q,
            WinitKeyCode::KeyR => Self::R,
            WinitKeyCode::KeyS => Self::S,
            WinitKeyCode::KeyT => Self::T,
            WinitKeyCode::KeyU => Self::U,
            WinitKeyCode::KeyV => Self::V,
            WinitKeyCode::KeyW => Self::W,
            WinitKeyCode::KeyX => Self::X,
            WinitKeyCode::KeyY => Self::Y,
            WinitKeyCode::KeyZ => Self::Z,

            // Numbers
            WinitKeyCode::Digit0 => Self::Key0,
            WinitKeyCode::Digit1 => Self::Key1,
            WinitKeyCode::Digit2 => Self::Key2,
            WinitKeyCode::Digit3 => Self::Key3,
            WinitKeyCode::Digit4 => Self::Key4,
            WinitKeyCode::Digit5 => Self::Key5,
            WinitKeyCode::Digit6 => Self::Key6,
            WinitKeyCode::Digit7 => Self::Key7,
            WinitKeyCode::Digit8 => Self::Key8,
            WinitKeyCode::Digit9 => Self::Key9,

            // Function keys
            WinitKeyCode::F1 => Self::F1,
            WinitKeyCode::F2 => Self::F2,
            WinitKeyCode::F3 => Self::F3,
            WinitKeyCode::F4 => Self::F4,
            WinitKeyCode::F5 => Self::F5,
            WinitKeyCode::F6 => Self::F6,
            WinitKeyCode::F7 => Self::F7,
            WinitKeyCode::F8 => Self::F8,
            WinitKeyCode::F9 => Self::F9,
            WinitKeyCode::F10 => Self::F10,
            WinitKeyCode::F11 => Self::F11,
            WinitKeyCode::F12 => Self::F12,

            // Arrow keys
            WinitKeyCode::ArrowUp => Self::Up,
            WinitKeyCode::ArrowDown => Self::Down,
            WinitKeyCode::ArrowLeft => Self::Left,
            WinitKeyCode::ArrowRight => Self::Right,

            // Modifiers
            WinitKeyCode::ShiftLeft => Self::LShift,
            WinitKeyCode::ShiftRight => Self::RShift,
            WinitKeyCode::ControlLeft => Self::LCtrl,
            WinitKeyCode::ControlRight => Self::RCtrl,
            WinitKeyCode::AltLeft => Self::LAlt,
            WinitKeyCode::AltRight => Self::RAlt,

            // Common keys
            WinitKeyCode::Space => Self::Space,
            WinitKeyCode::Enter => Self::Enter,
            WinitKeyCode::Escape => Self::Escape,
            WinitKeyCode::Tab => Self::Tab,
            WinitKeyCode::Backspace => Self::Backspace,

            _ => Self::Unknown,
        }
    }
}

/// Tracks the state of all keyboard keys
#[derive(Debug, Default)]
pub struct KeyboardState {
    keys: HashMap<KeyCode, ButtonState>,
    /// Keys that changed this frame (for end_frame cleanup)
    changed_keys: Vec<KeyCode>,
}

impl KeyboardState {
    /// Create a new keyboard state
    pub fn new() -> Self {
        Self::default()
    }

    /// Handle a key press event
    pub fn on_key_pressed(&mut self, key: KeyCode) {
        if key == KeyCode::Unknown {
            return;
        }

        let state = self.keys.entry(key).or_insert(ButtonState::Released);
        if !state.is_pressed() {
            *state = ButtonState::JustPressed;
            self.changed_keys.push(key);
        }
    }

    /// Handle a key release event
    pub fn on_key_released(&mut self, key: KeyCode) {
        if key == KeyCode::Unknown {
            return;
        }

        let state = self.keys.entry(key).or_insert(ButtonState::Released);
        if state.is_pressed() {
            *state = ButtonState::JustReleased;
            self.changed_keys.push(key);
        }
    }

    /// Update states at the end of each frame
    /// Transitions JustPressed -> Held and JustReleased -> Released
    pub fn end_frame(&mut self) {
        for key in self.changed_keys.drain(..) {
            if let Some(state) = self.keys.get_mut(&key) {
                match *state {
                    ButtonState::JustPressed => *state = ButtonState::Held,
                    ButtonState::JustReleased => *state = ButtonState::Released,
                    _ => {}
                }
            }
        }
    }

    /// Check if a key is currently pressed
    #[must_use]
    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.keys.get(&key).is_some_and(|s| s.is_pressed())
    }

    /// Check if a key was just pressed this frame
    #[must_use]
    pub fn is_just_pressed(&self, key: KeyCode) -> bool {
        self.keys.get(&key).is_some_and(|s| s.is_just_pressed())
    }

    /// Check if a key was just released this frame
    #[must_use]
    pub fn is_just_released(&self, key: KeyCode) -> bool {
        self.keys.get(&key).is_some_and(|s| s.is_just_released())
    }

    /// Get the state of a key
    #[must_use]
    pub fn get_state(&self, key: KeyCode) -> ButtonState {
        self.keys.get(&key).copied().unwrap_or_default()
    }
}
