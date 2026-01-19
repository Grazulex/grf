//! HUD (Heads-Up Display) components
//!
//! Basic UI elements rendered in screen space.

use engine_render::glam::{Vec2, Vec4};
use engine_render::Sprite;

/// Color palette for HUD elements
pub mod colors {
    use engine_render::glam::Vec4;

    pub const HEALTH_BAR: Vec4 = Vec4::new(0.8, 0.2, 0.2, 1.0); // Red
    pub const HEALTH_BG: Vec4 = Vec4::new(0.3, 0.1, 0.1, 0.8); // Dark red
    pub const STAMINA_BAR: Vec4 = Vec4::new(0.2, 0.7, 0.3, 1.0); // Green
    pub const STAMINA_BG: Vec4 = Vec4::new(0.1, 0.3, 0.1, 0.8); // Dark green
    pub const HOTBAR_BG: Vec4 = Vec4::new(0.2, 0.2, 0.2, 0.8); // Dark gray
    pub const HOTBAR_SELECTED: Vec4 = Vec4::new(0.8, 0.7, 0.2, 1.0); // Gold
    pub const HOTBAR_BORDER: Vec4 = Vec4::new(0.4, 0.4, 0.4, 1.0); // Gray
    pub const TIME_BG: Vec4 = Vec4::new(0.1, 0.1, 0.2, 0.8); // Dark blue
}

/// A simple progress bar (health, stamina, etc.)
#[derive(Debug, Clone)]
pub struct ProgressBar {
    /// Position (top-left corner in screen space)
    pub position: Vec2,
    /// Size of the bar
    pub size: Vec2,
    /// Current value (0.0 to 1.0)
    pub value: f32,
    /// Background color
    pub bg_color: Vec4,
    /// Fill color
    pub fill_color: Vec4,
    /// Border thickness
    pub border: f32,
}

impl ProgressBar {
    /// Create a new progress bar
    pub fn new(position: Vec2, size: Vec2, fill_color: Vec4, bg_color: Vec4) -> Self {
        Self {
            position,
            size,
            value: 1.0,
            bg_color,
            fill_color,
            border: 2.0,
        }
    }

    /// Create a health bar with default colors
    pub fn health(position: Vec2, size: Vec2) -> Self {
        Self::new(position, size, colors::HEALTH_BAR, colors::HEALTH_BG)
    }

    /// Create a stamina bar with default colors
    pub fn stamina(position: Vec2, size: Vec2) -> Self {
        Self::new(position, size, colors::STAMINA_BAR, colors::STAMINA_BG)
    }

    /// Set the current value (clamped to 0.0-1.0)
    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(0.0, 1.0);
    }

    /// Generate sprites for rendering
    pub fn sprites(&self) -> Vec<Sprite> {
        let mut sprites = Vec::with_capacity(2);

        // Background (full bar)
        let bg_sprite = Sprite {
            position: self.position + self.size * 0.5,
            size: self.size,
            origin: Vec2::new(0.5, 0.5),
            rotation: 0.0,
            color: self.bg_color,
            ..Default::default()
        };
        sprites.push(bg_sprite);

        // Fill (based on value)
        if self.value > 0.0 {
            let fill_width = (self.size.x - self.border * 2.0) * self.value;
            let fill_height = self.size.y - self.border * 2.0;

            let fill_sprite = Sprite {
                position: Vec2::new(
                    self.position.x + self.border + fill_width * 0.5,
                    self.position.y + self.size.y * 0.5,
                ),
                size: Vec2::new(fill_width, fill_height),
                origin: Vec2::new(0.5, 0.5),
                rotation: 0.0,
                color: self.fill_color,
                ..Default::default()
            };
            sprites.push(fill_sprite);
        }

        sprites
    }
}

/// A single slot in the hotbar
#[derive(Debug, Clone, Default)]
pub struct HotbarSlot {
    /// Item ID if slot has an item (None if empty)
    pub item_id: Option<u32>,
    /// Item count (for stackable items)
    pub count: u32,
}

/// Hotbar with multiple slots
#[derive(Debug, Clone)]
pub struct Hotbar {
    /// Position (center-bottom anchor)
    pub position: Vec2,
    /// Size of each slot
    pub slot_size: f32,
    /// Gap between slots
    pub gap: f32,
    /// Number of slots
    pub slot_count: usize,
    /// Currently selected slot index
    pub selected: usize,
    /// Slots data
    pub slots: Vec<HotbarSlot>,
}

impl Hotbar {
    /// Create a new hotbar
    pub fn new(position: Vec2, slot_count: usize) -> Self {
        Self {
            position,
            slot_size: 48.0,
            gap: 4.0,
            slot_count,
            selected: 0,
            slots: vec![HotbarSlot::default(); slot_count],
        }
    }

    /// Select a slot (0-indexed)
    pub fn select(&mut self, index: usize) {
        if index < self.slot_count {
            self.selected = index;
        }
    }

    /// Get total width of the hotbar
    pub fn total_width(&self) -> f32 {
        self.slot_count as f32 * self.slot_size + (self.slot_count - 1) as f32 * self.gap
    }

    /// Generate sprites for rendering
    pub fn sprites(&self) -> Vec<Sprite> {
        let mut sprites = Vec::with_capacity(self.slot_count * 2);
        let total_width = self.total_width();
        let start_x = self.position.x - total_width * 0.5;

        for i in 0..self.slot_count {
            let slot_x = start_x + i as f32 * (self.slot_size + self.gap) + self.slot_size * 0.5;
            let slot_y = self.position.y;

            // Slot background
            let bg_color = if i == self.selected {
                colors::HOTBAR_SELECTED
            } else {
                colors::HOTBAR_BG
            };

            let bg_sprite = Sprite {
                position: Vec2::new(slot_x, slot_y),
                size: Vec2::new(self.slot_size, self.slot_size),
                origin: Vec2::new(0.5, 0.5),
                rotation: 0.0,
                color: bg_color,
                ..Default::default()
            };
            sprites.push(bg_sprite);

            // Slot border
            let border_sprite = Sprite {
                position: Vec2::new(slot_x, slot_y),
                size: Vec2::new(self.slot_size + 4.0, self.slot_size + 4.0),
                origin: Vec2::new(0.5, 0.5),
                rotation: 0.0,
                color: colors::HOTBAR_BORDER,
                ..Default::default()
            };
            // Insert border behind background
            sprites.insert(sprites.len() - 1, border_sprite);
        }

        sprites
    }
}

/// Time display (in-game clock)
#[derive(Debug, Clone)]
pub struct TimeDisplay {
    /// Position (top-right anchor)
    pub position: Vec2,
    /// Size of the display box
    pub size: Vec2,
    /// Current hour (0-23)
    pub hour: u8,
    /// Current minute (0-59)
    pub minute: u8,
    /// Current day
    pub day: u32,
}

impl TimeDisplay {
    /// Create a new time display
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            size: Vec2::new(80.0, 40.0),
            hour: 6,
            minute: 0,
            day: 1,
        }
    }

    /// Set the time
    pub fn set_time(&mut self, hour: u8, minute: u8, day: u32) {
        self.hour = hour.min(23);
        self.minute = minute.min(59);
        self.day = day;
    }

    /// Get formatted time string
    pub fn time_string(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    /// Get day string
    pub fn day_string(&self) -> String {
        format!("Day {}", self.day)
    }

    /// Generate sprite for background
    pub fn background_sprite(&self) -> Sprite {
        Sprite {
            position: self.position - self.size * 0.5 + Vec2::new(self.size.x, self.size.y),
            size: self.size,
            origin: Vec2::new(0.5, 0.5),
            rotation: 0.0,
            color: colors::TIME_BG,
            ..Default::default()
        }
    }
}

/// Complete HUD state
#[derive(Debug, Clone)]
pub struct Hud {
    /// Health bar
    pub health: ProgressBar,
    /// Stamina bar
    pub stamina: ProgressBar,
    /// Hotbar
    pub hotbar: Hotbar,
    /// Time display
    pub time: TimeDisplay,
    /// Screen size for positioning
    screen_size: Vec2,
}

impl Hud {
    /// Create a new HUD for the given screen size
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let screen_size = Vec2::new(screen_width, screen_height);

        // Health bar - top left
        let health = ProgressBar::health(Vec2::new(10.0, 10.0), Vec2::new(200.0, 20.0));

        // Stamina bar - below health
        let stamina = ProgressBar::stamina(Vec2::new(10.0, 35.0), Vec2::new(200.0, 16.0));

        // Hotbar - bottom center
        let hotbar = Hotbar::new(Vec2::new(screen_width * 0.5, screen_height - 40.0), 9);

        // Time display - top right
        let time = TimeDisplay::new(Vec2::new(screen_width - 10.0, 10.0));

        Self {
            health,
            stamina,
            hotbar,
            time,
            screen_size,
        }
    }

    /// Update HUD layout when screen size changes
    pub fn resize(&mut self, screen_width: f32, screen_height: f32) {
        self.screen_size = Vec2::new(screen_width, screen_height);

        // Update hotbar position
        self.hotbar.position = Vec2::new(screen_width * 0.5, screen_height - 40.0);

        // Update time display position
        self.time.position = Vec2::new(screen_width - 10.0, 10.0);
    }

    /// Get all sprites for rendering the HUD
    pub fn sprites(&self) -> Vec<Sprite> {
        let mut sprites = Vec::new();

        // Health bar sprites
        sprites.extend(self.health.sprites());

        // Stamina bar sprites
        sprites.extend(self.stamina.sprites());

        // Hotbar sprites
        sprites.extend(self.hotbar.sprites());

        // Time display background
        sprites.push(self.time.background_sprite());

        sprites
    }
}
