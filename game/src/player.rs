//! Player module
//!
//! Thin wrapper around CharacterAnimator for player-specific logic.
//! All animation configuration is loaded from assets/data/characters/player.toml

pub use engine_render::CharacterAnimator;

/// Load player animator from config file
pub fn load_player_animator() -> Result<CharacterAnimator, engine_render::CharacterLoadError> {
    CharacterAnimator::from_file("assets/data/characters/player.toml")
}
