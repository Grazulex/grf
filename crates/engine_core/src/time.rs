//! Time management for the game loop

use instant::Instant;

/// Fixed timestep for game logic (60 updates per second)
pub const FIXED_TIMESTEP: f64 = 1.0 / 60.0;

/// Maximum delta time to prevent spiral of death
const MAX_DELTA: f64 = 0.25;

/// Tracks game time and frame information
#[derive(Debug)]
pub struct GameTime {
    /// Time since last frame in seconds
    pub delta: f64,
    /// Total elapsed time since start in seconds
    pub total: f64,
    /// Current frame number (render frames)
    pub frame_count: u64,
    /// Current fixed update number
    pub update_count: u64,
    /// Accumulator for fixed timestep
    pub accumulator: f64,
    /// Last frame instant
    last_instant: Instant,
    // FPS/UPS tracking
    fps_timer: f64,
    fps_frame_count: u32,
    ups_update_count: u32,
    /// Smoothed FPS (updated every second)
    current_fps: f64,
    /// Smoothed UPS (updated every second)
    current_ups: f64,
}

impl Default for GameTime {
    fn default() -> Self {
        Self::new()
    }
}

impl GameTime {
    /// Create a new GameTime instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            delta: 0.0,
            total: 0.0,
            frame_count: 0,
            update_count: 0,
            accumulator: 0.0,
            last_instant: Instant::now(),
            fps_timer: 0.0,
            fps_frame_count: 0,
            ups_update_count: 0,
            current_fps: 0.0,
            current_ups: 0.0,
        }
    }

    /// Update time at the start of each frame
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = (now - self.last_instant).as_secs_f64();
        self.last_instant = now;

        // Clamp delta to prevent spiral of death
        if self.delta > MAX_DELTA {
            self.delta = MAX_DELTA;
        }

        self.total += self.delta;
        self.frame_count += 1;
        self.accumulator += self.delta;

        // Track FPS
        self.fps_frame_count += 1;
        self.fps_timer += self.delta;

        // Update FPS/UPS counters every second
        if self.fps_timer >= 1.0 {
            self.current_fps = f64::from(self.fps_frame_count) / self.fps_timer;
            self.current_ups = f64::from(self.ups_update_count) / self.fps_timer;
            self.fps_frame_count = 0;
            self.ups_update_count = 0;
            self.fps_timer = 0.0;
        }
    }

    /// Check if a fixed update should run and consume time from accumulator
    pub fn should_fixed_update(&mut self) -> bool {
        if self.accumulator >= FIXED_TIMESTEP {
            self.accumulator -= FIXED_TIMESTEP;
            self.update_count += 1;
            self.ups_update_count += 1;
            true
        } else {
            false
        }
    }

    /// Get interpolation alpha for rendering between fixed updates
    ///
    /// Returns a value between 0.0 and 1.0 representing progress
    /// between the last fixed update and the next one.
    /// Use this to interpolate positions for smooth rendering.
    #[must_use]
    pub fn alpha(&self) -> f64 {
        self.accumulator / FIXED_TIMESTEP
    }

    /// Get current FPS (frames per second) - smoothed over 1 second
    #[must_use]
    pub fn fps(&self) -> f64 {
        self.current_fps
    }

    /// Get current UPS (updates per second) - smoothed over 1 second
    #[must_use]
    pub fn ups(&self) -> f64 {
        self.current_ups
    }

    /// Get instantaneous FPS from last frame delta
    #[must_use]
    pub fn instant_fps(&self) -> f64 {
        if self.delta > 0.0 {
            1.0 / self.delta
        } else {
            0.0
        }
    }

    /// Get the fixed timestep value
    #[must_use]
    pub fn fixed_timestep(&self) -> f64 {
        FIXED_TIMESTEP
    }

    /// Get total time in seconds since game start
    #[must_use]
    pub fn total_time(&self) -> f64 {
        self.total
    }
}
