//! GRF - Game Rust Framework
//!
//! A 2D RPG/Farming game engine built from scratch in Rust.

use std::sync::Arc;

use anyhow::Result;
use engine_core::GameTime;
use engine_input::{Input, KeyCode};
use engine_render::{glam::Vec2, wgpu, Camera2D, Renderer, Sprite, Texture};
use engine_window::{winit::event::KeyEvent, App, Window, WindowConfig};
use log::{error, info};
use winit::window::Window as WinitWindow;

/// Player state controlled by input
struct Player {
    position: Vec2,
    prev_position: Vec2,
    speed: f32,
}

impl Player {
    fn new(start_pos: Vec2) -> Self {
        Self {
            position: start_pos,
            prev_position: start_pos,
            speed: 300.0, // pixels per second
        }
    }

    /// Fixed update with input
    fn fixed_update(&mut self, input: &Input, bounds: (f32, f32)) {
        self.prev_position = self.position;

        // Get movement from input (WASD or arrows)
        let direction = input.get_movement_direction();
        let dt = engine_core::FIXED_TIMESTEP as f32;

        self.position += direction * self.speed * dt;

        // Clamp to bounds
        let margin = 32.0;
        self.position.x = self.position.x.clamp(margin, bounds.0 - margin);
        self.position.y = self.position.y.clamp(margin, bounds.1 - margin);
    }

    fn interpolated_position(&self, alpha: f32) -> Vec2 {
        self.prev_position.lerp(self.position, alpha)
    }
}

/// The main game application
struct Game {
    game_time: GameTime,
    input: Input,
    renderer: Option<Renderer>,
    camera: Option<Camera2D>,
    // Test texture
    test_texture: Option<Texture>,
    test_bind_group: Option<wgpu::BindGroup>,
    // Player
    player: Option<Player>,
    // Stats display
    last_log_time: f64,
}

impl Game {
    fn new() -> Self {
        Self {
            game_time: GameTime::new(),
            input: Input::new(),
            renderer: None,
            camera: None,
            test_texture: None,
            test_bind_group: None,
            player: None,
            last_log_time: 0.0,
        }
    }
}

impl App for Game {
    fn init(&mut self, window: Arc<WinitWindow>) {
        info!("Initializing game...");

        // Create renderer (requires async, use pollster to block)
        let renderer = pollster::block_on(Renderer::new(window));
        let size = renderer.size();

        // Load test texture
        match renderer.load_texture("assets/textures/test_sprite.png") {
            Ok(texture) => {
                info!("Loaded test texture: {}x{}", texture.size.0, texture.size.1);
                let bind_group = renderer.create_texture_bind_group(&texture);
                self.test_texture = Some(texture);
                self.test_bind_group = Some(bind_group);
            }
            Err(e) => {
                error!("Failed to load test texture: {}", e);
            }
        }

        // Initialize player at center of a larger world
        let player_start = Vec2::new(500.0, 500.0);
        self.player = Some(Player::new(player_start));

        // Initialize camera
        let mut camera = Camera2D::new(size.0 as f32, size.1 as f32);
        camera.follow(player_start, 5.0); // Smooth follow the player
        self.camera = Some(camera);

        self.renderer = Some(renderer);

        info!("Game initialized!");
        info!("Controls: WASD or Arrow keys to move, ESC to quit");
    }

    fn update(&mut self) {
        self.game_time.update();

        // Check for escape to quit
        if self.input.is_key_just_pressed(KeyCode::Escape) {
            info!("Escape pressed - exiting");
            std::process::exit(0);
        }

        // World bounds (larger than screen for camera demo)
        let world_bounds = (2000.0, 2000.0);

        // Fixed timestep updates
        while self.game_time.should_fixed_update() {
            if let Some(player) = &mut self.player {
                player.fixed_update(&self.input, world_bounds);
            }
        }

        // Update camera to follow player
        let dt = self.game_time.delta as f32;
        if let (Some(camera), Some(player)) = (&mut self.camera, &self.player) {
            camera.follow(player.position, 5.0);
            camera.update(dt);
        }

        // Log stats every second
        if self.game_time.total_time() - self.last_log_time >= 1.0 {
            self.last_log_time = self.game_time.total_time();
            info!(
                "FPS: {:.1} | UPS: {:.1} | Pos: ({:.0}, {:.0})",
                self.game_time.fps(),
                self.game_time.ups(),
                self.player.as_ref().map_or(0.0, |p| p.position.x),
                self.player.as_ref().map_or(0.0, |p| p.position.y),
            );
        }
    }

    fn render(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            // Apply camera
            if let Some(camera) = &self.camera {
                renderer.set_camera(camera);
            }

            match renderer.begin_frame() {
                Ok(frame) => {
                    let alpha = self.game_time.alpha() as f32;

                    // Draw player sprite (in world coordinates)
                    if let Some(player) = &self.player {
                        let pos = player.interpolated_position(alpha);
                        let sprite = Sprite::new(pos, Vec2::new(64.0, 64.0));
                        renderer.draw_sprite(&sprite);
                    }

                    renderer.end_frame(frame, self.test_bind_group.as_ref());
                }
                Err(wgpu::SurfaceError::Lost) => {
                    let size = renderer.size();
                    renderer.resize(size.0, size.1);
                }
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    error!("Out of GPU memory!");
                }
                Err(e) => {
                    error!("Render error: {:?}", e);
                }
            }
        }
    }

    fn end_frame(&mut self) {
        self.input.end_frame();
    }

    fn on_keyboard_event(&mut self, event: &KeyEvent) {
        self.input.on_keyboard_event(event);
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        info!("Window resized to {}x{}", width, height);
        if let Some(renderer) = &mut self.renderer {
            renderer.resize(width, height);
        }
        if let Some(camera) = &mut self.camera {
            camera.set_viewport(width as f32, height as f32);
        }
    }

    fn on_close_requested(&mut self) -> bool {
        info!("Goodbye!");
        true
    }
}

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("GRF - Game Rust Framework v{}", env!("CARGO_PKG_VERSION"));
    info!("Starting game...");

    // Create window configuration
    let config = WindowConfig::new("GRF - Game Rust Framework")
        .with_size(1280, 720)
        .with_resizable(true);

    // Create and run the game
    let window = Window::new(config);
    let game = Game::new();

    window.run(game)?;

    Ok(())
}
