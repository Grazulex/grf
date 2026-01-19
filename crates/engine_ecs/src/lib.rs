//! Engine ECS - Entity-Component-System
//!
//! A simple but efficient ECS implementation with:
//! - Generational entity IDs for safe references
//! - SparseSet component storage for O(1) access
//! - Type-erased component storage
//! - Query system for iterating entities with specific components

use std::any::{Any, TypeId};
use std::collections::HashMap;

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

/// Trait for components
pub trait Component: 'static + Send + Sync {}

/// Implement Component for any type that meets the requirements
impl<T: 'static + Send + Sync> Component for T {}

/// Sparse set storage for a single component type
struct SparseSet<T> {
    /// Sparse array: entity index -> dense index
    sparse: Vec<Option<usize>>,
    /// Dense array of components
    dense: Vec<T>,
    /// Entity indices for each dense element
    entities: Vec<u32>,
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SparseSet<T> {
    fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            entities: Vec::new(),
        }
    }

    fn insert(&mut self, entity_index: u32, component: T) {
        let idx = entity_index as usize;

        // Grow sparse array if needed
        if idx >= self.sparse.len() {
            self.sparse.resize(idx + 1, None);
        }

        if let Some(dense_idx) = self.sparse[idx] {
            // Update existing component
            self.dense[dense_idx] = component;
        } else {
            // Add new component
            let dense_idx = self.dense.len();
            self.sparse[idx] = Some(dense_idx);
            self.dense.push(component);
            self.entities.push(entity_index);
        }
    }

    fn remove(&mut self, entity_index: u32) -> Option<T> {
        let idx = entity_index as usize;

        if idx >= self.sparse.len() {
            return None;
        }

        if let Some(dense_idx) = self.sparse[idx].take() {
            // Swap remove from dense arrays
            let _last_idx = self.dense.len() - 1;
            let removed = self.dense.swap_remove(dense_idx);
            self.entities.swap_remove(dense_idx);

            // Update sparse array for swapped element
            if dense_idx < self.dense.len() {
                let swapped_entity = self.entities[dense_idx] as usize;
                self.sparse[swapped_entity] = Some(dense_idx);
            }

            Some(removed)
        } else {
            None
        }
    }

    fn get(&self, entity_index: u32) -> Option<&T> {
        let idx = entity_index as usize;
        self.sparse
            .get(idx)
            .and_then(|opt| opt.as_ref())
            .map(|&dense_idx| &self.dense[dense_idx])
    }

    fn get_mut(&mut self, entity_index: u32) -> Option<&mut T> {
        let idx = entity_index as usize;
        if let Some(Some(dense_idx)) = self.sparse.get(idx) {
            Some(&mut self.dense[*dense_idx])
        } else {
            None
        }
    }

    fn contains(&self, entity_index: u32) -> bool {
        let idx = entity_index as usize;
        self.sparse.get(idx).is_some_and(|opt| opt.is_some())
    }

    #[allow(dead_code)]
    fn iter(&self) -> impl Iterator<Item = (u32, &T)> {
        self.entities.iter().copied().zip(self.dense.iter())
    }

    #[allow(dead_code)]
    fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut T)> {
        self.entities.iter().copied().zip(self.dense.iter_mut())
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.dense.len()
    }
}

/// Type-erased component storage
trait ComponentStorage: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove_entity(&mut self, entity_index: u32);
    #[allow(dead_code)]
    fn len(&self) -> usize;
}

impl<T: Component> ComponentStorage for SparseSet<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove_entity(&mut self, entity_index: u32) {
        self.remove(entity_index);
    }

    fn len(&self) -> usize {
        self.len()
    }
}

/// Trait for resources (singleton data)
pub trait Resource: 'static + Send + Sync {}

/// Implement Resource for any type that meets the requirements
impl<T: 'static + Send + Sync> Resource for T {}

/// The World stores all entities and components
pub struct World {
    /// Entity generations (index -> generation)
    generations: Vec<u32>,
    /// Free entity indices for reuse
    free_indices: Vec<u32>,
    /// Component storages by type
    storages: HashMap<TypeId, Box<dyn ComponentStorage>>,
    /// Resources (singleton data) by type
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    /// Count of alive entities
    entity_count: usize,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    /// Create a new empty world
    #[must_use]
    pub fn new() -> Self {
        Self {
            generations: Vec::new(),
            free_indices: Vec::new(),
            storages: HashMap::new(),
            resources: HashMap::new(),
            entity_count: 0,
        }
    }

    /// Spawn a new entity
    pub fn spawn(&mut self) -> Entity {
        self.entity_count += 1;

        if let Some(index) = self.free_indices.pop() {
            // Reuse freed index with incremented generation
            let generation = self.generations[index as usize];
            Entity::new(index, generation)
        } else {
            // Allocate new index
            let index = self.generations.len() as u32;
            self.generations.push(0);
            Entity::new(index, 0)
        }
    }

    /// Despawn an entity and remove all its components
    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.is_alive(entity) {
            return false;
        }

        // Remove all components
        for storage in self.storages.values_mut() {
            storage.remove_entity(entity.index);
        }

        // Increment generation and mark as free
        self.generations[entity.index as usize] += 1;
        self.free_indices.push(entity.index);
        self.entity_count -= 1;

        true
    }

    /// Check if an entity is alive
    #[must_use]
    pub fn is_alive(&self, entity: Entity) -> bool {
        let idx = entity.index as usize;
        idx < self.generations.len() && self.generations[idx] == entity.generation
    }

    /// Get the number of alive entities
    #[must_use]
    pub fn entity_count(&self) -> usize {
        self.entity_count
    }

    /// Add a component to an entity
    pub fn insert<T: Component>(&mut self, entity: Entity, component: T) {
        if !self.is_alive(entity) {
            return;
        }

        let type_id = TypeId::of::<T>();
        let storage = self
            .storages
            .entry(type_id)
            .or_insert_with(|| Box::new(SparseSet::<T>::new()));

        storage
            .as_any_mut()
            .downcast_mut::<SparseSet<T>>()
            .unwrap()
            .insert(entity.index, component);
    }

    /// Remove a component from an entity
    pub fn remove<T: Component>(&mut self, entity: Entity) -> Option<T> {
        if !self.is_alive(entity) {
            return None;
        }

        let type_id = TypeId::of::<T>();
        self.storages.get_mut(&type_id).and_then(|storage| {
            storage
                .as_any_mut()
                .downcast_mut::<SparseSet<T>>()
                .unwrap()
                .remove(entity.index)
        })
    }

    /// Get a component reference
    #[must_use]
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        if !self.is_alive(entity) {
            return None;
        }

        let type_id = TypeId::of::<T>();
        self.storages.get(&type_id).and_then(|storage| {
            storage
                .as_any()
                .downcast_ref::<SparseSet<T>>()
                .unwrap()
                .get(entity.index)
        })
    }

    /// Get a mutable component reference
    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        if !self.is_alive(entity) {
            return None;
        }

        let type_id = TypeId::of::<T>();
        self.storages.get_mut(&type_id).and_then(|storage| {
            storage
                .as_any_mut()
                .downcast_mut::<SparseSet<T>>()
                .unwrap()
                .get_mut(entity.index)
        })
    }

    /// Check if an entity has a component
    #[must_use]
    pub fn has<T: Component>(&self, entity: Entity) -> bool {
        if !self.is_alive(entity) {
            return false;
        }

        let type_id = TypeId::of::<T>();
        self.storages.get(&type_id).is_some_and(|storage| {
            storage
                .as_any()
                .downcast_ref::<SparseSet<T>>()
                .unwrap()
                .contains(entity.index)
        })
    }

    /// Query entities with a specific component
    pub fn query<T: Component>(&self) -> QueryIter<'_, T> {
        QueryIter::new(self)
    }

    /// Query entities with a specific component (mutable)
    pub fn query_mut<T: Component>(&mut self) -> QueryIterMut<'_, T> {
        QueryIterMut::new(self)
    }

    /// Insert a resource (singleton data)
    pub fn insert_resource<T: Resource>(&mut self, resource: T) {
        let type_id = TypeId::of::<T>();
        self.resources.insert(type_id, Box::new(resource));
    }

    /// Get a resource reference
    #[must_use]
    pub fn get_resource<T: Resource>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.resources
            .get(&type_id)
            .and_then(|r| r.downcast_ref::<T>())
    }

    /// Get a mutable resource reference
    pub fn get_resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.resources
            .get_mut(&type_id)
            .and_then(|r| r.downcast_mut::<T>())
    }

    /// Remove a resource
    pub fn remove_resource<T: Resource>(&mut self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        self.resources
            .remove(&type_id)
            .and_then(|r| r.downcast::<T>().ok())
            .map(|b| *b)
    }

    /// Check if a resource exists
    #[must_use]
    pub fn has_resource<T: Resource>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.resources.contains_key(&type_id)
    }
}

/// Iterator for querying entities with a component
pub struct QueryIter<'a, T: Component> {
    world: &'a World,
    inner: Option<std::iter::Enumerate<std::slice::Iter<'a, T>>>,
    entities: Option<&'a [u32]>,
}

impl<'a, T: Component> QueryIter<'a, T> {
    fn new(world: &'a World) -> Self {
        let type_id = TypeId::of::<T>();
        if let Some(storage) = world.storages.get(&type_id) {
            let sparse_set = storage.as_any().downcast_ref::<SparseSet<T>>().unwrap();
            Self {
                world,
                inner: Some(sparse_set.dense.iter().enumerate()),
                entities: Some(&sparse_set.entities),
            }
        } else {
            Self {
                world,
                inner: None,
                entities: None,
            }
        }
    }
}

impl<'a, T: Component> Iterator for QueryIter<'a, T> {
    type Item = (Entity, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        let entities = self.entities?;

        for (dense_idx, component) in inner.by_ref() {
            let entity_index = entities[dense_idx];
            let generation = self.world.generations[entity_index as usize];
            let entity = Entity::new(entity_index, generation);

            if self.world.is_alive(entity) {
                return Some((entity, component));
            }
        }

        None
    }
}

/// Mutable iterator for querying entities with a component
pub struct QueryIterMut<'a, T: Component> {
    generations: &'a [u32],
    inner: Option<std::iter::Zip<std::slice::Iter<'a, u32>, std::slice::IterMut<'a, T>>>,
}

impl<'a, T: Component> QueryIterMut<'a, T> {
    fn new(world: &'a mut World) -> Self {
        let type_id = TypeId::of::<T>();
        let generations = &world.generations as *const Vec<u32>;

        if let Some(storage) = world.storages.get_mut(&type_id) {
            let sparse_set = storage
                .as_any_mut()
                .downcast_mut::<SparseSet<T>>()
                .unwrap();

            // Safety: we're borrowing entities immutably and dense mutably
            // They don't overlap
            let entities = &sparse_set.entities as *const Vec<u32>;
            let dense = &mut sparse_set.dense as *mut Vec<T>;

            unsafe {
                Self {
                    generations: &*generations,
                    inner: Some((*entities).iter().zip((*dense).iter_mut())),
                }
            }
        } else {
            unsafe {
                Self {
                    generations: &*generations,
                    inner: None,
                }
            }
        }
    }
}

impl<'a, T: Component> Iterator for QueryIterMut<'a, T> {
    type Item = (Entity, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;

        if let Some((&entity_index, component)) = inner.next() {
            let generation = self.generations[entity_index as usize];
            let entity = Entity::new(entity_index, generation);
            return Some((entity, component));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Debug, PartialEq)]
    struct Velocity {
        x: f32,
        y: f32,
    }

    #[derive(Debug)]
    struct Health(i32);

    #[test]
    fn test_spawn_despawn() {
        let mut world = World::new();

        let e1 = world.spawn();
        let e2 = world.spawn();

        assert!(world.is_alive(e1));
        assert!(world.is_alive(e2));
        assert_eq!(world.entity_count(), 2);

        world.despawn(e1);
        assert!(!world.is_alive(e1));
        assert!(world.is_alive(e2));
        assert_eq!(world.entity_count(), 1);
    }

    #[test]
    fn test_generation_reuse() {
        let mut world = World::new();

        let e1 = world.spawn();
        assert_eq!(e1.generation, 0);

        world.despawn(e1);

        let e2 = world.spawn();
        assert_eq!(e2.index, e1.index); // Same index
        assert_eq!(e2.generation, 1); // New generation

        // Old entity reference is stale
        assert!(!world.is_alive(e1));
        assert!(world.is_alive(e2));
    }

    #[test]
    fn test_insert_get_component() {
        let mut world = World::new();
        let entity = world.spawn();

        world.insert(entity, Position { x: 10.0, y: 20.0 });
        world.insert(entity, Velocity { x: 1.0, y: 2.0 });

        let pos = world.get::<Position>(entity).unwrap();
        assert_eq!(pos.x, 10.0);
        assert_eq!(pos.y, 20.0);

        let vel = world.get::<Velocity>(entity).unwrap();
        assert_eq!(vel.x, 1.0);
        assert_eq!(vel.y, 2.0);
    }

    #[test]
    fn test_remove_component() {
        let mut world = World::new();
        let entity = world.spawn();

        world.insert(entity, Health(100));
        assert!(world.has::<Health>(entity));

        let health = world.remove::<Health>(entity);
        assert_eq!(health.unwrap().0, 100);
        assert!(!world.has::<Health>(entity));
    }

    #[test]
    fn test_query() {
        let mut world = World::new();

        let e1 = world.spawn();
        let e2 = world.spawn();
        let e3 = world.spawn();

        world.insert(e1, Position { x: 1.0, y: 1.0 });
        world.insert(e2, Position { x: 2.0, y: 2.0 });
        world.insert(e3, Velocity { x: 3.0, y: 3.0 }); // No position

        let positions: Vec<_> = world.query::<Position>().collect();
        assert_eq!(positions.len(), 2);
    }

    #[test]
    fn test_query_mut() {
        let mut world = World::new();

        let e1 = world.spawn();
        let e2 = world.spawn();

        world.insert(e1, Position { x: 0.0, y: 0.0 });
        world.insert(e2, Position { x: 0.0, y: 0.0 });

        // Modify all positions
        for (_, pos) in world.query_mut::<Position>() {
            pos.x += 10.0;
        }

        assert_eq!(world.get::<Position>(e1).unwrap().x, 10.0);
        assert_eq!(world.get::<Position>(e2).unwrap().x, 10.0);
    }

    #[test]
    fn test_despawn_removes_components() {
        let mut world = World::new();
        let entity = world.spawn();

        world.insert(entity, Position { x: 0.0, y: 0.0 });
        world.insert(entity, Velocity { x: 1.0, y: 1.0 });

        world.despawn(entity);

        // Components should be gone
        let positions: Vec<_> = world.query::<Position>().collect();
        assert_eq!(positions.len(), 0);
    }

    #[test]
    fn test_resources() {
        #[derive(Debug, PartialEq)]
        struct GameTime {
            delta: f32,
            total: f32,
        }

        let mut world = World::new();

        // Insert resource
        world.insert_resource(GameTime { delta: 0.016, total: 0.0 });

        // Get resource
        assert!(world.has_resource::<GameTime>());
        let time = world.get_resource::<GameTime>().unwrap();
        assert_eq!(time.delta, 0.016);

        // Mutate resource
        {
            let time = world.get_resource_mut::<GameTime>().unwrap();
            time.total += time.delta;
        }
        assert_eq!(world.get_resource::<GameTime>().unwrap().total, 0.016);

        // Remove resource
        let removed = world.remove_resource::<GameTime>().unwrap();
        assert_eq!(removed.delta, 0.016);
        assert!(!world.has_resource::<GameTime>());
    }
}
