//! Engine Window - Window creation and event handling
//!
//! This crate wraps winit to provide cross-platform window management.

mod window;

pub use window::{App, Window, WindowConfig};
pub use winit;

/// Default window width
pub const DEFAULT_WIDTH: u32 = 1280;
/// Default window height
pub const DEFAULT_HEIGHT: u32 = 720;
/// Default window title
pub const DEFAULT_TITLE: &str = "GRF Game";
