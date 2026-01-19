//! Save/Load system for game state persistence
//!
//! Provides serialization and deserialization of the complete game state
//! with support for multiple save slots and version compatibility.

mod save_data;
mod save_manager;

pub use save_data::{GameClockData, PlayerData, SaveData, SAVE_VERSION};
pub use save_manager::{SaveManager, SaveSlotInfo, MAX_SAVE_SLOTS};
