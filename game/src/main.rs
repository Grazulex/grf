//! GRF - Game Rust Framework
//!
//! A 2D RPG/Farming game engine built from scratch in Rust.

mod components;
mod dialogue;
mod farming;
mod inventory;
mod items;
mod npc;
mod systems;

use std::sync::Arc;

use anyhow::Result;
use engine_core::GameTime;
use engine_ecs::{Entity, World};
use engine_input::{Input, KeyCode};
use engine_render::{glam::Vec2, wgpu, Camera2D, Renderer, Sprite, Texture, Tilemap};
use engine_window::{winit::event::KeyEvent, App, Window, WindowConfig};
use log::{error, info};
use winit::window::Window as WinitWindow;

use components::{CameraTarget, Collider, PlayerControlled, Position, SpriteRender, Velocity};
use systems::{camera_system, input_system, movement_system};

/// The main game application
struct Game {
    game_time: GameTime,
    /// ECS World containing all entities and components
    world: World,
    /// Reference to the player entity
    player_entity: Option<Entity>,
    // Renderer (not in ECS as it needs special handling)
    renderer: Option<Renderer>,
    // Textures and bind groups
    tileset_texture: Option<Texture>,
    tileset_bind_group: Option<wgpu::BindGroup>,
    player_texture: Option<Texture>,
    player_bind_group: Option<wgpu::BindGroup>,
}

impl Game {
    fn new() -> Self {
        let mut world = World::new();

        // Insert Input as a resource
        world.insert_resource(Input::new());

        Self {
            game_time: GameTime::new(),
            world,
            player_entity: None,
            renderer: None,
            tileset_texture: None,
            tileset_bind_group: None,
            player_texture: None,
            player_bind_group: None,
        }
    }

    /// Load a new map and position player at spawn point
    fn load_map(&mut self, map_path: &str, spawn_id: &str) {
        let Some(renderer) = &self.renderer else {
            return;
        };

        match Tilemap::load(map_path) {
            Ok(tilemap) => {
                // Get spawn position
                let spawn_pos = tilemap
                    .get_spawn(spawn_id)
                    .map(|s| s.position())
                    .unwrap_or_else(|| tilemap.default_spawn());

                // Load tileset texture (if different from current)
                if let Some(tileset) = tilemap.tilesets.first() {
                    match renderer.load_texture(&tileset.image) {
                        Ok(texture) => {
                            let bind_group = renderer.create_texture_bind_group(&texture);
                            self.tileset_texture = Some(texture);
                            self.tileset_bind_group = Some(bind_group);
                        }
                        Err(e) => {
                            error!("Failed to load tileset texture: {}", e);
                        }
                    }
                }

                // Position player at spawn
                if let Some(entity) = self.player_entity {
                    if let Some(pos) = self.world.get_mut::<Position>(entity) {
                        pos.current = spawn_pos;
                        pos.previous = spawn_pos;
                    }
                }

                // Snap camera to player
                if let Some(camera) = self.world.get_resource_mut::<Camera2D>() {
                    camera.set_position(spawn_pos);
                }

                // Update tilemap resource
                self.world.insert_resource(tilemap);
            }
            Err(e) => {
                error!("Failed to load tilemap '{}': {}", map_path, e);
            }
        }
    }

    /// Get player position for trigger checking
    fn get_player_position(&self) -> Option<Vec2> {
        self.player_entity
            .and_then(|e| self.world.get::<Position>(e))
            .map(|p| p.current)
    }
}

impl App for Game {
    fn init(&mut self, window: Arc<WinitWindow>) {
        // Create renderer
        let renderer = pollster::block_on(Renderer::new(window));
        let size = renderer.size();

        // Load tilemap
        let player_start = match Tilemap::load("assets/maps/test.json") {
            Ok(tilemap) => {
                // Load tileset texture
                if let Some(tileset) = tilemap.tilesets.first() {
                    if let Ok(texture) = renderer.load_texture(&tileset.image) {
                        let bind_group = renderer.create_texture_bind_group(&texture);
                        self.tileset_texture = Some(texture);
                        self.tileset_bind_group = Some(bind_group);
                    }
                }

                // Get map center for player start
                let (w, h) = tilemap.pixel_size();
                let start = Vec2::new(w as f32 / 2.0, h as f32 / 2.0);

                // Store tilemap as resource
                self.world.insert_resource(tilemap);

                start
            }
            Err(e) => {
                error!("Failed to load tilemap: {}", e);
                Vec2::new(160.0, 120.0)
            }
        };

        // Load player texture
        if let Ok(texture) = renderer.load_texture("assets/textures/test_sprite.png") {
            let bind_group = renderer.create_texture_bind_group(&texture);
            self.player_texture = Some(texture);
            self.player_bind_group = Some(bind_group);
        }

        // Create player entity with components
        let player = self.world.spawn();
        self.world.insert(player, Position::from_vec2(player_start));
        self.world.insert(player, Velocity::default());
        self.world.insert(player, PlayerControlled::new(300.0));
        self.world.insert(player, CameraTarget);
        self.world.insert(player, SpriteRender::new(32.0, 32.0));
        self.world.insert(player, Collider::new(32.0, 32.0));
        self.player_entity = Some(player);

        // Initialize camera as resource
        let mut camera = Camera2D::new(size.0 as f32, size.1 as f32);
        camera.follow(player_start, 5.0);
        self.world.insert_resource(camera);

        self.renderer = Some(renderer);
    }

    fn update(&mut self) {
        self.game_time.update();

        // Check for escape to quit
        {
            let should_quit = self
                .world
                .get_resource::<Input>()
                .map(|i| i.is_key_just_pressed(KeyCode::Escape))
                .unwrap_or(false);
            if should_quit {
                std::process::exit(0);
            }
        }

        // Fixed timestep updates
        while self.game_time.should_fixed_update() {
            // Run ECS systems
            input_system(&mut self.world);
            movement_system(&mut self.world);
        }

        // Check for map transition triggers
        let transition: Option<(String, String)> = {
            let player_pos = self.get_player_position();
            if let (Some(tilemap), Some(pos)) =
                (self.world.get_resource::<Tilemap>(), player_pos)
            {
                tilemap
                    .check_trigger(pos)
                    .map(|t| (t.target_map.clone(), t.target_spawn.clone()))
            } else {
                None
            }
        };
        if let Some((map_path, spawn_id)) = transition {
            self.load_map(&map_path, &spawn_id);
        }

        // Update camera to follow player
        let dt = self.game_time.delta as f32;
        camera_system(&mut self.world, dt);
    }

    fn render(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            // Apply camera
            if let Some(camera) = self.world.get_resource::<Camera2D>() {
                renderer.set_camera(camera);
            }

            match renderer.begin_frame() {
                Ok(frame) => {
                    // Get camera and tilemap for rendering
                    let camera_ptr = self
                        .world
                        .get_resource::<Camera2D>()
                        .map(|c| c as *const Camera2D);
                    let tilemap_ptr = self
                        .world
                        .get_resource::<Tilemap>()
                        .map(|t| t as *const Tilemap);

                    if let (Some(tm_ptr), Some(cam_ptr)) = (tilemap_ptr, camera_ptr) {
                        // Safety: we're only reading, and these are borrowed from world
                        let tilemap = unsafe { &*tm_ptr };
                        let camera = unsafe { &*cam_ptr };

                        // 1. Render layers BELOW entities (ground, decorations)
                        for layer_idx in tilemap.below_layers() {
                            let sprites = tilemap.get_visible_sprites(layer_idx, camera);
                            for (sprite, _tileset_idx) in sprites {
                                renderer.draw_sprite(&sprite);
                            }
                        }

                        // 2. Render entities with SpriteRender and Position
                        let alpha = self.game_time.alpha() as f32;
                        for (entity, sprite_render) in self.world.query::<SpriteRender>() {
                            if let Some(pos) = self.world.get::<Position>(entity) {
                                let render_pos = pos.interpolated(alpha);
                                let sprite = Sprite::new(render_pos, sprite_render.size());
                                renderer.draw_sprite(&sprite);
                            }
                        }

                        // 3. Render layers ABOVE entities (tree tops, roofs)
                        for layer_idx in tilemap.above_layers() {
                            let sprites = tilemap.get_visible_sprites(layer_idx, camera);
                            for (sprite, _tileset_idx) in sprites {
                                renderer.draw_sprite(&sprite);
                            }
                        }
                    }

                    renderer.end_frame(frame, self.tileset_bind_group.as_ref());
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
        if let Some(input) = self.world.get_resource_mut::<Input>() {
            input.end_frame();
        }
    }

    fn on_keyboard_event(&mut self, event: &KeyEvent) {
        if let Some(input) = self.world.get_resource_mut::<Input>() {
            input.on_keyboard_event(event);
        }
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        if let Some(renderer) = &mut self.renderer {
            renderer.resize(width, height);
        }
        if let Some(camera) = self.world.get_resource_mut::<Camera2D>() {
            camera.set_viewport(width as f32, height as f32);
        }
    }

    fn on_close_requested(&mut self) -> bool {
        true
    }
}

fn main() -> Result<()> {
    // Initialize logging (use RUST_LOG=debug for verbose output)
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("warn,grf_game=info,wgpu=error,naga=error"),
    )
    .init();

    info!(
        "GRF v{} - WASD to move, ESC to quit",
        env!("CARGO_PKG_VERSION")
    );

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
