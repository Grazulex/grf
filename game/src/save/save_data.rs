//! Save data structures for game state serialization

use engine_core::GameClock;
use engine_render::glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::components::{Collider, PlayerControlled, Position, SpriteRender, Velocity};
use crate::inventory::Inventory;

/// Current save file version
pub const SAVE_VERSION: u32 = 1;

/// Complete save file data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    /// Save file version for compatibility
    pub version: u32,
    /// Player state
    pub player: PlayerData,
    /// Game clock state
    pub game_clock: GameClockData,
    /// Current map path
    pub current_map: String,
    /// Player inventory
    pub inventory: Inventory,
}

impl SaveData {
    /// Create a new save data with current version
    pub fn new(
        player: PlayerData,
        game_clock: GameClockData,
        current_map: String,
        inventory: Inventory,
    ) -> Self {
        Self {
            version: SAVE_VERSION,
            player,
            game_clock,
            current_map,
            inventory,
        }
    }

    /// Check if save version is compatible
    pub fn is_compatible(&self) -> bool {
        self.version == SAVE_VERSION
    }
}

/// Player entity state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    /// Position in world
    pub position: Vec2,
    /// Movement speed
    pub speed: f32,
    /// Sprite size
    pub sprite_size: (f32, f32),
    /// Collider size
    pub collider_size: (f32, f32),
}

impl PlayerData {
    /// Create from ECS components
    pub fn from_components(
        position: &Position,
        player_ctrl: &PlayerControlled,
        sprite: &SpriteRender,
        collider: &Collider,
    ) -> Self {
        Self {
            position: position.current,
            speed: player_ctrl.speed,
            sprite_size: (sprite.width, sprite.height),
            collider_size: (collider.half_width * 2.0, collider.half_height * 2.0),
        }
    }

    /// Create Position component from save data
    pub fn to_position(&self) -> Position {
        Position::from_vec2(self.position)
    }

    /// Create PlayerControlled component from save data
    pub fn to_player_controlled(&self) -> PlayerControlled {
        PlayerControlled::new(self.speed)
    }

    /// Create SpriteRender component from save data
    pub fn to_sprite_render(&self) -> SpriteRender {
        SpriteRender::new(self.sprite_size.0, self.sprite_size.1)
    }

    /// Create Collider component from save data
    pub fn to_collider(&self) -> Collider {
        Collider::new(self.collider_size.0, self.collider_size.1)
    }

    /// Create Velocity component (zeroed)
    pub fn to_velocity(&self) -> Velocity {
        Velocity::default()
    }
}

/// Game clock state (subset of GameClock for saving)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameClockData {
    pub minute: u32,
    pub hour: u32,
    pub day: u32,
    pub season: String,
    pub year: u32,
}

impl GameClockData {
    /// Create from GameClock
    pub fn from_game_clock(clock: &GameClock) -> Self {
        Self {
            minute: clock.minute(),
            hour: clock.hour(),
            day: clock.day(),
            season: clock.season().name().to_string(),
            year: clock.year(),
        }
    }
}
