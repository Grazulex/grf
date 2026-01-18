//! Debug overlay with panels
//!
//! Provides the main debug UI overlay with various panels.

#![cfg(feature = "debug-tools")]

use crate::DebugConfig;
use egui::{Context, RichText, Window, Color32, Stroke, Rect, Pos2};
use engine_core::GameTime;
use glam::{Vec2, Mat4};

/// A debug collision box to render
#[derive(Debug, Clone)]
pub struct DebugBox {
    /// World-space minimum corner
    pub min: Vec2,
    /// World-space maximum corner
    pub max: Vec2,
    /// Color for the box
    pub color: Color32,
    /// Label to display (optional)
    pub label: Option<String>,
}

impl DebugBox {
    /// Create a new debug box
    #[must_use]
    pub fn new(min: Vec2, max: Vec2, color: Color32) -> Self {
        Self {
            min,
            max,
            color,
            label: None,
        }
    }

    /// Add a label to the box
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Collision debug data for rendering
#[derive(Debug, Default)]
pub struct CollisionDebugData {
    /// Entity collision boxes
    pub entity_boxes: Vec<DebugBox>,
    /// Tile collision boxes
    pub tile_boxes: Vec<DebugBox>,
    /// Camera view matrix for world-to-screen conversion
    pub view_matrix: Mat4,
    /// Screen size
    pub screen_size: (f32, f32),
}

/// Panel visibility state
#[derive(Debug, Default)]
pub struct PanelState {
    /// Show FPS/performance panel
    pub show_performance: bool,
    /// Show ECS entity inspector
    pub show_ecs_inspector: bool,
    /// Show collision overlay settings
    pub show_collision: bool,
    /// Show event log
    pub show_event_log: bool,
    /// Show debug console
    pub show_console: bool,
}

/// Debug overlay manager
pub struct DebugOverlay {
    /// Debug configuration
    pub config: DebugConfig,
    /// Panel visibility
    pub panels: PanelState,
    /// FPS history for graph
    fps_history: Vec<f32>,
    /// Frame time history
    frame_time_history: Vec<f32>,
    /// Console input buffer
    console_input: String,
    /// Console output history
    console_output: Vec<String>,
    /// Collision debug data
    collision_data: CollisionDebugData,
}

impl Default for DebugOverlay {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugOverlay {
    /// Create a new debug overlay
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: DebugConfig::new(),
            panels: PanelState::default(),
            fps_history: Vec::with_capacity(120),
            frame_time_history: Vec::with_capacity(120),
            console_input: String::new(),
            console_output: vec![
                "Debug Console initialized.".to_string(),
                "Type 'help' for available commands.".to_string(),
            ],
            collision_data: CollisionDebugData::default(),
        }
    }

    /// Check if collision visualization should be rendered
    #[must_use]
    pub fn should_show_collisions(&self) -> bool {
        self.config.enabled && self.config.show_collisions
    }

    /// Update collision debug data
    pub fn set_collision_data(&mut self, view_matrix: Mat4, screen_size: (f32, f32)) {
        self.collision_data.view_matrix = view_matrix;
        self.collision_data.screen_size = screen_size;
        self.collision_data.entity_boxes.clear();
        self.collision_data.tile_boxes.clear();
    }

    /// Add an entity collision box
    pub fn add_entity_box(&mut self, min: Vec2, max: Vec2, color: Color32) {
        self.collision_data.entity_boxes.push(DebugBox::new(min, max, color));
    }

    /// Add a tile collision box
    pub fn add_tile_box(&mut self, min: Vec2, max: Vec2) {
        self.collision_data.tile_boxes.push(
            DebugBox::new(min, max, Color32::from_rgba_unmultiplied(255, 165, 0, 100))
        );
    }

    /// Toggle debug mode (F12)
    pub fn toggle(&mut self) {
        self.config.toggle();
    }

    /// Check if debug mode is enabled
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Update FPS history
    pub fn update_fps(&mut self, game_time: &GameTime) {
        let fps = game_time.fps() as f32;
        let frame_time = (game_time.delta * 1000.0) as f32; // Convert to ms

        self.fps_history.push(fps);
        self.frame_time_history.push(frame_time);

        // Keep last 120 frames
        if self.fps_history.len() > 120 {
            self.fps_history.remove(0);
            self.frame_time_history.remove(0);
        }
    }

    /// Render the debug overlay
    pub fn render(&mut self, ctx: &Context, game_time: &GameTime) {
        if !self.config.enabled {
            return;
        }

        // Update FPS history
        self.update_fps(game_time);

        // Render collision boxes in world space (before UI panels)
        if self.config.show_collisions {
            self.render_collision_boxes(ctx);
        }

        // Main debug menu bar
        self.render_menu_bar(ctx);

        // Render active panels
        if self.panels.show_performance {
            self.render_performance_panel(ctx, game_time);
        }

        if self.panels.show_ecs_inspector {
            self.render_ecs_inspector(ctx);
        }

        if self.panels.show_collision {
            self.render_collision_panel(ctx);
        }

        if self.panels.show_event_log {
            self.render_event_log(ctx);
        }

        if self.panels.show_console {
            self.render_console(ctx);
        }
    }

    /// Convert world position to screen position
    fn world_to_screen(&self, world_pos: Vec2) -> Pos2 {
        let (screen_w, screen_h) = self.collision_data.screen_size;

        // Apply view matrix transformation
        let pos4 = self.collision_data.view_matrix * glam::Vec4::new(world_pos.x, world_pos.y, 0.0, 1.0);

        // NDC to screen coordinates
        let screen_x = (pos4.x + 1.0) * 0.5 * screen_w;
        let screen_y = (1.0 - pos4.y) * 0.5 * screen_h; // Flip Y

        Pos2::new(screen_x, screen_y)
    }

    /// Render collision debug boxes
    fn render_collision_boxes(&self, ctx: &Context) {
        let painter = ctx.layer_painter(egui::LayerId::background());

        // Render tile collision boxes (orange, semi-transparent fill)
        for debug_box in &self.collision_data.tile_boxes {
            let min = self.world_to_screen(debug_box.min);
            let max = self.world_to_screen(debug_box.max);

            let rect = Rect::from_min_max(
                Pos2::new(min.x.min(max.x), min.y.min(max.y)),
                Pos2::new(min.x.max(max.x), min.y.max(max.y)),
            );

            painter.rect(
                rect,
                0.0,
                Color32::from_rgba_unmultiplied(255, 165, 0, 50), // Orange fill
                Stroke::new(1.0, Color32::from_rgb(255, 165, 0)),  // Orange stroke
            );
        }

        // Render entity collision boxes (colored based on type)
        for debug_box in &self.collision_data.entity_boxes {
            let min = self.world_to_screen(debug_box.min);
            let max = self.world_to_screen(debug_box.max);

            let rect = Rect::from_min_max(
                Pos2::new(min.x.min(max.x), min.y.min(max.y)),
                Pos2::new(min.x.max(max.x), min.y.max(max.y)),
            );

            let fill_color = Color32::from_rgba_unmultiplied(
                debug_box.color.r(),
                debug_box.color.g(),
                debug_box.color.b(),
                50,
            );

            painter.rect(
                rect,
                0.0,
                fill_color,
                Stroke::new(2.0, debug_box.color),
            );

            // Draw label if present
            if let Some(label) = &debug_box.label {
                painter.text(
                    Pos2::new(rect.min.x, rect.min.y - 12.0),
                    egui::Align2::LEFT_BOTTOM,
                    label,
                    egui::FontId::proportional(10.0),
                    debug_box.color,
                );
            }
        }
    }

    /// Render the main menu bar
    fn render_menu_bar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("debug_menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.label(RichText::new("🔧 DEBUG").strong());
                ui.separator();

                ui.toggle_value(&mut self.panels.show_performance, "📊 Performance");
                ui.toggle_value(&mut self.panels.show_ecs_inspector, "🔍 ECS");
                ui.toggle_value(&mut self.panels.show_collision, "📦 Collision");
                ui.toggle_value(&mut self.panels.show_event_log, "📜 Events");
                ui.toggle_value(&mut self.panels.show_console, "💻 Console");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let fps = self.fps_history.last().copied().unwrap_or(0.0);
                    let color = if fps >= 55.0 {
                        egui::Color32::GREEN
                    } else if fps >= 30.0 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::RED
                    };
                    ui.label(RichText::new(format!("{:.0} FPS", fps)).color(color));
                });
            });
        });
    }

    /// Render performance panel
    fn render_performance_panel(&mut self, ctx: &Context, game_time: &GameTime) {
        Window::new("📊 Performance")
            .default_size([300.0, 200.0])
            .show(ctx, |ui| {
                ui.heading("Frame Stats");

                let fps = game_time.fps();
                let frame_time = game_time.delta * 1000.0;

                ui.horizontal(|ui| {
                    ui.label("FPS:");
                    ui.label(RichText::new(format!("{:.1}", fps)).strong());
                });

                ui.horizontal(|ui| {
                    ui.label("Frame Time:");
                    ui.label(format!("{:.2} ms", frame_time));
                });

                ui.horizontal(|ui| {
                    ui.label("Total Time:");
                    ui.label(format!("{:.1} s", game_time.total_time()));
                });

                ui.separator();
                ui.heading("FPS Graph");

                // Simple FPS graph
                let points: Vec<[f64; 2]> = self
                    .fps_history
                    .iter()
                    .enumerate()
                    .map(|(i, &fps)| [i as f64, fps as f64])
                    .collect();

                if !points.is_empty() {
                    egui_plot::Plot::new("fps_plot")
                        .height(100.0)
                        .show_axes(false)
                        .show(ui, |plot_ui| {
                            plot_ui.line(egui_plot::Line::new(points).color(egui::Color32::GREEN));
                        });
                }
            });
    }

    /// Render ECS inspector panel (stub for now)
    fn render_ecs_inspector(&mut self, ctx: &Context) {
        Window::new("🔍 ECS Inspector")
            .default_size([350.0, 400.0])
            .show(ctx, |ui| {
                ui.label("ECS Inspector - Coming Soon");
                ui.separator();
                ui.label("Will display:");
                ui.label("• Entity count");
                ui.label("• Component types");
                ui.label("• Entity details");
                ui.label("• Resource list");
            });
    }

    /// Render collision panel
    fn render_collision_panel(&mut self, ctx: &Context) {
        Window::new("📦 Collision Settings")
            .default_size([250.0, 150.0])
            .show(ctx, |ui| {
                ui.checkbox(&mut self.config.show_collisions, "Show Collision Boxes (Ctrl+C)");
                ui.checkbox(&mut self.config.show_z_order, "Show Z-Order Labels (Ctrl+Z)");
                ui.checkbox(&mut self.config.show_grid, "Show Tile Grid (Ctrl+G)");
            });
    }

    /// Render event log panel (stub for now)
    fn render_event_log(&mut self, ctx: &Context) {
        Window::new("📜 Event Log")
            .default_size([400.0, 300.0])
            .show(ctx, |ui| {
                ui.label("Event Log - Coming Soon");
                ui.separator();
                ui.label("Will display:");
                ui.label("• Input events");
                ui.label("• Game events");
                ui.label("• System events");
            });
    }

    /// Render console panel
    fn render_console(&mut self, ctx: &Context) {
        Window::new("💻 Debug Console")
            .default_size([500.0, 300.0])
            .show(ctx, |ui| {
                // Output area
                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for line in &self.console_output {
                            ui.label(line);
                        }
                    });

                ui.separator();

                // Input area
                ui.horizontal(|ui| {
                    ui.label(">");
                    let response = ui.text_edit_singleline(&mut self.console_input);

                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.execute_command();
                    }

                    if ui.button("Run").clicked() {
                        self.execute_command();
                    }
                });
            });
    }

    /// Execute a console command
    fn execute_command(&mut self) {
        let command = self.console_input.trim().to_string();
        if command.is_empty() {
            return;
        }

        self.console_output.push(format!("> {}", command));

        // Basic command handling
        match command.as_str() {
            "help" => {
                self.console_output.push("Available commands:".to_string());
                self.console_output.push("  help - Show this help".to_string());
                self.console_output.push("  clear - Clear console".to_string());
                self.console_output.push("  fps - Show current FPS".to_string());
                self.console_output.push("  collision on/off - Toggle collision boxes".to_string());
            }
            "clear" => {
                self.console_output.clear();
            }
            "fps" => {
                let fps = self.fps_history.last().copied().unwrap_or(0.0);
                self.console_output.push(format!("Current FPS: {:.1}", fps));
            }
            "collision on" => {
                self.config.show_collisions = true;
                self.console_output.push("Collision boxes enabled".to_string());
            }
            "collision off" => {
                self.config.show_collisions = false;
                self.console_output.push("Collision boxes disabled".to_string());
            }
            _ => {
                self.console_output.push(format!("Unknown command: {}", command));
            }
        }

        self.console_input.clear();
    }
}
