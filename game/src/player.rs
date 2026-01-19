//! Player module with animated character
//!
//! Demonstrates how to use the GRF framework for animated player characters:
//! - State machine (Idle, Walk, Run)
//! - Direction handling with flip for left
//! - Animation configuration from sprite sheets
//! - WASD input integration

use engine_input::{Input, KeyCode};
use engine_render::{Animation, AnimationController, SpriteRegion, glam::Vec2};

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
    /// Get the row index in the sprite sheet (0-2)
    pub fn sprite_row(self) -> u32 {
        match self {
            Direction::Down => 0,  // Front
            Direction::Up => 1,    // Back
            Direction::Left | Direction::Right => 2, // Side
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
}

/// Player movement state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    Running,
}

/// Player character with animations
pub struct Player {
    /// Current position in world
    pub position: Vec2,
    /// Movement velocity
    pub velocity: Vec2,
    /// Current facing direction
    pub direction: Direction,
    /// Current movement state
    pub state: PlayerState,
    /// Animation controller managing all animations
    pub animations: AnimationController,
    /// Movement speed (pixels/second)
    pub walk_speed: f32,
    /// Run speed multiplier
    pub run_speed: f32,
    /// Should sprite be flipped?
    pub flip_x: bool,
}

impl Player {
    /// Sprite size (32x32 pixels)
    pub const SPRITE_SIZE: u32 = 32;

    /// Create a new player at the given position
    pub fn new(position: Vec2) -> Self {
        let mut player = Self {
            position,
            velocity: Vec2::ZERO,
            direction: Direction::Down,
            state: PlayerState::Idle,
            animations: AnimationController::new(),
            walk_speed: 100.0,
            run_speed: 180.0,
            flip_x: false,
        };

        // Load all animations
        player.setup_animations();
        player.animations.play("idle_down");

        player
    }

    /// Configure all animations from sprite sheets
    ///
    /// Sprite sheet structure (32x32 sprites):
    /// - Idle.png: 128x96 (4 frames x 3 rows)
    /// - Walk.png: 192x96 (6 frames x 3 rows)
    /// - Run.png:  256x96 (8 frames x 3 rows)
    ///
    /// Row layout:
    /// - Row 0: Front (down)
    /// - Row 1: Back (up)
    /// - Row 2: Side (use flip_x for left)
    fn setup_animations(&mut self) {
        // Frame durations
        const IDLE_FRAME_DURATION: f32 = 0.2;  // 4 frames / ~0.8s cycle
        const WALK_FRAME_DURATION: f32 = 0.12; // 6 frames / ~0.72s cycle
        const RUN_FRAME_DURATION: f32 = 0.08;  // 8 frames / ~0.64s cycle

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
                4,
                row,
                Self::SPRITE_SIZE,
                IDLE_SHEET_W,
                SHEET_H,
                IDLE_FRAME_DURATION,
            );
            self.animations.add(idle_anim);

            // Walk animations (6 frames)
            let walk_anim = Self::create_animation(
                &format!("walk_{}", dir_name),
                6,
                row,
                Self::SPRITE_SIZE,
                WALK_SHEET_W,
                SHEET_H,
                WALK_FRAME_DURATION,
            );
            self.animations.add(walk_anim);

            // Run animations (8 frames)
            let run_anim = Self::create_animation(
                &format!("run_{}", dir_name),
                8,
                row,
                Self::SPRITE_SIZE,
                RUN_SHEET_W,
                SHEET_H,
                RUN_FRAME_DURATION,
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
                    col * sprite_size,      // x
                    row * sprite_size,      // y
                    sprite_size,            // width
                    sprite_size,            // height
                    sheet_width,
                    sheet_height,
                )
            })
            .collect();

        Animation::from_regions(name, regions, frame_duration, true)
    }

    /// Update player from input
    pub fn handle_input(&mut self, input: &Input) {
        // Get movement direction from WASD/Arrows
        let mut move_dir = Vec2::ZERO;

        if input.is_key_pressed(KeyCode::W) || input.is_key_pressed(KeyCode::Up) {
            move_dir.y -= 1.0;
        }
        if input.is_key_pressed(KeyCode::S) || input.is_key_pressed(KeyCode::Down) {
            move_dir.y += 1.0;
        }
        if input.is_key_pressed(KeyCode::A) || input.is_key_pressed(KeyCode::Left) {
            move_dir.x -= 1.0;
        }
        if input.is_key_pressed(KeyCode::D) || input.is_key_pressed(KeyCode::Right) {
            move_dir.x += 1.0;
        }

        // Check if running (Shift held)
        let running = input.is_key_pressed(KeyCode::LShift) || input.is_key_pressed(KeyCode::RShift);

        // Update state based on movement
        if move_dir == Vec2::ZERO {
            self.state = PlayerState::Idle;
            self.velocity = Vec2::ZERO;
        } else {
            // Normalize diagonal movement
            move_dir = move_dir.normalize();

            // Set state and speed
            if running {
                self.state = PlayerState::Running;
                self.velocity = move_dir * self.run_speed;
            } else {
                self.state = PlayerState::Walking;
                self.velocity = move_dir * self.walk_speed;
            }

            // Update direction based on movement
            // Prioritize horizontal movement for direction
            if move_dir.x.abs() > move_dir.y.abs() {
                self.direction = if move_dir.x < 0.0 {
                    Direction::Left
                } else {
                    Direction::Right
                };
            } else {
                self.direction = if move_dir.y < 0.0 {
                    Direction::Up
                } else {
                    Direction::Down
                };
            }
        }

        // Update flip_x based on direction
        self.flip_x = self.direction.flip_x();

        // Update animation based on state and direction
        self.update_animation();
    }

    /// Update the current animation based on state and direction
    fn update_animation(&mut self) {
        let anim_name = match self.state {
            PlayerState::Idle => format!("idle_{}", self.direction.suffix()),
            PlayerState::Walking => format!("walk_{}", self.direction.suffix()),
            PlayerState::Running => format!("run_{}", self.direction.suffix()),
        };

        // Only change animation if different (play_if_different doesn't reset time)
        self.animations.play_if_different(&anim_name);
    }

    /// Update player (call each frame)
    pub fn update(&mut self, dt: f32) {
        // Update position
        self.position += self.velocity * dt;

        // Update animation time
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_row() {
        assert_eq!(Direction::Down.sprite_row(), 0);
        assert_eq!(Direction::Up.sprite_row(), 1);
        assert_eq!(Direction::Left.sprite_row(), 2);
        assert_eq!(Direction::Right.sprite_row(), 2);
    }

    #[test]
    fn test_direction_flip() {
        assert!(!Direction::Down.flip_x());
        assert!(!Direction::Up.flip_x());
        assert!(Direction::Left.flip_x());
        assert!(!Direction::Right.flip_x());
    }

    #[test]
    fn test_player_creation() {
        let player = Player::new(Vec2::new(100.0, 100.0));
        assert_eq!(player.position, Vec2::new(100.0, 100.0));
        assert_eq!(player.state, PlayerState::Idle);
        assert_eq!(player.direction, Direction::Down);
        assert_eq!(player.animations.current_name(), Some("idle_down"));
    }

    #[test]
    fn test_animation_setup() {
        let player = Player::new(Vec2::ZERO);

        // Should have 9 animations (3 states x 3 directions)
        // idle_down, idle_up, idle_side
        // walk_down, walk_up, walk_side
        // run_down, run_up, run_side

        // Test that we can play each animation
        let mut ctrl = player.animations;
        ctrl.play("idle_down");
        assert_eq!(ctrl.current_name(), Some("idle_down"));

        ctrl.play("walk_side");
        assert_eq!(ctrl.current_name(), Some("walk_side"));

        ctrl.play("run_up");
        assert_eq!(ctrl.current_name(), Some("run_up"));
    }
}
