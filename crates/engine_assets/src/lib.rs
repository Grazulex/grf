//! Engine Assets - Asset loading and caching
//!
//! This crate provides asset management with:
//! - Type-safe handles
//! - Centralized storage and caching
//! - Path-based deduplication

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use thiserror::Error;

/// Asset loading errors
#[derive(Error, Debug)]
pub enum AssetError {
    /// File not found
    #[error("Asset not found: {0}")]
    NotFound(String),
    /// Failed to load asset
    #[error("Failed to load asset: {0}")]
    LoadFailed(String),
    /// Invalid asset format
    #[error("Invalid asset format: {0}")]
    InvalidFormat(String),
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Type-safe handle to a loaded asset
#[derive(Debug)]
pub struct Handle<T> {
    /// Unique identifier for the asset
    id: u32,
    /// Phantom data to carry the type
    _marker: PhantomData<T>,
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Handle<T> {}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Handle<T> {}

impl<T> std::hash::Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> Handle<T> {
    /// Create a new handle with the given ID
    fn new(id: u32) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    /// Get the raw ID of this handle
    #[must_use]
    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Trait for types that can be loaded as assets
pub trait Asset: 'static + Send + Sync {}

/// Implement Asset for any type that meets the requirements
impl<T: 'static + Send + Sync> Asset for T {}

/// Storage for assets of a specific type
struct AssetStorage<T: Asset> {
    /// Assets stored by handle ID
    assets: HashMap<u32, Arc<T>>,
    /// Path to handle mapping for deduplication
    path_to_handle: HashMap<PathBuf, u32>,
    /// Next handle ID to assign
    next_id: u32,
}

impl<T: Asset> Default for AssetStorage<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Asset> AssetStorage<T> {
    /// Create a new empty storage
    fn new() -> Self {
        Self {
            assets: HashMap::new(),
            path_to_handle: HashMap::new(),
            next_id: 0,
        }
    }

    /// Insert an asset and return its handle
    fn insert(&mut self, asset: T) -> Handle<T> {
        let id = self.next_id;
        self.next_id += 1;
        self.assets.insert(id, Arc::new(asset));
        Handle::new(id)
    }

    /// Insert an asset with a path for caching
    fn insert_with_path(&mut self, asset: T, path: &Path) -> Handle<T> {
        let id = self.next_id;
        self.next_id += 1;
        self.assets.insert(id, Arc::new(asset));
        self.path_to_handle.insert(path.to_path_buf(), id);
        Handle::new(id)
    }

    /// Get an asset by handle
    fn get(&self, handle: Handle<T>) -> Option<&T> {
        self.assets.get(&handle.id).map(|arc| arc.as_ref())
    }

    /// Get an Arc to an asset by handle
    fn get_arc(&self, handle: Handle<T>) -> Option<Arc<T>> {
        self.assets.get(&handle.id).cloned()
    }

    /// Check if an asset exists for the given path
    fn get_by_path(&self, path: &Path) -> Option<Handle<T>> {
        self.path_to_handle.get(path).map(|&id| Handle::new(id))
    }

    /// Remove an asset by handle
    fn remove(&mut self, handle: Handle<T>) -> Option<Arc<T>> {
        // Remove from path mapping
        self.path_to_handle.retain(|_, &mut id| id != handle.id);
        self.assets.remove(&handle.id)
    }

    /// Get the number of stored assets
    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.assets.len()
    }
}

/// Type-erased asset storage
trait AnyAssetStorage: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Asset> AnyAssetStorage for AssetStorage<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Centralized resource manager for all assets
pub struct ResourceManager {
    /// Storages by asset type
    storages: HashMap<TypeId, Box<dyn AnyAssetStorage>>,
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceManager {
    /// Create a new resource manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }

    /// Get or create storage for a specific asset type
    fn get_storage<T: Asset>(&self) -> Option<&AssetStorage<T>> {
        let type_id = TypeId::of::<T>();
        self.storages
            .get(&type_id)
            .and_then(|s| s.as_any().downcast_ref())
    }

    /// Get or create mutable storage for a specific asset type
    fn get_storage_mut<T: Asset>(&mut self) -> &mut AssetStorage<T> {
        let type_id = TypeId::of::<T>();
        self.storages
            .entry(type_id)
            .or_insert_with(|| Box::new(AssetStorage::<T>::new()))
            .as_any_mut()
            .downcast_mut()
            .unwrap()
    }

    /// Insert an asset directly and return its handle
    pub fn insert<T: Asset>(&mut self, asset: T) -> Handle<T> {
        self.get_storage_mut().insert(asset)
    }

    /// Insert an asset with a path for caching
    pub fn insert_with_path<T: Asset>(&mut self, asset: T, path: impl AsRef<Path>) -> Handle<T> {
        self.get_storage_mut().insert_with_path(asset, path.as_ref())
    }

    /// Get an asset by handle
    pub fn get<T: Asset>(&self, handle: Handle<T>) -> Option<&T> {
        self.get_storage().and_then(|s| s.get(handle))
    }

    /// Get an Arc to an asset by handle
    pub fn get_arc<T: Asset>(&self, handle: Handle<T>) -> Option<Arc<T>> {
        self.get_storage().and_then(|s| s.get_arc(handle))
    }

    /// Check if an asset is already loaded for the given path
    pub fn get_by_path<T: Asset>(&self, path: impl AsRef<Path>) -> Option<Handle<T>> {
        self.get_storage()
            .and_then(|s| s.get_by_path(path.as_ref()))
    }

    /// Remove an asset by handle
    pub fn remove<T: Asset>(&mut self, handle: Handle<T>) -> Option<Arc<T>> {
        self.get_storage_mut().remove(handle)
    }
}

/// Bytes asset for raw data
#[derive(Debug, Clone)]
pub struct Bytes(pub Vec<u8>);

impl ResourceManager {
    /// Load raw bytes from a file path
    pub fn load_bytes(&mut self, path: impl AsRef<Path>) -> Result<Handle<Bytes>, AssetError> {
        let path = path.as_ref();

        // Check cache first
        if let Some(handle) = self.get_by_path::<Bytes>(path) {
            return Ok(handle);
        }

        // Load from disk
        let data = std::fs::read(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AssetError::NotFound(path.display().to_string())
            } else {
                AssetError::Io(e)
            }
        })?;

        Ok(self.insert_with_path(Bytes(data), path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestAsset {
        value: i32,
    }

    #[test]
    fn test_insert_and_get() {
        let mut rm = ResourceManager::new();

        let handle = rm.insert(TestAsset { value: 42 });
        let asset = rm.get(handle).unwrap();

        assert_eq!(asset.value, 42);
    }

    #[test]
    fn test_path_caching() {
        let mut rm = ResourceManager::new();
        let path = Path::new("test/path.txt");

        let handle1 = rm.insert_with_path(TestAsset { value: 1 }, path);
        let handle2 = rm.get_by_path::<TestAsset>(path).unwrap();

        assert_eq!(handle1.id(), handle2.id());
    }

    #[test]
    fn test_remove() {
        let mut rm = ResourceManager::new();

        let handle = rm.insert(TestAsset { value: 42 });
        assert!(rm.get(handle).is_some());

        rm.remove(handle);
        assert!(rm.get(handle).is_none());
    }

    #[test]
    fn test_multiple_types() {
        let mut rm = ResourceManager::new();

        #[derive(Debug)]
        struct TypeA(i32);
        #[derive(Debug)]
        struct TypeB(String);

        let handle_a = rm.insert(TypeA(42));
        let handle_b = rm.insert(TypeB("hello".to_string()));

        assert_eq!(rm.get(handle_a).unwrap().0, 42);
        assert_eq!(rm.get(handle_b).unwrap().0, "hello");
    }
}
