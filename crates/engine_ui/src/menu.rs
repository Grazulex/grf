//! Menu UI components
//!
//! Provides reusable menu widgets for main menu, pause menu, etc.

use engine_render::{glam::Vec2, Sprite};

/// Menu item state
#[derive(Debug, Clone)]
pub struct MenuItem {
    /// Display label for the item
    pub label: String,
    /// Unique identifier for the item
    pub id: String,
    /// Whether the item is enabled
    pub enabled: bool,
}

impl MenuItem {
    /// Create a new menu item
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            enabled: true,
        }
    }

    /// Create a disabled menu item
    pub fn disabled(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            enabled: false,
        }
    }
}

/// Menu configuration and appearance
#[derive(Debug, Clone)]
pub struct MenuStyle {
    /// Background color [r, g, b, a]
    pub background_color: [f32; 4],
    /// Item normal color
    pub item_color: [f32; 4],
    /// Item selected/highlighted color
    pub item_selected_color: [f32; 4],
    /// Item disabled color
    pub item_disabled_color: [f32; 4],
    /// Selection indicator color
    pub indicator_color: [f32; 4],
    /// Item width in pixels
    pub item_width: f32,
    /// Item height in pixels
    pub item_height: f32,
    /// Spacing between items
    pub item_spacing: f32,
}

impl Default for MenuStyle {
    fn default() -> Self {
        Self {
            background_color: [0.1, 0.1, 0.15, 1.0],
            item_color: [0.2, 0.2, 0.25, 1.0],
            item_selected_color: [0.3, 0.5, 0.7, 1.0],
            item_disabled_color: [0.15, 0.15, 0.15, 0.5],
            indicator_color: [1.0, 1.0, 0.3, 1.0],
            item_width: 200.0,
            item_height: 50.0,
            item_spacing: 15.0,
        }
    }
}

/// Generic menu widget
#[derive(Debug)]
pub struct Menu {
    /// Menu items
    items: Vec<MenuItem>,
    /// Currently selected index
    selected: usize,
    /// Menu appearance
    style: MenuStyle,
    /// Position (center of menu)
    position: Vec2,
    /// Title (optional)
    title: Option<String>,
}

impl Menu {
    /// Create a new menu
    pub fn new(position: Vec2) -> Self {
        Self {
            items: Vec::new(),
            selected: 0,
            style: MenuStyle::default(),
            position,
            title: None,
        }
    }

    /// Set the menu style
    pub fn with_style(mut self, style: MenuStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the menu title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add an item to the menu
    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
    }

    /// Set all items at once
    pub fn set_items(&mut self, items: Vec<MenuItem>) {
        self.items = items;
        if self.selected >= self.items.len() && !self.items.is_empty() {
            self.selected = 0;
        }
    }

    /// Get the number of items
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Get the currently selected index
    pub fn selected_index(&self) -> usize {
        self.selected
    }

    /// Get the currently selected item
    pub fn selected_item(&self) -> Option<&MenuItem> {
        self.items.get(self.selected)
    }

    /// Get all items
    pub fn items(&self) -> &[MenuItem] {
        &self.items
    }

    /// Move selection up (with wrap-around)
    pub fn move_up(&mut self) {
        if self.items.is_empty() {
            return;
        }
        loop {
            if self.selected > 0 {
                self.selected -= 1;
            } else {
                self.selected = self.items.len() - 1;
            }
            // Skip disabled items
            if self.items[self.selected].enabled {
                break;
            }
            // Safety: prevent infinite loop if all items disabled
            if self.items.iter().all(|i| !i.enabled) {
                break;
            }
        }
    }

    /// Move selection down (with wrap-around)
    pub fn move_down(&mut self) {
        if self.items.is_empty() {
            return;
        }
        loop {
            if self.selected < self.items.len() - 1 {
                self.selected += 1;
            } else {
                self.selected = 0;
            }
            // Skip disabled items
            if self.items[self.selected].enabled {
                break;
            }
            // Safety: prevent infinite loop if all items disabled
            if self.items.iter().all(|i| !i.enabled) {
                break;
            }
        }
    }

    /// Select a specific item by index
    pub fn select(&mut self, index: usize) {
        if index < self.items.len() && self.items[index].enabled {
            self.selected = index;
        }
    }

    /// Reset selection to first enabled item
    pub fn reset(&mut self) {
        self.selected = 0;
        // Find first enabled item
        for (i, item) in self.items.iter().enumerate() {
            if item.enabled {
                self.selected = i;
                break;
            }
        }
    }

    /// Generate sprites for rendering the menu
    pub fn sprites(&self, screen_size: (f32, f32)) -> Vec<Sprite> {
        let mut sprites = Vec::new();

        // Full-screen background
        sprites.push(Sprite::colored(
            Vec2::new(screen_size.0 / 2.0, screen_size.1 / 2.0),
            Vec2::new(screen_size.0, screen_size.1),
            self.style.background_color,
        ));

        // Calculate menu start position
        let total_height = self.items.len() as f32 * (self.style.item_height + self.style.item_spacing)
            - self.style.item_spacing;
        let start_y = self.position.y - total_height / 2.0 + self.style.item_height / 2.0;

        // Title bar (if title is set)
        if self.title.is_some() {
            sprites.push(Sprite::colored(
                Vec2::new(self.position.x, self.position.y - total_height / 2.0 - 80.0),
                Vec2::new(300.0, 60.0),
                [0.2, 0.4, 0.6, 1.0],
            ));
        }

        // Draw menu items
        for (i, item) in self.items.iter().enumerate() {
            let y = start_y + i as f32 * (self.style.item_height + self.style.item_spacing);
            let is_selected = i == self.selected;

            // Item background color
            let bg_color = if !item.enabled {
                self.style.item_disabled_color
            } else if is_selected {
                self.style.item_selected_color
            } else {
                self.style.item_color
            };

            // Item background
            sprites.push(Sprite::colored(
                Vec2::new(self.position.x, y),
                Vec2::new(self.style.item_width, self.style.item_height),
                bg_color,
            ));

            // Selection indicator (arrow/marker on left)
            if is_selected && item.enabled {
                sprites.push(Sprite::colored(
                    Vec2::new(
                        self.position.x - self.style.item_width / 2.0 - 20.0,
                        y,
                    ),
                    Vec2::new(15.0, 15.0),
                    self.style.indicator_color,
                ));
            }

            // Item type indicator (colored square on left inside the button)
            // This is a placeholder for text - each item gets a unique color
            let indicator_hue = (i as f32 / self.items.len().max(1) as f32) * 0.8;
            let indicator_color = hsv_to_rgb(indicator_hue, 0.7, 0.9);
            sprites.push(Sprite::colored(
                Vec2::new(
                    self.position.x - self.style.item_width / 2.0 + 25.0,
                    y,
                ),
                Vec2::new(20.0, 20.0),
                [indicator_color[0], indicator_color[1], indicator_color[2], 1.0],
            ));
        }

        sprites
    }
}

/// Convert HSV to RGB (h: 0-1, s: 0-1, v: 0-1)
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [f32; 3] {
    let c = v * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match (h * 6.0) as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    [r + m, g + m, b + m]
}

/// Main menu presets
pub mod presets {
    use super::*;

    /// Standard main menu items
    pub fn main_menu_items() -> Vec<MenuItem> {
        vec![
            MenuItem::new("new_game", "New Game"),
            MenuItem::new("load_game", "Load Game"),
            MenuItem::new("settings", "Settings"),
            MenuItem::new("quit", "Quit"),
        ]
    }

    /// Standard pause menu items
    pub fn pause_menu_items() -> Vec<MenuItem> {
        vec![
            MenuItem::new("resume", "Resume"),
            MenuItem::new("settings", "Settings"),
            MenuItem::new("save_game", "Save Game"),
            MenuItem::new("main_menu", "Main Menu"),
            MenuItem::new("quit", "Quit"),
        ]
    }

    /// Create a centered main menu
    pub fn main_menu(screen_size: (f32, f32)) -> Menu {
        let mut menu = Menu::new(Vec2::new(screen_size.0 / 2.0, screen_size.1 / 2.0))
            .with_title("GRF");
        menu.set_items(main_menu_items());
        menu
    }

    /// Create a centered pause menu
    pub fn pause_menu(screen_size: (f32, f32)) -> Menu {
        let mut menu = Menu::new(Vec2::new(screen_size.0 / 2.0, screen_size.1 / 2.0))
            .with_title("Paused");
        menu.set_items(pause_menu_items());
        menu
    }
}
