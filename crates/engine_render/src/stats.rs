//! Render statistics for profiling

/// Statistics collected during rendering
#[derive(Debug, Clone, Default)]
pub struct RenderStats {
    /// Number of sprites rendered this frame
    pub sprites: usize,
    /// Number of vertices submitted this frame
    pub vertices: usize,
    /// Number of draw calls this frame
    pub draw_calls: usize,
    /// Number of texture binds this frame
    pub texture_binds: usize,
}

impl RenderStats {
    /// Create new empty stats
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset stats for a new frame
    pub fn reset(&mut self) {
        self.sprites = 0;
        self.vertices = 0;
        self.draw_calls = 0;
        self.texture_binds = 0;
    }

    /// Record a draw call with sprite count
    pub fn record_draw(&mut self, sprites: usize) {
        self.sprites += sprites;
        self.vertices += sprites * 4; // 4 vertices per sprite
        self.draw_calls += 1;
    }

    /// Record a texture bind
    pub fn record_texture_bind(&mut self) {
        self.texture_binds += 1;
    }
}
