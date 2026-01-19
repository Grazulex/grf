//! Character animation configuration loaded from TOML files
//!
//! This module provides data-driven character animations, allowing
//! artists to define sprite sheets and animations without code changes.

use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use crate::{Animation, AnimationController, SpriteRegion};

/// Character configuration loaded from TOML
#[derive(Debug, Deserialize)]
pub struct CharacterConfig {
    /// Movement parameters
    pub movement: MovementConfig,
    /// Sprite sheets by name
    pub spritesheets: HashMap<String, SpriteSheetConfig>,
    /// Animations by state name (idle, walk, run)
    pub animations: HashMap<String, AnimationConfig>,
}

/// Movement configuration
#[derive(Debug, Clone, Deserialize)]
pub struct MovementConfig {
    /// Base walking speed in pixels per second
    pub walk_speed: f32,
    /// Speed multiplier when running
    pub run_multiplier: f32,
}

/// Sprite sheet configuration
#[derive(Debug, Clone, Deserialize)]
pub struct SpriteSheetConfig {
    /// Path to the texture file (relative to assets/)
    pub path: String,
    /// Sheet size [width, height] in pixels
    pub size: [u32; 2],
    /// Size of each frame in pixels
    pub frame_size: u32,
}

/// Animation configuration
#[derive(Debug, Clone, Deserialize)]
pub struct AnimationConfig {
    /// Reference to spritesheet name
    pub sheet: String,
    /// Number of frames in the animation
    pub frames: u32,
    /// Frames per second
    pub fps: f32,
}

/// Direction for character facing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

impl Direction {
    /// Get the row index in the sprite sheet
    pub fn row(self) -> u32 {
        match self {
            Direction::Down => 0,
            Direction::Up => 1,
            Direction::Left | Direction::Right => 2,
        }
    }

    /// Should the sprite be flipped horizontally?
    pub fn flip_x(self) -> bool {
        matches!(self, Direction::Left)
    }

    /// Get direction suffix for animation names
    pub fn suffix(self) -> &'static str {
        match self {
            Direction::Down => "down",
            Direction::Up => "up",
            Direction::Left | Direction::Right => "side",
        }
    }

    /// Determine direction from velocity
    pub fn from_velocity(vx: f32, vy: f32) -> Option<Self> {
        if vx.abs() < 0.1 && vy.abs() < 0.1 {
            return None;
        }
        Some(if vx.abs() > vy.abs() {
            if vx < 0.0 { Direction::Left } else { Direction::Right }
        } else {
            if vy < 0.0 { Direction::Up } else { Direction::Down }
        })
    }
}

/// Character state (idle, walking, running)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CharacterState {
    #[default]
    Idle,
    Walking,
    Running,
}

impl CharacterState {
    /// Get the animation key for this state
    pub fn key(self) -> &'static str {
        match self {
            CharacterState::Idle => "idle",
            CharacterState::Walking => "walk",
            CharacterState::Running => "run",
        }
    }
}

/// Runtime character animation controller
/// Uses config to manage animations without hardcoded values
pub struct CharacterAnimator {
    /// Loaded configuration (public for texture loading)
    pub config: CharacterConfig,
    /// Animation controller
    pub controller: AnimationController,
    /// Current state
    pub state: CharacterState,
    /// Current direction
    pub direction: Direction,
    /// Should flip horizontally
    pub flip_x: bool,
}

impl CharacterAnimator {
    /// Load character from TOML config file
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, CharacterLoadError> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| CharacterLoadError::Io(e.to_string()))?;
        Self::from_toml(&content)
    }

    /// Parse character from TOML string
    pub fn from_toml(content: &str) -> Result<Self, CharacterLoadError> {
        let config: CharacterConfig = toml::from_str(content)
            .map_err(|e| CharacterLoadError::Parse(e.to_string()))?;

        let mut animator = Self {
            config,
            controller: AnimationController::new(),
            state: CharacterState::Idle,
            direction: Direction::Down,
            flip_x: false,
        };

        animator.build_animations()?;
        animator.controller.play("idle_down");

        Ok(animator)
    }

    /// Build all animations from config
    fn build_animations(&mut self) -> Result<(), CharacterLoadError> {
        for (state_name, anim_config) in &self.config.animations {
            let sheet = self.config.spritesheets.get(&anim_config.sheet)
                .ok_or_else(|| CharacterLoadError::MissingSheet(anim_config.sheet.clone()))?;

            let frame_duration = 1.0 / anim_config.fps;

            // Create animation for each direction (rows 0, 1, 2)
            for (dir_name, row) in [("down", 0), ("up", 1), ("side", 2)] {
                let anim_name = format!("{}_{}", state_name, dir_name);
                let regions = self.create_frame_regions(
                    anim_config.frames,
                    row,
                    sheet.frame_size,
                    sheet.size[0],
                    sheet.size[1],
                );
                let anim = Animation::from_regions(&anim_name, regions, frame_duration, true);
                self.controller.add(anim);
            }
        }
        Ok(())
    }

    /// Create sprite regions for animation frames
    fn create_frame_regions(
        &self,
        frame_count: u32,
        row: u32,
        frame_size: u32,
        sheet_width: u32,
        sheet_height: u32,
    ) -> Vec<SpriteRegion> {
        (0..frame_count)
            .map(|col| {
                SpriteRegion::from_pixels(
                    col * frame_size,
                    row * frame_size,
                    frame_size,
                    frame_size,
                    sheet_width,
                    sheet_height,
                )
            })
            .collect()
    }

    /// Get movement config
    pub fn movement(&self) -> &MovementConfig {
        &self.config.movement
    }

    /// Get walk speed
    pub fn walk_speed(&self) -> f32 {
        self.config.movement.walk_speed
    }

    /// Get run speed
    pub fn run_speed(&self) -> f32 {
        self.config.movement.walk_speed * self.config.movement.run_multiplier
    }

    /// Get spritesheet config for current state
    pub fn current_sheet(&self) -> Option<&SpriteSheetConfig> {
        let anim_config = self.config.animations.get(self.state.key())?;
        self.config.spritesheets.get(&anim_config.sheet)
    }

    /// Get the texture path for current state
    pub fn current_texture_path(&self) -> Option<&str> {
        self.current_sheet().map(|s| s.path.as_str())
    }

    /// Update state based on velocity and running flag
    pub fn update_state(&mut self, vx: f32, vy: f32, is_running: bool) {
        // Update direction if moving
        if let Some(dir) = Direction::from_velocity(vx, vy) {
            self.direction = dir;
        }

        // Update state
        let is_moving = vx.abs() > 0.1 || vy.abs() > 0.1;
        self.state = if !is_moving {
            CharacterState::Idle
        } else if is_running {
            CharacterState::Running
        } else {
            CharacterState::Walking
        };

        // Update flip
        self.flip_x = self.direction.flip_x();

        // Update animation
        let anim_name = format!("{}_{}", self.state.key(), self.direction.suffix());
        self.controller.play_if_different(&anim_name);
    }

    /// Update animation timer
    pub fn update(&mut self, dt: f32) {
        self.controller.update(dt);
    }

    /// Get current sprite region
    pub fn current_region(&self) -> Option<SpriteRegion> {
        self.controller.current_region()
    }

    /// Get frame size from config
    pub fn frame_size(&self) -> u32 {
        self.current_sheet().map(|s| s.frame_size).unwrap_or(32)
    }
}

/// Errors that can occur loading a character
#[derive(Debug)]
pub enum CharacterLoadError {
    /// IO error reading file
    Io(String),
    /// TOML parsing error
    Parse(String),
    /// Referenced spritesheet not found
    MissingSheet(String),
}

impl std::fmt::Display for CharacterLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharacterLoadError::Io(e) => write!(f, "IO error: {}", e),
            CharacterLoadError::Parse(e) => write!(f, "Parse error: {}", e),
            CharacterLoadError::MissingSheet(s) => write!(f, "Missing spritesheet: {}", s),
        }
    }
}

impl std::error::Error for CharacterLoadError {}
