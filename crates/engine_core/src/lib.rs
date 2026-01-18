//! Engine Core - Game loop, time management and lifecycle
//!
//! This crate provides the core game loop with fixed timestep update
//! and variable render rate.

mod time;

pub use time::{GameTime, FIXED_TIMESTEP};

/// Fixed update rate: 60 updates per second
pub const UPDATES_PER_SECOND: u32 = 60;
