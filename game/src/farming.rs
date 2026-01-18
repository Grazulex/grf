//! Farming system - planting, growing, and harvesting crops
//!
//! Provides crop management with growth stages, watering, and season validation.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::items::{ItemDatabase, ItemType};

/// Growth stage of a crop
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GrowthStage {
    /// Just planted, seed visible
    Seed,
    /// Small sprout
    Sprout,
    /// Growing plant
    Growing,
    /// Almost mature
    Mature,
    /// Ready to harvest
    Harvestable,
    /// Dead (from neglect or wrong season)
    Dead,
}

impl Default for GrowthStage {
    fn default() -> Self {
        Self::Seed
    }
}

impl GrowthStage {
    /// Get the next growth stage
    #[must_use]
    pub fn next(self) -> Option<Self> {
        match self {
            Self::Seed => Some(Self::Sprout),
            Self::Sprout => Some(Self::Growing),
            Self::Growing => Some(Self::Mature),
            Self::Mature => Some(Self::Harvestable),
            Self::Harvestable | Self::Dead => None,
        }
    }

    /// Check if crop can be harvested
    #[must_use]
    pub fn can_harvest(self) -> bool {
        self == Self::Harvestable
    }

    /// Check if crop is alive
    #[must_use]
    pub fn is_alive(self) -> bool {
        self != Self::Dead
    }
}

/// A planted crop entity component
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Crop {
    /// The crop item id (e.g., "parsnip", "tomato")
    pub crop_id: String,
    /// The seed item id that was planted
    pub seed_id: String,
    /// Current growth stage
    pub stage: GrowthStage,
    /// Days since planted
    pub days_growing: u32,
    /// Total days needed to mature
    pub days_to_mature: u32,
    /// Whether the crop was watered today
    pub watered_today: bool,
    /// Number of days without water (for dying logic)
    pub days_without_water: u32,
    /// Whether this crop regrows after harvest
    pub regrows: bool,
    /// Seasons when this crop can grow
    pub valid_seasons: Vec<String>,
}

impl Crop {
    /// Create a new crop from a seed
    #[must_use]
    pub fn from_seed(seed_id: &str, db: &ItemDatabase) -> Option<Self> {
        let seed = db.get(seed_id)?;

        // Only seeds can be planted
        if seed.item_type != ItemType::Seed {
            return None;
        }

        // Get the crop this seed grows into
        let crop_id = seed.grows_into.as_ref()?.clone();

        // Get crop info for growth time
        let crop_def = db.get(&crop_id)?;

        Some(Self {
            crop_id,
            seed_id: seed_id.to_string(),
            stage: GrowthStage::Seed,
            days_growing: 0,
            days_to_mature: crop_def.growth_days.unwrap_or(4),
            watered_today: false,
            days_without_water: 0,
            regrows: crop_def.regrows,
            valid_seasons: seed.seasons.clone(),
        })
    }

    /// Check if crop can grow in the given season
    #[must_use]
    pub fn can_grow_in_season(&self, season: &str) -> bool {
        self.valid_seasons.is_empty() || self.valid_seasons.iter().any(|s| s == season)
    }

    /// Water the crop
    pub fn water(&mut self) {
        self.watered_today = true;
        self.days_without_water = 0;
    }

    /// Advance a day for this crop
    /// Returns true if the crop grew to next stage
    pub fn advance_day(&mut self, current_season: &str) -> bool {
        // Check if crop can survive in current season
        if !self.can_grow_in_season(current_season) {
            self.stage = GrowthStage::Dead;
            return false;
        }

        // Track water
        if !self.watered_today {
            self.days_without_water += 1;
            // Die after 3 days without water
            if self.days_without_water >= 3 {
                self.stage = GrowthStage::Dead;
                return false;
            }
        }

        // Reset watered flag for next day
        self.watered_today = false;

        // Only grow if watered yesterday (days_without_water is already updated)
        if self.days_without_water == 0 && self.stage.is_alive() && !self.stage.can_harvest() {
            self.days_growing += 1;

            // Check if we should advance stage
            let progress = self.growth_progress();
            let new_stage = self.stage_for_progress(progress);

            if new_stage != self.stage {
                self.stage = new_stage;
                return true;
            }
        }

        false
    }

    /// Get growth progress as 0.0 to 1.0
    #[must_use]
    pub fn growth_progress(&self) -> f32 {
        if self.days_to_mature == 0 {
            return 1.0;
        }
        (self.days_growing as f32 / self.days_to_mature as f32).min(1.0)
    }

    /// Get the stage for a given progress
    #[must_use]
    fn stage_for_progress(&self, progress: f32) -> GrowthStage {
        if progress >= 1.0 {
            GrowthStage::Harvestable
        } else if progress >= 0.75 {
            GrowthStage::Mature
        } else if progress >= 0.5 {
            GrowthStage::Growing
        } else if progress >= 0.25 {
            GrowthStage::Sprout
        } else {
            GrowthStage::Seed
        }
    }

    /// Harvest the crop
    /// Returns the crop_id if harvestable, resets to Seed if regrows
    pub fn harvest(&mut self) -> Option<String> {
        if self.stage != GrowthStage::Harvestable {
            return None;
        }

        let harvested = self.crop_id.clone();

        if self.regrows {
            // Reset to growing state for regrowable crops
            self.stage = GrowthStage::Growing;
            // Takes 4 more days typically for regrow
            self.days_growing = self.days_to_mature.saturating_sub(4);
        } else {
            // Crop is done, will be removed
            self.stage = GrowthStage::Dead;
        }

        Some(harvested)
    }
}

/// State of a farmable tile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TileState {
    /// Natural grass/dirt, cannot plant
    #[default]
    Natural,
    /// Tilled with hoe, ready to plant
    Tilled,
    /// Tilled and watered
    Watered,
    /// Has a crop planted
    Planted,
}

impl TileState {
    /// Check if seeds can be planted on this tile
    #[must_use]
    pub fn can_plant(self) -> bool {
        matches!(self, Self::Tilled | Self::Watered)
    }

    /// Check if tile can be watered
    #[must_use]
    pub fn can_water(self) -> bool {
        matches!(self, Self::Tilled | Self::Planted)
    }

    /// Check if tile can be tilled
    #[must_use]
    pub fn can_till(self) -> bool {
        self == Self::Natural
    }
}

/// Farm tile component for tiles that can be farmed
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FarmTile {
    /// Current state of the tile
    pub state: TileState,
    /// Whether the tile was watered today
    pub watered_today: bool,
}

impl FarmTile {
    /// Create a new natural farm tile
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: TileState::Natural,
            watered_today: false,
        }
    }

    /// Till the tile with a hoe
    pub fn till(&mut self) -> bool {
        if self.state.can_till() {
            self.state = TileState::Tilled;
            true
        } else {
            false
        }
    }

    /// Water the tile
    pub fn water(&mut self) -> bool {
        if self.state.can_water() {
            self.watered_today = true;
            if self.state == TileState::Tilled {
                self.state = TileState::Watered;
            }
            true
        } else {
            false
        }
    }

    /// Plant a seed on this tile
    pub fn plant(&mut self) -> bool {
        if self.state.can_plant() {
            self.state = TileState::Planted;
            true
        } else {
            false
        }
    }

    /// Advance a day, reset watered state
    pub fn advance_day(&mut self) {
        self.watered_today = false;
        // Watered tiles return to tilled if nothing planted
        if self.state == TileState::Watered {
            self.state = TileState::Tilled;
        }
    }

    /// Clear the tile (after harvest or crop death)
    pub fn clear(&mut self) {
        self.state = TileState::Tilled;
    }
}

/// Result of a planting attempt
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlantResult {
    /// Successfully planted
    Success,
    /// Tile is not tilled
    NotTilled,
    /// Item is not a seed
    NotASeed,
    /// Wrong season for this crop
    WrongSeason,
    /// Tile already has a crop
    AlreadyPlanted,
    /// Seed not found in database
    UnknownSeed,
}

/// Try to plant a seed on a farm tile
pub fn try_plant(
    tile: &mut FarmTile,
    seed_id: &str,
    current_season: &str,
    db: &ItemDatabase,
) -> Result<Crop, PlantResult> {
    // Check if tile can be planted
    if !tile.state.can_plant() {
        return Err(if tile.state == TileState::Planted {
            PlantResult::AlreadyPlanted
        } else {
            PlantResult::NotTilled
        });
    }

    // Get seed definition
    let seed = db.get(seed_id).ok_or(PlantResult::UnknownSeed)?;

    // Check if it's actually a seed
    if seed.item_type != ItemType::Seed {
        return Err(PlantResult::NotASeed);
    }

    // Check season
    if !seed.seasons.is_empty() && !seed.seasons.iter().any(|s| s == current_season) {
        return Err(PlantResult::WrongSeason);
    }

    // Create the crop
    let crop = Crop::from_seed(seed_id, db).ok_or(PlantResult::UnknownSeed)?;

    // Mark tile as planted
    tile.plant();

    Ok(crop)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_db() -> ItemDatabase {
        let mut db = ItemDatabase::new();
        db.load_from_str(r#"
[[items]]
id = "parsnip_seeds"
name = "Parsnip Seeds"
type = "seed"
grows_into = "parsnip"
seasons = ["spring"]

[[items]]
id = "parsnip"
name = "Parsnip"
type = "crop"
growth_days = 4

[[items]]
id = "tomato_seeds"
name = "Tomato Seeds"
type = "seed"
grows_into = "tomato"
seasons = ["summer"]

[[items]]
id = "tomato"
name = "Tomato"
type = "crop"
growth_days = 11
regrows = true

[[items]]
id = "wood"
name = "Wood"
type = "resource"
"#).unwrap();
        db
    }

    #[test]
    fn test_growth_stages() {
        assert_eq!(GrowthStage::Seed.next(), Some(GrowthStage::Sprout));
        assert_eq!(GrowthStage::Sprout.next(), Some(GrowthStage::Growing));
        assert_eq!(GrowthStage::Harvestable.next(), None);
        assert!(GrowthStage::Harvestable.can_harvest());
        assert!(!GrowthStage::Growing.can_harvest());
    }

    #[test]
    fn test_farm_tile_workflow() {
        let mut tile = FarmTile::new();

        assert_eq!(tile.state, TileState::Natural);
        assert!(!tile.state.can_plant());

        // Till the tile
        assert!(tile.till());
        assert_eq!(tile.state, TileState::Tilled);
        assert!(tile.state.can_plant());

        // Water it
        assert!(tile.water());
        assert_eq!(tile.state, TileState::Watered);
        assert!(tile.state.can_plant());

        // Plant
        assert!(tile.plant());
        assert_eq!(tile.state, TileState::Planted);
        assert!(!tile.state.can_plant());
    }

    #[test]
    fn test_create_crop_from_seed() {
        let db = create_test_db();

        let crop = Crop::from_seed("parsnip_seeds", &db).unwrap();
        assert_eq!(crop.crop_id, "parsnip");
        assert_eq!(crop.seed_id, "parsnip_seeds");
        assert_eq!(crop.days_to_mature, 4);
        assert_eq!(crop.stage, GrowthStage::Seed);
        assert!(!crop.regrows);
    }

    #[test]
    fn test_regrowable_crop() {
        let db = create_test_db();

        let crop = Crop::from_seed("tomato_seeds", &db).unwrap();
        assert!(crop.regrows);
        assert_eq!(crop.days_to_mature, 11);
    }

    #[test]
    fn test_crop_growth() {
        let db = create_test_db();
        let mut crop = Crop::from_seed("parsnip_seeds", &db).unwrap();

        // Water and advance 4 days
        for _ in 0..4 {
            crop.water();
            crop.advance_day("spring");
        }

        assert_eq!(crop.stage, GrowthStage::Harvestable);
        assert!(crop.stage.can_harvest());
    }

    #[test]
    fn test_crop_dies_without_water() {
        let db = create_test_db();
        let mut crop = Crop::from_seed("parsnip_seeds", &db).unwrap();

        // Don't water for 3 days
        for _ in 0..3 {
            crop.advance_day("spring");
        }

        assert_eq!(crop.stage, GrowthStage::Dead);
        assert!(!crop.stage.is_alive());
    }

    #[test]
    fn test_crop_dies_wrong_season() {
        let db = create_test_db();
        let mut crop = Crop::from_seed("parsnip_seeds", &db).unwrap();

        crop.water();
        crop.advance_day("summer"); // Wrong season!

        assert_eq!(crop.stage, GrowthStage::Dead);
    }

    #[test]
    fn test_try_plant_success() {
        let db = create_test_db();
        let mut tile = FarmTile::new();
        tile.till();

        let result = try_plant(&mut tile, "parsnip_seeds", "spring", &db);
        assert!(result.is_ok());

        let crop = result.unwrap();
        assert_eq!(crop.crop_id, "parsnip");
        assert_eq!(tile.state, TileState::Planted);
    }

    #[test]
    fn test_try_plant_not_tilled() {
        let db = create_test_db();
        let mut tile = FarmTile::new();

        let result = try_plant(&mut tile, "parsnip_seeds", "spring", &db);
        assert_eq!(result, Err(PlantResult::NotTilled));
    }

    #[test]
    fn test_try_plant_wrong_season() {
        let db = create_test_db();
        let mut tile = FarmTile::new();
        tile.till();

        let result = try_plant(&mut tile, "parsnip_seeds", "summer", &db);
        assert_eq!(result, Err(PlantResult::WrongSeason));
    }

    #[test]
    fn test_try_plant_not_seed() {
        let db = create_test_db();
        let mut tile = FarmTile::new();
        tile.till();

        let result = try_plant(&mut tile, "wood", "spring", &db);
        assert_eq!(result, Err(PlantResult::NotASeed));
    }

    #[test]
    fn test_harvest_regrowable() {
        let db = create_test_db();
        let mut crop = Crop::from_seed("tomato_seeds", &db).unwrap();

        // Grow to harvestable
        for _ in 0..11 {
            crop.water();
            crop.advance_day("summer");
        }

        assert_eq!(crop.stage, GrowthStage::Harvestable);

        // Harvest
        let harvested = crop.harvest();
        assert_eq!(harvested, Some("tomato".to_string()));

        // Should still be alive and growing
        assert!(crop.stage.is_alive());
        assert_eq!(crop.stage, GrowthStage::Growing);
    }

    #[test]
    fn test_harvest_non_regrowable() {
        let db = create_test_db();
        let mut crop = Crop::from_seed("parsnip_seeds", &db).unwrap();

        // Grow to harvestable
        for _ in 0..4 {
            crop.water();
            crop.advance_day("spring");
        }

        // Harvest
        let harvested = crop.harvest();
        assert_eq!(harvested, Some("parsnip".to_string()));

        // Should be dead (non-regrowable)
        assert_eq!(crop.stage, GrowthStage::Dead);
    }
}
