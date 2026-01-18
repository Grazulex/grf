//! Engine Debug - Debug tools and overlays
//!
//! This crate provides debug visualization and tools:
//! - Collision box overlay
//! - Z-order visualization
//! - Entity inspector
//! - Performance profiler
//! - Debug console
//!
//! All features are gated behind the `debug-tools` feature.

#[cfg(feature = "debug-tools")]
mod egui_renderer;
#[cfg(feature = "debug-tools")]
mod overlay;

#[cfg(feature = "debug-tools")]
pub use egui_renderer::EguiRenderer;
#[cfg(feature = "debug-tools")]
pub use overlay::{DebugOverlay, PanelState};

/// Debug overlay toggle key
pub const DEBUG_TOGGLE_KEY: &str = "F12";

/// Debug panel configuration
#[derive(Debug, Default)]
pub struct DebugConfig {
    /// Master toggle for all debug features
    pub enabled: bool,
    /// Show collision boxes (Ctrl+C)
    pub show_collisions: bool,
    /// Show z-order labels (Ctrl+Z)
    pub show_z_order: bool,
    /// Show tile grid (Ctrl+G)
    pub show_grid: bool,
    /// Show FPS counter
    pub show_fps: bool,
}

impl DebugConfig {
    /// Create a new debug config with defaults
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Toggle master debug mode
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}
