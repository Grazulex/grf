//! Settings menu UI component
//!
//! Provides a menu for displaying and adjusting game settings.

use engine_core::{SettingEntry, SettingValue};
use engine_render::{glam::Vec2, Sprite};

/// Settings menu widget
#[derive(Debug)]
pub struct SettingsMenu {
    /// All settings entries
    entries: Vec<SettingEntry>,
    /// Currently selected setting index
    selected: usize,
    /// Position (center of menu)
    position: Vec2,
    /// Whether the menu is visible
    visible: bool,
}

impl SettingsMenu {
    /// Create a new settings menu
    pub fn new(position: Vec2) -> Self {
        Self {
            entries: Vec::new(),
            selected: 0,
            position,
            visible: true,
        }
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Load settings entries
    pub fn set_entries(&mut self, entries: Vec<SettingEntry>) {
        self.entries = entries;
        if self.selected >= self.entries.len() && !self.entries.is_empty() {
            self.selected = 0;
        }
    }

    /// Get all entries
    pub fn entries(&self) -> &[SettingEntry] {
        &self.entries
    }

    /// Get mutable reference to entries
    pub fn entries_mut(&mut self) -> &mut Vec<SettingEntry> {
        &mut self.entries
    }

    /// Get currently selected entry
    pub fn selected_entry(&self) -> Option<&SettingEntry> {
        self.entries.get(self.selected)
    }

    /// Get mutable reference to currently selected entry
    pub fn selected_entry_mut(&mut self) -> Option<&mut SettingEntry> {
        self.entries.get_mut(self.selected)
    }

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.selected
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.entries.len() - 1;
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        if self.selected < self.entries.len() - 1 {
            self.selected += 1;
        } else {
            self.selected = 0;
        }
    }

    /// Adjust the selected setting value (left = decrease, right = increase)
    /// For toggles, any direction toggles the value
    pub fn adjust_left(&mut self) {
        if let Some(entry) = self.entries.get_mut(self.selected) {
            match &mut entry.value {
                SettingValue::Toggle(v) => *v = !*v,
                SettingValue::Slider { value, min, step, .. } => {
                    *value = (*value - *step).max(*min);
                }
                SettingValue::Integer { value, min, .. } => {
                    *value = (*value - 1).max(*min);
                }
                SettingValue::Choice { selected, options } => {
                    if *selected > 0 {
                        *selected -= 1;
                    } else {
                        *selected = options.len().saturating_sub(1);
                    }
                }
            }
        }
    }

    /// Adjust the selected setting value to the right (increase)
    pub fn adjust_right(&mut self) {
        if let Some(entry) = self.entries.get_mut(self.selected) {
            match &mut entry.value {
                SettingValue::Toggle(v) => *v = !*v,
                SettingValue::Slider { value, max, step, .. } => {
                    *value = (*value + *step).min(*max);
                }
                SettingValue::Integer { value, max, .. } => {
                    *value = (*value + 1).min(*max);
                }
                SettingValue::Choice { selected, options } => {
                    if *selected < options.len().saturating_sub(1) {
                        *selected += 1;
                    } else {
                        *selected = 0;
                    }
                }
            }
        }
    }

    /// Toggle the selected setting (for toggles) or adjust right (for others)
    pub fn toggle_or_adjust(&mut self) {
        self.adjust_right();
    }

    /// Reset selection to the top
    pub fn reset(&mut self) {
        self.selected = 0;
    }

    /// Generate sprites for rendering the settings menu
    pub fn sprites(&self, screen_size: (f32, f32)) -> Vec<Sprite> {
        if !self.visible || self.entries.is_empty() {
            return Vec::new();
        }

        let mut sprites = Vec::new();

        // Style constants
        let bg_color = [0.1, 0.1, 0.15, 0.95];
        let item_color = [0.15, 0.15, 0.2, 1.0];
        let selected_color = [0.2, 0.3, 0.5, 1.0];
        let label_color = [0.4, 0.4, 0.5, 1.0];
        let value_on_color = [0.3, 0.7, 0.3, 1.0];
        let value_off_color = [0.5, 0.3, 0.3, 1.0];
        let slider_bg_color = [0.1, 0.1, 0.12, 1.0];
        let slider_fill_color = [0.3, 0.5, 0.7, 1.0];
        let indicator_color = [1.0, 1.0, 0.3, 1.0];

        let item_width = 400.0;
        let item_height = 45.0;
        let item_spacing = 8.0;
        let label_width = 180.0;
        let value_width = 150.0;

        // Full-screen semi-transparent background
        sprites.push(Sprite::colored(
            Vec2::new(screen_size.0 / 2.0, screen_size.1 / 2.0),
            Vec2::new(screen_size.0, screen_size.1),
            bg_color,
        ));

        // Calculate menu bounds
        let total_height = self.entries.len() as f32 * (item_height + item_spacing) - item_spacing;
        let start_y = self.position.y - total_height / 2.0 + item_height / 2.0;

        // Title bar
        sprites.push(Sprite::colored(
            Vec2::new(self.position.x, self.position.y - total_height / 2.0 - 60.0),
            Vec2::new(350.0, 50.0),
            [0.2, 0.4, 0.6, 1.0],
        ));

        // Group settings by category for visual separation
        let mut current_category = String::new();

        for (i, entry) in self.entries.iter().enumerate() {
            // Check for category change
            if entry.category != current_category {
                current_category = entry.category.clone();

                // Category header (small colored bar)
                let cat_y = start_y + i as f32 * (item_height + item_spacing)
                    + if i > 0 { 15.0 } else { 0.0 } - 25.0;
                let cat_color = category_color(&entry.category);
                sprites.push(Sprite::colored(
                    Vec2::new(self.position.x, cat_y),
                    Vec2::new(item_width - 40.0, 5.0),
                    cat_color,
                ));
            }

            let y = start_y + i as f32 * (item_height + item_spacing);
            let is_selected = i == self.selected;

            // Item background
            let bg = if is_selected { selected_color } else { item_color };
            sprites.push(Sprite::colored(
                Vec2::new(self.position.x, y),
                Vec2::new(item_width, item_height),
                bg,
            ));

            // Selection indicator
            if is_selected {
                sprites.push(Sprite::colored(
                    Vec2::new(self.position.x - item_width / 2.0 - 15.0, y),
                    Vec2::new(10.0, 10.0),
                    indicator_color,
                ));
            }

            // Label placeholder (colored rectangle on the left)
            sprites.push(Sprite::colored(
                Vec2::new(self.position.x - item_width / 2.0 + label_width / 2.0 + 15.0, y),
                Vec2::new(label_width, item_height - 15.0),
                label_color,
            ));

            // Value representation based on type
            let value_x = self.position.x + item_width / 2.0 - value_width / 2.0 - 15.0;
            match &entry.value {
                SettingValue::Toggle(v) => {
                    // Toggle indicator
                    let toggle_color = if *v { value_on_color } else { value_off_color };
                    sprites.push(Sprite::colored(
                        Vec2::new(value_x, y),
                        Vec2::new(60.0, 30.0),
                        toggle_color,
                    ));
                    // Toggle knob position
                    let knob_x = if *v { value_x + 15.0 } else { value_x - 15.0 };
                    sprites.push(Sprite::colored(
                        Vec2::new(knob_x, y),
                        Vec2::new(25.0, 25.0),
                        [0.9, 0.9, 0.9, 1.0],
                    ));
                }
                SettingValue::Slider { value, min, max, .. } => {
                    // Slider background
                    sprites.push(Sprite::colored(
                        Vec2::new(value_x, y),
                        Vec2::new(value_width - 10.0, 12.0),
                        slider_bg_color,
                    ));
                    // Slider fill
                    let fill_ratio = (value - min) / (max - min);
                    let fill_width = (value_width - 10.0) * fill_ratio;
                    if fill_width > 0.0 {
                        sprites.push(Sprite::colored(
                            Vec2::new(
                                value_x - (value_width - 10.0) / 2.0 + fill_width / 2.0,
                                y,
                            ),
                            Vec2::new(fill_width, 12.0),
                            slider_fill_color,
                        ));
                    }
                    // Slider knob
                    sprites.push(Sprite::colored(
                        Vec2::new(
                            value_x - (value_width - 10.0) / 2.0 + fill_width,
                            y,
                        ),
                        Vec2::new(8.0, 20.0),
                        [0.9, 0.9, 0.9, 1.0],
                    ));
                }
                SettingValue::Integer { value, min, max } => {
                    // Display as small slider with numeric indicator
                    let fill_ratio = (*value - min) as f32 / (*max - min).max(1) as f32;
                    // Background
                    sprites.push(Sprite::colored(
                        Vec2::new(value_x, y),
                        Vec2::new(80.0, 30.0),
                        slider_bg_color,
                    ));
                    // Fill
                    let fill_width = 80.0 * fill_ratio;
                    if fill_width > 0.0 {
                        sprites.push(Sprite::colored(
                            Vec2::new(value_x - 40.0 + fill_width / 2.0, y),
                            Vec2::new(fill_width, 30.0),
                            slider_fill_color,
                        ));
                    }
                    // Left/right arrows
                    sprites.push(Sprite::colored(
                        Vec2::new(value_x - 55.0, y),
                        Vec2::new(15.0, 15.0),
                        [0.7, 0.7, 0.7, 1.0],
                    ));
                    sprites.push(Sprite::colored(
                        Vec2::new(value_x + 55.0, y),
                        Vec2::new(15.0, 15.0),
                        [0.7, 0.7, 0.7, 1.0],
                    ));
                }
                SettingValue::Choice { selected, options } => {
                    // Choice indicator with arrows
                    let num_options = options.len();
                    let fill_ratio = if num_options > 1 {
                        *selected as f32 / (num_options - 1) as f32
                    } else {
                        0.5
                    };
                    // Background
                    sprites.push(Sprite::colored(
                        Vec2::new(value_x, y),
                        Vec2::new(100.0, 30.0),
                        slider_bg_color,
                    ));
                    // Selection indicator position
                    let sel_x = value_x - 40.0 + 80.0 * fill_ratio;
                    sprites.push(Sprite::colored(
                        Vec2::new(sel_x, y),
                        Vec2::new(20.0, 25.0),
                        slider_fill_color,
                    ));
                    // Arrows
                    sprites.push(Sprite::colored(
                        Vec2::new(value_x - 60.0, y),
                        Vec2::new(12.0, 12.0),
                        [0.7, 0.7, 0.7, 1.0],
                    ));
                    sprites.push(Sprite::colored(
                        Vec2::new(value_x + 60.0, y),
                        Vec2::new(12.0, 12.0),
                        [0.7, 0.7, 0.7, 1.0],
                    ));
                }
            }
        }

        // Back button indicator at bottom
        sprites.push(Sprite::colored(
            Vec2::new(self.position.x, self.position.y + total_height / 2.0 + 50.0),
            Vec2::new(150.0, 35.0),
            [0.3, 0.3, 0.35, 1.0],
        ));

        sprites
    }
}

/// Get a color for a category
fn category_color(category: &str) -> [f32; 4] {
    match category {
        "Audio" => [0.4, 0.6, 0.4, 1.0],     // Green
        "Video" => [0.4, 0.5, 0.7, 1.0],     // Blue
        "Gameplay" => [0.7, 0.5, 0.4, 1.0],  // Orange
        _ => [0.5, 0.5, 0.5, 1.0],           // Gray
    }
}

/// Settings menu presets
pub mod presets {
    use super::*;
    use engine_core::GameSettings;

    /// Create a centered settings menu from game settings
    pub fn settings_menu(screen_size: (f32, f32), settings: &GameSettings) -> SettingsMenu {
        let mut menu = SettingsMenu::new(Vec2::new(screen_size.0 / 2.0, screen_size.1 / 2.0));
        menu.set_entries(settings.to_entries());
        menu
    }
}
