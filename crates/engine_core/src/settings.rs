//! Game settings with persistence
//!
//! Provides configuration for audio, video, and gameplay settings
//! with automatic save/load to a configuration file.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Default settings file name
const SETTINGS_FILE: &str = "settings.json";

/// Audio settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    /// Master volume (0.0 to 1.0)
    pub master_volume: f32,
    /// Music volume (0.0 to 1.0)
    pub music_volume: f32,
    /// Sound effects volume (0.0 to 1.0)
    pub sfx_volume: f32,
    /// Whether audio is muted
    pub muted: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.7,
            sfx_volume: 1.0,
            muted: false,
        }
    }
}

/// Video/display settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSettings {
    /// Window width in pixels
    pub width: u32,
    /// Window height in pixels
    pub height: u32,
    /// Fullscreen mode
    pub fullscreen: bool,
    /// VSync enabled
    pub vsync: bool,
    /// UI scale (1.0 = 100%)
    pub ui_scale: f32,
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            fullscreen: false,
            vsync: true,
            ui_scale: 1.0,
        }
    }
}

/// Gameplay settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplaySettings {
    /// Camera follow smoothness (0.0 = instant, 1.0 = very smooth)
    pub camera_smoothing: f32,
    /// Show FPS counter
    pub show_fps: bool,
    /// Auto-save interval in minutes (0 = disabled)
    pub autosave_interval: u32,
}

impl Default for GameplaySettings {
    fn default() -> Self {
        Self {
            camera_smoothing: 0.1,
            show_fps: false,
            autosave_interval: 5,
        }
    }
}

/// Complete game settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameSettings {
    /// Audio settings
    pub audio: AudioSettings,
    /// Video settings
    pub video: VideoSettings,
    /// Gameplay settings
    pub gameplay: GameplaySettings,
}

impl GameSettings {
    /// Create new default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the default settings path
    pub fn default_path() -> PathBuf {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(SETTINGS_FILE)
    }

    /// Load settings from default path, or create default if not found
    pub fn load() -> Self {
        Self::load_from(&Self::default_path())
    }

    /// Load settings from a specific path
    pub fn load_from(path: &Path) -> Self {
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(settings) => {
                            log::info!("Settings loaded from {:?}", path);
                            return settings;
                        }
                        Err(e) => {
                            log::warn!("Failed to parse settings file: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Failed to read settings file: {}", e);
                }
            }
        }

        // Return default settings
        let settings = Self::default();
        // Try to save default settings
        if let Err(e) = settings.save_to(path) {
            log::warn!("Failed to save default settings: {}", e);
        }
        settings
    }

    /// Save settings to default path
    pub fn save(&self) -> Result<(), String> {
        self.save_to(&Self::default_path())
    }

    /// Save settings to a specific path
    pub fn save_to(&self, path: &Path) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        fs::write(path, content)
            .map_err(|e| format!("Failed to write settings file: {}", e))?;

        log::info!("Settings saved to {:?}", path);
        Ok(())
    }

    /// Get effective master volume (considering mute)
    pub fn effective_master_volume(&self) -> f32 {
        if self.audio.muted {
            0.0
        } else {
            self.audio.master_volume
        }
    }

    /// Get effective music volume (master * music)
    pub fn effective_music_volume(&self) -> f32 {
        self.effective_master_volume() * self.audio.music_volume
    }

    /// Get effective SFX volume (master * sfx)
    pub fn effective_sfx_volume(&self) -> f32 {
        self.effective_master_volume() * self.audio.sfx_volume
    }
}

/// Setting value types for UI display
#[derive(Debug, Clone)]
pub enum SettingValue {
    /// Boolean toggle (on/off)
    Toggle(bool),
    /// Float slider with min/max
    Slider { value: f32, min: f32, max: f32, step: f32 },
    /// Integer with min/max
    Integer { value: i32, min: i32, max: i32 },
    /// Choice from list of options
    Choice { selected: usize, options: Vec<String> },
}

/// A single setting entry for UI display
#[derive(Debug, Clone)]
pub struct SettingEntry {
    /// Setting identifier
    pub id: String,
    /// Display label
    pub label: String,
    /// Current value
    pub value: SettingValue,
    /// Category (audio, video, gameplay)
    pub category: String,
}

impl SettingEntry {
    /// Create a toggle setting
    pub fn toggle(id: &str, label: &str, category: &str, value: bool) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            value: SettingValue::Toggle(value),
            category: category.to_string(),
        }
    }

    /// Create a slider setting
    pub fn slider(id: &str, label: &str, category: &str, value: f32, min: f32, max: f32, step: f32) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            value: SettingValue::Slider { value, min, max, step },
            category: category.to_string(),
        }
    }

    /// Create an integer setting
    pub fn integer(id: &str, label: &str, category: &str, value: i32, min: i32, max: i32) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            value: SettingValue::Integer { value, min, max },
            category: category.to_string(),
        }
    }

    /// Create a choice setting
    pub fn choice(id: &str, label: &str, category: &str, selected: usize, options: Vec<String>) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            value: SettingValue::Choice { selected, options },
            category: category.to_string(),
        }
    }
}

impl GameSettings {
    /// Convert settings to UI-friendly entries
    pub fn to_entries(&self) -> Vec<SettingEntry> {
        vec![
            // Audio
            SettingEntry::slider("master_volume", "Master Volume", "Audio", self.audio.master_volume, 0.0, 1.0, 0.1),
            SettingEntry::slider("music_volume", "Music Volume", "Audio", self.audio.music_volume, 0.0, 1.0, 0.1),
            SettingEntry::slider("sfx_volume", "SFX Volume", "Audio", self.audio.sfx_volume, 0.0, 1.0, 0.1),
            SettingEntry::toggle("muted", "Mute All", "Audio", self.audio.muted),
            // Video
            SettingEntry::toggle("fullscreen", "Fullscreen", "Video", self.video.fullscreen),
            SettingEntry::toggle("vsync", "VSync", "Video", self.video.vsync),
            SettingEntry::slider("ui_scale", "UI Scale", "Video", self.video.ui_scale, 0.5, 2.0, 0.25),
            // Gameplay
            SettingEntry::slider("camera_smoothing", "Camera Smoothing", "Gameplay", self.gameplay.camera_smoothing, 0.0, 1.0, 0.1),
            SettingEntry::toggle("show_fps", "Show FPS", "Gameplay", self.gameplay.show_fps),
            SettingEntry::integer("autosave_interval", "Autosave (min)", "Gameplay", self.gameplay.autosave_interval as i32, 0, 30),
        ]
    }

    /// Update a setting by ID
    pub fn update_from_entry(&mut self, entry: &SettingEntry) {
        match entry.id.as_str() {
            // Audio
            "master_volume" => {
                if let SettingValue::Slider { value, .. } = entry.value {
                    self.audio.master_volume = value;
                }
            }
            "music_volume" => {
                if let SettingValue::Slider { value, .. } = entry.value {
                    self.audio.music_volume = value;
                }
            }
            "sfx_volume" => {
                if let SettingValue::Slider { value, .. } = entry.value {
                    self.audio.sfx_volume = value;
                }
            }
            "muted" => {
                if let SettingValue::Toggle(value) = entry.value {
                    self.audio.muted = value;
                }
            }
            // Video
            "fullscreen" => {
                if let SettingValue::Toggle(value) = entry.value {
                    self.video.fullscreen = value;
                }
            }
            "vsync" => {
                if let SettingValue::Toggle(value) = entry.value {
                    self.video.vsync = value;
                }
            }
            "ui_scale" => {
                if let SettingValue::Slider { value, .. } = entry.value {
                    self.video.ui_scale = value;
                }
            }
            // Gameplay
            "camera_smoothing" => {
                if let SettingValue::Slider { value, .. } = entry.value {
                    self.gameplay.camera_smoothing = value;
                }
            }
            "show_fps" => {
                if let SettingValue::Toggle(value) = entry.value {
                    self.gameplay.show_fps = value;
                }
            }
            "autosave_interval" => {
                if let SettingValue::Integer { value, .. } = entry.value {
                    self.gameplay.autosave_interval = value as u32;
                }
            }
            _ => {
                log::warn!("Unknown setting: {}", entry.id);
            }
        }
    }
}
