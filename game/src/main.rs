//! GRF - Game Rust Framework
//!
//! A 2D RPG/Farming game engine built from scratch in Rust.

mod components;
mod dialogue;
mod farming;
mod inventory;
mod items;
mod menu;
mod npc;
mod player;
mod save;
mod systems;

use std::sync::Arc;

use anyhow::Result;
use engine_core::{GameSettings, GameTime};
use engine_ecs::{Entity, World};
use engine_input::{Input, KeyCode};
use engine_render::{glam, glam::Vec2, wgpu, Camera2D, Renderer, Sprite, Texture, Tilemap};
use engine_ui::{Hud, Menu, MenuItem, SettingsMenu};
use engine_window::{winit::event::{KeyEvent, WindowEvent}, App, Window, WindowConfig};
use log::{error, info};
use winit::window::Window as WinitWindow;

#[cfg(feature = "debug-tools")]
use engine_debug::{ConsoleCommand, DebugOverlay, EguiRenderer};

use components::{CameraTarget, Collider, PlayerControlled, Position, SpriteRender, Velocity};
use inventory::Inventory;
use menu::{GameState, PreviousState};
use player::{load_player_animator, CharacterAnimator};
use save::{GameClockData, PlayerData, SaveData, SaveManager};
use systems::{camera_system, input_system, movement_system};

/// The main game application
struct Game {
    game_time: GameTime,
    /// Current game state (menu, playing, paused)
    game_state: GameState,
    /// Previous state to return to from settings
    previous_state: PreviousState,
    /// Main menu (from engine_ui)
    main_menu: Menu,
    /// Pause menu (from engine_ui)
    pause_menu: Menu,
    /// Settings menu (from engine_ui)
    settings_menu: SettingsMenu,
    /// Game settings (from engine_core)
    settings: GameSettings,
    /// ECS World containing all entities and components
    world: World,
    /// Reference to the player entity
    player_entity: Option<Entity>,
    // Renderer (not in ECS as it needs special handling)
    renderer: Option<Renderer>,
    // Textures and bind groups
    tileset_texture: Option<Texture>,
    tileset_bind_group: Option<wgpu::BindGroup>,
    // Player animation (loaded from config)
    player_animator: Option<CharacterAnimator>,
    // Player textures keyed by sheet name (idle, walk, run)
    player_textures: std::collections::HashMap<String, (Texture, wgpu::BindGroup)>,
    // Window reference for egui
    window: Option<Arc<WinitWindow>>,
    // HUD
    hud: Option<Hud>,
    // Save system
    save_manager: SaveManager,
    current_map: String,
    // Debug tools (feature-gated)
    #[cfg(feature = "debug-tools")]
    egui_renderer: Option<EguiRenderer>,
    #[cfg(feature = "debug-tools")]
    debug_overlay: DebugOverlay,
}

impl Game {
    /// Create the main menu with standard options
    fn create_main_menu() -> Menu {
        let mut menu = Menu::new(Vec2::new(400.0, 300.0)); // Will be repositioned on render
        menu.set_items(vec![
            MenuItem::new("new_game", "New Game"),
            MenuItem::new("load_game", "Load Game"),
            MenuItem::new("settings", "Settings"),
            MenuItem::new("quit", "Quit"),
        ]);
        menu
    }

    /// Create the pause menu with standard options
    fn create_pause_menu() -> Menu {
        let mut menu = Menu::new(Vec2::new(400.0, 300.0)); // Will be repositioned on render
        menu.set_items(vec![
            MenuItem::new("resume", "Resume"),
            MenuItem::new("save_game", "Save Game"),
            MenuItem::new("settings", "Settings"),
            MenuItem::new("main_menu", "Main Menu"),
            MenuItem::new("quit", "Quit"),
        ]);
        menu
    }

    /// Create the settings menu from current settings
    fn create_settings_menu(settings: &GameSettings) -> SettingsMenu {
        let mut menu = SettingsMenu::new(Vec2::new(400.0, 300.0)); // Will be repositioned on render
        menu.set_entries(settings.to_entries());
        menu
    }

    fn new() -> Self {
        let mut world = World::new();

        // Insert Input as a resource
        world.insert_resource(Input::new());

        // Load settings (or use defaults)
        let settings = GameSettings::load();
        let settings_menu = Self::create_settings_menu(&settings);

        Self {
            game_time: GameTime::new(),
            game_state: GameState::MainMenu,
            previous_state: PreviousState::MainMenu,
            main_menu: Self::create_main_menu(),
            pause_menu: Self::create_pause_menu(),
            settings_menu,
            settings,
            world,
            player_entity: None,
            renderer: None,
            tileset_texture: None,
            tileset_bind_group: None,
            player_animator: None,
            player_textures: std::collections::HashMap::new(),
            window: None,
            hud: None,
            save_manager: SaveManager::new(),
            current_map: String::new(),
            #[cfg(feature = "debug-tools")]
            egui_renderer: None,
            #[cfg(feature = "debug-tools")]
            debug_overlay: DebugOverlay::new(),
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

                // Store current map path for save system
                self.current_map = map_path.to_string();
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

    /// Save game to slot 0
    fn save_game(&self) {
        let Some(entity) = self.player_entity else {
            error!("Cannot save: no player entity");
            return;
        };

        // Gather player components
        let Some(position) = self.world.get::<Position>(entity) else {
            error!("Cannot save: player has no Position");
            return;
        };
        let Some(player_ctrl) = self.world.get::<PlayerControlled>(entity) else {
            error!("Cannot save: player has no PlayerControlled");
            return;
        };
        let Some(sprite) = self.world.get::<SpriteRender>(entity) else {
            error!("Cannot save: player has no SpriteRender");
            return;
        };
        let Some(collider) = self.world.get::<Collider>(entity) else {
            error!("Cannot save: player has no Collider");
            return;
        };

        let player_data = PlayerData::from_components(position, player_ctrl, sprite, collider);

        // Get game clock (use default if not available)
        let game_clock_data = self
            .world
            .get_resource::<engine_core::GameClock>()
            .map(GameClockData::from_game_clock)
            .unwrap_or_else(|| GameClockData {
                minute: 0,
                hour: 6,
                day: 1,
                season: "Spring".to_string(),
                year: 1,
            });

        // Get inventory (use default if not available)
        let inventory = self
            .world
            .get_resource::<Inventory>()
            .cloned()
            .unwrap_or_default();

        let save_data = SaveData::new(
            player_data,
            game_clock_data,
            self.current_map.clone(),
            inventory,
        );

        match self.save_manager.save(0, &save_data) {
            Ok(()) => info!("Game saved successfully!"),
            Err(e) => error!("Failed to save game: {}", e),
        }
    }

    /// Load game from slot 0
    fn load_game(&mut self) {
        let save_data = match self.save_manager.load(0) {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to load game: {}", e);
                return;
            }
        };

        // Restore player state
        if let Some(entity) = self.player_entity {
            if let Some(pos) = self.world.get_mut::<Position>(entity) {
                *pos = save_data.player.to_position();
            }
            if let Some(player_ctrl) = self.world.get_mut::<PlayerControlled>(entity) {
                *player_ctrl = save_data.player.to_player_controlled();
            }
            if let Some(sprite) = self.world.get_mut::<SpriteRender>(entity) {
                *sprite = save_data.player.to_sprite_render();
            }
            if let Some(collider) = self.world.get_mut::<Collider>(entity) {
                *collider = save_data.player.to_collider();
            }

            // Snap camera to loaded position
            if let Some(camera) = self.world.get_resource_mut::<Camera2D>() {
                camera.set_position(save_data.player.position);
            }
        }

        // Restore inventory
        self.world.insert_resource(save_data.inventory);

        // Load map if different
        if !save_data.current_map.is_empty() && save_data.current_map != self.current_map {
            self.load_map(&save_data.current_map, "default");
            // Override position from save (load_map would use spawn point)
            if let Some(entity) = self.player_entity {
                if let Some(pos) = self.world.get_mut::<Position>(entity) {
                    pos.current = save_data.player.position;
                    pos.previous = save_data.player.position;
                }
            }
        }

        info!("Game loaded successfully!");
    }

    /// Update main menu state
    fn update_main_menu(&mut self) {
        // Handle menu input
        let confirmed = {
            if let Some(input) = self.world.get_resource::<Input>() {
                if input.is_key_just_pressed(KeyCode::Up) || input.is_key_just_pressed(KeyCode::W) {
                    self.main_menu.move_up();
                }
                if input.is_key_just_pressed(KeyCode::Down) || input.is_key_just_pressed(KeyCode::S) {
                    self.main_menu.move_down();
                }
                input.is_key_just_pressed(KeyCode::Enter) || input.is_key_just_pressed(KeyCode::Space)
            } else {
                false
            }
        };

        // Handle confirmed selection
        if confirmed {
            if let Some(item) = self.main_menu.selected_item() {
                match item.id.as_str() {
                    "new_game" => {
                        info!("Starting new game...");
                        self.start_new_game();
                    }
                    "load_game" => {
                        info!("Loading game...");
                        self.load_game();
                        self.game_state = GameState::Playing;
                    }
                    "settings" => {
                        info!("Opening settings from main menu...");
                        self.previous_state = PreviousState::MainMenu;
                        self.settings_menu.reset();
                        self.game_state = GameState::Settings;
                    }
                    "quit" => {
                        info!("Goodbye!");
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Start a new game
    fn start_new_game(&mut self) {
        // Reset game clock
        self.world.insert_resource(engine_core::GameClock::new());

        // Reset inventory
        self.world.insert_resource(Inventory::default());

        // Load the starting map
        self.load_map("game/assets/Tilesets/test.json", "default");

        // Switch to playing state
        self.game_state = GameState::Playing;

        info!("New game started!");
    }

    /// Update pause menu state
    fn update_pause_menu(&mut self) {
        // Handle menu input
        let (confirmed, escape_pressed) = {
            if let Some(input) = self.world.get_resource::<Input>() {
                if input.is_key_just_pressed(KeyCode::Up) || input.is_key_just_pressed(KeyCode::W) {
                    self.pause_menu.move_up();
                }
                if input.is_key_just_pressed(KeyCode::Down) || input.is_key_just_pressed(KeyCode::S) {
                    self.pause_menu.move_down();
                }
                (
                    input.is_key_just_pressed(KeyCode::Enter) || input.is_key_just_pressed(KeyCode::Space),
                    input.is_key_just_pressed(KeyCode::Escape),
                )
            } else {
                (false, false)
            }
        };

        // Escape also resumes
        if escape_pressed {
            info!("Game resumed");
            self.game_state = GameState::Playing;
            return;
        }

        // Handle confirmed selection
        if confirmed {
            if let Some(item) = self.pause_menu.selected_item() {
                match item.id.as_str() {
                    "resume" => {
                        info!("Game resumed");
                        self.game_state = GameState::Playing;
                    }
                    "save_game" => {
                        info!("Saving game...");
                        self.save_game();
                    }
                    "settings" => {
                        info!("Opening settings from pause menu...");
                        self.previous_state = PreviousState::Paused;
                        self.settings_menu.reset();
                        self.game_state = GameState::Settings;
                    }
                    "main_menu" => {
                        info!("Returning to main menu...");
                        self.main_menu.reset();
                        self.game_state = GameState::MainMenu;
                    }
                    "quit" => {
                        info!("Goodbye!");
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Update settings menu state
    fn update_settings(&mut self) {
        // Handle settings menu input
        let (escape_pressed, left_pressed, right_pressed) = {
            if let Some(input) = self.world.get_resource::<Input>() {
                if input.is_key_just_pressed(KeyCode::Up) || input.is_key_just_pressed(KeyCode::W) {
                    self.settings_menu.move_up();
                }
                if input.is_key_just_pressed(KeyCode::Down) || input.is_key_just_pressed(KeyCode::S) {
                    self.settings_menu.move_down();
                }
                // Enter/Space toggles (for toggles) or does nothing (for sliders)
                if input.is_key_just_pressed(KeyCode::Enter) || input.is_key_just_pressed(KeyCode::Space) {
                    self.settings_menu.toggle_or_adjust();
                }
                (
                    input.is_key_just_pressed(KeyCode::Escape),
                    input.is_key_just_pressed(KeyCode::Left) || input.is_key_just_pressed(KeyCode::A),
                    input.is_key_just_pressed(KeyCode::Right) || input.is_key_just_pressed(KeyCode::D),
                )
            } else {
                (false, false, false)
            }
        };

        // Left/Right adjust values
        if left_pressed {
            self.settings_menu.adjust_left();
        }
        if right_pressed {
            self.settings_menu.adjust_right();
        }

        // Escape returns to previous state and saves settings
        if escape_pressed {
            // Apply settings from menu entries
            for entry in self.settings_menu.entries() {
                self.settings.update_from_entry(entry);
            }

            // Save settings to file
            if let Err(e) = self.settings.save() {
                error!("Failed to save settings: {}", e);
            } else {
                info!("Settings saved");
            }

            // Return to previous state
            match self.previous_state {
                PreviousState::MainMenu => {
                    self.game_state = GameState::MainMenu;
                }
                PreviousState::Paused => {
                    self.game_state = GameState::Paused;
                }
            }
        }
    }

    /// Process pending console commands
    #[cfg(feature = "debug-tools")]
    fn process_console_commands(&mut self) {
        let commands = self.debug_overlay.take_pending_commands();

        for cmd in commands {
            match cmd {
                ConsoleCommand::Teleport { x, y } => {
                    if let Some(entity) = self.player_entity {
                        if let Some(pos) = self.world.get_mut::<Position>(entity) {
                            pos.current = Vec2::new(x, y);
                            pos.previous = pos.current;
                        }
                        // Also snap camera
                        if let Some(camera) = self.world.get_resource_mut::<Camera2D>() {
                            camera.set_position(Vec2::new(x, y));
                        }
                        self.debug_overlay.log_game(
                            self.game_time.total_time(),
                            format!("Teleported to ({}, {})", x, y),
                        );
                    }
                }
                ConsoleCommand::SetSpeed(speed) => {
                    if let Some(entity) = self.player_entity {
                        if let Some(player) = self.world.get_mut::<PlayerControlled>(entity) {
                            player.set_speed(speed);
                            self.debug_overlay.log_debug(
                                self.game_time.total_time(),
                                format!("Player speed set to {}", speed),
                            );
                        }
                    }
                }
                ConsoleCommand::SetTimescale(scale) => {
                    self.game_time.set_time_scale(scale as f64);
                    self.debug_overlay.log_debug(
                        self.game_time.total_time(),
                        format!("Timescale set to {}", scale),
                    );
                }
                ConsoleCommand::GetPosition => {
                    if let Some(pos) = self.get_player_position() {
                        self.debug_overlay.console_print(format!(
                            "Player position: ({:.1}, {:.1})",
                            pos.x, pos.y
                        ));
                    } else {
                        self.debug_overlay.console_print("No player entity found");
                    }
                }
                ConsoleCommand::ListEntities => {
                    let mut count = 0;
                    for (entity, pos) in self.world.query::<Position>() {
                        let name = if Some(entity) == self.player_entity {
                            "Player"
                        } else {
                            "Entity"
                        };
                        self.debug_overlay.console_print(format!(
                            "  [{}] {} at ({:.0}, {:.0})",
                            entity.index, name, pos.current.x, pos.current.y
                        ));
                        count += 1;
                    }
                    self.debug_overlay.console_print(format!("Total: {} entities", count));
                }
                ConsoleCommand::ShowStats => {
                    if let Some(renderer) = &self.renderer {
                        let stats = renderer.stats();
                        self.debug_overlay.console_print(format!(
                            "Sprites: {}, Vertices: {}, Draw calls: {}",
                            stats.sprites, stats.vertices, stats.draw_calls
                        ));
                    }
                }
            }
        }
    }
}

impl App for Game {
    fn init(&mut self, window: Arc<WinitWindow>) {
        // Store window reference for egui
        self.window = Some(Arc::clone(&window));

        // Create renderer
        let renderer = pollster::block_on(Renderer::new(Arc::clone(&window)));
        let size = renderer.size();

        // Initialize debug tools
        #[cfg(feature = "debug-tools")]
        {
            let egui_renderer = EguiRenderer::new(
                renderer.device(),
                renderer.surface_format(),
                &window,
                window.scale_factor() as f32,
            );
            self.egui_renderer = Some(egui_renderer);
            self.debug_overlay.log_system(0.0, "Debug tools initialized");
            info!("Debug tools initialized (F12 to toggle)");
        }

        // Load tilemap (Tiled export)
        let player_start = match Tilemap::load("game/assets/Tilesets/test.json") {
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

                #[cfg(feature = "debug-tools")]
                self.debug_overlay.log_system(0.0, format!("Map loaded: test.json ({}x{}) - Tiled format", w, h));

                // Store tilemap as resource
                self.world.insert_resource(tilemap);

                // Store current map for save system
                self.current_map = "game/assets/Tilesets/test.json".to_string();

                start
            }
            Err(e) => {
                error!("Failed to load tilemap: {}", e);
                #[cfg(feature = "debug-tools")]
                self.debug_overlay.log_system(0.0, format!("Map load failed: {}", e));
                Vec2::new(160.0, 120.0)
            }
        };

        // Load player animator from config
        let walk_speed = match load_player_animator() {
            Ok(animator) => {
                let walk_speed = animator.walk_speed();
                info!("Player animator loaded (walk: {} px/s)", walk_speed);

                // Load textures from config paths
                for (name, sheet) in &animator.config.spritesheets {
                    let path = format!("assets/{}", sheet.path);
                    match renderer.load_texture(&path) {
                        Ok(texture) => {
                            info!("Player {} texture loaded: {}x{}", name, texture.size.0, texture.size.1);
                            let bind_group = renderer.create_texture_bind_group(&texture);
                            self.player_textures.insert(name.clone(), (texture, bind_group));
                        }
                        Err(e) => {
                            error!("Failed to load player {} texture: {:?}", name, e);
                        }
                    }
                }

                self.player_animator = Some(animator);
                walk_speed
            }
            Err(e) => {
                error!("Failed to load player config: {}", e);
                120.0 // fallback speed
            }
        };

        // Create player entity with components
        let player = self.world.spawn();
        self.world.insert(player, Position::from_vec2(player_start));
        self.world.insert(player, Velocity::default());
        self.world.insert(player, PlayerControlled::new(walk_speed));
        self.world.insert(player, CameraTarget);
        self.world.insert(player, SpriteRender::new(32.0, 32.0));
        self.world.insert(player, Collider::new(12.0, 8.0)); // Small hitbox at feet
        self.player_entity = Some(player);

        // Initialize camera as resource
        let mut camera = Camera2D::new(size.0 as f32, size.1 as f32);
        camera.follow(player_start, 5.0);
        self.world.insert_resource(camera);

        // Initialize HUD
        self.hud = Some(Hud::new(size.0 as f32, size.1 as f32));

        self.renderer = Some(renderer);
    }

    fn update(&mut self) {
        self.game_time.update();

        // Handle state-specific updates
        match self.game_state {
            GameState::MainMenu => {
                self.update_main_menu();
                return;
            }
            GameState::Paused => {
                self.update_pause_menu();
                return;
            }
            GameState::Settings => {
                self.update_settings();
                return;
            }
            GameState::Playing => {
                // Continue with game logic below
            }
        }

        // === Playing state logic ===

        // Check for escape to pause
        {
            let escape_pressed = self
                .world
                .get_resource::<Input>()
                .map(|i| i.is_key_just_pressed(KeyCode::Escape))
                .unwrap_or(false);
            if escape_pressed {
                info!("Game paused");
                self.pause_menu.reset(); // Reset selection to first item
                self.game_state = GameState::Paused;
                return;
            }
        }

        // Handle hotbar slot selection (keys 1-9)
        if let Some(hud) = &mut self.hud {
            if let Some(input) = self.world.get_resource::<Input>() {
                if input.is_key_just_pressed(KeyCode::Key1) {
                    hud.hotbar.select(0);
                } else if input.is_key_just_pressed(KeyCode::Key2) {
                    hud.hotbar.select(1);
                } else if input.is_key_just_pressed(KeyCode::Key3) {
                    hud.hotbar.select(2);
                } else if input.is_key_just_pressed(KeyCode::Key4) {
                    hud.hotbar.select(3);
                } else if input.is_key_just_pressed(KeyCode::Key5) {
                    hud.hotbar.select(4);
                } else if input.is_key_just_pressed(KeyCode::Key6) {
                    hud.hotbar.select(5);
                } else if input.is_key_just_pressed(KeyCode::Key7) {
                    hud.hotbar.select(6);
                } else if input.is_key_just_pressed(KeyCode::Key8) {
                    hud.hotbar.select(7);
                } else if input.is_key_just_pressed(KeyCode::Key9) {
                    hud.hotbar.select(8);
                }
            }
        }

        // Handle save/load (F5 to save, F9 to load)
        {
            let save_pressed = self
                .world
                .get_resource::<Input>()
                .map(|i| i.is_key_just_pressed(KeyCode::F5))
                .unwrap_or(false);
            let load_pressed = self
                .world
                .get_resource::<Input>()
                .map(|i| i.is_key_just_pressed(KeyCode::F9))
                .unwrap_or(false);

            if save_pressed {
                self.save_game();
            }
            if load_pressed {
                self.load_game();
            }
        }

        // Toggle debug overlay with F12
        #[cfg(feature = "debug-tools")]
        {
            let input = self.world.get_resource::<Input>();

            // F12 toggles main debug overlay
            let toggle_debug = input.map(|i| i.is_key_just_pressed(KeyCode::F12)).unwrap_or(false);
            if toggle_debug {
                self.debug_overlay.toggle();
                let state = if self.debug_overlay.is_enabled() { "enabled" } else { "disabled" };
                self.debug_overlay.log_debug(self.game_time.total_time(), format!("Debug overlay {}", state));
                info!("Debug overlay {}", state);
            }

            // Ctrl+C toggles collision boxes
            let toggle_collision = input
                .map(|i| i.is_key_pressed(KeyCode::LCtrl) && i.is_key_just_pressed(KeyCode::C))
                .unwrap_or(false);
            if toggle_collision && self.debug_overlay.is_enabled() {
                self.debug_overlay.config.show_collisions = !self.debug_overlay.config.show_collisions;
                let state = if self.debug_overlay.config.show_collisions { "enabled" } else { "disabled" };
                self.debug_overlay.log_debug(self.game_time.total_time(), format!("Collision boxes {}", state));
                info!("Collision boxes {}", state);
            }

            // Ctrl+Z toggles z-order labels
            let toggle_zorder = input
                .map(|i| i.is_key_pressed(KeyCode::LCtrl) && i.is_key_just_pressed(KeyCode::Z))
                .unwrap_or(false);
            if toggle_zorder && self.debug_overlay.is_enabled() {
                self.debug_overlay.config.show_z_order = !self.debug_overlay.config.show_z_order;
                let state = if self.debug_overlay.config.show_z_order { "enabled" } else { "disabled" };
                self.debug_overlay.log_debug(self.game_time.total_time(), format!("Z-order labels {}", state));
                info!("Z-order labels {}", state);
            }

            // Process console commands
            self.process_console_commands();
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
            #[cfg(feature = "debug-tools")]
            self.debug_overlay.log_game(
                self.game_time.total_time(),
                format!("Map transition: {} -> {}", map_path, spawn_id),
            );
            self.load_map(&map_path, &spawn_id);
        }

        // Update camera to follow player
        let dt = self.game_time.delta as f32;
        camera_system(&mut self.world, dt);

        // Update player animator based on ECS velocity
        if let (Some(entity), Some(animator)) = (self.player_entity, &mut self.player_animator) {
            let (vx, vy) = self
                .world
                .get::<Velocity>(entity)
                .map(|v| (v.x, v.y))
                .unwrap_or((0.0, 0.0));

            let is_running = self
                .world
                .get_resource::<Input>()
                .map(|i| i.is_key_pressed(KeyCode::LShift) || i.is_key_pressed(KeyCode::RShift))
                .unwrap_or(false);

            animator.update_state(vx, vy, is_running);
            animator.update(dt);
        }
    }

    fn render(&mut self) {
        // Start egui frame (must be before begin_frame for proper input handling)
        #[cfg(feature = "debug-tools")]
        if let (Some(egui_renderer), Some(window)) = (&mut self.egui_renderer, &self.window) {
            egui_renderer.begin_frame(window);
        }

        // Populate collision debug data if visualization is enabled
        #[cfg(feature = "debug-tools")]
        if self.debug_overlay.should_show_collisions() {
            if let Some(renderer) = &self.renderer {
                let size = renderer.size();

                // Get view matrix from camera
                let view_matrix = self
                    .world
                    .get_resource::<Camera2D>()
                    .map(|c| c.view_matrix())
                    .unwrap_or(glam::Mat4::IDENTITY);

                // Set collision data with camera transform
                self.debug_overlay.set_collision_data(view_matrix, (size.0 as f32, size.1 as f32));

                // Add entity collision boxes
                for (entity, collider) in self.world.query::<Collider>() {
                    if let Some(pos) = self.world.get::<Position>(entity) {
                        let half = collider.half_size();
                        let min = pos.current - half;
                        let max = pos.current + half;

                        // Player is green, others are cyan
                        let color = if Some(entity) == self.player_entity {
                            engine_debug::DebugColor::GREEN
                        } else {
                            engine_debug::DebugColor::LIGHT_BLUE
                        };

                        self.debug_overlay.add_entity_box(min, max, color);
                    }
                }

                // Add tile collision boxes from tilemap (visible tiles only)
                if let (Some(tilemap), Some(camera)) = (
                    self.world.get_resource::<Tilemap>(),
                    self.world.get_resource::<Camera2D>(),
                ) {
                    if tilemap.has_collision() {
                        let visible = camera.visible_bounds();
                        let (tile_w, tile_h) = (tilemap.tile_width as f32, tilemap.tile_height as f32);
                        let (map_w, map_h) = (tilemap.width as i32, tilemap.height as i32);

                        // Calculate visible tile range (clamped to map bounds)
                        let start_x = ((visible.0.x / tile_w).floor() as i32).max(0);
                        let start_y = ((visible.0.y / tile_h).floor() as i32).max(0);
                        let end_x = ((visible.1.x / tile_w).ceil() as i32).min(map_w);
                        let end_y = ((visible.1.y / tile_h).ceil() as i32).min(map_h);

                        for ty in start_y..end_y {
                            for tx in start_x..end_x {
                                let idx = (ty * map_w + tx) as usize;
                                if idx < tilemap.collision.len() && tilemap.collision[idx] {
                                    let min = Vec2::new(tx as f32 * tile_w, ty as f32 * tile_h);
                                    let max = min + Vec2::new(tile_w, tile_h);
                                    self.debug_overlay.add_tile_box(min, max);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Populate z-order debug data if visualization is enabled
        #[cfg(feature = "debug-tools")]
        if self.debug_overlay.should_show_zorder() {
            if let Some(renderer) = &self.renderer {
                let size = renderer.size();

                // Get view matrix from camera
                let view_matrix = self
                    .world
                    .get_resource::<Camera2D>()
                    .map(|c| c.view_matrix())
                    .unwrap_or(glam::Mat4::IDENTITY);

                // Set z-order data with camera transform
                self.debug_overlay.set_zorder_data(view_matrix, (size.0 as f32, size.1 as f32));

                // Add entity z-order labels (z = y position for y-sorting)
                for (entity, pos) in self.world.query::<Position>() {
                    let z_order = pos.current.y;
                    let label = if Some(entity) == self.player_entity {
                        "Player"
                    } else {
                        "Entity"
                    };
                    let color = if Some(entity) == self.player_entity {
                        engine_debug::DebugColor::GREEN
                    } else {
                        engine_debug::DebugColor::YELLOW
                    };
                    self.debug_overlay.add_zorder_label(pos.current, z_order, label, color);
                }

                // Add layer info for legend - show actual tilemap layer names
                if let Some(tilemap) = self.world.get_resource::<Tilemap>() {
                    let below_indices = tilemap.below_layers();
                    let above_indices = tilemap.above_layers();
                    let entity_count = self.world.query::<Position>().count();

                    let mut layer_index = 0;

                    // Show layers below player (with actual names)
                    for idx in &below_indices {
                        if let Some(layer) = tilemap.layers.get(*idx) {
                            let label = format!("↓ {} (z:{})", layer.name, layer.z_order);
                            self.debug_overlay.add_layer_info(
                                layer_index,
                                label,
                                engine_debug::DebugColor::from_rgb(139, 90, 43),
                                1,
                            );
                            layer_index += 1;
                        }
                    }

                    // Show player/entities layer
                    self.debug_overlay.add_layer_info(
                        layer_index,
                        "▶ PLAYER (Y-sorted)",
                        engine_debug::DebugColor::GREEN,
                        entity_count,
                    );
                    layer_index += 1;

                    // Show layers above player (with actual names)
                    for idx in &above_indices {
                        if let Some(layer) = tilemap.layers.get(*idx) {
                            let label = format!("↑ {} (z:{})", layer.name, layer.z_order);
                            self.debug_overlay.add_layer_info(
                                layer_index,
                                label,
                                engine_debug::DebugColor::from_rgb(135, 206, 250),
                                1,
                            );
                            layer_index += 1;
                        }
                    }
                }
            }
        }

        // Populate ECS inspector data when panel is visible
        #[cfg(feature = "debug-tools")]
        if self.debug_overlay.panels.show_ecs_inspector {
            self.debug_overlay.clear_ecs_data();

            // Count entities and components
            let mut entity_count = 0;
            let mut component_count = 0;

            // Collect entity data
            for (entity, pos) in self.world.query::<Position>() {
                entity_count += 1;

                let name = if Some(entity) == self.player_entity {
                    "Player"
                } else {
                    "Entity"
                };

                let mut entity_info = engine_debug::EntityInfo::new(entity.index, name);

                // Add Position component
                entity_info.add_component(engine_debug::ComponentInfo::vec2(
                    "Position",
                    pos.current.x,
                    pos.current.y,
                    false,
                ));
                component_count += 1;

                // Add Velocity if present
                if let Some(vel) = self.world.get::<Velocity>(entity) {
                    entity_info.add_component(engine_debug::ComponentInfo::vec2(
                        "Velocity",
                        vel.x,
                        vel.y,
                        false,
                    ));
                    component_count += 1;
                }

                // Add Collider if present
                if let Some(collider) = self.world.get::<Collider>(entity) {
                    let half = collider.half_size();
                    entity_info.add_component(engine_debug::ComponentInfo::size(
                        "Collider",
                        half.x * 2.0,
                        half.y * 2.0,
                    ));
                    component_count += 1;
                }

                // Add SpriteRender if present
                if let Some(sprite) = self.world.get::<SpriteRender>(entity) {
                    let size = sprite.size();
                    entity_info.add_component(engine_debug::ComponentInfo::size(
                        "SpriteRender",
                        size.x,
                        size.y,
                    ));
                    component_count += 1;
                }

                self.debug_overlay.add_entity(entity_info);
            }

            self.debug_overlay.set_ecs_stats(entity_count, component_count);
        }

        if let Some(renderer) = &mut self.renderer {
            // Apply camera (only for gameplay)
            if self.game_state == GameState::Playing {
                if let Some(camera) = self.world.get_resource::<Camera2D>() {
                    renderer.set_camera(camera);
                }
            }

            match renderer.begin_frame() {
                Ok(mut frame) => {
                    // Render based on game state
                    match self.game_state {
                        GameState::MainMenu => {
                            // Render main menu using engine_ui sprites
                            let size = renderer.size();
                            let menu_sprites = self.main_menu.sprites((size.0 as f32, size.1 as f32));
                            renderer.set_screen_space();
                            for sprite in menu_sprites {
                                renderer.draw_sprite(&sprite);
                            }
                            renderer.flush_sprites(&mut frame, None);
                            renderer.set_world_space();
                        }
                        GameState::Settings => {
                            // Render settings menu
                            let size = renderer.size();
                            let menu_sprites = self.settings_menu.sprites((size.0 as f32, size.1 as f32));
                            renderer.set_screen_space();
                            for sprite in menu_sprites {
                                renderer.draw_sprite(&sprite);
                            }
                            renderer.flush_sprites(&mut frame, None);
                            renderer.set_world_space();
                        }
                        GameState::Playing | GameState::Paused => {
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

                                // Flush below layers
                                renderer.flush_sprites(&mut frame, self.tileset_bind_group.as_ref());

                                // 2. Render player with animation texture in SEPARATE batch
                                if let Some(animator) = &self.player_animator {
                                    let alpha = self.game_time.alpha() as f32;
                                    for (entity, _sprite_render) in self.world.query::<SpriteRender>() {
                                        if let Some(pos) = self.world.get::<Position>(entity) {
                                            let render_pos = pos.interpolated(alpha);
                                            let frame_size = animator.frame_size() as f32;

                                            // Create sprite with animation region
                                            let mut sprite = Sprite::new(render_pos, Vec2::new(frame_size, frame_size));
                                            if let Some(region) = animator.current_region() {
                                                sprite.region = region;
                                            }

                                            // Handle horizontal flip for left-facing direction
                                            if animator.flip_x {
                                                std::mem::swap(&mut sprite.region.u_min, &mut sprite.region.u_max);
                                            }

                                            renderer.draw_sprite(&sprite);
                                        }
                                    }

                                    // Get the correct texture based on player state
                                    let sheet_name = animator.state.key();
                                    let player_bind_group = self.player_textures
                                        .get(sheet_name)
                                        .map(|(_, bg)| bg);

                                    // Flush player with animation texture
                                    renderer.flush_sprites_no_clear(&mut frame, player_bind_group);
                                }

                                // 3. Render layers ABOVE entities
                                for layer_idx in tilemap.above_layers() {
                                    let sprites = tilemap.get_visible_sprites(layer_idx, camera);
                                    for (sprite, _tileset_idx) in sprites {
                                        renderer.draw_sprite(&sprite);
                                    }
                                }

                                // Flush above layers
                                renderer.flush_sprites_no_clear(&mut frame, self.tileset_bind_group.as_ref());
                            }

                            // Render HUD in screen-space (on top of world, no clear)
                            if let Some(hud) = &self.hud {
                                renderer.set_screen_space();
                                for sprite in hud.sprites() {
                                    renderer.draw_sprite(&sprite);
                                }
                                renderer.flush_sprites_no_clear(&mut frame, None);
                                renderer.set_world_space();
                            }

                            // Render pause menu overlay when paused
                            if self.game_state == GameState::Paused {
                                let size = renderer.size();
                                let menu_sprites = self.pause_menu.sprites((size.0 as f32, size.1 as f32));
                                renderer.set_screen_space();
                                for sprite in menu_sprites {
                                    renderer.draw_sprite(&sprite);
                                }
                                renderer.flush_sprites_no_clear(&mut frame, None);
                                renderer.set_world_space();
                            }
                        }
                    }

                    // Update render stats for profiler
                    #[cfg(feature = "debug-tools")]
                    {
                        let stats = renderer.stats();
                        self.debug_overlay.update_render_stats(
                            stats.sprites,
                            stats.vertices,
                            stats.draw_calls,
                            stats.texture_binds,
                        );
                    }

                    // Render debug overlay on top (egui)
                    #[cfg(feature = "debug-tools")]
                    if let (Some(egui_renderer), Some(window)) = (&mut self.egui_renderer, &self.window) {
                        // Render debug overlay UI
                        self.debug_overlay.render(egui_renderer.context(), &self.game_time);

                        // End egui frame and render to the current frame
                        egui_renderer.end_frame_and_render(
                            renderer.device(),
                            renderer.queue(),
                            &mut frame.encoder,
                            &frame.view,
                            window,
                        );
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

    fn on_window_event(&mut self, event: &WindowEvent) -> bool {
        // Pass events to egui for input handling
        #[cfg(feature = "debug-tools")]
        if let (Some(egui_renderer), Some(window)) = (&mut self.egui_renderer, &self.window) {
            return egui_renderer.handle_event(window, event);
        }
        false
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
        // Update HUD layout
        if let Some(hud) = &mut self.hud {
            hud.resize(width as f32, height as f32);
        }
        #[cfg(feature = "debug-tools")]
        if let Some(egui_renderer) = &mut self.egui_renderer {
            let scale = self.window.as_ref().map(|w| w.scale_factor()).unwrap_or(1.0) as f32;
            egui_renderer.resize(width, height, scale);
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
