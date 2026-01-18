//! Engine Render - 2D rendering pipeline
//!
//! This crate provides sprite rendering, tilemaps, and camera management
//! using wgpu for GPU abstraction.

mod camera;
mod renderer;
mod sprite;
mod texture;

pub use camera::Camera2D;
pub use glam;
pub use renderer::Renderer;
pub use sprite::{Sprite, SpriteBatch, SpriteRegion, SpriteVertex};
pub use texture::Texture;
pub use wgpu;

/// Default clear color (dark blue)
pub const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.1,
    g: 0.1,
    b: 0.2,
    a: 1.0,
};
