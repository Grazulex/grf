//! Save manager for loading and saving game state

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use super::save_data::SaveData;

/// Default save directory name
const SAVE_DIR: &str = "saves";
/// Maximum number of save slots
pub const MAX_SAVE_SLOTS: usize = 5;

/// Save file manager
pub struct SaveManager {
    /// Base path for save files
    save_path: PathBuf,
}

impl SaveManager {
    /// Create a new save manager
    pub fn new() -> Self {
        // Use current directory + saves/
        let save_path = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(SAVE_DIR);

        Self { save_path }
    }

    /// Create save manager with custom path
    #[allow(dead_code)]
    pub fn with_path(path: impl Into<PathBuf>) -> Self {
        Self {
            save_path: path.into(),
        }
    }

    /// Ensure save directory exists
    fn ensure_dir(&self) -> Result<()> {
        if !self.save_path.exists() {
            fs::create_dir_all(&self.save_path)
                .context("Failed to create save directory")?;
        }
        Ok(())
    }

    /// Get path for a save slot
    fn slot_path(&self, slot: usize) -> PathBuf {
        self.save_path.join(format!("save_{}.json", slot))
    }

    /// Get path for backup of a save slot
    fn backup_path(&self, slot: usize) -> PathBuf {
        self.save_path.join(format!("save_{}.json.bak", slot))
    }

    /// Save game state to a slot
    pub fn save(&self, slot: usize, data: &SaveData) -> Result<()> {
        if slot >= MAX_SAVE_SLOTS {
            anyhow::bail!("Invalid save slot: {} (max: {})", slot, MAX_SAVE_SLOTS - 1);
        }

        self.ensure_dir()?;

        let path = self.slot_path(slot);
        let backup = self.backup_path(slot);

        // Create backup of existing save
        if path.exists() {
            fs::copy(&path, &backup)
                .context("Failed to create backup")?;
        }

        // Serialize to JSON
        let json = serde_json::to_string_pretty(data)
            .context("Failed to serialize save data")?;

        // Write to file
        fs::write(&path, json)
            .context("Failed to write save file")?;

        log::info!("Game saved to slot {}", slot);
        Ok(())
    }

    /// Load game state from a slot
    pub fn load(&self, slot: usize) -> Result<SaveData> {
        if slot >= MAX_SAVE_SLOTS {
            anyhow::bail!("Invalid save slot: {} (max: {})", slot, MAX_SAVE_SLOTS - 1);
        }

        let path = self.slot_path(slot);

        if !path.exists() {
            anyhow::bail!("Save slot {} is empty", slot);
        }

        // Read file
        let json = fs::read_to_string(&path)
            .context("Failed to read save file")?;

        // Deserialize
        let data: SaveData = serde_json::from_str(&json)
            .context("Failed to parse save file (may be corrupted)")?;

        // Check version compatibility
        if !data.is_compatible() {
            log::warn!(
                "Save file version {} differs from current version {}",
                data.version,
                super::save_data::SAVE_VERSION
            );
        }

        log::info!("Game loaded from slot {}", slot);
        Ok(data)
    }

    /// Check if a save slot exists
    #[allow(dead_code)]
    pub fn slot_exists(&self, slot: usize) -> bool {
        if slot >= MAX_SAVE_SLOTS {
            return false;
        }
        self.slot_path(slot).exists()
    }

    /// Get list of existing save slots with metadata
    #[allow(dead_code)]
    pub fn list_saves(&self) -> Vec<SaveSlotInfo> {
        (0..MAX_SAVE_SLOTS)
            .filter_map(|slot| {
                let path = self.slot_path(slot);
                if path.exists() {
                    // Try to get basic info without full load
                    let metadata = fs::metadata(&path).ok()?;
                    let modified = metadata.modified().ok()?;

                    Some(SaveSlotInfo {
                        slot,
                        modified,
                        path,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Delete a save slot
    #[allow(dead_code)]
    pub fn delete(&self, slot: usize) -> Result<()> {
        if slot >= MAX_SAVE_SLOTS {
            anyhow::bail!("Invalid save slot: {}", slot);
        }

        let path = self.slot_path(slot);
        let backup = self.backup_path(slot);

        if path.exists() {
            fs::remove_file(&path)
                .context("Failed to delete save file")?;
        }

        if backup.exists() {
            fs::remove_file(&backup).ok(); // Ignore backup deletion errors
        }

        log::info!("Save slot {} deleted", slot);
        Ok(())
    }

    /// Restore save from backup
    #[allow(dead_code)]
    pub fn restore_backup(&self, slot: usize) -> Result<()> {
        if slot >= MAX_SAVE_SLOTS {
            anyhow::bail!("Invalid save slot: {}", slot);
        }

        let path = self.slot_path(slot);
        let backup = self.backup_path(slot);

        if !backup.exists() {
            anyhow::bail!("No backup exists for slot {}", slot);
        }

        fs::copy(&backup, &path)
            .context("Failed to restore backup")?;

        log::info!("Save slot {} restored from backup", slot);
        Ok(())
    }
}

impl Default for SaveManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a save slot
#[allow(dead_code)]
#[derive(Debug)]
pub struct SaveSlotInfo {
    /// Slot number
    pub slot: usize,
    /// Last modified time
    pub modified: std::time::SystemTime,
    /// Full path to save file
    pub path: PathBuf,
}
