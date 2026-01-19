//! Game state management
//!
//! Handles game state transitions (menu, playing, paused).

/// Game state enum for managing different screens
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// Main menu screen
    MainMenu,
    /// Actively playing the game
    Playing,
    /// Game is paused (pause menu shown)
    Paused,
}

impl Default for GameState {
    fn default() -> Self {
        Self::MainMenu
    }
}
