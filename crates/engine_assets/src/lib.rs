//! Engine Assets - Asset loading and caching
//!
//! This crate provides asset management with:
//! - Async loading
//! - Handle-based references
//! - Caching and hot reloading (dev mode)

use thiserror::Error;

/// Asset loading errors
#[derive(Error, Debug)]
pub enum AssetError {
    /// File not found
    #[error("Asset not found: {0}")]
    NotFound(String),
    /// Failed to load asset
    #[error("Failed to load asset: {0}")]
    LoadFailed(String),
    /// Invalid asset format
    #[error("Invalid asset format: {0}")]
    InvalidFormat(String),
}

/// Handle to a loaded asset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetHandle {
    /// Unique identifier for the asset
    pub id: u32,
}

impl AssetHandle {
    /// Create a new asset handle
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self { id }
    }
}
