//! Engine ECS - Entity-Component-System
//!
//! This crate provides a custom ECS implementation with:
//! - Generational entity IDs
//! - Sparse set component storage
//! - Query system for iterating entities
//! - Resource management for global state

/// Entity identifier with generation for safe references
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    /// Index in the entity array
    pub index: u32,
    /// Generation to detect stale references
    pub generation: u32,
}

impl Entity {
    /// Create a new entity with given index and generation
    #[must_use]
    pub const fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }
}
