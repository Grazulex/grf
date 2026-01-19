//! Player animation module
//!
//! Handles player sprite animations based on movement state.
//! Movement is handled by the ECS system - this only manages visuals.

use engine_input::{Input, KeyCode};
use engine_render::{Animation, AnimationController, SpriteRegion};

/// Player facing direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Down,  // Front-facing (row 0)
    Up,    // Back-facing (row 1)
    Left,  // Side with flip_x = true
    Right, // Side (row 2)
}

impl Direction {
    /// Get direction suffix for animation names
    pub fn suffix(self) -> &'static str {
        match self {
            Direction::Down => "down",
            Direction::Up => "up",
            Direction::Left | Direction::Right => "side",
        }
    }

    /// Should the sprite be flipped horizontally?
    pub fn flip_x(self) -> bool {
        matches!(self, Direction::Left)
    }
}

/// Player movement state for animation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    Running,
}

/// Player animation controller
///
/// This struct only handles animations - movement is done by ECS.
pub struct PlayerAnimations {
    /// Current facing direction
    pub direction: Direction,
    /// Current movement state
    pub state: PlayerState,
    /// Animation controller managing all animations
    pub animations: AnimationController,
    /// Should sprite be flipped?
    pub flip_x: bool,
}

impl PlayerAnimations {
    /// Sprite size (32x32 pixels)
    pub const SPRITE_SIZE: u32 = 32;

    /// Create a new player animation controller
    pub fn new() -> Self {
        let mut player = Self {
            direction: Direction::Down,
            state: PlayerState::Idle,
            animations: AnimationController::new(),
            flip_x: false,
        };

        // Load all animations
        player.setup_animations();
        player.animations.play("idle_down");

        player
    }

    /// Configure all animations from sprite sheets
    fn setup_animations(&mut self) {
        // Frame durations
        const IDLE_FRAME_DURATION: f32 = 0.2;
        const WALK_FRAME_DURATION: f32 = 0.12;
        const RUN_FRAME_DURATION: f32 = 0.08;

        // Sprite sheet dimensions
        const IDLE_SHEET_W: u32 = 128;
        const WALK_SHEET_W: u32 = 192;
        const RUN_SHEET_W: u32 = 256;
        const SHEET_H: u32 = 96;

        // Generate animations for each direction
        for (dir_name, row) in [("down", 0), ("up", 1), ("side", 2)] {
            // Idle animations (4 frames)
            let idle_anim = Self::create_animation(
                &format!("idle_{}", dir_name),
                4, row, Self::SPRITE_SIZE,
                IDLE_SHEET_W, SHEET_H, IDLE_FRAME_DURATION,
            );
            self.animations.add(idle_anim);

            // Walk animations (6 frames)
            let walk_anim = Self::create_animation(
                &format!("walk_{}", dir_name),
                6, row, Self::SPRITE_SIZE,
                WALK_SHEET_W, SHEET_H, WALK_FRAME_DURATION,
            );
            self.animations.add(walk_anim);

            // Run animations (8 frames)
            let run_anim = Self::create_animation(
                &format!("run_{}", dir_name),
                8, row, Self::SPRITE_SIZE,
                RUN_SHEET_W, SHEET_H, RUN_FRAME_DURATION,
            );
            self.animations.add(run_anim);
        }
    }

    /// Helper to create an animation from a sprite strip
    fn create_animation(
        name: &str,
        frame_count: u32,
        row: u32,
        sprite_size: u32,
        sheet_width: u32,
        sheet_height: u32,
        frame_duration: f32,
    ) -> Animation {
        let regions: Vec<SpriteRegion> = (0..frame_count)
            .map(|col| {
                SpriteRegion::from_pixels(
                    col * sprite_size,
                    row * sprite_size,
                    sprite_size,
                    sprite_size,
                    sheet_width,
                    sheet_height,
                )
            })
            .collect();

        Animation::from_regions(name, regions, frame_duration, true)
    }

    /// Update animation state based on input
    /// Call this each frame with the current input
    pub fn update_from_input(&mut self, input: &Input, velocity_x: f32, velocity_y: f32) {
        let is_moving = velocity_x.abs() > 0.1 || velocity_y.abs() > 0.1;
        let is_running = input.is_key_pressed(KeyCode::LShift) || input.is_key_pressed(KeyCode::RShift);

        // Update state
        if !is_moving {
            self.state = PlayerState::Idle;
        } else if is_running {
            self.state = PlayerState::Running;
        } else {
            self.state = PlayerState::Walking;
        }

        // Update direction based on velocity
        if is_moving {
            if velocity_x.abs() > velocity_y.abs() {
                self.direction = if velocity_x < 0.0 { Direction::Left } else { Direction::Right };
            } else {
                self.direction = if velocity_y < 0.0 { Direction::Up } else { Direction::Down };
            }
        }

        // Update flip_x
        self.flip_x = self.direction.flip_x();

        // Update animation
        self.update_animation();
    }

    /// Update the current animation based on state and direction
    fn update_animation(&mut self) {
        let anim_name = match self.state {
            PlayerState::Idle => format!("idle_{}", self.direction.suffix()),
            PlayerState::Walking => format!("walk_{}", self.direction.suffix()),
            PlayerState::Running => format!("run_{}", self.direction.suffix()),
        };

        self.animations.play_if_different(&anim_name);
    }

    /// Update animation time (call each frame)
    pub fn update(&mut self, dt: f32) {
        self.animations.update(dt);
    }

    /// Get the current sprite region to render
    pub fn current_sprite_region(&self) -> Option<SpriteRegion> {
        self.animations.current_region()
    }

    /// Get which texture to use based on current state
    pub fn current_texture_name(&self) -> &'static str {
        match self.state {
            PlayerState::Idle => "player_idle",
            PlayerState::Walking => "player_walk",
            PlayerState::Running => "player_run",
        }
    }
}

impl Default for PlayerAnimations {
    fn default() -> Self {
        Self::new()
    }
}
