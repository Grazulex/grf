//! Engine UI - In-game UI system
//!
//! This crate provides basic UI widgets for in-game interfaces:
//! - HUD (health, stamina, time, hotbar)
//! - Progress bars
//! - Inventory display (future)
//! - Dialogue boxes (future)
//! - Menus (future)

mod hud;

pub use hud::{colors, Hotbar, HotbarSlot, Hud, ProgressBar, TimeDisplay};

/// UI layer z-order (rendered on top)
pub const UI_Z_ORDER: i32 = 100;
