//! Save/Load system for game state persistence
//!
//! Provides serialization and deserialization of the complete game state
//! with support for multiple save slots and version compatibility.

mod save_data;
mod save_manager;

pub use save_data::{GameClockData, PlayerData, SaveData};
pub use save_manager::SaveManager;

// Re-exports for future save menu implementation
#[allow(unused_imports)]
pub use save_data::SAVE_VERSION;
#[allow(unused_imports)]
pub use save_manager::{SaveSlotInfo, MAX_SAVE_SLOTS};
