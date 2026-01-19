//! Engine Render - 2D rendering pipeline
//!
//! This crate provides sprite rendering, tilemaps, and camera management
//! using wgpu for GPU abstraction.

mod animation;
mod camera;
mod character;
mod day_night;
mod renderer;
mod sprite;
mod stats;
mod texture;
mod tilemap;

pub use animation::{Animation, AnimationController, AnimationFrame};
pub use camera::Camera2D;
pub use character::{
    CharacterAnimator, CharacterConfig, CharacterLoadError, CharacterState, Direction,
    MovementConfig, SpriteSheetConfig,
};
pub use day_night::{Color, DayNightCycle};
pub use glam;
pub use renderer::{Frame, Renderer};
pub use sprite::{Sprite, SpriteBatch, SpriteRegion, SpriteVertex};
pub use stats::RenderStats;
pub use texture::Texture;
pub use tilemap::{LayerType, SpawnPoint, TileLayer, Tilemap, TilemapError, Tileset, Trigger};
pub use wgpu;

/// Default clear color (dark blue)
pub const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.1,
    g: 0.1,
    b: 0.2,
    a: 1.0,
};
