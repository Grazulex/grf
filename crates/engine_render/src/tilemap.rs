//! Tilemap loading and rendering
//!
//! Supports JSON-based tilemaps with multiple layers and tilesets.
//! Compatible with both custom format and Tiled editor exports.

use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::{Camera2D, Sprite, SpriteRegion};

// ============================================================================
// Tiled JSON format structures (for parsing Tiled exports)
// ============================================================================

#[derive(Debug, Deserialize)]
struct TiledMap {
    width: u32,
    height: u32,
    tilewidth: u32,
    tileheight: u32,
    layers: Vec<TiledLayerUnion>,
    tilesets: Vec<TiledTilesetRef>,
}

/// Tiled layer can be tile layer or object layer
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum TiledLayerUnion {
    #[serde(rename = "tilelayer")]
    TileLayer(TiledTileLayer),
    #[serde(rename = "objectgroup")]
    ObjectGroup(TiledObjectGroup),
}

#[derive(Debug, Deserialize)]
struct TiledTileLayer {
    name: String,
    width: u32,
    height: u32,
    data: Vec<u32>,
    #[serde(default = "default_visible")]
    visible: bool,
    #[serde(default = "default_opacity")]
    opacity: f32,
    #[serde(default)]
    properties: Vec<TiledProperty>,
}

#[derive(Debug, Deserialize)]
struct TiledObjectGroup {
    name: String,
    #[serde(default)]
    objects: Vec<TiledObject>,
    #[serde(default = "default_visible")]
    visible: bool,
}

#[derive(Debug, Deserialize)]
struct TiledObject {
    #[serde(default)]
    name: String,
    x: f32,
    y: f32,
    #[serde(default)]
    width: Option<f32>,
    #[serde(default)]
    height: Option<f32>,
    #[serde(default, rename = "type")]
    obj_type: String,
    #[serde(default)]
    properties: Vec<TiledProperty>,
}

#[derive(Debug, Deserialize)]
struct TiledProperty {
    name: String,
    #[serde(default, rename = "type")]
    prop_type: String,
    value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct TiledTilesetRef {
    firstgid: u32,
    #[serde(default)]
    source: Option<String>,
    // Embedded tileset fields (when not using external .tsx)
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    image: Option<String>,
    #[serde(default)]
    tilewidth: Option<u32>,
    #[serde(default)]
    tileheight: Option<u32>,
    #[serde(default)]
    columns: Option<u32>,
    #[serde(default)]
    tilecount: Option<u32>,
    #[serde(default)]
    imagewidth: Option<u32>,
    #[serde(default)]
    imageheight: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct TiledExternalTileset {
    name: String,
    image: String,
    tilewidth: u32,
    tileheight: u32,
    columns: u32,
    tilecount: u32,
    #[serde(default)]
    imagewidth: Option<u32>,
    #[serde(default)]
    imageheight: Option<u32>,
}

// ============================================================================
// Our internal format structures
// ============================================================================

/// A tileset definition (sprite sheet for tiles)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tileset {
    /// Name of the tileset
    pub name: String,
    /// Path to the tileset image (relative to assets)
    pub image: String,
    /// Width of each tile in pixels
    pub tile_width: u32,
    /// Height of each tile in pixels
    pub tile_height: u32,
    /// Number of columns in the tileset image
    pub columns: u32,
    /// Number of rows in the tileset image
    pub rows: u32,
    /// First tile ID in this tileset (for multi-tileset maps)
    #[serde(default)]
    pub first_gid: u32,
}

impl Tileset {
    /// Get the UV region for a tile ID (local to this tileset)
    #[must_use]
    pub fn get_tile_region(&self, local_tile_id: u32) -> SpriteRegion {
        let col = local_tile_id % self.columns;
        let row = local_tile_id / self.columns;

        let tex_width = self.columns * self.tile_width;
        let tex_height = self.rows * self.tile_height;

        SpriteRegion::from_pixels(
            col * self.tile_width,
            row * self.tile_height,
            self.tile_width,
            self.tile_height,
            tex_width,
            tex_height,
        )
    }

    /// Get the total number of tiles in this tileset
    #[must_use]
    pub fn tile_count(&self) -> u32 {
        self.columns * self.rows
    }
}

/// Layer rendering order relative to entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LayerType {
    /// Rendered below all entities (ground, floors)
    #[default]
    Below,
    /// Rendered above all entities (roofs, overlays)
    Above,
}

/// A spawn point for player positioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnPoint {
    /// Unique identifier for this spawn
    pub id: String,
    /// X position in pixels
    pub x: f32,
    /// Y position in pixels
    pub y: f32,
}

impl SpawnPoint {
    /// Get the position as a Vec2
    #[must_use]
    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

/// A trigger zone for map transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    /// Trigger zone bounds
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    /// Target map path
    pub target_map: String,
    /// Spawn point ID in target map
    pub target_spawn: String,
}

impl Trigger {
    /// Check if a point is inside this trigger
    #[must_use]
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width
            && point.y >= self.y
            && point.y < self.y + self.height
    }

    /// Get the trigger bounds as min/max
    #[must_use]
    pub fn bounds(&self) -> (Vec2, Vec2) {
        (
            Vec2::new(self.x, self.y),
            Vec2::new(self.x + self.width, self.y + self.height),
        )
    }
}

/// A single layer of tiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileLayer {
    /// Layer name
    pub name: String,
    /// Width in tiles
    pub width: u32,
    /// Height in tiles
    pub height: u32,
    /// Tile data (row-major order, 0 = empty)
    pub data: Vec<u32>,
    /// Layer visibility
    #[serde(default = "default_visible")]
    pub visible: bool,
    /// Layer opacity (0.0 to 1.0)
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    /// Z-order within layer type (lower = rendered first)
    #[serde(default)]
    pub z_order: i32,
    /// Layer type (below or above entities)
    #[serde(default)]
    pub layer_type: LayerType,
}

fn default_visible() -> bool {
    true
}

fn default_opacity() -> f32 {
    1.0
}

impl TileLayer {
    /// Get the tile ID at a position (0 = empty)
    #[must_use]
    pub fn get_tile(&self, x: u32, y: u32) -> u32 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        let index = (y * self.width + x) as usize;
        self.data.get(index).copied().unwrap_or(0)
    }

    /// Set the tile ID at a position
    pub fn set_tile(&mut self, x: u32, y: u32, tile_id: u32) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            if index < self.data.len() {
                self.data[index] = tile_id;
            }
        }
    }
}

/// A complete tilemap with multiple layers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tilemap {
    /// Map name
    pub name: String,
    /// Width in tiles
    pub width: u32,
    /// Height in tiles
    pub height: u32,
    /// Tile width in pixels
    pub tile_width: u32,
    /// Tile height in pixels
    pub tile_height: u32,
    /// Tilesets used by this map
    pub tilesets: Vec<Tileset>,
    /// Layers (bottom to top)
    pub layers: Vec<TileLayer>,
    /// Collision data (true = solid tile, row-major order)
    #[serde(default)]
    pub collision: Vec<bool>,
    /// Spawn points for player positioning
    #[serde(default)]
    pub spawns: Vec<SpawnPoint>,
    /// Trigger zones for map transitions
    #[serde(default)]
    pub triggers: Vec<Trigger>,
}

impl Tilemap {
    /// Load a tilemap from a JSON file (supports both custom and Tiled formats)
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, TilemapError> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path)
            .map_err(|e| TilemapError::IoError(e.to_string()))?;

        // Try our custom format first
        if let Ok(tilemap) = serde_json::from_str::<Tilemap>(&contents) {
            return Ok(tilemap);
        }

        // Try Tiled format
        let tiled: TiledMap = serde_json::from_str(&contents)
            .map_err(|e| TilemapError::ParseError(format!("Failed to parse as Tiled format: {}", e)))?;

        // Convert Tiled format to our format
        Self::from_tiled(tiled, path)
    }

    /// Convert a Tiled map to our internal format
    fn from_tiled(tiled: TiledMap, map_path: &Path) -> Result<Self, TilemapError> {
        let map_dir = map_path.parent().unwrap_or(Path::new("."));

        // Convert tilesets and collect collision tile IDs
        let mut tilesets = Vec::new();
        let mut collision_tile_ids: std::collections::HashSet<u32> = std::collections::HashSet::new();

        for ts_ref in &tiled.tilesets {
            let (tileset, tile_collisions) = if let Some(source) = &ts_ref.source {
                // External tileset (.tsx file) - need to load and parse it
                let tsx_path = map_dir.join(source);
                Self::load_external_tileset_with_collisions(&tsx_path, ts_ref.firstgid, map_dir)?
            } else {
                // Embedded tileset
                let columns = ts_ref.columns.unwrap_or(1);
                let tilecount = ts_ref.tilecount.unwrap_or(columns);
                let rows = if columns > 0 { tilecount / columns } else { 1 };

                let ts = Tileset {
                    name: ts_ref.name.clone().unwrap_or_else(|| "tileset".to_string()),
                    image: ts_ref.image.clone().unwrap_or_default(),
                    tile_width: ts_ref.tilewidth.unwrap_or(16),
                    tile_height: ts_ref.tileheight.unwrap_or(16),
                    columns,
                    rows,
                    first_gid: ts_ref.firstgid,
                };
                (ts, Vec::new())
            };

            // Add collision tile IDs (local_id + first_gid)
            for local_id in tile_collisions {
                collision_tile_ids.insert(local_id + ts_ref.firstgid);
            }

            tilesets.push(tileset);
        }

        // Convert layers and extract spawns/triggers from object layers
        let mut layers = Vec::new();
        let mut spawns = Vec::new();
        let mut triggers = Vec::new();

        for layer in tiled.layers {
            match layer {
                TiledLayerUnion::TileLayer(tl) => {
                    // Get z-index from properties (default to 0)
                    let z_index = Self::get_property_int(&tl.properties, "z-index").unwrap_or(0);

                    // Determine layer type: z-index >= 0 with high value = above, negative = below
                    // Convention: z-index < 0 = below entities, z-index >= 100 = above entities
                    let layer_type = if z_index >= 100 {
                        LayerType::Above
                    } else if tl.name.to_lowercase().contains("above")
                        || tl.name.to_lowercase().contains("over")
                        || tl.name.to_lowercase().contains("roof") {
                        LayerType::Above
                    } else {
                        LayerType::Below
                    };

                    layers.push(TileLayer {
                        name: tl.name,
                        width: tl.width,
                        height: tl.height,
                        data: tl.data,
                        visible: tl.visible,
                        opacity: tl.opacity,
                        z_order: z_index,
                        layer_type,
                    });
                }
                TiledLayerUnion::ObjectGroup(og) => {
                    // Parse objects based on layer name or object type
                    let layer_name_lower = og.name.to_lowercase();

                    for obj in og.objects {
                        let obj_type_lower = obj.obj_type.to_lowercase();

                        if layer_name_lower.contains("spawn") || obj_type_lower == "spawn" {
                            // Spawn point
                            let id = if obj.name.is_empty() { "default".to_string() } else { obj.name };
                            spawns.push(SpawnPoint { id, x: obj.x, y: obj.y });
                        } else if layer_name_lower.contains("trigger") || obj_type_lower == "trigger" {
                            // Trigger zone
                            if let (Some(w), Some(h)) = (obj.width, obj.height) {
                                let target_map = Self::get_property_string(&obj.properties, "target_map")
                                    .unwrap_or_default();
                                let target_spawn = Self::get_property_string(&obj.properties, "target_spawn")
                                    .unwrap_or_else(|| "default".to_string());

                                triggers.push(Trigger {
                                    x: obj.x,
                                    y: obj.y,
                                    width: w,
                                    height: h,
                                    target_map,
                                    target_spawn,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Build collision array from tile data and collision tile IDs
        let map_size = (tiled.width * tiled.height) as usize;
        let mut collision = vec![false; map_size];

        if !collision_tile_ids.is_empty() {
            for layer in &layers {
                for (i, &tile_id) in layer.data.iter().enumerate() {
                    if collision_tile_ids.contains(&tile_id) {
                        collision[i] = true;
                    }
                }
            }
        }

        // Extract map name from filename
        let name = map_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unnamed")
            .to_string();

        Ok(Tilemap {
            name,
            width: tiled.width,
            height: tiled.height,
            tile_width: tiled.tilewidth,
            tile_height: tiled.tileheight,
            tilesets,
            layers,
            collision,
            spawns,
            triggers,
        })
    }

    /// Get string property from Tiled properties array
    fn get_property_string(props: &[TiledProperty], name: &str) -> Option<String> {
        props.iter()
            .find(|p| p.name == name)
            .and_then(|p| p.value.as_str().map(|s| s.to_string()))
    }

    /// Get integer property from Tiled properties array
    fn get_property_int(props: &[TiledProperty], name: &str) -> Option<i32> {
        props.iter()
            .find(|p| p.name == name)
            .and_then(|p| p.value.as_i64().map(|v| v as i32))
    }

    /// Load an external .tsx tileset file with collision data
    /// Returns (Tileset, Vec<local_tile_ids_with_collision>)
    fn load_external_tileset_with_collisions(tsx_path: &Path, first_gid: u32, map_dir: &Path) -> Result<(Tileset, Vec<u32>), TilemapError> {
        let contents = std::fs::read_to_string(tsx_path)
            .map_err(|e| TilemapError::IoError(format!("Failed to load tileset {}: {}", tsx_path.display(), e)))?;

        // Try JSON format first
        if let Ok(ts) = serde_json::from_str::<TiledExternalTileset>(&contents) {
            let rows = if ts.columns > 0 { ts.tilecount / ts.columns } else { 1 };

            // Resolve image path relative to map directory
            let image_path = map_dir.join(&ts.image);
            let image = image_path.to_string_lossy().to_string();

            let tileset = Tileset {
                name: ts.name,
                image,
                tile_width: ts.tilewidth,
                tile_height: ts.tileheight,
                columns: ts.columns,
                rows,
                first_gid,
            };
            // JSON format doesn't have inline collision - would need separate parsing
            return Ok((tileset, Vec::new()));
        }

        // Try XML format (.tsx files are usually XML)
        Self::parse_tsx_xml_with_collisions(&contents, first_gid, map_dir)
    }

    /// Parse XML .tsx tileset file with collision data
    fn parse_tsx_xml_with_collisions(contents: &str, first_gid: u32, map_dir: &Path) -> Result<(Tileset, Vec<u32>), TilemapError> {
        // Simple XML parsing for <tileset> and <image> tags
        let name = Self::extract_xml_attr(contents, "tileset", "name")
            .unwrap_or_else(|| "tileset".to_string());
        let tile_width: u32 = Self::extract_xml_attr(contents, "tileset", "tilewidth")
            .and_then(|s| s.parse().ok())
            .unwrap_or(16);
        let tile_height: u32 = Self::extract_xml_attr(contents, "tileset", "tileheight")
            .and_then(|s| s.parse().ok())
            .unwrap_or(16);
        let columns: u32 = Self::extract_xml_attr(contents, "tileset", "columns")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let tilecount: u32 = Self::extract_xml_attr(contents, "tileset", "tilecount")
            .and_then(|s| s.parse().ok())
            .unwrap_or(columns);

        let image_source = Self::extract_xml_attr(contents, "image", "source")
            .unwrap_or_default();

        // Resolve image path relative to map directory
        let image_path = map_dir.join(&image_source);
        let image = image_path.to_string_lossy().to_string();

        let rows = if columns > 0 { tilecount / columns } else { 1 };

        // Extract tiles with collision (tiles that have <objectgroup> children)
        let collision_tiles = Self::extract_collision_tile_ids(contents);

        let tileset = Tileset {
            name,
            image,
            tile_width,
            tile_height,
            columns,
            rows,
            first_gid,
        };

        Ok((tileset, collision_tiles))
    }

    /// Extract tile IDs that have collision defined (contain <objectgroup>)
    fn extract_collision_tile_ids(xml: &str) -> Vec<u32> {
        let mut collision_ids = Vec::new();

        // Find all <tile id="X"> tags that contain <objectgroup>
        let mut search_pos = 0;
        while let Some(tile_start) = xml[search_pos..].find("<tile ") {
            let tile_start = search_pos + tile_start;

            // Find the end of this tile element
            let tile_end = if let Some(end) = xml[tile_start..].find("</tile>") {
                tile_start + end + 7
            } else if let Some(end) = xml[tile_start..].find("/>") {
                tile_start + end + 2
            } else {
                break;
            };

            let tile_content = &xml[tile_start..tile_end];

            // Check if this tile has an objectgroup (collision)
            if tile_content.contains("<objectgroup") {
                // Extract the tile id
                if let Some(id) = Self::extract_xml_attr(tile_content, "tile", "id") {
                    if let Ok(tile_id) = id.parse::<u32>() {
                        collision_ids.push(tile_id);
                    }
                }
            }

            search_pos = tile_end;
        }

        collision_ids
    }

    /// Simple XML attribute extraction (no full XML parser needed)
    fn extract_xml_attr(xml: &str, tag: &str, attr: &str) -> Option<String> {
        let tag_start = xml.find(&format!("<{}", tag))?;
        let tag_end = xml[tag_start..].find('>')? + tag_start;
        let tag_content = &xml[tag_start..tag_end];

        let attr_pattern = format!("{}=\"", attr);
        let attr_start = tag_content.find(&attr_pattern)? + attr_pattern.len();
        let attr_end = tag_content[attr_start..].find('"')? + attr_start;

        Some(tag_content[attr_start..attr_end].to_string())
    }

    /// Get the pixel dimensions of the map
    #[must_use]
    pub fn pixel_size(&self) -> (u32, u32) {
        (self.width * self.tile_width, self.height * self.tile_height)
    }

    /// Get the tileset for a given global tile ID
    #[must_use]
    pub fn get_tileset_for_gid(&self, gid: u32) -> Option<(&Tileset, u32)> {
        if gid == 0 {
            return None;
        }

        // Find the tileset that contains this GID
        let mut best_tileset: Option<&Tileset> = None;
        for tileset in &self.tilesets {
            if gid >= tileset.first_gid
                && (best_tileset.is_none()
                    || tileset.first_gid > best_tileset.unwrap().first_gid)
            {
                best_tileset = Some(tileset);
            }
        }

        best_tileset.map(|ts| {
            let local_id = gid - ts.first_gid;
            (ts, local_id)
        })
    }

    /// Generate sprites for visible tiles in a layer
    /// Returns sprites to be rendered (culled to camera view)
    pub fn get_visible_sprites(
        &self,
        layer_index: usize,
        camera: &Camera2D,
    ) -> Vec<(Sprite, usize)> {
        let Some(layer) = self.layers.get(layer_index) else {
            return Vec::new();
        };

        if !layer.visible {
            return Vec::new();
        }

        let mut sprites = Vec::new();
        let (cam_min, cam_max) = camera.visible_bounds();

        // Calculate visible tile range
        let tile_w = self.tile_width as f32;
        let tile_h = self.tile_height as f32;

        let start_x = ((cam_min.x / tile_w).floor() as i32).max(0) as u32;
        let start_y = ((cam_min.y / tile_h).floor() as i32).max(0) as u32;
        let end_x = ((cam_max.x / tile_w).ceil() as u32 + 1).min(layer.width);
        let end_y = ((cam_max.y / tile_h).ceil() as u32 + 1).min(layer.height);

        for y in start_y..end_y {
            for x in start_x..end_x {
                let gid = layer.get_tile(x, y);
                if gid == 0 {
                    continue;
                }

                if let Some((tileset, local_id)) = self.get_tileset_for_gid(gid) {
                    let region = tileset.get_tile_region(local_id);

                    // Position is center of tile
                    let pos = Vec2::new(
                        x as f32 * tile_w + tile_w * 0.5,
                        y as f32 * tile_h + tile_h * 0.5,
                    );

                    let mut sprite = Sprite::new(pos, Vec2::new(tile_w, tile_h));
                    sprite.region = region;
                    sprite.color.w = layer.opacity;

                    // Find tileset index for batching
                    let tileset_idx = self.tilesets.iter()
                        .position(|ts| ts.name == tileset.name)
                        .unwrap_or(0);

                    sprites.push((sprite, tileset_idx));
                }
            }
        }

        sprites
    }

    /// Get indices of layers that render below entities (sorted by z_order)
    #[must_use]
    pub fn below_layers(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = self
            .layers
            .iter()
            .enumerate()
            .filter(|(_, l)| l.layer_type == LayerType::Below && l.visible)
            .map(|(i, _)| i)
            .collect();
        indices.sort_by_key(|&i| self.layers[i].z_order);
        indices
    }

    /// Get indices of layers that render above entities (sorted by z_order)
    #[must_use]
    pub fn above_layers(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = self
            .layers
            .iter()
            .enumerate()
            .filter(|(_, l)| l.layer_type == LayerType::Above && l.visible)
            .map(|(i, _)| i)
            .collect();
        indices.sort_by_key(|&i| self.layers[i].z_order);
        indices
    }

    /// Check if a tile at the given coordinates is solid
    #[must_use]
    pub fn is_tile_solid(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            return true; // Out of bounds is solid
        }
        if self.collision.is_empty() {
            return false; // No collision data
        }
        let index = (y * self.width + x) as usize;
        self.collision.get(index).copied().unwrap_or(false)
    }

    /// Get solid tiles that overlap with a world-space rectangle
    /// Returns tile coordinates (x, y) and their world-space bounds (min, max)
    #[must_use]
    pub fn get_solid_tiles_in_rect(&self, rect_min: Vec2, rect_max: Vec2) -> Vec<(u32, u32, Vec2, Vec2)> {
        if self.collision.is_empty() {
            return Vec::new();
        }

        let tile_w = self.tile_width as f32;
        let tile_h = self.tile_height as f32;

        // Convert to tile coordinates
        let start_x = ((rect_min.x / tile_w).floor() as i32).max(0) as u32;
        let start_y = ((rect_min.y / tile_h).floor() as i32).max(0) as u32;
        let end_x = ((rect_max.x / tile_w).ceil() as u32).min(self.width);
        let end_y = ((rect_max.y / tile_h).ceil() as u32).min(self.height);

        let mut solid_tiles = Vec::new();

        for y in start_y..end_y {
            for x in start_x..end_x {
                if self.is_tile_solid(x, y) {
                    let tile_min = Vec2::new(x as f32 * tile_w, y as f32 * tile_h);
                    let tile_max = Vec2::new(tile_min.x + tile_w, tile_min.y + tile_h);
                    solid_tiles.push((x, y, tile_min, tile_max));
                }
            }
        }

        solid_tiles
    }

    /// Check if there's a collision layer defined
    #[must_use]
    pub fn has_collision(&self) -> bool {
        !self.collision.is_empty()
    }

    /// Get a spawn point by ID
    #[must_use]
    pub fn get_spawn(&self, id: &str) -> Option<&SpawnPoint> {
        self.spawns.iter().find(|s| s.id == id)
    }

    /// Get the default spawn point (first one, or center of map)
    #[must_use]
    pub fn default_spawn(&self) -> Vec2 {
        self.spawns
            .first()
            .map(|s| s.position())
            .unwrap_or_else(|| {
                let (w, h) = self.pixel_size();
                Vec2::new(w as f32 / 2.0, h as f32 / 2.0)
            })
    }

    /// Check if a position triggers a map transition
    /// Returns the trigger if one is activated
    #[must_use]
    pub fn check_trigger(&self, position: Vec2) -> Option<&Trigger> {
        self.triggers.iter().find(|t| t.contains(position))
    }
}

/// Tilemap loading errors
#[derive(Debug)]
pub enum TilemapError {
    IoError(String),
    ParseError(String),
}

impl std::fmt::Display for TilemapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for TilemapError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tileset_region() {
        let tileset = Tileset {
            name: "test".to_string(),
            image: "test.png".to_string(),
            tile_width: 16,
            tile_height: 16,
            columns: 4,
            rows: 4,
            first_gid: 1,
        };

        // Tile 0 (top-left)
        let region = tileset.get_tile_region(0);
        assert!((region.u_min - 0.0).abs() < 0.001);
        assert!((region.v_min - 0.0).abs() < 0.001);
        assert!((region.u_max - 0.25).abs() < 0.001);
        assert!((region.v_max - 0.25).abs() < 0.001);

        // Tile 5 (second row, second column)
        let region = tileset.get_tile_region(5);
        assert!((region.u_min - 0.25).abs() < 0.001);
        assert!((region.v_min - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_layer_get_tile() {
        let layer = TileLayer {
            name: "ground".to_string(),
            width: 3,
            height: 3,
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            visible: true,
            opacity: 1.0,
            z_order: 0,
            layer_type: LayerType::Below,
        };

        assert_eq!(layer.get_tile(0, 0), 1);
        assert_eq!(layer.get_tile(1, 1), 5);
        assert_eq!(layer.get_tile(2, 2), 9);
        assert_eq!(layer.get_tile(3, 3), 0); // Out of bounds
    }

    #[test]
    fn test_tile_collision() {
        let tilemap = Tilemap {
            name: "test".to_string(),
            width: 3,
            height: 3,
            tile_width: 16,
            tile_height: 16,
            tilesets: vec![],
            layers: vec![],
            collision: vec![
                false, true, false,
                true, false, true,
                false, true, false,
            ],
            spawns: vec![],
            triggers: vec![],
        };

        // Check solid tiles
        assert!(!tilemap.is_tile_solid(0, 0));
        assert!(tilemap.is_tile_solid(1, 0));
        assert!(tilemap.is_tile_solid(0, 1));
        assert!(!tilemap.is_tile_solid(1, 1));
        assert!(tilemap.is_tile_solid(1, 2));

        // Out of bounds is solid
        assert!(tilemap.is_tile_solid(5, 5));

        // Get solid tiles in rect (covers tiles 0,0 to 1,1)
        let solid = tilemap.get_solid_tiles_in_rect(Vec2::new(0.0, 0.0), Vec2::new(32.0, 32.0));
        assert_eq!(solid.len(), 2); // (1,0) and (0,1) are solid
    }
}
