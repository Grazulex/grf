# engine_assets

Module de gestion des ressources avec chargement et cache.

## Vue d'ensemble

`engine_assets` fournit un systeme de gestion des ressources:
- **AssetManager**: Gestionnaire centralise des ressources
- **Handle**: References typees aux assets
- **Loaders**: Chargeurs specialises par type

---

## AssetManager

Gestionnaire centralise des ressources avec cache.

### Structure

```rust
pub struct AssetManager {
    // Caches par type
    textures: HashMap<String, Texture>,
    sounds: HashMap<String, Sound>,
    tilemaps: HashMap<String, Tilemap>,

    // Chemin de base
    base_path: PathBuf,

    // Statistiques
    stats: AssetStats,
}

pub struct AssetStats {
    pub textures_loaded: usize,
    pub sounds_loaded: usize,
    pub total_memory: usize,
}
```

### Methodes

```rust
impl AssetManager {
    /// Cree un gestionnaire avec chemin de base
    pub fn new(base_path: impl AsRef<Path>) -> Self;

    /// Cree avec chemin "assets/"
    pub fn with_default_path() -> Self;
}
```

### Textures

```rust
impl AssetManager {
    /// Charge une texture
    pub fn load_texture(&mut self, path: &str, renderer: &Renderer) -> Result<Handle<Texture>, AssetError>;

    /// Retourne une texture deja chargee
    pub fn get_texture(&self, path: &str) -> Option<&Texture>;

    /// Retourne une texture par handle
    pub fn texture(&self, handle: Handle<Texture>) -> Option<&Texture>;

    /// Precharge plusieurs textures
    pub fn preload_textures(&mut self, paths: &[&str], renderer: &Renderer) -> Result<(), AssetError>;

    /// Decharge une texture
    pub fn unload_texture(&mut self, path: &str);
}
```

### Sons

```rust
impl AssetManager {
    /// Charge un son
    pub fn load_sound(&mut self, path: &str) -> Result<Handle<Sound>, AssetError>;

    /// Retourne un son
    pub fn get_sound(&self, path: &str) -> Option<&Sound>;

    /// Retourne un son par handle
    pub fn sound(&self, handle: Handle<Sound>) -> Option<&Sound>;
}
```

### Tilemaps

```rust
impl AssetManager {
    /// Charge une tilemap depuis JSON
    pub fn load_tilemap(&mut self, path: &str) -> Result<Handle<Tilemap>, AssetError>;

    /// Retourne une tilemap
    pub fn get_tilemap(&self, path: &str) -> Option<&Tilemap>;
}
```

### Utilitaires

```rust
impl AssetManager {
    /// Chemin complet depuis relatif
    pub fn resolve_path(&self, path: &str) -> PathBuf;

    /// Statistiques d'utilisation
    pub fn stats(&self) -> &AssetStats;

    /// Vide tout le cache
    pub fn clear(&mut self);

    /// Vide le cache des textures
    pub fn clear_textures(&mut self);
}
```

---

## Handle

Reference typee vers une ressource.

### Structure

```rust
#[derive(Clone, Copy)]
pub struct Handle<T> {
    /// Index dans le cache
    index: u32,

    /// Generation pour validation
    generation: u32,

    /// Marqueur de type
    _marker: PhantomData<T>,
}
```

### Methodes

```rust
impl<T> Handle<T> {
    /// Handle invalide
    pub const INVALID: Handle<T>;

    /// Verifie si le handle est valide
    pub fn is_valid(&self) -> bool;
}
```

### Utilisation

```rust
// Chargement retourne un handle
let texture_handle = assets.load_texture("player.png", &renderer)?;

// Acces via handle
if let Some(texture) = assets.texture(texture_handle) {
    renderer.draw_sprite(&sprite, texture);
}

// Ou acces direct par chemin
if let Some(texture) = assets.get_texture("player.png") {
    renderer.draw_sprite(&sprite, texture);
}
```

---

## AssetError

Erreurs de chargement.

```rust
#[derive(Debug)]
pub enum AssetError {
    /// Fichier non trouve
    NotFound { path: String },

    /// Erreur de lecture
    IoError { path: String, error: String },

    /// Format invalide
    InvalidFormat { path: String, error: String },

    /// Type non supporte
    UnsupportedType { path: String, extension: String },
}

impl std::error::Error for AssetError {}
```

---

## Structure des assets

```
assets/
├── textures/
│   ├── characters/
│   │   ├── player.png
│   │   └── npc.png
│   ├── tilesets/
│   │   └── farm.png
│   ├── items/
│   │   └── items.png
│   └── ui/
│       └── hud.png
├── audio/
│   ├── music/
│   │   └── main_theme.ogg
│   └── sfx/
│       ├── footstep.wav
│       └── pickup.wav
├── maps/
│   ├── farm.json
│   └── house.json
└── data/
    ├── items.toml
    └── npcs.toml
```

---

## Utilisation complete

```rust
use engine_assets::{AssetManager, Handle, AssetError};

fn main() -> Result<(), AssetError> {
    // Creer le gestionnaire
    let mut assets = AssetManager::with_default_path();

    // Precharger les textures au demarrage
    assets.preload_textures(&[
        "textures/characters/player.png",
        "textures/tilesets/farm.png",
        "textures/ui/hud.png",
    ], &renderer)?;

    // Charger la map
    let map_handle = assets.load_tilemap("maps/farm.json")?;

    // Charger la musique
    let music_handle = assets.load_sound("audio/music/main_theme.ogg")?;

    println!("Assets charges: {:?}", assets.stats());

    // Dans la boucle de jeu
    loop {
        // Acces aux textures pour le rendu
        if let Some(player_tex) = assets.get_texture("textures/characters/player.png") {
            renderer.draw_sprite(&player_sprite, player_tex);
        }

        if let Some(tileset) = assets.get_texture("textures/tilesets/farm.png") {
            if let Some(map) = assets.get_tilemap("maps/farm.json") {
                renderer.draw_tilemap(map, tileset);
            }
        }
    }
}
```

---

## Chargement paresseux vs preload

### Preload (recommande pour assets critiques)

```rust
// Au demarrage, charge tout d'un coup
assets.preload_textures(&[
    "player.png",
    "tileset.png",
    "ui.png",
], &renderer)?;
```

### Chargement paresseux

```rust
// Charge a la demande
fn get_or_load_texture(
    assets: &mut AssetManager,
    path: &str,
    renderer: &Renderer,
) -> Option<&Texture> {
    if assets.get_texture(path).is_none() {
        let _ = assets.load_texture(path, renderer);
    }
    assets.get_texture(path)
}
```

---

## Bonnes pratiques

1. **Precharger au demarrage**: Charger les assets frequents au lancement
2. **Utiliser des handles**: Eviter de stocker des references directes
3. **Gerer les erreurs**: Toujours gerer les cas d'echec de chargement
4. **Nettoyer**: Decharger les assets non utilises entre les niveaux
5. **Chemins relatifs**: Toujours utiliser des chemins relatifs au dossier assets
