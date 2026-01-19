//! Engine Core - Game loop, time management, settings and lifecycle
//!
//! This crate provides the core game loop with fixed timestep update
//! and variable render rate, plus game settings management.

mod game_clock;
mod settings;
mod time;

pub use game_clock::{
    DayOfWeek, GameClock, Season, TimeOfDay, DAYS_PER_SEASON, HOURS_PER_DAY, MINUTES_PER_HOUR,
    SEASONS_PER_YEAR,
};
pub use settings::{
    AudioSettings, GameSettings, GameplaySettings, SettingEntry, SettingValue, VideoSettings,
};
pub use time::{GameTime, FIXED_TIMESTEP};

/// Fixed update rate: 60 updates per second
pub const UPDATES_PER_SECOND: u32 = 60;
