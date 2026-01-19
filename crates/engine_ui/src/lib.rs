//! Engine UI - In-game UI system
//!
//! This crate provides basic UI widgets for in-game interfaces:
//! - HUD (health, stamina, time, hotbar)
//! - Progress bars
//! - Menus (main menu, pause menu)
//! - Inventory display (future)
//! - Dialogue boxes (future)

mod hud;
mod menu;
mod settings_menu;

pub use hud::{colors, Hotbar, HotbarSlot, Hud, ProgressBar, TimeDisplay};
pub use menu::{Menu, MenuItem, MenuStyle, presets as menu_presets};
pub use settings_menu::{SettingsMenu, presets as settings_presets};

/// UI layer z-order (rendered on top)
pub const UI_Z_ORDER: i32 = 100;
