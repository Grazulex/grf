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

/// A z-order label to display on an entity
#[derive(Debug, Clone)]
pub struct ZOrderLabel {
    /// World-space position (center of entity)
    pub position: Vec2,
    /// Z-order value (typically Y position for y-sorting)
    pub z_order: f32,
    /// Entity name/type for display
    pub label: String,
    /// Color for the label
    pub color: Color32,
}

impl ZOrderLabel {
    /// Create a new z-order label
    #[must_use]
    pub fn new(position: Vec2, z_order: f32, label: impl Into<String>, color: Color32) -> Self {
        Self {
            position,
            z_order,
            label: label.into(),
            color,
        }
    }
}

/// Layer information for the legend
#[derive(Debug, Clone)]
pub struct LayerInfo {
    /// Layer index
    pub index: usize,
    /// Layer name
    pub name: String,
    /// Color for display
    pub color: Color32,
    /// Number of items in this layer
    pub count: usize,
}

/// Z-order debug data for rendering
#[derive(Debug, Default)]
pub struct ZOrderDebugData {
    /// Entity z-order labels
    pub labels: Vec<ZOrderLabel>,
    /// Layer information for legend
    pub layers: Vec<LayerInfo>,
    /// Camera view matrix for world-to-screen conversion
    pub view_matrix: Mat4,
    /// Screen size
    pub screen_size: (f32, f32),
}

/// Event type for categorization and filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    /// Input events (key press, mouse click)
    Input,
    /// Game events (map transition, item pickup, dialogue)
    Game,
    /// System events (init, resize, error)
    System,
    /// Debug events (toggle, command)
    Debug,
}

impl EventType {
    /// Get the display name for this event type
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            EventType::Input => "Input",
            EventType::Game => "Game",
            EventType::System => "System",
            EventType::Debug => "Debug",
        }
    }

    /// Get the color for this event type
    #[must_use]
    pub fn color(&self) -> Color32 {
        match self {
            EventType::Input => Color32::from_rgb(100, 200, 255),  // Light blue
            EventType::Game => Color32::from_rgb(100, 255, 100),   // Light green
            EventType::System => Color32::from_rgb(255, 200, 100), // Orange
            EventType::Debug => Color32::from_rgb(200, 150, 255),  // Purple
        }
    }
}

/// A single event log entry
#[derive(Debug, Clone)]
pub struct EventLogEntry {
    /// Timestamp in seconds since game start
    pub timestamp: f64,
    /// Type of event
    pub event_type: EventType,
    /// Event message
    pub message: String,
}

impl EventLogEntry {
    /// Create a new event log entry
    #[must_use]
    pub fn new(timestamp: f64, event_type: EventType, message: impl Into<String>) -> Self {
        Self {
            timestamp,
            event_type,
            message: message.into(),
        }
    }
}

/// Event log filter state
#[derive(Debug, Clone)]
pub struct EventLogFilter {
    /// Show input events
    pub show_input: bool,
    /// Show game events
    pub show_game: bool,
    /// Show system events
    pub show_system: bool,
    /// Show debug events
    pub show_debug: bool,
}

impl Default for EventLogFilter {
    fn default() -> Self {
        Self {
            show_input: true,
            show_game: true,
            show_system: true,
            show_debug: true,
        }
    }
}

impl EventLogFilter {
    /// Check if an event type should be shown
    #[must_use]
    pub fn should_show(&self, event_type: EventType) -> bool {
        match event_type {
            EventType::Input => self.show_input,
            EventType::Game => self.show_game,
            EventType::System => self.show_system,
            EventType::Debug => self.show_debug,
        }
    }
}

/// Console command that requires game-side execution
#[derive(Debug, Clone)]
pub enum ConsoleCommand {
    /// Teleport player to position
    Teleport { x: f32, y: f32 },
    /// Set player movement speed
    SetSpeed(f32),
    /// Set game timescale
    SetTimescale(f32),
    /// Request player position (response via console output)
    GetPosition,
    /// Request entity list
    ListEntities,
    /// Request render stats
    ShowStats,
}

/// Component value for display/editing
#[derive(Debug, Clone)]
pub enum ComponentValue {
    /// Vec2 value (Position, Velocity)
    Vec2 { x: f32, y: f32 },
    /// Float value
    Float(f32),
    /// Integer value
    Int(i32),
    /// Boolean value
    Bool(bool),
    /// String value
    String(String),
    /// Size (width, height)
    Size { width: f32, height: f32 },
}

/// Component information for the inspector
#[derive(Debug, Clone)]
pub struct ComponentInfo {
    /// Component name
    pub name: String,
    /// Component value
    pub value: ComponentValue,
    /// Whether this component is editable
    pub editable: bool,
}

impl ComponentInfo {
    /// Create a new component info
    #[must_use]
    pub fn new(name: impl Into<String>, value: ComponentValue, editable: bool) -> Self {
        Self {
            name: name.into(),
            value,
            editable,
        }
    }

    /// Create a Vec2 component
    #[must_use]
    pub fn vec2(name: impl Into<String>, x: f32, y: f32, editable: bool) -> Self {
        Self::new(name, ComponentValue::Vec2 { x, y }, editable)
    }

    /// Create a Size component
    #[must_use]
    pub fn size(name: impl Into<String>, width: f32, height: f32) -> Self {
        Self::new(name, ComponentValue::Size { width, height }, false)
    }

    /// Create a Bool component
    #[must_use]
    pub fn bool(name: impl Into<String>, value: bool) -> Self {
        Self::new(name, ComponentValue::Bool(value), false)
    }
}

/// Entity information for the inspector
#[derive(Debug, Clone)]
pub struct EntityInfo {
    /// Entity ID
    pub id: u32,
    /// Entity name/label
    pub name: String,
    /// Components on this entity
    pub components: Vec<ComponentInfo>,
}

impl EntityInfo {
    /// Create a new entity info
    #[must_use]
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            components: Vec::new(),
        }
    }

    /// Add a component
    pub fn add_component(&mut self, component: ComponentInfo) {
        self.components.push(component);
    }
}

/// ECS inspector data
#[derive(Debug, Default)]
pub struct EcsInspectorData {
    /// All entities
    pub entities: Vec<EntityInfo>,
    /// Total entity count
    pub entity_count: usize,
    /// Total component count
    pub component_count: usize,
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
    /// Show z-order/layer legend
    pub show_zorder: bool,
    /// Show event log
    pub show_event_log: bool,
    /// Show debug console
    pub show_console: bool,
}

/// Render statistics for display
#[derive(Debug, Clone, Default)]
pub struct DisplayRenderStats {
    /// Sprites rendered
    pub sprites: usize,
    /// Vertices submitted
    pub vertices: usize,
    /// Draw calls
    pub draw_calls: usize,
    /// Texture binds
    pub texture_binds: usize,
}

/// Debug overlay manager
/// Maximum number of events to keep in the log
const MAX_EVENT_LOG_ENTRIES: usize = 200;

pub struct DebugOverlay {
    /// Debug configuration
    pub config: DebugConfig,
    /// Panel visibility
    pub panels: PanelState,
    /// FPS history for graph
    fps_history: Vec<f32>,
    /// Frame time history
    frame_time_history: Vec<f32>,
    /// Draw calls history
    draw_calls_history: Vec<f32>,
    /// Sprites history
    sprites_history: Vec<f32>,
    /// Current render stats
    render_stats: DisplayRenderStats,
    /// Memory usage history (MB)
    memory_history: Vec<f32>,
    /// Console input buffer
    console_input: String,
    /// Console output history
    console_output: Vec<String>,
    /// Collision debug data
    collision_data: CollisionDebugData,
    /// Z-order debug data
    zorder_data: ZOrderDebugData,
    /// ECS inspector data
    ecs_data: EcsInspectorData,
    /// Currently selected entity ID
    selected_entity: Option<u32>,
    /// Event log entries
    event_log: Vec<EventLogEntry>,
    /// Event log filter
    event_filter: EventLogFilter,
    /// Auto-scroll to bottom
    event_log_auto_scroll: bool,
    /// Pending console commands for game execution
    pending_commands: Vec<ConsoleCommand>,
    /// Command history for up/down navigation
    command_history: Vec<String>,
    /// Current position in command history
    history_index: Option<usize>,
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
            draw_calls_history: Vec::with_capacity(120),
            sprites_history: Vec::with_capacity(120),
            render_stats: DisplayRenderStats::default(),
            memory_history: Vec::with_capacity(120),
            console_input: String::new(),
            console_output: vec![
                "Debug Console initialized.".to_string(),
                "Type 'help' for available commands.".to_string(),
            ],
            collision_data: CollisionDebugData::default(),
            zorder_data: ZOrderDebugData::default(),
            ecs_data: EcsInspectorData::default(),
            selected_entity: None,
            event_log: Vec::with_capacity(MAX_EVENT_LOG_ENTRIES),
            event_filter: EventLogFilter::default(),
            event_log_auto_scroll: true,
            pending_commands: Vec::new(),
            command_history: Vec::new(),
            history_index: None,
        }
    }

    /// Take pending commands for game execution (drains the queue)
    pub fn take_pending_commands(&mut self) -> Vec<ConsoleCommand> {
        std::mem::take(&mut self.pending_commands)
    }

    /// Add a response line to the console output
    pub fn console_print(&mut self, message: impl Into<String>) {
        self.console_output.push(message.into());
    }

    /// Log an event to the event log
    pub fn log_event(&mut self, timestamp: f64, event_type: EventType, message: impl Into<String>) {
        let entry = EventLogEntry::new(timestamp, event_type, message);
        self.event_log.push(entry);

        // Keep only the last MAX_EVENT_LOG_ENTRIES
        if self.event_log.len() > MAX_EVENT_LOG_ENTRIES {
            self.event_log.remove(0);
        }
    }

    /// Log an input event
    pub fn log_input(&mut self, timestamp: f64, message: impl Into<String>) {
        self.log_event(timestamp, EventType::Input, message);
    }

    /// Log a game event
    pub fn log_game(&mut self, timestamp: f64, message: impl Into<String>) {
        self.log_event(timestamp, EventType::Game, message);
    }

    /// Log a system event
    pub fn log_system(&mut self, timestamp: f64, message: impl Into<String>) {
        self.log_event(timestamp, EventType::System, message);
    }

    /// Log a debug event
    pub fn log_debug(&mut self, timestamp: f64, message: impl Into<String>) {
        self.log_event(timestamp, EventType::Debug, message);
    }

    /// Clear all events from the log
    pub fn clear_event_log(&mut self) {
        self.event_log.clear();
    }

    /// Get the number of events by type
    pub fn event_counts(&self) -> (usize, usize, usize, usize) {
        let mut input = 0;
        let mut game = 0;
        let mut system = 0;
        let mut debug = 0;

        for entry in &self.event_log {
            match entry.event_type {
                EventType::Input => input += 1,
                EventType::Game => game += 1,
                EventType::System => system += 1,
                EventType::Debug => debug += 1,
            }
        }

        (input, game, system, debug)
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

    /// Check if z-order visualization should be rendered
    #[must_use]
    pub fn should_show_zorder(&self) -> bool {
        self.config.enabled && self.config.show_z_order
    }

    /// Update z-order debug data
    pub fn set_zorder_data(&mut self, view_matrix: Mat4, screen_size: (f32, f32)) {
        self.zorder_data.view_matrix = view_matrix;
        self.zorder_data.screen_size = screen_size;
        self.zorder_data.labels.clear();
        self.zorder_data.layers.clear();
    }

    /// Add a z-order label for an entity
    pub fn add_zorder_label(&mut self, position: Vec2, z_order: f32, label: impl Into<String>, color: Color32) {
        self.zorder_data.labels.push(ZOrderLabel::new(position, z_order, label, color));
    }

    /// Add layer info for the legend
    pub fn add_layer_info(&mut self, index: usize, name: impl Into<String>, color: Color32, count: usize) {
        self.zorder_data.layers.push(LayerInfo {
            index,
            name: name.into(),
            color,
            count,
        });
    }

    /// Clear ECS inspector data for new frame
    pub fn clear_ecs_data(&mut self) {
        self.ecs_data.entities.clear();
        self.ecs_data.entity_count = 0;
        self.ecs_data.component_count = 0;
    }

    /// Set ECS statistics
    pub fn set_ecs_stats(&mut self, entity_count: usize, component_count: usize) {
        self.ecs_data.entity_count = entity_count;
        self.ecs_data.component_count = component_count;
    }

    /// Add an entity to the inspector
    pub fn add_entity(&mut self, entity: EntityInfo) {
        self.ecs_data.entities.push(entity);
    }

    /// Get the currently selected entity ID
    #[must_use]
    pub fn selected_entity(&self) -> Option<u32> {
        self.selected_entity
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

    /// Update render statistics
    pub fn update_render_stats(&mut self, sprites: usize, vertices: usize, draw_calls: usize, texture_binds: usize) {
        self.render_stats.sprites = sprites;
        self.render_stats.vertices = vertices;
        self.render_stats.draw_calls = draw_calls;
        self.render_stats.texture_binds = texture_binds;

        // Add to history
        self.draw_calls_history.push(draw_calls as f32);
        self.sprites_history.push(sprites as f32);

        // Keep last 120 frames
        if self.draw_calls_history.len() > 120 {
            self.draw_calls_history.remove(0);
            self.sprites_history.remove(0);
        }
    }

    /// Update memory stats (approximate heap usage in MB)
    pub fn update_memory(&mut self) {
        // Use a simple approximation based on allocated memory
        // In a real scenario, you might use a memory allocator tracker
        // For now, we'll estimate based on the history vectors themselves
        let estimated_mb = 0.0; // Placeholder - real tracking requires custom allocator
        self.memory_history.push(estimated_mb);

        if self.memory_history.len() > 120 {
            self.memory_history.remove(0);
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

        // Render z-order labels in world space (before UI panels)
        if self.config.show_z_order {
            self.render_zorder_labels(ctx);
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

        if self.panels.show_zorder {
            self.render_layer_legend(ctx);
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
                ui.toggle_value(&mut self.panels.show_zorder, "📐 Z-Order");
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
        Window::new("📊 Performance Profiler")
            .default_size([350.0, 450.0])
            .show(ctx, |ui| {
                // Frame Stats Section
                ui.collapsing(RichText::new("Frame Stats").strong(), |ui| {
                    let fps = game_time.fps();
                    let frame_time = game_time.delta * 1000.0;

                    // Calculate min/max/avg FPS
                    let (min_fps, max_fps, avg_fps) = if !self.fps_history.is_empty() {
                        let min = self.fps_history.iter().cloned().fold(f32::MAX, f32::min);
                        let max = self.fps_history.iter().cloned().fold(f32::MIN, f32::max);
                        let avg = self.fps_history.iter().sum::<f32>() / self.fps_history.len() as f32;
                        (min, max, avg)
                    } else {
                        (0.0, 0.0, 0.0)
                    };

                    egui::Grid::new("frame_stats_grid")
                        .num_columns(2)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("FPS:");
                            let fps_color = if fps >= 55.0 {
                                Color32::GREEN
                            } else if fps >= 30.0 {
                                Color32::YELLOW
                            } else {
                                Color32::RED
                            };
                            ui.label(RichText::new(format!("{:.1}", fps)).color(fps_color).strong());
                            ui.end_row();

                            ui.label("Frame Time:");
                            ui.label(format!("{:.2} ms", frame_time));
                            ui.end_row();

                            ui.label("Min/Max/Avg FPS:");
                            ui.label(format!("{:.0} / {:.0} / {:.0}", min_fps, max_fps, avg_fps));
                            ui.end_row();

                            ui.label("Total Time:");
                            ui.label(format!("{:.1} s", game_time.total_time()));
                            ui.end_row();
                        });
                }).header_response.clicked();

                ui.separator();

                // FPS Graph
                ui.collapsing(RichText::new("FPS Graph").strong(), |ui| {
                    let fps_points: Vec<[f64; 2]> = self
                        .fps_history
                        .iter()
                        .enumerate()
                        .map(|(i, &fps)| [i as f64, fps as f64])
                        .collect();

                    if !fps_points.is_empty() {
                        egui_plot::Plot::new("fps_plot")
                            .height(80.0)
                            .show_axes([false, true])
                            .include_y(0.0)
                            .include_y(70.0)
                            .show(ui, |plot_ui| {
                                plot_ui.line(egui_plot::Line::new(fps_points).color(Color32::GREEN).name("FPS"));
                                // Draw 60 FPS target line
                                plot_ui.hline(egui_plot::HLine::new(60.0).color(Color32::from_rgb(100, 100, 100)).style(egui_plot::LineStyle::dashed_dense()));
                            });
                    }
                }).header_response.clicked();

                // Frame Time Graph
                ui.collapsing(RichText::new("Frame Time Graph").strong(), |ui| {
                    let frame_time_points: Vec<[f64; 2]> = self
                        .frame_time_history
                        .iter()
                        .enumerate()
                        .map(|(i, &ft)| [i as f64, ft as f64])
                        .collect();

                    if !frame_time_points.is_empty() {
                        egui_plot::Plot::new("frame_time_plot")
                            .height(80.0)
                            .show_axes([false, true])
                            .include_y(0.0)
                            .show(ui, |plot_ui| {
                                plot_ui.line(egui_plot::Line::new(frame_time_points).color(Color32::from_rgb(255, 165, 0)).name("ms"));
                                // Draw 16.67ms target (60 FPS)
                                plot_ui.hline(egui_plot::HLine::new(16.67).color(Color32::from_rgb(100, 100, 100)).style(egui_plot::LineStyle::dashed_dense()));
                            });
                    }
                }).header_response.clicked();

                ui.separator();

                // Render Stats Section
                ui.collapsing(RichText::new("Render Stats").strong(), |ui| {
                    egui::Grid::new("render_stats_grid")
                        .num_columns(2)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("Sprites:");
                            ui.label(RichText::new(format!("{}", self.render_stats.sprites)).strong());
                            ui.end_row();

                            ui.label("Vertices:");
                            ui.label(format!("{}", self.render_stats.vertices));
                            ui.end_row();

                            ui.label("Draw Calls:");
                            ui.label(RichText::new(format!("{}", self.render_stats.draw_calls)).strong());
                            ui.end_row();

                            ui.label("Texture Binds:");
                            ui.label(format!("{}", self.render_stats.texture_binds));
                            ui.end_row();
                        });

                    // Sprites graph
                    if !self.sprites_history.is_empty() {
                        ui.add_space(4.0);
                        ui.label("Sprites over time:");
                        let sprites_points: Vec<[f64; 2]> = self
                            .sprites_history
                            .iter()
                            .enumerate()
                            .map(|(i, &s)| [i as f64, s as f64])
                            .collect();

                        egui_plot::Plot::new("sprites_plot")
                            .height(60.0)
                            .show_axes([false, true])
                            .include_y(0.0)
                            .show(ui, |plot_ui| {
                                plot_ui.line(egui_plot::Line::new(sprites_points).color(Color32::from_rgb(135, 206, 250)).name("Sprites"));
                            });
                    }
                }).header_response.clicked();

                ui.separator();

                // Hotkey hints
                ui.horizontal(|ui| {
                    ui.label(RichText::new("F2").color(Color32::GRAY).small());
                    ui.label(RichText::new("Performance").small());
                });
            });
    }

    /// Render ECS inspector panel
    fn render_ecs_inspector(&mut self, ctx: &Context) {
        Window::new("🔍 ECS Inspector")
            .default_size([380.0, 450.0])
            .show(ctx, |ui| {
                // Stats header
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("Entities: {}", self.ecs_data.entity_count)).strong());
                    ui.separator();
                    ui.label(format!("Components: {}", self.ecs_data.component_count));
                });
                ui.separator();

                // Two-column layout: entity list on left, details on right
                ui.horizontal(|ui| {
                    // Entity list (left panel)
                    ui.vertical(|ui| {
                        ui.set_min_width(140.0);
                        ui.heading("Entities");

                        egui::ScrollArea::vertical()
                            .max_height(350.0)
                            .id_source("entity_list")
                            .show(ui, |ui| {
                                for entity in &self.ecs_data.entities {
                                    let is_selected = self.selected_entity == Some(entity.id);
                                    let label = format!("[{}] {}", entity.id, entity.name);

                                    if ui.selectable_label(is_selected, &label).clicked() {
                                        self.selected_entity = Some(entity.id);
                                    }
                                }
                            });
                    });

                    ui.separator();

                    // Entity details (right panel)
                    ui.vertical(|ui| {
                        ui.set_min_width(200.0);

                        if let Some(selected_id) = self.selected_entity {
                            if let Some(entity) = self.ecs_data.entities.iter().find(|e| e.id == selected_id) {
                                ui.heading(format!("{} (ID: {})", entity.name, entity.id));
                                ui.separator();

                                egui::ScrollArea::vertical()
                                    .max_height(320.0)
                                    .id_source("component_list")
                                    .show(ui, |ui| {
                                        for component in &entity.components {
                                            ui.collapsing(&component.name, |ui| {
                                                self.render_component_value(ui, &component.value);
                                            });
                                        }
                                    });
                            } else {
                                ui.label("Entity not found");
                                self.selected_entity = None;
                            }
                        } else {
                            ui.label("Select an entity to view details");
                        }
                    });
                });
            });
    }

    /// Render a component value in the inspector
    fn render_component_value(&self, ui: &mut egui::Ui, value: &ComponentValue) {
        match value {
            ComponentValue::Vec2 { x, y } => {
                ui.horizontal(|ui| {
                    ui.label("x:");
                    ui.label(format!("{:.2}", x));
                    ui.label("y:");
                    ui.label(format!("{:.2}", y));
                });
            }
            ComponentValue::Size { width, height } => {
                ui.horizontal(|ui| {
                    ui.label("w:");
                    ui.label(format!("{:.1}", width));
                    ui.label("h:");
                    ui.label(format!("{:.1}", height));
                });
            }
            ComponentValue::Float(v) => {
                ui.label(format!("{:.3}", v));
            }
            ComponentValue::Int(v) => {
                ui.label(format!("{}", v));
            }
            ComponentValue::Bool(v) => {
                ui.label(if *v { "true" } else { "false" });
            }
            ComponentValue::String(s) => {
                ui.label(s);
            }
        }
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

    /// Render z-order labels on entities
    fn render_zorder_labels(&self, ctx: &Context) {
        let painter = ctx.layer_painter(egui::LayerId::background());

        for label in &self.zorder_data.labels {
            // Use zorder_data's view matrix for transformation
            let (screen_w, screen_h) = self.zorder_data.screen_size;
            let pos4 = self.zorder_data.view_matrix * glam::Vec4::new(label.position.x, label.position.y, 0.0, 1.0);
            let screen_x = (pos4.x + 1.0) * 0.5 * screen_w;
            let screen_y = (1.0 - pos4.y) * 0.5 * screen_h;
            let screen_pos = Pos2::new(screen_x, screen_y);

            // Draw background box for readability
            let text = format!("{} (z:{:.0})", label.label, label.z_order);
            let font = egui::FontId::proportional(11.0);

            // Estimate text size (approximate)
            let text_width = text.len() as f32 * 6.0;
            let text_height = 14.0;
            let padding = 2.0;

            let bg_rect = Rect::from_min_size(
                Pos2::new(screen_pos.x - padding, screen_pos.y - text_height - padding),
                egui::Vec2::new(text_width + padding * 2.0, text_height + padding * 2.0),
            );

            painter.rect(
                bg_rect,
                2.0,
                Color32::from_rgba_unmultiplied(0, 0, 0, 180),
                Stroke::NONE,
            );

            // Draw text
            painter.text(
                screen_pos,
                egui::Align2::LEFT_BOTTOM,
                &text,
                font,
                label.color,
            );
        }
    }

    /// Render layer legend window
    fn render_layer_legend(&mut self, ctx: &Context) {
        Window::new("📐 Z-Order / Layers")
            .default_size([280.0, 200.0])
            .show(ctx, |ui| {
                ui.heading("Render Order (bottom to top)");
                ui.separator();

                // Standard layer descriptions
                let standard_layers = [
                    (0, "Ground", Color32::from_rgb(139, 90, 43)),
                    (1, "Ground Decor", Color32::from_rgb(34, 139, 34)),
                    (2, "Shadows", Color32::from_rgb(64, 64, 64)),
                    (3, "Entities (Y-sorted)", Color32::from_rgb(255, 215, 0)),
                    (4, "Above Entities", Color32::from_rgb(135, 206, 250)),
                    (5, "Weather/Effects", Color32::from_rgb(255, 255, 255)),
                ];

                for (z, name, color) in standard_layers {
                    ui.horizontal(|ui| {
                        // Color indicator
                        let (rect, _) = ui.allocate_exact_size(egui::Vec2::new(12.0, 12.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, 2.0, color);

                        // Layer info
                        ui.label(RichText::new(format!("Z{}: {}", z, name)).color(color));
                    });
                }

                ui.separator();
                ui.checkbox(&mut self.config.show_z_order, "Show Z-Order Labels (Ctrl+Z)");

                // Show dynamic layer info if available
                if !self.zorder_data.layers.is_empty() {
                    ui.separator();
                    ui.heading("Current Frame Layers");

                    for layer in &self.zorder_data.layers {
                        ui.horizontal(|ui| {
                            let (rect, _) = ui.allocate_exact_size(egui::Vec2::new(12.0, 12.0), egui::Sense::hover());
                            ui.painter().rect_filled(rect, 2.0, layer.color);

                            ui.label(format!("{}: {} items", layer.name, layer.count));
                        });
                    }
                }
            });
    }

    /// Render event log panel
    fn render_event_log(&mut self, ctx: &Context) {
        Window::new("📜 Event Log")
            .default_size([450.0, 350.0])
            .show(ctx, |ui| {
                // Header with counts and controls
                ui.horizontal(|ui| {
                    let (input, game, system, debug) = self.event_counts();
                    let total = input + game + system + debug;

                    ui.label(RichText::new(format!("Total: {}", total)).strong());
                    ui.separator();

                    // Filter checkboxes with colored labels
                    ui.checkbox(&mut self.event_filter.show_input, "");
                    ui.label(RichText::new(format!("Input ({})", input)).color(EventType::Input.color()));

                    ui.checkbox(&mut self.event_filter.show_game, "");
                    ui.label(RichText::new(format!("Game ({})", game)).color(EventType::Game.color()));

                    ui.checkbox(&mut self.event_filter.show_system, "");
                    ui.label(RichText::new(format!("Sys ({})", system)).color(EventType::System.color()));

                    ui.checkbox(&mut self.event_filter.show_debug, "");
                    ui.label(RichText::new(format!("Dbg ({})", debug)).color(EventType::Debug.color()));
                });

                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.event_log_auto_scroll, "Auto-scroll");
                    ui.separator();
                    if ui.button("Clear").clicked() {
                        self.event_log.clear();
                    }
                });

                ui.separator();

                // Event list
                let row_height = 18.0;
                egui::ScrollArea::vertical()
                    .max_height(250.0)
                    .stick_to_bottom(self.event_log_auto_scroll)
                    .show(ui, |ui| {
                        for entry in &self.event_log {
                            // Skip filtered events
                            if !self.event_filter.should_show(entry.event_type) {
                                continue;
                            }

                            ui.horizontal(|ui| {
                                // Timestamp
                                ui.label(
                                    RichText::new(format!("[{:>7.2}]", entry.timestamp))
                                        .color(Color32::GRAY)
                                        .small()
                                );

                                // Event type badge
                                let type_color = entry.event_type.color();
                                ui.label(
                                    RichText::new(format!("[{}]", entry.event_type.name()))
                                        .color(type_color)
                                        .small()
                                );

                                // Message
                                ui.label(RichText::new(&entry.message).small());
                            });

                            ui.add_space(row_height - 14.0);
                        }

                        // Show message if no events match filter
                        let visible_count = self.event_log.iter()
                            .filter(|e| self.event_filter.should_show(e.event_type))
                            .count();

                        if visible_count == 0 {
                            if self.event_log.is_empty() {
                                ui.label(RichText::new("No events recorded yet.").color(Color32::GRAY));
                            } else {
                                ui.label(RichText::new("No events match current filters.").color(Color32::GRAY));
                            }
                        }
                    });

                // Footer with hotkey hint
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(RichText::new("F6").color(Color32::GRAY).small());
                    ui.label(RichText::new("Event Log").small());
                });
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

        // Add to history
        if self.command_history.last() != Some(&command) {
            self.command_history.push(command.clone());
            if self.command_history.len() > 50 {
                self.command_history.remove(0);
            }
        }
        self.history_index = None;

        self.console_output.push(format!("> {}", command));

        // Parse command and arguments
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            self.console_input.clear();
            return;
        }

        let cmd = parts[0].to_lowercase();
        let args = &parts[1..];

        match cmd.as_str() {
            "help" | "?" => {
                self.console_output.push("Available commands:".to_string());
                self.console_output.push("  help          - Show this help".to_string());
                self.console_output.push("  clear         - Clear console".to_string());
                self.console_output.push("  fps           - Show current FPS".to_string());
                self.console_output.push("  stats         - Show render stats".to_string());
                self.console_output.push("  pos           - Show player position".to_string());
                self.console_output.push("  tp <x> <y>    - Teleport player".to_string());
                self.console_output.push("  speed <val>   - Set player speed".to_string());
                self.console_output.push("  timescale <v> - Set game speed (0.1-10)".to_string());
                self.console_output.push("  entities      - List all entities".to_string());
                self.console_output.push("  collision on/off - Toggle collision boxes".to_string());
                self.console_output.push("  zorder on/off - Toggle z-order labels".to_string());
            }
            "clear" | "cls" => {
                self.console_output.clear();
            }
            "fps" => {
                let fps = self.fps_history.last().copied().unwrap_or(0.0);
                let avg = if !self.fps_history.is_empty() {
                    self.fps_history.iter().sum::<f32>() / self.fps_history.len() as f32
                } else {
                    0.0
                };
                self.console_output.push(format!("FPS: {:.1} (avg: {:.1})", fps, avg));
            }
            "stats" => {
                self.pending_commands.push(ConsoleCommand::ShowStats);
            }
            "pos" | "position" => {
                self.pending_commands.push(ConsoleCommand::GetPosition);
            }
            "tp" | "teleport" => {
                if args.len() >= 2 {
                    match (args[0].parse::<f32>(), args[1].parse::<f32>()) {
                        (Ok(x), Ok(y)) => {
                            self.pending_commands.push(ConsoleCommand::Teleport { x, y });
                            self.console_output.push(format!("Teleporting to ({}, {})", x, y));
                        }
                        _ => {
                            self.console_output.push("Usage: tp <x> <y>".to_string());
                        }
                    }
                } else {
                    self.console_output.push("Usage: tp <x> <y>".to_string());
                }
            }
            "speed" => {
                if let Some(val) = args.first() {
                    if let Ok(speed) = val.parse::<f32>() {
                        if speed > 0.0 && speed <= 2000.0 {
                            self.pending_commands.push(ConsoleCommand::SetSpeed(speed));
                            self.console_output.push(format!("Speed set to {}", speed));
                        } else {
                            self.console_output.push("Speed must be between 0 and 2000".to_string());
                        }
                    } else {
                        self.console_output.push("Usage: speed <value>".to_string());
                    }
                } else {
                    self.console_output.push("Usage: speed <value>".to_string());
                }
            }
            "timescale" | "ts" => {
                if let Some(val) = args.first() {
                    if let Ok(scale) = val.parse::<f32>() {
                        if scale >= 0.1 && scale <= 10.0 {
                            self.pending_commands.push(ConsoleCommand::SetTimescale(scale));
                            self.console_output.push(format!("Timescale set to {}", scale));
                        } else {
                            self.console_output.push("Timescale must be between 0.1 and 10".to_string());
                        }
                    } else {
                        self.console_output.push("Usage: timescale <value>".to_string());
                    }
                } else {
                    self.console_output.push("Usage: timescale <value>".to_string());
                }
            }
            "entities" | "ents" => {
                self.pending_commands.push(ConsoleCommand::ListEntities);
            }
            "collision" | "col" => {
                if let Some(state) = args.first() {
                    match *state {
                        "on" | "1" | "true" => {
                            self.config.show_collisions = true;
                            self.console_output.push("Collision boxes enabled".to_string());
                        }
                        "off" | "0" | "false" => {
                            self.config.show_collisions = false;
                            self.console_output.push("Collision boxes disabled".to_string());
                        }
                        _ => {
                            self.console_output.push("Usage: collision on/off".to_string());
                        }
                    }
                } else {
                    // Toggle
                    self.config.show_collisions = !self.config.show_collisions;
                    let state = if self.config.show_collisions { "enabled" } else { "disabled" };
                    self.console_output.push(format!("Collision boxes {}", state));
                }
            }
            "zorder" | "zo" => {
                if let Some(state) = args.first() {
                    match *state {
                        "on" | "1" | "true" => {
                            self.config.show_z_order = true;
                            self.console_output.push("Z-order labels enabled".to_string());
                        }
                        "off" | "0" | "false" => {
                            self.config.show_z_order = false;
                            self.console_output.push("Z-order labels disabled".to_string());
                        }
                        _ => {
                            self.console_output.push("Usage: zorder on/off".to_string());
                        }
                    }
                } else {
                    // Toggle
                    self.config.show_z_order = !self.config.show_z_order;
                    let state = if self.config.show_z_order { "enabled" } else { "disabled" };
                    self.console_output.push(format!("Z-order labels {}", state));
                }
            }
            _ => {
                self.console_output.push(format!("Unknown command: {}. Type 'help' for commands.", cmd));
            }
        }

        self.console_input.clear();
    }
}
