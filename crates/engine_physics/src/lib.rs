//! Engine Physics - 2D Collision Detection
//!
//! This crate provides AABB collision detection with spatial
//! partitioning for efficient broad-phase collision detection.

use glam::Vec2;
use std::collections::HashMap;

/// Entity ID type (will be replaced by ECS Entity later)
pub type EntityId = u32;

/// Axis-Aligned Bounding Box for collision detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    /// Minimum corner (bottom-left)
    pub min: Vec2,
    /// Maximum corner (top-right)
    pub max: Vec2,
}

impl AABB {
    /// Create a new AABB from position and size
    #[must_use]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            min: Vec2::new(x, y),
            max: Vec2::new(x + width, y + height),
        }
    }

    /// Check if this AABB intersects with another
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
    }

    /// Check if this AABB contains a point
    #[must_use]
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    /// Get the width of this AABB
    #[must_use]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    /// Get the height of this AABB
    #[must_use]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// Get the center point of this AABB
    #[must_use]
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }
}

/// Default spatial grid cell size in pixels
pub const DEFAULT_CELL_SIZE: f32 = 64.0;

/// Information about a collision between two AABBs
#[derive(Debug, Clone, Copy)]
pub struct CollisionInfo {
    /// Overlap amount on each axis
    pub overlap: Vec2,
    /// Normal vector pointing from A to B (direction to push A out of B)
    pub normal: Vec2,
    /// Minimum translation vector to separate the boxes
    pub mtv: Vec2,
}

impl AABB {
    /// Get detailed collision information with another AABB
    /// Returns None if no collision
    #[must_use]
    pub fn get_collision(&self, other: &Self) -> Option<CollisionInfo> {
        // Calculate overlap on each axis
        let overlap_x = (self.max.x.min(other.max.x) - self.min.x.max(other.min.x)).max(0.0);
        let overlap_y = (self.max.y.min(other.max.y) - self.min.y.max(other.min.y)).max(0.0);

        if overlap_x <= 0.0 || overlap_y <= 0.0 {
            return None;
        }

        let overlap = Vec2::new(overlap_x, overlap_y);

        // Determine push direction (smallest axis)
        let self_center = self.center();
        let other_center = other.center();
        let diff = self_center - other_center;

        let (normal, mtv) = if overlap_x < overlap_y {
            // Push on X axis
            let nx = if diff.x >= 0.0 { 1.0 } else { -1.0 };
            (Vec2::new(nx, 0.0), Vec2::new(overlap_x * nx, 0.0))
        } else {
            // Push on Y axis
            let ny = if diff.y >= 0.0 { 1.0 } else { -1.0 };
            (Vec2::new(0.0, ny), Vec2::new(0.0, overlap_y * ny))
        };

        Some(CollisionInfo { overlap, normal, mtv })
    }

    /// Create AABB from center and half-extents
    #[must_use]
    pub fn from_center(center: Vec2, half_size: Vec2) -> Self {
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    /// Move the AABB by an offset
    #[must_use]
    pub fn translated(&self, offset: Vec2) -> Self {
        Self {
            min: self.min + offset,
            max: self.max + offset,
        }
    }
}

/// Spatial grid for broad-phase collision detection
/// Divides space into cells to reduce collision tests from O(n²) to O(n)
#[derive(Debug)]
pub struct SpatialGrid {
    /// Cell size in pixels
    cell_size: f32,
    /// Map from cell coordinate to entities in that cell
    cells: HashMap<(i32, i32), Vec<EntityId>>,
    /// Map from entity to its current cells
    entity_cells: HashMap<EntityId, Vec<(i32, i32)>>,
}

impl Default for SpatialGrid {
    fn default() -> Self {
        Self::new(DEFAULT_CELL_SIZE)
    }
}

impl SpatialGrid {
    /// Create a new spatial grid with the given cell size
    #[must_use]
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
            entity_cells: HashMap::new(),
        }
    }

    /// Get the cell coordinate for a world position
    fn cell_coord(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }

    /// Get all cells that an AABB overlaps
    fn get_cells_for_aabb(&self, aabb: &AABB) -> Vec<(i32, i32)> {
        let min_cell = self.cell_coord(aabb.min);
        let max_cell = self.cell_coord(aabb.max);

        let mut cells = Vec::new();
        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                cells.push((x, y));
            }
        }
        cells
    }

    /// Insert an entity into the grid
    pub fn insert(&mut self, entity: EntityId, aabb: &AABB) {
        let cells = self.get_cells_for_aabb(aabb);

        for &cell in &cells {
            self.cells.entry(cell).or_default().push(entity);
        }

        self.entity_cells.insert(entity, cells);
    }

    /// Remove an entity from the grid
    pub fn remove(&mut self, entity: EntityId) {
        if let Some(cells) = self.entity_cells.remove(&entity) {
            for cell in cells {
                if let Some(entities) = self.cells.get_mut(&cell) {
                    entities.retain(|&e| e != entity);
                }
            }
        }
    }

    /// Update an entity's position in the grid
    pub fn update(&mut self, entity: EntityId, aabb: &AABB) {
        self.remove(entity);
        self.insert(entity, aabb);
    }

    /// Query for potential collision candidates with an AABB
    /// Returns entity IDs that might collide (broad phase)
    #[must_use]
    pub fn query(&self, aabb: &AABB) -> Vec<EntityId> {
        let cells = self.get_cells_for_aabb(aabb);
        let mut candidates = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for cell in cells {
            if let Some(entities) = self.cells.get(&cell) {
                for &entity in entities {
                    if seen.insert(entity) {
                        candidates.push(entity);
                    }
                }
            }
        }

        candidates
    }

    /// Clear all entities from the grid
    pub fn clear(&mut self) {
        self.cells.clear();
        self.entity_cells.clear();
    }

    /// Get the number of entities in the grid
    #[must_use]
    pub fn len(&self) -> usize {
        self.entity_cells.len()
    }

    /// Check if the grid is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entity_cells.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_intersects() {
        let a = AABB::new(0.0, 0.0, 10.0, 10.0);
        let b = AABB::new(5.0, 5.0, 10.0, 10.0);
        let c = AABB::new(20.0, 20.0, 10.0, 10.0);

        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
        assert!(!a.intersects(&c));
    }

    #[test]
    fn test_aabb_contains_point() {
        let aabb = AABB::new(0.0, 0.0, 10.0, 10.0);

        assert!(aabb.contains_point(Vec2::new(5.0, 5.0)));
        assert!(aabb.contains_point(Vec2::new(0.0, 0.0)));
        assert!(!aabb.contains_point(Vec2::new(-1.0, 5.0)));
        assert!(!aabb.contains_point(Vec2::new(11.0, 5.0)));
    }

    #[test]
    fn test_collision_info() {
        let a = AABB::new(0.0, 0.0, 10.0, 10.0);
        let b = AABB::new(8.0, 0.0, 10.0, 10.0);

        let info = a.get_collision(&b).unwrap();
        assert!((info.overlap.x - 2.0).abs() < 0.001);
        assert!(info.normal.x < 0.0); // A should push left (away from B)
    }

    #[test]
    fn test_spatial_grid_query() {
        let mut grid = SpatialGrid::new(64.0);

        grid.insert(1, &AABB::new(0.0, 0.0, 32.0, 32.0));
        grid.insert(2, &AABB::new(100.0, 100.0, 32.0, 32.0));
        grid.insert(3, &AABB::new(16.0, 16.0, 32.0, 32.0));

        // Query near entity 1 and 3
        let candidates = grid.query(&AABB::new(0.0, 0.0, 50.0, 50.0));
        assert!(candidates.contains(&1));
        assert!(candidates.contains(&3));
        assert!(!candidates.contains(&2));
    }

    #[test]
    fn test_spatial_grid_update() {
        let mut grid = SpatialGrid::new(64.0);

        grid.insert(1, &AABB::new(0.0, 0.0, 32.0, 32.0));

        // Entity 1 should be found near origin
        let candidates = grid.query(&AABB::new(0.0, 0.0, 10.0, 10.0));
        assert!(candidates.contains(&1));

        // Move entity far away
        grid.update(1, &AABB::new(500.0, 500.0, 32.0, 32.0));

        // Entity 1 should no longer be found near origin
        let candidates = grid.query(&AABB::new(0.0, 0.0, 10.0, 10.0));
        assert!(!candidates.contains(&1));

        // But should be found at new location
        let candidates = grid.query(&AABB::new(500.0, 500.0, 10.0, 10.0));
        assert!(candidates.contains(&1));
    }
}
