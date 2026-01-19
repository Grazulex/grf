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
    /// Settings menu (accessible from main menu or pause menu)
    Settings,
}

/// Track where we came from to return to the right state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviousState {
    MainMenu,
    Paused,
}

impl Default for GameState {
    fn default() -> Self {
        Self::MainMenu
    }
}
