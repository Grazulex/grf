//! Item definitions and data loading
//!
//! Provides item type definitions loaded from TOML files.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Item type categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    /// Tools (hoe, watering can, axe, pickaxe, etc.)
    Tool,
    /// Seeds for planting
    Seed,
    /// Harvested crops
    Crop,
    /// Raw resources (wood, stone, ore)
    Resource,
    /// Food and potions
    Consumable,
    /// Furniture and decorations
    Furniture,
    /// Quest items and special objects
    Special,
}

impl Default for ItemType {
    fn default() -> Self {
        Self::Resource
    }
}

/// Tool type for specialized tools
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    Hoe,
    WateringCan,
    Axe,
    Pickaxe,
    Scythe,
    FishingRod,
}

/// Item definition loaded from data files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDefinition {
    /// Unique identifier (e.g., "parsnip_seeds", "wood")
    pub id: String,
    /// Display name
    pub name: String,
    /// Item category
    #[serde(rename = "type")]
    pub item_type: ItemType,
    /// Maximum stack size (default: 999)
    #[serde(default = "default_max_stack")]
    pub max_stack: u32,
    /// Base sell price in gold
    #[serde(default)]
    pub sell_price: u32,
    /// Optional description
    #[serde(default)]
    pub description: String,
    /// For seeds: what crop they grow into
    #[serde(default)]
    pub grows_into: Option<String>,
    /// For crops: growth time in days
    #[serde(default)]
    pub growth_days: Option<u32>,
    /// For crops: regrows after harvest
    #[serde(default)]
    pub regrows: bool,
    /// For tools: the tool type
    #[serde(default)]
    pub tool_type: Option<ToolType>,
    /// For consumables: energy restored
    #[serde(default)]
    pub energy: Option<i32>,
    /// For consumables: health restored
    #[serde(default)]
    pub health: Option<i32>,
    /// Seasons when this can be planted/found
    #[serde(default)]
    pub seasons: Vec<String>,
}

fn default_max_stack() -> u32 {
    999
}

impl ItemDefinition {
    /// Check if this item is stackable
    #[must_use]
    pub fn is_stackable(&self) -> bool {
        self.max_stack > 1
    }

    /// Check if this item is a tool
    #[must_use]
    pub fn is_tool(&self) -> bool {
        self.item_type == ItemType::Tool
    }

    /// Check if this item is a seed
    #[must_use]
    pub fn is_seed(&self) -> bool {
        self.item_type == ItemType::Seed
    }

    /// Check if this item is edible
    #[must_use]
    pub fn is_edible(&self) -> bool {
        self.item_type == ItemType::Consumable || self.item_type == ItemType::Crop
    }
}

/// TOML structure for loading multiple items
#[derive(Debug, Deserialize)]
struct ItemsFile {
    items: Vec<ItemDefinition>,
}

/// Database of all item definitions
#[derive(Debug, Default)]
pub struct ItemDatabase {
    items: HashMap<String, ItemDefinition>,
}

impl ItemDatabase {
    /// Create an empty database
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Load items from a TOML file
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<usize, ItemLoadError> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| ItemLoadError::Io {
            path: path.as_ref().to_string_lossy().to_string(),
            error: e.to_string(),
        })?;

        self.load_from_str(&content)
    }

    /// Load items from a TOML string
    pub fn load_from_str(&mut self, content: &str) -> Result<usize, ItemLoadError> {
        let file: ItemsFile = toml::from_str(content).map_err(|e| ItemLoadError::Parse {
            error: e.to_string(),
        })?;

        let count = file.items.len();
        for item in file.items {
            self.items.insert(item.id.clone(), item);
        }

        Ok(count)
    }

    /// Get an item definition by ID
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&ItemDefinition> {
        self.items.get(id)
    }

    /// Check if an item exists
    #[must_use]
    pub fn contains(&self, id: &str) -> bool {
        self.items.contains_key(id)
    }

    /// Get all items of a specific type
    #[must_use]
    pub fn get_by_type(&self, item_type: ItemType) -> Vec<&ItemDefinition> {
        self.items
            .values()
            .filter(|item| item.item_type == item_type)
            .collect()
    }

    /// Get all item IDs
    #[must_use]
    pub fn all_ids(&self) -> Vec<&str> {
        self.items.keys().map(|s| s.as_str()).collect()
    }

    /// Get total number of items
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if database is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Iterate over all items
    pub fn iter(&self) -> impl Iterator<Item = (&str, &ItemDefinition)> {
        self.items.iter().map(|(k, v)| (k.as_str(), v))
    }
}

/// Errors that can occur when loading items
#[derive(Debug)]
pub enum ItemLoadError {
    /// File I/O error
    Io { path: String, error: String },
    /// TOML parsing error
    Parse { error: String },
}

impl std::fmt::Display for ItemLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, error } => write!(f, "Failed to read {}: {}", path, error),
            Self::Parse { error } => write!(f, "Failed to parse TOML: {}", error),
        }
    }
}

impl std::error::Error for ItemLoadError {}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TOML: &str = r#"
[[items]]
id = "parsnip_seeds"
name = "Parsnip Seeds"
type = "seed"
sell_price = 10
description = "Plant in spring"
grows_into = "parsnip"
seasons = ["spring"]

[[items]]
id = "parsnip"
name = "Parsnip"
type = "crop"
sell_price = 35
growth_days = 4
seasons = ["spring"]

[[items]]
id = "wood"
name = "Wood"
type = "resource"
sell_price = 2

[[items]]
id = "hoe"
name = "Hoe"
type = "tool"
max_stack = 1
tool_type = "hoe"

[[items]]
id = "salad"
name = "Salad"
type = "consumable"
sell_price = 110
energy = 113
health = 50
"#;

    #[test]
    fn test_load_items() {
        let mut db = ItemDatabase::new();
        let count = db.load_from_str(TEST_TOML).unwrap();

        assert_eq!(count, 5);
        assert_eq!(db.len(), 5);
    }

    #[test]
    fn test_get_item() {
        let mut db = ItemDatabase::new();
        db.load_from_str(TEST_TOML).unwrap();

        let parsnip = db.get("parsnip").unwrap();
        assert_eq!(parsnip.name, "Parsnip");
        assert_eq!(parsnip.item_type, ItemType::Crop);
        assert_eq!(parsnip.sell_price, 35);
    }

    #[test]
    fn test_get_by_type() {
        let mut db = ItemDatabase::new();
        db.load_from_str(TEST_TOML).unwrap();

        let seeds = db.get_by_type(ItemType::Seed);
        assert_eq!(seeds.len(), 1);
        assert_eq!(seeds[0].id, "parsnip_seeds");

        let tools = db.get_by_type(ItemType::Tool);
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].id, "hoe");
    }

    #[test]
    fn test_item_properties() {
        let mut db = ItemDatabase::new();
        db.load_from_str(TEST_TOML).unwrap();

        let hoe = db.get("hoe").unwrap();
        assert!(hoe.is_tool());
        assert!(!hoe.is_stackable());
        assert_eq!(hoe.tool_type, Some(ToolType::Hoe));

        let seeds = db.get("parsnip_seeds").unwrap();
        assert!(seeds.is_seed());
        assert!(seeds.is_stackable());
        assert_eq!(seeds.grows_into, Some("parsnip".to_string()));
    }

    #[test]
    fn test_consumable() {
        let mut db = ItemDatabase::new();
        db.load_from_str(TEST_TOML).unwrap();

        let salad = db.get("salad").unwrap();
        assert!(salad.is_edible());
        assert_eq!(salad.energy, Some(113));
        assert_eq!(salad.health, Some(50));
    }

    #[test]
    fn test_default_max_stack() {
        let mut db = ItemDatabase::new();
        db.load_from_str(TEST_TOML).unwrap();

        let wood = db.get("wood").unwrap();
        assert_eq!(wood.max_stack, 999);
    }

    #[test]
    fn test_contains() {
        let mut db = ItemDatabase::new();
        db.load_from_str(TEST_TOML).unwrap();

        assert!(db.contains("wood"));
        assert!(db.contains("hoe"));
        assert!(!db.contains("nonexistent"));
    }
}
