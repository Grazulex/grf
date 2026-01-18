# Moteur de Jeu 2D From Scratch — Document d'Architecture

**Projet** : Moteur 2D orienté RPG/Farming Top-Down  
**Langage** : Rust  
**Plateformes cibles** : Linux, Windows, macOS  
**Auteur** : Jean-Marc  
**Date** : Janvier 2026

---

## 1. Vision et Objectifs

### 1.1 Philosophie du projet

Construire un moteur de jeu 2D from scratch signifie maîtriser chaque couche, de la création de fenêtre jusqu'au rendu final. L'objectif n'est pas de concurrencer Unity ou Godot, mais de créer un outil taillé sur mesure pour un style de jeu précis : le RPG/farming top-down à la Stardew Valley.

### 1.2 Objectifs techniques

- **Performance** : 60 FPS stable sur hardware modeste
- **Portabilité** : Un seul codebase pour Linux, Windows et macOS
- **Extensibilité** : Architecture modulaire permettant l'ajout de features sans refactoring majeur
- **Data-driven** : Le contenu du jeu (maps, items, dialogues) est externalisé dans des fichiers de configuration
- **Débogage** : Outils intégrés pour visualiser collisions, performances, états

### 1.3 Ce que le moteur gère vs ce qu'il ne gère pas

| Inclus | Exclus |
|--------|--------|
| Fenêtrage et event loop | Physique 3D |
| Rendu 2D (sprites, tilemaps) | Networking avancé |
| Audio basique | Scripting runtime (Lua, etc.) |
| Input (clavier, manette, souris) | Éditeur visuel WYSIWYG |
| Collision 2D AABB | Support mobile |
| Système ECS | VR/AR |

---

## 2. Stack Technique

### 2.1 Dépendances minimales

Le principe "from scratch" n'exclut pas l'utilisation de bibliothèques bas niveau qui abstraient les différences OS. Voici la stack recommandée :

```toml
[dependencies]
# Fenêtrage et événements
winit = "0.29"

# Abstraction GPU cross-platform
wgpu = "0.19"

# Mathématiques (vecteurs, matrices)
glam = "0.25"

# Sérialisation (configs, saves)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# Audio
rodio = "0.17"

# Images
image = "0.24"

# Temps et profiling
instant = "0.1"

# Debug UI (optionnel, feature-gated)
egui = { version = "0.27", optional = true }
egui-wgpu = { version = "0.27", optional = true }
egui-winit = { version = "0.27", optional = true }

[features]
default = ["debug-tools"]
debug-tools = ["egui", "egui-wgpu", "egui-winit"]
```

### 2.2 Justification des choix

**winit** : Abstraction cross-platform pour la création de fenêtres et la gestion des événements. C'est le standard de facto dans l'écosystème Rust. Alternative : SDL2 via `sdl2` crate.

**wgpu** : Abstraction GPU qui compile vers Vulkan (Linux), Metal (macOS), DX12 (Windows) et WebGPU. Plus moderne que OpenGL, avec une API explicite qui force à comprendre le pipeline graphique.

**glam** : Bibliothèque mathématique optimisée avec SIMD. Simple, rapide, sans dépendances.

**rodio** : Audio simple et cross-platform. Suffisant pour effets sonores et musique de fond.

### 2.3 Structure du workspace Cargo

```
game-engine/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── engine_core/        # Event loop, temps, lifecycle
│   ├── engine_window/      # Fenêtrage (winit wrapper)
│   ├── engine_render/      # Pipeline de rendu 2D
│   ├── engine_input/       # Abstraction input
│   ├── engine_audio/       # Système audio
│   ├── engine_ecs/         # Entity-Component-System
│   ├── engine_physics/     # Collision 2D
│   ├── engine_assets/      # Chargement et cache des ressources
│   ├── engine_ui/          # UI in-game basique
│   └── engine_debug/       # Outils de debug (feature-gated)
└── game/                   # Le jeu lui-même
    ├── Cargo.toml
    └── src/
```

---

## 3. Architecture Core

### 3.1 Game Loop

Le cœur du moteur est une boucle fixe avec interpolation pour le rendu :

```
┌─────────────────────────────────────────────────────────────┐
│                        GAME LOOP                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐  │
│  │  Input  │───▶│  Update │───▶│  Render │───▶│  Swap   │  │
│  │ (poll)  │    │ (fixed) │    │(interp) │    │(present)│  │
│  └─────────┘    └─────────┘    └─────────┘    └─────────┘  │
│       │              │              │              │        │
│       ▼              ▼              ▼              ▼        │
│   InputState    GameState     FrameBuffer      Display     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Fixed timestep** : La logique de jeu tourne à un pas fixe (ex: 60 updates/sec) pour garantir un comportement déterministe.

**Variable render** : Le rendu tourne aussi vite que possible, avec interpolation entre deux états de jeu pour un mouvement fluide.

```rust
// Pseudo-code du game loop
const FIXED_TIMESTEP: f64 = 1.0 / 60.0;
let mut accumulator = 0.0;
let mut previous_time = Instant::now();

loop {
    let current_time = Instant::now();
    let frame_time = (current_time - previous_time).as_secs_f64();
    previous_time = current_time;
    
    accumulator += frame_time;
    
    // Process input une fois par frame
    input_system.poll_events(&window);
    
    // Fixed update loop
    while accumulator >= FIXED_TIMESTEP {
        game_state.update(FIXED_TIMESTEP, &input_state);
        accumulator -= FIXED_TIMESTEP;
    }
    
    // Render avec interpolation
    let alpha = accumulator / FIXED_TIMESTEP;
    renderer.render(&game_state, alpha);
}
```

### 3.2 Architecture ECS (Entity-Component-System)

L'ECS est le pattern dominant pour les moteurs de jeux modernes. Il favorise la composition sur l'héritage et offre d'excellentes performances grâce à la localité mémoire.

```
┌─────────────────────────────────────────────────────────────┐
│                          WORLD                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ENTITIES          COMPONENTS              SYSTEMS          │
│  ┌───────┐         ┌─────────────┐        ┌─────────────┐  │
│  │ E0001 │────────▶│ Position    │◀───────│ Movement    │  │
│  │ E0002 │────────▶│ Velocity    │◀───────│ System      │  │
│  │ E0003 │         │ Sprite      │        └─────────────┘  │
│  │ ...   │         │ Collider    │        ┌─────────────┐  │
│  └───────┘         │ Health      │◀───────│ Render      │  │
│                    │ Inventory   │        │ System      │  │
│                    │ ...         │        └─────────────┘  │
│                    └─────────────┘        ┌─────────────┐  │
│                                           │ Collision   │  │
│                                           │ System      │  │
│                                           └─────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

**Entity** : Simple identifiant unique (u64 ou générational index)

**Component** : Données pures sans logique (struct avec des champs)

**System** : Logique qui opère sur des ensembles de components

```rust
// Exemple de components
#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Sprite {
    pub texture_id: TextureId,
    pub region: Rect,
    pub z_order: i32,
}

// Exemple de system
fn movement_system(world: &mut World, dt: f32) {
    for (entity, (pos, vel)) in world.query::<(&mut Position, &Velocity)>() {
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
    }
}
```

### 3.3 Resource Management

Les ressources (textures, sons, fonts) sont gérées via un système centralisé avec chargement asynchrone et cache.

```
┌─────────────────────────────────────────────────────────────┐
│                     ASSET MANAGER                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │   Loader    │───▶│    Cache    │───▶│   Handle    │     │
│  │  (async)    │    │  (HashMap)  │    │  (Arc<T>)   │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
│        │                  │                  │              │
│        ▼                  ▼                  ▼              │
│   File System       Memory Pool        Game Systems        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Handle** : Référence légère vers une ressource. Permet de référencer une texture avant qu'elle soit chargée.

**Hot reloading** (dev mode) : Surveiller les fichiers et recharger automatiquement les assets modifiés.

---

## 4. Systèmes Détaillés

### 4.1 Système de Rendu 2D

Le pipeline de rendu pour un jeu top-down se décompose ainsi :

```
┌─────────────────────────────────────────────────────────────┐
│                    RENDER PIPELINE                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. COLLECT                                                 │
│     ├── Tilemap layers                                      │
│     ├── Sprite entities                                     │
│     └── UI elements                                         │
│                                                             │
│  2. SORT                                                    │
│     └── Par Z-order puis par Y (pour l'effet top-down)      │
│                                                             │
│  3. BATCH                                                   │
│     └── Grouper par texture atlas                           │
│                                                             │
│  4. SUBMIT                                                  │
│     └── Draw calls vers GPU                                 │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### 4.1.1 Sprite Batching

Le batching est crucial pour les performances. Au lieu d'un draw call par sprite, on regroupe tous les sprites utilisant la même texture.

```rust
pub struct SpriteBatch {
    vertices: Vec<SpriteVertex>,
    indices: Vec<u32>,
    texture: TextureId,
}

#[repr(C)]
pub struct SpriteVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    color: [f32; 4],
}
```

#### 4.1.2 Z-Ordering pour Top-Down

Dans un jeu top-down, les entités doivent être triées pour créer l'illusion de profondeur :

```
Z-Order layers:
├── 0: Ground tiles
├── 1: Ground decorations (flowers, paths)
├── 2: Shadows
├── 3: Entities (sorted by Y position)
├── 4: Above-head elements (speech bubbles)
└── 5: Weather effects, UI
```

La formule de tri pour les entités sur la même couche :

```rust
// Plus Y est grand (bas de l'écran), plus l'entité est rendue tard (devant)
entities.sort_by(|a, b| {
    let z_cmp = a.z_order.cmp(&b.z_order);
    if z_cmp != Ordering::Equal {
        z_cmp
    } else {
        a.position.y.partial_cmp(&b.position.y).unwrap()
    }
});
```

#### 4.1.3 Tilemap Rendering

```rust
pub struct Tilemap {
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
    pub layers: Vec<TileLayer>,
}

pub struct TileLayer {
    pub name: String,
    pub tiles: Vec<Option<TileId>>,  // None = transparent
    pub z_order: i32,
}

pub struct Tileset {
    pub texture: TextureId,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
}
```

**Optimisation** : Ne rendre que les tiles visibles dans la caméra (culling).

#### 4.1.4 Caméra

```rust
pub struct Camera2D {
    pub position: Vec2,      // Centre de la caméra dans le monde
    pub zoom: f32,           // 1.0 = normal, 2.0 = zoom x2
    pub rotation: f32,       // En radians (rarement utilisé en top-down)
    pub viewport: Rect,      // Taille de l'écran en pixels
}

impl Camera2D {
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let relative = world_pos - self.position;
        let scaled = relative * self.zoom;
        Vec2::new(
            scaled.x + self.viewport.width / 2.0,
            scaled.y + self.viewport.height / 2.0,
        )
    }
    
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let centered = Vec2::new(
            screen_pos.x - self.viewport.width / 2.0,
            screen_pos.y - self.viewport.height / 2.0,
        );
        centered / self.zoom + self.position
    }
    
    pub fn visible_bounds(&self) -> Rect {
        let half_size = Vec2::new(
            self.viewport.width / (2.0 * self.zoom),
            self.viewport.height / (2.0 * self.zoom),
        );
        Rect {
            x: self.position.x - half_size.x,
            y: self.position.y - half_size.y,
            width: half_size.x * 2.0,
            height: half_size.y * 2.0,
        }
    }
}
```

**Smooth follow** : La caméra suit le joueur avec un léger délai pour éviter les mouvements brusques.

```rust
fn camera_follow_system(camera: &mut Camera2D, target: Vec2, dt: f32) {
    let smoothing = 5.0;  // Plus élevé = plus réactif
    camera.position = camera.position.lerp(target, smoothing * dt);
}
```

### 4.2 Système de Collision 2D

Pour un RPG/farming, la collision AABB (Axis-Aligned Bounding Box) suffit largement.

#### 4.2.1 Structures de base

```rust
#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            min: Vec2::new(x, y),
            max: Vec2::new(x + width, y + height),
        }
    }
    
    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x < other.max.x &&
        self.max.x > other.min.x &&
        self.min.y < other.max.y &&
        self.max.y > other.min.y
    }
    
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.min.x &&
        point.x <= self.max.x &&
        point.y >= self.min.y &&
        point.y <= self.max.y
    }
}
```

#### 4.2.2 Spatial Partitioning

Pour éviter de tester chaque paire d'entités (O(n²)), on utilise une grille spatiale :

```rust
pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl SpatialGrid {
    pub fn insert(&mut self, entity: Entity, bounds: &AABB) {
        let min_cell = self.world_to_cell(bounds.min);
        let max_cell = self.world_to_cell(bounds.max);
        
        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                self.cells.entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(entity);
            }
        }
    }
    
    pub fn query(&self, bounds: &AABB) -> Vec<Entity> {
        let mut result = Vec::new();
        let min_cell = self.world_to_cell(bounds.min);
        let max_cell = self.world_to_cell(bounds.max);
        
        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                if let Some(entities) = self.cells.get(&(x, y)) {
                    result.extend(entities);
                }
            }
        }
        result.sort();
        result.dedup();
        result
    }
    
    fn world_to_cell(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }
}
```

#### 4.2.3 Collision Response

```rust
pub struct CollisionInfo {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub overlap: Vec2,
    pub normal: Vec2,
}

fn resolve_collision(pos: &mut Position, vel: &mut Velocity, collision: &CollisionInfo) {
    // Séparer les entités
    pos.x += collision.normal.x * collision.overlap.x.abs();
    pos.y += collision.normal.y * collision.overlap.y.abs();
    
    // Optionnel: annuler la vélocité dans la direction de la collision
    if collision.normal.x != 0.0 {
        vel.x = 0.0;
    }
    if collision.normal.y != 0.0 {
        vel.y = 0.0;
    }
}
```

### 4.3 Système d'Input

Abstraction des inputs pour supporter clavier, souris et manette de façon uniforme.

```rust
#[derive(Default)]
pub struct InputState {
    // Actions abstraites
    pub actions: HashMap<Action, ButtonState>,
    // Axes analogiques
    pub axes: HashMap<Axis, f32>,
    // Position souris
    pub mouse_position: Vec2,
    pub mouse_world_position: Vec2,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Interact,
    Attack,
    OpenInventory,
    Pause,
    // ...
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    MoveHorizontal,
    MoveVertical,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Released,
    JustPressed,
    Held,
    JustReleased,
}

impl InputState {
    pub fn is_action_pressed(&self, action: Action) -> bool {
        matches!(
            self.actions.get(&action),
            Some(ButtonState::JustPressed) | Some(ButtonState::Held)
        )
    }
    
    pub fn is_action_just_pressed(&self, action: Action) -> bool {
        matches!(self.actions.get(&action), Some(ButtonState::JustPressed))
    }
}
```

**Input mapping** : Fichier de configuration pour remapper les touches.

```toml
# input_mapping.toml
[keyboard]
MoveUp = ["W", "Up"]
MoveDown = ["S", "Down"]
MoveLeft = ["A", "Left"]
MoveRight = ["D", "Right"]
Interact = ["E", "Space"]

[gamepad]
MoveHorizontal = "LeftStickX"
MoveVertical = "LeftStickY"
Interact = "South"  # A button on Xbox, X on PlayStation
```

### 4.4 Système Audio

```rust
pub struct AudioManager {
    sound_effects: HashMap<String, SoundHandle>,
    music_player: MusicPlayer,
    master_volume: f32,
    sfx_volume: f32,
    music_volume: f32,
}

impl AudioManager {
    pub fn play_sfx(&self, name: &str) {
        if let Some(handle) = self.sound_effects.get(name) {
            handle.play(self.master_volume * self.sfx_volume);
        }
    }
    
    pub fn play_music(&mut self, name: &str, fade_in: f32) {
        self.music_player.crossfade_to(name, fade_in);
    }
    
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
        self.music_player.set_volume(self.master_volume * self.music_volume);
    }
}
```

**Spatial audio** (optionnel) : Atténuation basée sur la distance pour les sons d'ambiance.

### 4.5 Animation System

```rust
pub struct AnimationController {
    pub current: String,
    pub animations: HashMap<String, Animation>,
    pub frame_index: usize,
    pub timer: f32,
}

pub struct Animation {
    pub frames: Vec<AnimationFrame>,
    pub looping: bool,
}

pub struct AnimationFrame {
    pub sprite_region: Rect,
    pub duration: f32,  // En secondes
}

impl AnimationController {
    pub fn play(&mut self, name: &str) {
        if self.current != name {
            self.current = name.to_string();
            self.frame_index = 0;
            self.timer = 0.0;
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        let anim = match self.animations.get(&self.current) {
            Some(a) => a,
            None => return,
        };
        
        self.timer += dt;
        
        let frame = &anim.frames[self.frame_index];
        if self.timer >= frame.duration {
            self.timer -= frame.duration;
            self.frame_index += 1;
            
            if self.frame_index >= anim.frames.len() {
                if anim.looping {
                    self.frame_index = 0;
                } else {
                    self.frame_index = anim.frames.len() - 1;
                }
            }
        }
    }
    
    pub fn current_frame(&self) -> Option<&AnimationFrame> {
        self.animations
            .get(&self.current)
            .and_then(|a| a.frames.get(self.frame_index))
    }
}
```

---

## 5. Systèmes Spécifiques RPG/Farming

### 5.1 Système de Temps In-Game

```rust
pub struct GameTime {
    pub total_minutes: u32,  // Minutes depuis le début du jeu
    pub time_scale: f32,     // 1.0 = temps réel, 10.0 = 10x plus rapide
}

impl GameTime {
    pub fn day(&self) -> u32 {
        self.total_minutes / (24 * 60) + 1
    }
    
    pub fn hour(&self) -> u32 {
        (self.total_minutes / 60) % 24
    }
    
    pub fn minute(&self) -> u32 {
        self.total_minutes % 60
    }
    
    pub fn season(&self) -> Season {
        match ((self.day() - 1) / 28) % 4 {
            0 => Season::Spring,
            1 => Season::Summer,
            2 => Season::Fall,
            _ => Season::Winter,
        }
    }
    
    pub fn day_progress(&self) -> f32 {
        // 0.0 = minuit, 0.5 = midi, 1.0 = minuit suivant
        ((self.hour() * 60 + self.minute()) as f32) / (24.0 * 60.0)
    }
}
```

### 5.2 Système de Culture (Farming)

```rust
#[derive(Component)]
pub struct Crop {
    pub crop_type: CropType,
    pub growth_stage: u8,
    pub days_in_stage: u8,
    pub watered_today: bool,
    pub quality: Quality,
}

pub struct CropDefinition {
    pub name: String,
    pub growth_stages: Vec<GrowthStage>,
    pub seasons: Vec<Season>,
    pub harvest_item: ItemId,
    pub regrows: bool,
}

pub struct GrowthStage {
    pub days_required: u8,
    pub sprite_region: Rect,
}

fn crop_growth_system(world: &mut World, game_time: &GameTime) {
    // Appelé une fois par jour in-game
    for (entity, crop) in world.query::<&mut Crop>() {
        if !crop.watered_today {
            continue;  // Pas d'eau = pas de croissance
        }
        
        let definition = get_crop_definition(crop.crop_type);
        let current_stage = &definition.growth_stages[crop.growth_stage as usize];
        
        crop.days_in_stage += 1;
        
        if crop.days_in_stage >= current_stage.days_required {
            crop.growth_stage += 1;
            crop.days_in_stage = 0;
        }
        
        crop.watered_today = false;  // Reset pour le lendemain
    }
}
```

### 5.3 Système d'Inventaire

```rust
#[derive(Component)]
pub struct Inventory {
    pub slots: Vec<Option<ItemStack>>,
    pub capacity: usize,
    pub selected_slot: usize,
}

pub struct ItemStack {
    pub item_id: ItemId,
    pub quantity: u32,
    pub quality: Quality,
}

pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub icon: SpriteRegion,
    pub max_stack: u32,
    pub item_type: ItemType,
    pub sell_price: u32,
}

pub enum ItemType {
    Tool { tool_type: ToolType },
    Seed { crop_type: CropType },
    Crop { edible: bool, energy: i32 },
    Material,
    Furniture,
}

impl Inventory {
    pub fn add_item(&mut self, item: ItemStack) -> Option<ItemStack> {
        // D'abord, chercher un stack existant
        for slot in &mut self.slots {
            if let Some(existing) = slot {
                if existing.item_id == item.item_id && existing.quality == item.quality {
                    let max = get_item_definition(item.item_id).max_stack;
                    let space = max - existing.quantity;
                    if space >= item.quantity {
                        existing.quantity += item.quantity;
                        return None;  // Tout ajouté
                    }
                }
            }
        }
        
        // Sinon, chercher un slot vide
        for slot in &mut self.slots {
            if slot.is_none() {
                *slot = Some(item);
                return None;
            }
        }
        
        Some(item)  // Inventaire plein, retourner l'item
    }
}
```

### 5.4 Système de Dialogue

```rust
pub struct DialogueManager {
    pub current_dialogue: Option<DialogueTree>,
    pub current_node: NodeId,
    pub text_progress: f32,  // Pour l'effet "typewriter"
}

pub struct DialogueTree {
    pub nodes: HashMap<NodeId, DialogueNode>,
    pub start_node: NodeId,
}

pub enum DialogueNode {
    Text {
        speaker: String,
        portrait: Option<SpriteRegion>,
        text: String,
        next: NodeId,
    },
    Choice {
        prompt: String,
        choices: Vec<DialogueChoice>,
    },
    Condition {
        condition: DialogueCondition,
        if_true: NodeId,
        if_false: NodeId,
    },
    Action {
        action: DialogueAction,
        next: NodeId,
    },
    End,
}

pub struct DialogueChoice {
    pub text: String,
    pub next: NodeId,
    pub condition: Option<DialogueCondition>,
}

pub enum DialogueCondition {
    HasItem(ItemId, u32),
    FriendshipLevel(NpcId, u32),
    QuestComplete(QuestId),
    Flag(String),
}

pub enum DialogueAction {
    GiveItem(ItemId, u32),
    TakeItem(ItemId, u32),
    SetFlag(String),
    StartQuest(QuestId),
    AddFriendship(NpcId, i32),
}
```

---

## 6. Structure des Fichiers de Données

### 6.1 Format des Maps (JSON/TOML)

```json
{
  "name": "farm",
  "width": 64,
  "height": 64,
  "tile_size": 16,
  "tileset": "tilesets/farm.png",
  "layers": [
    {
      "name": "ground",
      "z_order": 0,
      "data": [1, 1, 1, 2, 2, ...]
    },
    {
      "name": "objects",
      "z_order": 2,
      "data": [0, 0, 45, 0, ...]
    }
  ],
  "collision_layer": [1, 1, 0, 0, ...],
  "spawn_points": {
    "player_start": { "x": 32, "y": 48 }
  },
  "triggers": [
    {
      "bounds": { "x": 0, "y": 30, "w": 2, "h": 4 },
      "action": { "type": "transition", "target": "town", "spawn": "from_farm" }
    }
  ]
}
```

### 6.2 Définition des Items

```toml
# items/crops.toml
[[items]]
id = "parsnip"
name = "Parsnip"
description = "A spring tuber closely related to the carrot."
icon = { x = 0, y = 0, w = 16, h = 16 }
max_stack = 999
sell_price = 35
type = { Crop = { edible = true, energy = 10 } }

[[items]]
id = "parsnip_seeds"
name = "Parsnip Seeds"
description = "Plant these in spring. Takes 4 days to mature."
icon = { x = 16, y = 0, w = 16, h = 16 }
max_stack = 999
sell_price = 10
type = { Seed = { crop_type = "parsnip" } }
```

### 6.3 Définition des Dialogues

```json
{
  "id": "mayor_intro",
  "nodes": {
    "start": {
      "type": "text",
      "speaker": "Mayor Lewis",
      "portrait": "portraits/lewis_happy",
      "text": "Welcome to Pelican Town! You must be the new farmer.",
      "next": "choice1"
    },
    "choice1": {
      "type": "choice",
      "prompt": "",
      "choices": [
        { "text": "Yes, I'm excited to start!", "next": "positive" },
        { "text": "This place looks... rustic.", "next": "negative" }
      ]
    },
    "positive": {
      "type": "action",
      "action": { "AddFriendship": ["lewis", 10] },
      "next": "end"
    },
    "negative": {
      "type": "text",
      "speaker": "Mayor Lewis",
      "text": "Ahem... well, it has character!",
      "next": "end"
    },
    "end": { "type": "end" }
  },
  "start_node": "start"
}
```

---

## 7. Plan de Développement

### Phase 1 : Fondations (4-6 semaines)

**Objectif** : Une fenêtre qui affiche un sprite qui bouge.

| Semaine | Tâches |
|---------|--------|
| 1 | Setup workspace Cargo, création fenêtre avec winit |
| 2 | Initialisation wgpu, premier triangle |
| 3 | Sprite rendering basique, chargement texture |
| 4 | Game loop avec fixed timestep |
| 5 | Input system (clavier) |
| 6 | Caméra 2D basique |

**Milestone** : Sprite controllable par clavier, caméra qui suit.

### Phase 2 : Monde (4-6 semaines)

**Objectif** : Naviguer dans un monde avec des tiles et des collisions.

| Semaine | Tâches |
|---------|--------|
| 7 | Tilemap loading et rendering |
| 8 | Multi-layer tilemaps, z-ordering |
| 9 | Collision AABB, spatial grid |
| 10 | Tile-based collision map |
| 11 | Transitions entre maps |
| 12 | Animation system |

**Milestone** : Personnage animé qui navigue dans plusieurs maps avec collisions.

### Phase 3 : ECS et Systems (3-4 semaines)

**Objectif** : Architecture propre et extensible.

| Semaine | Tâches |
|---------|--------|
| 13 | Implémentation ECS (ou intégration hecs/legion) |
| 14 | Refactoring vers ECS |
| 15 | Resource manager, asset loading |
| 16 | Audio system (rodio) |

**Milestone** : Code restructuré en ECS, sons et musique.

### Phase 4 : Gameplay Core (6-8 semaines)

**Objectif** : Les mécaniques de farming et RPG.

| Semaine | Tâches |
|---------|--------|
| 17-18 | Système de temps (jour/nuit, saisons) |
| 19-20 | Système d'inventaire et items |
| 21-22 | Système de farming (planter, arroser, récolter) |
| 23-24 | NPCs basiques et dialogues |

**Milestone** : Boucle de gameplay complète (planter → grandir → récolter → vendre).

### Phase 5 : Debug Tools et Polish (6-8 semaines)

**Objectif** : Outils de debug professionnels et qualité.

| Semaine | Tâches |
|---------|--------|
| 25-26 | Intégration egui, debug overlay base |
| 27 | Collision visualizer, spatial grid debug |
| 28 | Z-order debug, layer visualization |
| 29 | Entity Inspector (ECS debug) |
| 30 | Event Log system, profiler intégré |
| 31 | Debug Console avec commandes |
| 32 | UI in-game (menus, HUD) |

**Milestone** : Framework avec outils de debug complets et professionnels.

### Phase 6 : Finition (4+ semaines)

**Objectif** : Jeu jouable et complet.

| Semaine | Tâches |
|---------|--------|
| 33-34 | Save/Load system |
| 35+ | Polish, bug fixes, optimisations |
| 35+ | Documentation du framework |

**Milestone** : Jeu jouable avec sauvegarde.

---

## 8. Structure des Répertoires Finale

```
game-engine/
├── Cargo.toml
├── crates/
│   ├── engine_core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── game_loop.rs
│   │       ├── time.rs
│   │       └── app.rs
│   │
│   ├── engine_window/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── window.rs
│   │
│   ├── engine_render/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── renderer.rs
│   │       ├── sprite.rs
│   │       ├── sprite_batch.rs
│   │       ├── tilemap.rs
│   │       ├── camera.rs
│   │       ├── texture.rs
│   │       └── shaders/
│   │           ├── sprite.wgsl
│   │           └── tilemap.wgsl
│   │
│   ├── engine_input/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── input_state.rs
│   │       ├── keyboard.rs
│   │       ├── mouse.rs
│   │       └── gamepad.rs
│   │
│   ├── engine_audio/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── audio_manager.rs
│   │       └── music.rs
│   │
│   ├── engine_ecs/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── world.rs
│   │       ├── entity.rs
│   │       ├── component.rs
│   │       └── system.rs
│   │
│   ├── engine_physics/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── aabb.rs
│   │       ├── spatial_grid.rs
│   │       └── collision.rs
│   │
│   ├── engine_assets/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── asset_manager.rs
│   │       ├── handle.rs
│   │       └── loaders/
│   │           ├── texture_loader.rs
│   │           ├── audio_loader.rs
│   │           └── data_loader.rs
│   │
│   ├── engine_debug/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── debug_manager.rs
│   │       ├── collision_debug.rs
│   │       ├── zorder_debug.rs
│   │       ├── entity_inspector.rs
│   │       ├── event_log.rs
│   │       ├── profiler.rs
│   │       └── console.rs
│   │
│   └── engine_ui/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── widget.rs
│           ├── button.rs
│           ├── text.rs
│           └── inventory_ui.rs
│
├── game/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── states/
│       │   ├── mod.rs
│       │   ├── menu.rs
│       │   ├── playing.rs
│       │   └── pause.rs
│       ├── components/
│       │   ├── mod.rs
│       │   ├── player.rs
│       │   ├── npc.rs
│       │   ├── crop.rs
│       │   └── item.rs
│       ├── systems/
│       │   ├── mod.rs
│       │   ├── player_controller.rs
│       │   ├── farming.rs
│       │   ├── dialogue.rs
│       │   └── day_cycle.rs
│       └── data/
│           └── definitions.rs
│
└── assets/
    ├── textures/
    │   ├── characters/
    │   ├── tilesets/
    │   ├── items/
    │   └── ui/
    ├── audio/
    │   ├── music/
    │   └── sfx/
    ├── maps/
    ├── dialogues/
    └── data/
        ├── items.toml
        ├── crops.toml
        └── npcs.toml
```

---

## 9. Considérations Techniques

### 9.1 Cross-Compilation

```bash
# Depuis Linux vers Windows
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# Depuis Linux vers macOS (nécessite osxcross)
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
```

**CI/CD** : GitHub Actions avec des runners pour chaque plateforme.

### 9.2 Profiling

```rust
// Macro simple pour mesurer le temps
macro_rules! profile {
    ($name:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let elapsed = start.elapsed();
        tracing::debug!("{}: {:?}", $name, elapsed);
        result
    }};
}

// Usage
profile!("render", {
    renderer.render(&game_state);
});
```

**Outils** : Tracy, perf (Linux), Instruments (macOS).

### 9.3 Gestion Mémoire

Rust gère la mémoire automatiquement, mais quelques points d'attention :

- **Préallocation** : Éviter les allocations dans la boucle de jeu
- **Object pools** : Pour les entités fréquemment créées/détruites (projectiles, particles)
- **Arena allocators** : Pour les données temporaires par frame

### 9.4 Hot Reloading (Dev)

```rust
#[cfg(debug_assertions)]
fn watch_assets(asset_manager: &mut AssetManager) {
    use notify::{Watcher, RecursiveMode, watcher};
    
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher.watch("assets/", RecursiveMode::Recursive).unwrap();
    
    // Dans le game loop
    if let Ok(event) = rx.try_recv() {
        asset_manager.reload_changed(&event.path);
    }
}
```

---

## 10. Outils de Debug Intégrés

Le debug est le nerf de la guerre dans le développement de jeux. Sans outils visuels et performants, tu passes plus de temps à chercher des bugs qu'à créer du contenu. Cette section décrit un système de debug complet, activable/désactivable à la volée.

### 10.1 Architecture du Debug System

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         DEBUG OVERLAY SYSTEM                            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │  F1: Help   │  │ F2: Perf    │  │ F3: Physics │  │ F4: Render  │    │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘    │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │ F5: ECS     │  │ F6: Events  │  │ F7: Console │  │ F8: Pause   │    │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘    │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐ │
│  │                      IMGUI OVERLAY (egui)                         │ │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐   │ │
│  │  │ Entity Inspector│  │ Performance     │  │ Console/Log     │   │ │
│  │  │                 │  │ Graphs          │  │                 │   │ │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────┘   │ │
│  └───────────────────────────────────────────────────────────────────┘ │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

**Dépendance recommandée** : `egui` pour l'UI de debug (léger, GPU-accelerated, immediate mode).

```toml
[dependencies]
egui = "0.27"
egui-wgpu = "0.27"
egui-winit = "0.27"
```

### 10.2 Structure du Debug Manager

```rust
pub struct DebugManager {
    pub enabled: bool,
    pub panels: DebugPanels,
    pub overlays: DebugOverlays,
    pub console: DebugConsole,
    pub profiler: FrameProfiler,
    pub event_log: EventLog,
}

#[derive(Default)]
pub struct DebugPanels {
    pub show_help: bool,           // F1
    pub show_performance: bool,    // F2
    pub show_physics: bool,        // F3
    pub show_render: bool,         // F4
    pub show_ecs_inspector: bool,  // F5
    pub show_event_log: bool,      // F6
    pub show_console: bool,        // F7
    pub game_paused: bool,         // F8
}

#[derive(Default)]
pub struct DebugOverlays {
    pub collision_boxes: bool,
    pub collision_grid: bool,
    pub sprite_bounds: bool,
    pub z_order_labels: bool,
    pub tile_grid: bool,
    pub entity_ids: bool,
    pub velocity_vectors: bool,
    pub trigger_zones: bool,
    pub pathfinding_grid: bool,
    pub camera_bounds: bool,
}

impl DebugManager {
    pub fn handle_input(&mut self, input: &InputState) {
        // Toggle master debug mode
        if input.is_key_just_pressed(KeyCode::F12) {
            self.enabled = !self.enabled;
        }
        
        if !self.enabled {
            return;
        }
        
        // Panel toggles
        if input.is_key_just_pressed(KeyCode::F1) {
            self.panels.show_help = !self.panels.show_help;
        }
        if input.is_key_just_pressed(KeyCode::F2) {
            self.panels.show_performance = !self.panels.show_performance;
        }
        // ... etc
        
        // Overlay toggles (with Ctrl modifier)
        if input.is_key_held(KeyCode::LControl) {
            if input.is_key_just_pressed(KeyCode::C) {
                self.overlays.collision_boxes = !self.overlays.collision_boxes;
            }
            if input.is_key_just_pressed(KeyCode::Z) {
                self.overlays.z_order_labels = !self.overlays.z_order_labels;
            }
            // ... etc
        }
    }
}
```

### 10.3 Visualisation des Collisions

Le debug de collision est critique pour un RPG top-down. Plusieurs niveaux de visualisation :

```rust
pub struct CollisionDebugRenderer {
    // Couleurs configurables
    pub color_static: Color,      // Murs, obstacles (rouge)
    pub color_dynamic: Color,     // Entités mobiles (vert)
    pub color_trigger: Color,     // Zones de trigger (jaune transparent)
    pub color_sensor: Color,      // Sensors sans collision (bleu)
    pub color_overlap: Color,     // Collisions actives (magenta)
}

impl CollisionDebugRenderer {
    pub fn render(&self, ctx: &mut RenderContext, world: &World, camera: &Camera2D) {
        if !ctx.debug.overlays.collision_boxes {
            return;
        }
        
        // 1. Render collision grid (spatial partitioning)
        if ctx.debug.overlays.collision_grid {
            self.render_spatial_grid(ctx, world, camera);
        }
        
        // 2. Render all AABB boxes
        for (entity, (collider, transform)) in world.query::<(&Collider, &Transform)>() {
            let bounds = collider.world_bounds(transform);
            let color = match collider.layer {
                CollisionLayer::Static => self.color_static,
                CollisionLayer::Dynamic => self.color_dynamic,
                CollisionLayer::Trigger => self.color_trigger,
                CollisionLayer::Sensor => self.color_sensor,
            };
            
            // Highlight si en collision
            let final_color = if collider.is_colliding {
                self.color_overlap
            } else {
                color
            };
            
            ctx.draw_rect_outline(bounds, final_color, 2.0);
            
            // Afficher l'ID de l'entité
            if ctx.debug.overlays.entity_ids {
                ctx.draw_text(
                    &format!("{:?}", entity),
                    bounds.center(),
                    12.0,
                    Color::WHITE,
                );
            }
        }
        
        // 3. Render tile collision map
        self.render_tile_collisions(ctx, world, camera);
    }
    
    fn render_spatial_grid(&self, ctx: &mut RenderContext, world: &World, camera: &Camera2D) {
        let grid = world.resource::<SpatialGrid>();
        let visible = camera.visible_bounds();
        
        let cell_size = grid.cell_size;
        let start_x = (visible.min.x / cell_size).floor() as i32;
        let end_x = (visible.max.x / cell_size).ceil() as i32;
        let start_y = (visible.min.y / cell_size).floor() as i32;
        let end_y = (visible.max.y / cell_size).ceil() as i32;
        
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                let cell_bounds = Rect::new(
                    x as f32 * cell_size,
                    y as f32 * cell_size,
                    cell_size,
                    cell_size,
                );
                
                // Couleur basée sur le nombre d'entités dans la cellule
                let count = grid.count_in_cell(x, y);
                let alpha = (count as f32 * 0.1).min(0.5);
                let color = Color::new(0.0, 1.0, 0.0, alpha);
                
                ctx.draw_rect_filled(cell_bounds, color);
                ctx.draw_rect_outline(cell_bounds, Color::new(0.0, 0.5, 0.0, 0.3), 1.0);
                
                // Afficher le compte
                if count > 0 {
                    ctx.draw_text(
                        &count.to_string(),
                        cell_bounds.center(),
                        10.0,
                        Color::WHITE,
                    );
                }
            }
        }
    }
    
    fn render_tile_collisions(&self, ctx: &mut RenderContext, world: &World, camera: &Camera2D) {
        let tilemap = world.resource::<Tilemap>();
        let visible = camera.visible_bounds();
        
        let start_x = (visible.min.x / tilemap.tile_size as f32).floor() as i32;
        let end_x = (visible.max.x / tilemap.tile_size as f32).ceil() as i32;
        let start_y = (visible.min.y / tilemap.tile_size as f32).floor() as i32;
        let end_y = (visible.max.y / tilemap.tile_size as f32).ceil() as i32;
        
        for x in start_x.max(0)..end_x.min(tilemap.width as i32) {
            for y in start_y.max(0)..end_y.min(tilemap.height as i32) {
                if tilemap.is_solid(x as u32, y as u32) {
                    let tile_bounds = Rect::new(
                        x as f32 * tilemap.tile_size as f32,
                        y as f32 * tilemap.tile_size as f32,
                        tilemap.tile_size as f32,
                        tilemap.tile_size as f32,
                    );
                    ctx.draw_rect_filled(tile_bounds, Color::new(1.0, 0.0, 0.0, 0.3));
                }
            }
        }
    }
}
```

### 10.4 Visualisation du Z-Order et Layers

Pour un jeu top-down, le z-ordering est crucial. Visualiser l'ordre de rendu aide à débugger les problèmes de superposition.

```rust
pub struct ZOrderDebugRenderer {
    pub show_labels: bool,
    pub show_layers: bool,
    pub highlight_layer: Option<i32>,
}

impl ZOrderDebugRenderer {
    pub fn render(&self, ctx: &mut RenderContext, world: &World, camera: &Camera2D) {
        if !ctx.debug.overlays.z_order_labels {
            return;
        }
        
        // Collecter et trier toutes les entités rendables
        let mut renderables: Vec<(Entity, i32, f32, Vec2)> = Vec::new();
        
        for (entity, (transform, sprite)) in world.query::<(&Transform, &Sprite)>() {
            let z_order = sprite.z_order;
            let y_sort = transform.position.y + sprite.y_sort_offset;
            renderables.push((entity, z_order, y_sort, transform.position));
        }
        
        // Trier comme le renderer le ferait
        renderables.sort_by(|a, b| {
            a.1.cmp(&b.1).then_with(|| a.2.partial_cmp(&b.2).unwrap())
        });
        
        // Afficher l'ordre de rendu
        for (index, (entity, z_order, y_sort, pos)) in renderables.iter().enumerate() {
            // Couleur par layer
            let layer_color = self.layer_to_color(*z_order);
            
            // Skip si on highlight un layer spécifique
            if let Some(highlight) = self.highlight_layer {
                if *z_order != highlight {
                    continue;
                }
            }
            
            // Label avec ordre de rendu, z-order et y-sort
            let label = format!(
                "#{} z:{} y:{:.0}",
                index, z_order, y_sort
            );
            
            // Background pour lisibilité
            let text_pos = camera.world_to_screen(*pos);
            ctx.draw_rect_filled(
                Rect::new(text_pos.x - 2.0, text_pos.y - 12.0, 80.0, 14.0),
                Color::new(0.0, 0.0, 0.0, 0.7),
            );
            ctx.draw_text(&label, text_pos, 10.0, layer_color);
            
            // Ligne vers le sprite center
            ctx.draw_line(
                text_pos,
                camera.world_to_screen(*pos + Vec2::new(0.0, 8.0)),
                layer_color,
                1.0,
            );
        }
        
        // Légende des layers
        if self.show_layers {
            self.render_layer_legend(ctx);
        }
    }
    
    fn layer_to_color(&self, z_order: i32) -> Color {
        match z_order {
            0 => Color::new(0.5, 0.5, 0.5, 1.0),   // Ground
            1 => Color::new(0.7, 0.5, 0.3, 1.0),   // Ground decor
            2 => Color::new(0.3, 0.3, 0.3, 1.0),   // Shadows
            3 => Color::new(0.0, 1.0, 0.0, 1.0),   // Entities (main)
            4 => Color::new(1.0, 1.0, 0.0, 1.0),   // Above entities
            5 => Color::new(0.0, 0.7, 1.0, 1.0),   // Weather/UI
            _ => Color::new(1.0, 0.0, 1.0, 1.0),   // Unknown
        }
    }
    
    fn render_layer_legend(&self, ctx: &mut RenderContext) {
        let layers = [
            (0, "Ground"),
            (1, "Ground Decor"),
            (2, "Shadows"),
            (3, "Entities"),
            (4, "Above Entities"),
            (5, "Weather/UI"),
        ];
        
        let start_y = 100.0;
        for (i, (z, name)) in layers.iter().enumerate() {
            let y = start_y + i as f32 * 16.0;
            let color = self.layer_to_color(*z);
            ctx.draw_rect_filled(Rect::new(10.0, y, 12.0, 12.0), color);
            ctx.draw_text(
                &format!("Z{}: {}", z, name),
                Vec2::new(26.0, y + 10.0),
                12.0,
                Color::WHITE,
            );
        }
    }
}
```

### 10.5 Entity Inspector (ECS Debug)

Un inspecteur d'entités permet de voir et modifier l'état de n'importe quelle entité en temps réel.

```rust
pub struct EntityInspector {
    pub selected_entity: Option<Entity>,
    pub filter: String,
    pub show_all_components: bool,
}

impl EntityInspector {
    pub fn render_ui(&mut self, ui: &mut egui::Ui, world: &mut World) {
        // Entity list panel
        egui::SidePanel::left("entity_list").show_inside(ui, |ui| {
            ui.heading("Entities");
            
            // Filter
            ui.horizontal(|ui| {
                ui.label("Filter:");
                ui.text_edit_singleline(&mut self.filter);
            });
            
            // Entity count
            let total = world.entity_count();
            ui.label(format!("Total: {}", total));
            
            ui.separator();
            
            // Scrollable entity list
            egui::ScrollArea::vertical().show(ui, |ui| {
                for entity in world.entities() {
                    let label = self.entity_label(world, entity);
                    
                    // Filter
                    if !self.filter.is_empty() && !label.to_lowercase().contains(&self.filter.to_lowercase()) {
                        continue;
                    }
                    
                    let selected = self.selected_entity == Some(entity);
                    if ui.selectable_label(selected, &label).clicked() {
                        self.selected_entity = Some(entity);
                    }
                }
            });
        });
        
        // Component inspector panel
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if let Some(entity) = self.selected_entity {
                self.render_component_inspector(ui, world, entity);
            } else {
                ui.label("Select an entity to inspect");
            }
        });
    }
    
    fn entity_label(&self, world: &World, entity: Entity) -> String {
        // Try to get a name component, fallback to entity ID
        if let Some(name) = world.get::<Name>(entity) {
            format!("{} ({:?})", name.0, entity)
        } else if let Some(_) = world.get::<Player>(entity) {
            format!("Player ({:?})", entity)
        } else if let Some(npc) = world.get::<Npc>(entity) {
            format!("NPC: {} ({:?})", npc.name, entity)
        } else if let Some(_) = world.get::<Crop>(entity) {
            format!("Crop ({:?})", entity)
        } else {
            format!("{:?}", entity)
        }
    }
    
    fn render_component_inspector(&mut self, ui: &mut egui::Ui, world: &mut World, entity: Entity) {
        ui.heading(format!("Entity {:?}", entity));
        ui.separator();
        
        // Transform component
        if let Some(mut transform) = world.get_mut::<Transform>(entity) {
            egui::CollapsingHeader::new("Transform")
                .default_open(true)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Position:");
                        ui.add(egui::DragValue::new(&mut transform.position.x).prefix("x: "));
                        ui.add(egui::DragValue::new(&mut transform.position.y).prefix("y: "));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Rotation:");
                        ui.add(egui::DragValue::new(&mut transform.rotation).speed(0.01));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Scale:");
                        ui.add(egui::DragValue::new(&mut transform.scale.x).prefix("x: "));
                        ui.add(egui::DragValue::new(&mut transform.scale.y).prefix("y: "));
                    });
                });
        }
        
        // Velocity component
        if let Some(mut velocity) = world.get_mut::<Velocity>(entity) {
            egui::CollapsingHeader::new("Velocity").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Velocity:");
                    ui.add(egui::DragValue::new(&mut velocity.x).prefix("x: "));
                    ui.add(egui::DragValue::new(&mut velocity.y).prefix("y: "));
                });
                ui.label(format!("Speed: {:.2}", velocity.length()));
            });
        }
        
        // Sprite component
        if let Some(mut sprite) = world.get_mut::<Sprite>(entity) {
            egui::CollapsingHeader::new("Sprite").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Z-Order:");
                    ui.add(egui::DragValue::new(&mut sprite.z_order));
                });
                ui.horizontal(|ui| {
                    ui.label("Y-Sort Offset:");
                    ui.add(egui::DragValue::new(&mut sprite.y_sort_offset));
                });
                ui.checkbox(&mut sprite.visible, "Visible");
                ui.checkbox(&mut sprite.flip_x, "Flip X");
                ui.checkbox(&mut sprite.flip_y, "Flip Y");
            });
        }
        
        // Health component
        if let Some(mut health) = world.get_mut::<Health>(entity) {
            egui::CollapsingHeader::new("Health").show(ui, |ui| {
                let progress = health.current as f32 / health.max as f32;
                ui.add(egui::ProgressBar::new(progress).text(format!("{}/{}", health.current, health.max)));
                ui.horizontal(|ui| {
                    if ui.button("-10").clicked() {
                        health.current = health.current.saturating_sub(10);
                    }
                    if ui.button("+10").clicked() {
                        health.current = (health.current + 10).min(health.max);
                    }
                    if ui.button("Full").clicked() {
                        health.current = health.max;
                    }
                });
            });
        }
        
        // Inventory component
        if let Some(inventory) = world.get::<Inventory>(entity) {
            egui::CollapsingHeader::new("Inventory").show(ui, |ui| {
                ui.label(format!("Slots: {}/{}", inventory.used_slots(), inventory.capacity));
                for (i, slot) in inventory.slots.iter().enumerate() {
                    if let Some(stack) = slot {
                        let item_def = get_item_definition(stack.item_id);
                        ui.label(format!("[{}] {} x{}", i, item_def.name, stack.quantity));
                    }
                }
            });
        }
        
        // Crop component
        if let Some(crop) = world.get::<Crop>(entity) {
            egui::CollapsingHeader::new("Crop").show(ui, |ui| {
                ui.label(format!("Type: {:?}", crop.crop_type));
                ui.label(format!("Stage: {}", crop.growth_stage));
                ui.label(format!("Days in stage: {}", crop.days_in_stage));
                ui.label(format!("Watered: {}", crop.watered_today));
                ui.label(format!("Quality: {:?}", crop.quality));
            });
        }
        
        // State Machine component (for NPCs, etc.)
        if let Some(state_machine) = world.get::<StateMachine>(entity) {
            egui::CollapsingHeader::new("State Machine").show(ui, |ui| {
                ui.label(format!("Current: {:?}", state_machine.current_state));
                ui.label(format!("Previous: {:?}", state_machine.previous_state));
                ui.label(format!("Time in state: {:.2}s", state_machine.time_in_state));
            });
        }
    }
}
```

### 10.6 Live Resource Editor

Au-delà de l'inspection d'entités, pouvoir modifier à la volée les ressources globales et les valeurs de n'importe quel component est crucial pour itérer rapidement.

#### 10.6.1 Architecture du Live Editor

```rust
pub struct LiveEditor {
    // Panels ouverts
    pub open_panels: Vec<EditorPanel>,
    // Historique pour undo/redo
    pub history: EditHistory,
    // Watch list (valeurs à surveiller)
    pub watch_list: Vec<WatchEntry>,
    // Presets sauvegardés
    pub presets: HashMap<String, PresetData>,
}

pub enum EditorPanel {
    ResourceEditor(String),      // Nom de la ressource
    ComponentEditor(Entity),     // Entité spécifique
    GlobalSettings,
    WatchList,
    Presets,
}

pub struct WatchEntry {
    pub label: String,
    pub path: ValuePath,
    pub display_type: DisplayType,
}

#[derive(Clone)]
pub enum ValuePath {
    // Ressource globale
    Resource { type_name: &'static str, field: String },
    // Component d'une entité
    Component { entity: Entity, type_name: &'static str, field: String },
    // Expression custom
    Expression(String),
}

pub enum DisplayType {
    Text,
    Slider { min: f32, max: f32 },
    Color,
    Checkbox,
    Dropdown(Vec<String>),
    Graph { history_size: usize },
}
```

#### 10.6.2 Resource Editor Panel

```rust
pub struct ResourceEditor {
    pub selected_resource: Option<String>,
}

impl ResourceEditor {
    pub fn render_ui(&mut self, ui: &mut egui::Ui, world: &mut World) {
        // Liste des ressources disponibles
        egui::SidePanel::left("resource_list").show_inside(ui, |ui| {
            ui.heading("Resources");
            
            if ui.selectable_label(self.selected_resource.as_deref() == Some("GameTime"), "⏰ GameTime").clicked() {
                self.selected_resource = Some("GameTime".to_string());
            }
            if ui.selectable_label(self.selected_resource.as_deref() == Some("Camera"), "📷 Camera").clicked() {
                self.selected_resource = Some("Camera".to_string());
            }
            if ui.selectable_label(self.selected_resource.as_deref() == Some("RenderSettings"), "🎨 RenderSettings").clicked() {
                self.selected_resource = Some("RenderSettings".to_string());
            }
            if ui.selectable_label(self.selected_resource.as_deref() == Some("PhysicsSettings"), "⚡ PhysicsSettings").clicked() {
                self.selected_resource = Some("PhysicsSettings".to_string());
            }
            if ui.selectable_label(self.selected_resource.as_deref() == Some("AudioSettings"), "🔊 AudioSettings").clicked() {
                self.selected_resource = Some("AudioSettings".to_string());
            }
            if ui.selectable_label(self.selected_resource.as_deref() == Some("DebugSettings"), "🐛 DebugSettings").clicked() {
                self.selected_resource = Some("DebugSettings".to_string());
            }
        });
        
        // Éditeur de la ressource sélectionnée
        egui::CentralPanel::default().show_inside(ui, |ui| {
            match self.selected_resource.as_deref() {
                Some("GameTime") => self.edit_game_time(ui, world),
                Some("Camera") => self.edit_camera(ui, world),
                Some("RenderSettings") => self.edit_render_settings(ui, world),
                Some("PhysicsSettings") => self.edit_physics_settings(ui, world),
                Some("AudioSettings") => self.edit_audio_settings(ui, world),
                Some("DebugSettings") => self.edit_debug_settings(ui, world),
                _ => { ui.label("Select a resource to edit"); }
            }
        });
    }
    
    fn edit_game_time(&self, ui: &mut egui::Ui, world: &mut World) {
        let game_time = world.resource_mut::<GameTime>();
        
        ui.heading("⏰ Game Time");
        ui.separator();
        
        // Affichage actuel
        ui.horizontal(|ui| {
            ui.label("Current:");
            ui.strong(format!(
                "Day {} - {:02}:{:02} ({})",
                game_time.day(),
                game_time.hour(),
                game_time.minute(),
                game_time.season()
            ));
        });
        
        ui.separator();
        
        // Contrôles d'édition
        ui.horizontal(|ui| {
            ui.label("Hour:");
            let mut hour = game_time.hour() as i32;
            if ui.add(egui::Slider::new(&mut hour, 0..=23)).changed() {
                let day = game_time.day();
                let minute = game_time.minute();
                game_time.total_minutes = (day - 1) * 24 * 60 + hour as u32 * 60 + minute;
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Minute:");
            let mut minute = game_time.minute() as i32;
            if ui.add(egui::Slider::new(&mut minute, 0..=59)).changed() {
                let day = game_time.day();
                let hour = game_time.hour();
                game_time.total_minutes = (day - 1) * 24 * 60 + hour * 60 + minute as u32;
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Day:");
            let mut day = game_time.day() as i32;
            if ui.add(egui::DragValue::new(&mut day).clamp_range(1..=9999)).changed() {
                let hour = game_time.hour();
                let minute = game_time.minute();
                game_time.total_minutes = (day as u32 - 1) * 24 * 60 + hour * 60 + minute;
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Time Scale:");
            ui.add(egui::Slider::new(&mut game_time.time_scale, 0.0..=100.0).logarithmic(true));
            if ui.button("1x").clicked() { game_time.time_scale = 1.0; }
            if ui.button("10x").clicked() { game_time.time_scale = 10.0; }
            if ui.button("60x").clicked() { game_time.time_scale = 60.0; }
        });
        
        // Raccourcis rapides
        ui.separator();
        ui.label("Quick Set:");
        ui.horizontal(|ui| {
            if ui.button("🌅 6:00 (Dawn)").clicked() {
                self.set_time(game_time, 6, 0);
            }
            if ui.button("☀️ 12:00 (Noon)").clicked() {
                self.set_time(game_time, 12, 0);
            }
            if ui.button("🌆 18:00 (Dusk)").clicked() {
                self.set_time(game_time, 18, 0);
            }
            if ui.button("🌙 0:00 (Midnight)").clicked() {
                self.set_time(game_time, 0, 0);
            }
        });
        
        ui.horizontal(|ui| {
            if ui.button("🌸 Spring Day 1").clicked() {
                game_time.total_minutes = 0 * 28 * 24 * 60 + 6 * 60;
            }
            if ui.button("☀️ Summer Day 1").clicked() {
                game_time.total_minutes = 1 * 28 * 24 * 60 + 6 * 60;
            }
            if ui.button("🍂 Fall Day 1").clicked() {
                game_time.total_minutes = 2 * 28 * 24 * 60 + 6 * 60;
            }
            if ui.button("❄️ Winter Day 1").clicked() {
                game_time.total_minutes = 3 * 28 * 24 * 60 + 6 * 60;
            }
        });
    }
    
    fn edit_camera(&self, ui: &mut egui::Ui, world: &mut World) {
        let camera = world.resource_mut::<Camera2D>();
        
        ui.heading("📷 Camera");
        ui.separator();
        
        // Position
        ui.collapsing("Position", |ui| {
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.add(egui::DragValue::new(&mut camera.position.x).speed(1.0));
                ui.label("Y:");
                ui.add(egui::DragValue::new(&mut camera.position.y).speed(1.0));
            });
            
            if ui.button("Center on Player").clicked() {
                if let Some((_, transform)) = world.query::<(&Player, &Transform)>().iter().next() {
                    camera.position = transform.position;
                }
            }
            
            if ui.button("Reset to Origin").clicked() {
                camera.position = Vec2::ZERO;
            }
        });
        
        // Zoom
        ui.collapsing("Zoom", |ui| {
            ui.add(egui::Slider::new(&mut camera.zoom, 0.25..=4.0).text("Zoom"));
            ui.horizontal(|ui| {
                if ui.button("0.5x").clicked() { camera.zoom = 0.5; }
                if ui.button("1x").clicked() { camera.zoom = 1.0; }
                if ui.button("2x").clicked() { camera.zoom = 2.0; }
                if ui.button("4x").clicked() { camera.zoom = 4.0; }
            });
        });
        
        // Rotation
        ui.collapsing("Rotation", |ui| {
            let mut degrees = camera.rotation.to_degrees();
            if ui.add(egui::Slider::new(&mut degrees, -180.0..=180.0).text("Degrees")).changed() {
                camera.rotation = degrees.to_radians();
            }
            if ui.button("Reset").clicked() {
                camera.rotation = 0.0;
            }
        });
        
        // Bounds info
        ui.separator();
        ui.label("Visible Bounds:");
        let bounds = camera.visible_bounds();
        ui.label(format!(
            "  ({:.0}, {:.0}) to ({:.0}, {:.0})",
            bounds.min.x, bounds.min.y, bounds.max.x, bounds.max.y
        ));
        ui.label(format!(
            "  Size: {:.0} x {:.0}",
            bounds.width(), bounds.height()
        ));
    }
    
    fn edit_render_settings(&self, ui: &mut egui::Ui, world: &mut World) {
        let settings = world.resource_mut::<RenderSettings>();
        
        ui.heading("🎨 Render Settings");
        ui.separator();
        
        // Ambient light / Day-night
        ui.collapsing("Lighting", |ui| {
            ui.horizontal(|ui| {
                ui.label("Ambient Color:");
                let mut color = [settings.ambient_color.r, settings.ambient_color.g, settings.ambient_color.b];
                if ui.color_edit_button_rgb(&mut color).changed() {
                    settings.ambient_color = Color::rgb(color[0], color[1], color[2]);
                }
            });
            
            ui.add(egui::Slider::new(&mut settings.ambient_intensity, 0.0..=2.0).text("Intensity"));
            
            ui.checkbox(&mut settings.enable_day_night_cycle, "Auto Day/Night Cycle");
        });
        
        // Z-order settings
        ui.collapsing("Z-Order & Sorting", |ui| {
            ui.checkbox(&mut settings.y_sort_enabled, "Y-Sort within layers");
            ui.add(egui::Slider::new(&mut settings.y_sort_offset, -50.0..=50.0).text("Y-Sort Offset"));
        });
        
        // Visual effects
        ui.collapsing("Effects", |ui| {
            ui.checkbox(&mut settings.enable_shadows, "Shadows");
            ui.checkbox(&mut settings.enable_particles, "Particles");
            ui.checkbox(&mut settings.enable_weather, "Weather Effects");
            
            if settings.enable_shadows {
                ui.indent("shadow_settings", |ui| {
                    ui.add(egui::Slider::new(&mut settings.shadow_opacity, 0.0..=1.0).text("Shadow Opacity"));
                    ui.horizontal(|ui| {
                        ui.label("Shadow Offset:");
                        ui.add(egui::DragValue::new(&mut settings.shadow_offset.x).prefix("X: "));
                        ui.add(egui::DragValue::new(&mut settings.shadow_offset.y).prefix("Y: "));
                    });
                });
            }
        });
        
        // Debug visualization options
        ui.collapsing("Debug Visualization", |ui| {
            ui.checkbox(&mut settings.show_sprite_bounds, "Sprite Bounds");
            ui.checkbox(&mut settings.show_origin_points, "Origin Points");
            ui.checkbox(&mut settings.wireframe_mode, "Wireframe Mode");
        });
    }
    
    fn edit_physics_settings(&self, ui: &mut egui::Ui, world: &mut World) {
        let settings = world.resource_mut::<PhysicsSettings>();
        
        ui.heading("⚡ Physics Settings");
        ui.separator();
        
        // Spatial grid
        ui.collapsing("Spatial Grid", |ui| {
            ui.add(egui::Slider::new(&mut settings.grid_cell_size, 16.0..=256.0).text("Cell Size"));
            ui.label(format!("Current cells: {}", settings.active_cells_count));
        });
        
        // Collision
        ui.collapsing("Collision", |ui| {
            ui.checkbox(&mut settings.collision_enabled, "Enable Collisions");
            ui.add(egui::Slider::new(&mut settings.collision_iterations, 1..=10).text("Iterations"));
            
            // Layer matrix (simplified)
            ui.label("Collision Layers:");
            ui.horizontal(|ui| {
                ui.checkbox(&mut settings.layer_matrix[0][1], "Player ↔ NPC");
                ui.checkbox(&mut settings.layer_matrix[0][2], "Player ↔ Static");
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut settings.layer_matrix[1][2], "NPC ↔ Static");
                ui.checkbox(&mut settings.layer_matrix[0][3], "Player ↔ Trigger");
            });
        });
        
        // Movement
        ui.collapsing("Movement", |ui| {
            ui.add(egui::Slider::new(&mut settings.default_friction, 0.0..=1.0).text("Default Friction"));
            ui.add(egui::Slider::new(&mut settings.max_velocity, 100.0..=1000.0).text("Max Velocity"));
        });
    }
    
    fn edit_audio_settings(&self, ui: &mut egui::Ui, world: &mut World) {
        let audio = world.resource_mut::<AudioManager>();
        
        ui.heading("🔊 Audio Settings");
        ui.separator();
        
        // Master volume
        ui.add(egui::Slider::new(&mut audio.master_volume, 0.0..=1.0).text("Master Volume"));
        
        ui.separator();
        
        // Individual channels
        ui.add(egui::Slider::new(&mut audio.music_volume, 0.0..=1.0).text("Music"));
        ui.add(egui::Slider::new(&mut audio.sfx_volume, 0.0..=1.0).text("SFX"));
        ui.add(egui::Slider::new(&mut audio.ambient_volume, 0.0..=1.0).text("Ambient"));
        ui.add(egui::Slider::new(&mut audio.ui_volume, 0.0..=1.0).text("UI"));
        
        ui.separator();
        
        // Current playing
        ui.label("Now Playing:");
        if let Some(track) = &audio.current_music {
            ui.label(format!("  🎵 {}", track));
        } else {
            ui.label("  (none)");
        }
        
        // Quick controls
        ui.horizontal(|ui| {
            if ui.button("⏸ Pause All").clicked() {
                audio.pause_all();
            }
            if ui.button("▶ Resume All").clicked() {
                audio.resume_all();
            }
            if ui.button("⏹ Stop All").clicked() {
                audio.stop_all();
            }
        });
    }
    
    fn edit_debug_settings(&self, ui: &mut egui::Ui, world: &mut World) {
        let debug = world.resource_mut::<DebugSettings>();
        
        ui.heading("🐛 Debug Settings");
        ui.separator();
        
        ui.collapsing("Overlays", |ui| {
            ui.checkbox(&mut debug.show_fps, "Show FPS");
            ui.checkbox(&mut debug.show_entity_count, "Show Entity Count");
            ui.checkbox(&mut debug.show_position, "Show Player Position");
            ui.checkbox(&mut debug.show_current_tile, "Show Current Tile");
            ui.checkbox(&mut debug.show_game_time, "Show Game Time");
        });
        
        ui.collapsing("Logging", |ui| {
            ui.checkbox(&mut debug.log_collisions, "Log Collisions");
            ui.checkbox(&mut debug.log_state_changes, "Log State Changes");
            ui.checkbox(&mut debug.log_events, "Log Events");
            ui.add(egui::Slider::new(&mut debug.log_level, 0..=4).text("Log Level"));
        });
        
        ui.collapsing("Cheats", |ui| {
            ui.checkbox(&mut debug.god_mode, "God Mode");
            ui.checkbox(&mut debug.infinite_stamina, "Infinite Stamina");
            ui.checkbox(&mut debug.instant_grow, "Instant Crop Growth");
            ui.checkbox(&mut debug.no_clip, "No Clip");
            ui.checkbox(&mut debug.unlock_all, "Unlock All Areas");
        });
    }
    
    fn set_time(&self, game_time: &mut GameTime, hour: u32, minute: u32) {
        let day = game_time.day();
        game_time.total_minutes = (day - 1) * 24 * 60 + hour * 60 + minute;
    }
}
```

#### 10.6.3 Quick Edit Widget (Inline)

Pour modifier rapidement une valeur sans ouvrir un panel complet :

```rust
pub struct QuickEditWidget {
    active_edit: Option<ActiveEdit>,
}

struct ActiveEdit {
    entity: Option<Entity>,
    component_type: &'static str,
    field_name: String,
    original_value: EditValue,
}

#[derive(Clone)]
pub enum EditValue {
    Float(f32),
    Int(i32),
    Bool(bool),
    Vec2(Vec2),
    Color(Color),
    String(String),
    Enum { current: usize, options: Vec<String> },
}

impl QuickEditWidget {
    /// Affiche un widget inline éditable
    /// Clic gauche = éditer, clic droit = reset
    pub fn editable_value(
        &mut self,
        ui: &mut egui::Ui,
        label: &str,
        value: &mut EditValue,
    ) -> bool {
        let mut changed = false;
        
        ui.horizontal(|ui| {
            ui.label(format!("{}:", label));
            
            match value {
                EditValue::Float(v) => {
                    changed = ui.add(egui::DragValue::new(v).speed(0.1)).changed();
                }
                EditValue::Int(v) => {
                    changed = ui.add(egui::DragValue::new(v)).changed();
                }
                EditValue::Bool(v) => {
                    changed = ui.checkbox(v, "").changed();
                }
                EditValue::Vec2(v) => {
                    ui.horizontal(|ui| {
                        changed |= ui.add(egui::DragValue::new(&mut v.x).prefix("x:").speed(0.5)).changed();
                        changed |= ui.add(egui::DragValue::new(&mut v.y).prefix("y:").speed(0.5)).changed();
                    });
                }
                EditValue::Color(c) => {
                    let mut rgb = [c.r, c.g, c.b];
                    if ui.color_edit_button_rgb(&mut rgb).changed() {
                        *c = Color::rgb(rgb[0], rgb[1], rgb[2]);
                        changed = true;
                    }
                }
                EditValue::String(s) => {
                    changed = ui.text_edit_singleline(s).changed();
                }
                EditValue::Enum { current, options } => {
                    egui::ComboBox::from_id_source(label)
                        .selected_text(&options[*current])
                        .show_ui(ui, |ui| {
                            for (i, option) in options.iter().enumerate() {
                                if ui.selectable_label(*current == i, option).clicked() {
                                    *current = i;
                                    changed = true;
                                }
                            }
                        });
                }
            }
        });
        
        changed
    }
}
```

#### 10.6.4 Watch List (Surveillance de Valeurs)

```rust
pub struct WatchList {
    entries: Vec<WatchEntry>,
    show_graphs: bool,
}

pub struct WatchEntry {
    pub id: u64,
    pub label: String,
    pub path: ValuePath,
    pub display: WatchDisplay,
    pub history: VecDeque<f32>,  // Pour les graphes
}

pub enum WatchDisplay {
    Value,                           // Juste la valeur
    Slider { min: f32, max: f32 },   // Avec slider éditable
    Graph { min: f32, max: f32 },    // Graphe historique
    Bar { max: f32 },                // Barre de progression
}

impl WatchList {
    pub fn add_watch(&mut self, label: &str, path: ValuePath, display: WatchDisplay) {
        self.entries.push(WatchEntry {
            id: generate_id(),
            label: label.to_string(),
            path,
            display,
            history: VecDeque::with_capacity(120),
        });
    }
    
    pub fn render_ui(&mut self, ui: &mut egui::Ui, world: &mut World) {
        ui.heading("👁 Watch List");
        
        ui.horizontal(|ui| {
            if ui.button("+ Add Watch").clicked() {
                // Ouvrir dialog pour ajouter
            }
            ui.checkbox(&mut self.show_graphs, "Show Graphs");
        });
        
        ui.separator();
        
        let mut to_remove = Vec::new();
        
        for entry in &mut self.entries {
            ui.horizontal(|ui| {
                // Remove button
                if ui.small_button("×").clicked() {
                    to_remove.push(entry.id);
                }
                
                ui.label(&entry.label);
                ui.label("=");
                
                // Get current value
                if let Some(value) = self.get_value(world, &entry.path) {
                    // Update history for graphs
                    if let Some(float_val) = value.as_float() {
                        entry.history.push_back(float_val);
                        if entry.history.len() > 120 {
                            entry.history.pop_front();
                        }
                    }
                    
                    // Display based on type
                    match &entry.display {
                        WatchDisplay::Value => {
                            ui.label(format!("{}", value));
                        }
                        WatchDisplay::Slider { min, max } => {
                            if let Some(mut v) = value.as_float() {
                                if ui.add(egui::Slider::new(&mut v, *min..=*max)).changed() {
                                    self.set_value(world, &entry.path, EditValue::Float(v));
                                }
                            }
                        }
                        WatchDisplay::Graph { min, max } => {
                            if self.show_graphs {
                                let points: egui::plot::PlotPoints = entry.history
                                    .iter()
                                    .enumerate()
                                    .map(|(i, v)| [i as f64, *v as f64])
                                    .collect();
                                
                                egui::plot::Plot::new(entry.id)
                                    .height(40.0)
                                    .show_axes([false, true])
                                    .include_y(*min)
                                    .include_y(*max)
                                    .show(ui, |plot_ui| {
                                        plot_ui.line(egui::plot::Line::new(points));
                                    });
                            } else {
                                ui.label(format!("{:.2}", value));
                            }
                        }
                        WatchDisplay::Bar { max } => {
                            if let Some(v) = value.as_float() {
                                ui.add(egui::ProgressBar::new(v / max).text(format!("{:.0}/{:.0}", v, max)));
                            }
                        }
                    }
                } else {
                    ui.colored_label(egui::Color32::RED, "N/A");
                }
            });
        }
        
        // Remove deleted entries
        self.entries.retain(|e| !to_remove.contains(&e.id));
    }
    
    fn get_value(&self, world: &World, path: &ValuePath) -> Option<EditValue> {
        match path {
            ValuePath::Resource { type_name, field } => {
                // Utiliser reflection ou match manuel
                match *type_name {
                    "GameTime" => {
                        let gt = world.resource::<GameTime>();
                        match field.as_str() {
                            "total_minutes" => Some(EditValue::Int(gt.total_minutes as i32)),
                            "time_scale" => Some(EditValue::Float(gt.time_scale)),
                            "hour" => Some(EditValue::Int(gt.hour() as i32)),
                            _ => None,
                        }
                    }
                    "Camera2D" => {
                        let cam = world.resource::<Camera2D>();
                        match field.as_str() {
                            "position" => Some(EditValue::Vec2(cam.position)),
                            "zoom" => Some(EditValue::Float(cam.zoom)),
                            "rotation" => Some(EditValue::Float(cam.rotation)),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            ValuePath::Component { entity, type_name, field } => {
                match *type_name {
                    "Transform" => {
                        if let Some(t) = world.get::<Transform>(*entity) {
                            match field.as_str() {
                                "position" => Some(EditValue::Vec2(t.position)),
                                "position.x" => Some(EditValue::Float(t.position.x)),
                                "position.y" => Some(EditValue::Float(t.position.y)),
                                "rotation" => Some(EditValue::Float(t.rotation)),
                                _ => None,
                            }
                        } else { None }
                    }
                    "Sprite" => {
                        if let Some(s) = world.get::<Sprite>(*entity) {
                            match field.as_str() {
                                "z_order" => Some(EditValue::Int(s.z_order)),
                                "visible" => Some(EditValue::Bool(s.visible)),
                                _ => None,
                            }
                        } else { None }
                    }
                    "Health" => {
                        if let Some(h) = world.get::<Health>(*entity) {
                            match field.as_str() {
                                "current" => Some(EditValue::Int(h.current as i32)),
                                "max" => Some(EditValue::Int(h.max as i32)),
                                _ => None,
                            }
                        } else { None }
                    }
                    _ => None,
                }
            }
            ValuePath::Expression(_) => None, // Pour plus tard
        }
    }
    
    fn set_value(&self, world: &mut World, path: &ValuePath, value: EditValue) {
        // Implémentation similaire à get_value mais avec mutation
        // ...
    }
}
```

#### 10.6.5 Click-to-Edit dans le Viewport

Pouvoir cliquer sur une entité directement dans le jeu pour l'éditer :

```rust
pub struct ViewportEditor {
    pub enabled: bool,
    pub hovered_entity: Option<Entity>,
    pub selected_entity: Option<Entity>,
    pub drag_mode: DragMode,
    pub gizmo_type: GizmoType,
}

pub enum DragMode {
    None,
    Move(Vec2),      // Offset depuis le début du drag
    Rotate(f32),     // Angle de départ
    Scale(Vec2),     // Scale de départ
}

pub enum GizmoType {
    None,
    Translate,
    Rotate,
    Scale,
    Bounds,  // Redimensionner AABB
}

impl ViewportEditor {
    pub fn update(&mut self, input: &InputState, world: &mut World, camera: &Camera2D) {
        if !self.enabled {
            return;
        }
        
        let mouse_world = camera.screen_to_world(input.mouse_position);
        
        // Hover detection
        self.hovered_entity = self.pick_entity_at(world, mouse_world);
        
        // Selection
        if input.is_mouse_just_pressed(MouseButton::Left) {
            self.selected_entity = self.hovered_entity;
            
            if let Some(entity) = self.selected_entity {
                if let Some(transform) = world.get::<Transform>(entity) {
                    self.drag_mode = DragMode::Move(mouse_world - transform.position);
                }
            }
        }
        
        // Drag
        if input.is_mouse_held(MouseButton::Left) {
            if let (Some(entity), DragMode::Move(offset)) = (self.selected_entity, &self.drag_mode) {
                if let Some(mut transform) = world.get_mut::<Transform>(entity) {
                    transform.position = mouse_world - *offset;
                }
            }
        }
        
        if input.is_mouse_just_released(MouseButton::Left) {
            self.drag_mode = DragMode::None;
        }
        
        // Keyboard shortcuts for selected entity
        if let Some(entity) = self.selected_entity {
            // Delete
            if input.is_key_just_pressed(KeyCode::Delete) {
                world.despawn(entity);
                self.selected_entity = None;
            }
            
            // Duplicate
            if input.is_key_held(KeyCode::LControl) && input.is_key_just_pressed(KeyCode::D) {
                if let Some(new_entity) = self.duplicate_entity(world, entity) {
                    self.selected_entity = Some(new_entity);
                }
            }
            
            // Nudge with arrow keys
            let nudge = if input.is_key_held(KeyCode::LShift) { 10.0 } else { 1.0 };
            if let Some(mut transform) = world.get_mut::<Transform>(entity) {
                if input.is_key_just_pressed(KeyCode::Up) { transform.position.y -= nudge; }
                if input.is_key_just_pressed(KeyCode::Down) { transform.position.y += nudge; }
                if input.is_key_just_pressed(KeyCode::Left) { transform.position.x -= nudge; }
                if input.is_key_just_pressed(KeyCode::Right) { transform.position.x += nudge; }
                
                // Z-order with Page Up/Down
                if let Some(mut sprite) = world.get_mut::<Sprite>(entity) {
                    if input.is_key_just_pressed(KeyCode::PageUp) { sprite.z_order += 1; }
                    if input.is_key_just_pressed(KeyCode::PageDown) { sprite.z_order -= 1; }
                }
            }
        }
    }
    
    pub fn render_gizmos(&self, ctx: &mut RenderContext, world: &World, camera: &Camera2D) {
        // Highlight hovered
        if let Some(entity) = self.hovered_entity {
            if let Some(transform) = world.get::<Transform>(entity) {
                if let Some(sprite) = world.get::<Sprite>(entity) {
                    let bounds = sprite.world_bounds(transform);
                    ctx.draw_rect_outline(bounds, Color::new(1.0, 1.0, 0.0, 0.5), 1.0);
                }
            }
        }
        
        // Draw selection gizmo
        if let Some(entity) = self.selected_entity {
            if let Some(transform) = world.get::<Transform>(entity) {
                let pos = transform.position;
                
                match self.gizmo_type {
                    GizmoType::Translate => {
                        // Draw move arrows
                        ctx.draw_arrow(pos, pos + Vec2::new(30.0, 0.0), Color::RED, 2.0);
                        ctx.draw_arrow(pos, pos + Vec2::new(0.0, 30.0), Color::GREEN, 2.0);
                    }
                    GizmoType::Bounds => {
                        if let Some(sprite) = world.get::<Sprite>(entity) {
                            let bounds = sprite.world_bounds(transform);
                            ctx.draw_rect_outline(bounds, Color::CYAN, 2.0);
                            // Corner handles
                            for corner in bounds.corners() {
                                ctx.draw_rect_filled(
                                    Rect::from_center(corner, Vec2::splat(8.0)),
                                    Color::CYAN,
                                );
                            }
                        }
                    }
                    _ => {
                        // Simple selection box
                        if let Some(sprite) = world.get::<Sprite>(entity) {
                            let bounds = sprite.world_bounds(transform);
                            ctx.draw_rect_outline(bounds, Color::CYAN, 2.0);
                        }
                    }
                }
                
                // Info tooltip
                let screen_pos = camera.world_to_screen(pos);
                ctx.draw_text(
                    &format!(
                        "Entity {:?}\nPos: ({:.0}, {:.0})\nZ: {}",
                        entity,
                        pos.x, pos.y,
                        world.get::<Sprite>(entity).map(|s| s.z_order).unwrap_or(0)
                    ),
                    screen_pos + Vec2::new(20.0, -20.0),
                    12.0,
                    Color::WHITE,
                );
            }
        }
    }
    
    fn pick_entity_at(&self, world: &World, pos: Vec2) -> Option<Entity> {
        let mut best: Option<(Entity, i32)> = None;
        
        for (entity, (transform, sprite)) in world.query::<(&Transform, &Sprite)>() {
            let bounds = sprite.world_bounds(transform);
            if bounds.contains(pos) {
                // Prendre l'entité avec le z-order le plus haut (devant)
                if best.is_none() || sprite.z_order > best.unwrap().1 {
                    best = Some((entity, sprite.z_order));
                }
            }
        }
        
        best.map(|(e, _)| e)
    }
    
    fn duplicate_entity(&self, world: &mut World, entity: Entity) -> Option<Entity> {
        // Clone tous les components de l'entité
        // (implémentation dépend de ton ECS)
        None // Placeholder
    }
}
```

#### 10.6.6 Presets et Sauvegarde de Configurations

```rust
pub struct PresetManager {
    presets: HashMap<String, PresetData>,
    current_preset: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PresetData {
    pub name: String,
    pub description: String,
    pub values: HashMap<String, serde_json::Value>,
}

impl PresetManager {
    pub fn save_preset(&mut self, name: &str, world: &World) {
        let mut values = HashMap::new();
        
        // Sauvegarder les ressources importantes
        if let Some(gt) = world.try_resource::<GameTime>() {
            values.insert("GameTime.time_scale".to_string(), json!(gt.time_scale));
        }
        
        if let Some(cam) = world.try_resource::<Camera2D>() {
            values.insert("Camera2D.position.x".to_string(), json!(cam.position.x));
            values.insert("Camera2D.position.y".to_string(), json!(cam.position.y));
            values.insert("Camera2D.zoom".to_string(), json!(cam.zoom));
        }
        
        if let Some(debug) = world.try_resource::<DebugSettings>() {
            values.insert("DebugSettings.god_mode".to_string(), json!(debug.god_mode));
            values.insert("DebugSettings.no_clip".to_string(), json!(debug.no_clip));
        }
        
        self.presets.insert(name.to_string(), PresetData {
            name: name.to_string(),
            description: String::new(),
            values,
        });
        
        // Sauvegarder sur disque
        self.save_to_file();
    }
    
    pub fn load_preset(&mut self, name: &str, world: &mut World) -> Result<(), String> {
        let preset = self.presets.get(name).ok_or("Preset not found")?;
        
        for (path, value) in &preset.values {
            self.apply_value(world, path, value);
        }
        
        self.current_preset = Some(name.to_string());
        Ok(())
    }
    
    pub fn render_ui(&mut self, ui: &mut egui::Ui, world: &mut World) {
        ui.heading("📁 Presets");
        
        // Current preset
        if let Some(current) = &self.current_preset {
            ui.label(format!("Current: {}", current));
        }
        
        ui.separator();
        
        // Save new preset
        ui.horizontal(|ui| {
            static mut NEW_NAME: String = String::new();
            unsafe {
                ui.text_edit_singleline(&mut NEW_NAME);
                if ui.button("💾 Save").clicked() && !NEW_NAME.is_empty() {
                    self.save_preset(&NEW_NAME, world);
                    NEW_NAME.clear();
                }
            }
        });
        
        ui.separator();
        
        // List presets
        let preset_names: Vec<_> = self.presets.keys().cloned().collect();
        for name in preset_names {
            ui.horizontal(|ui| {
                if ui.button("📂 Load").clicked() {
                    let _ = self.load_preset(&name, world);
                }
                if ui.button("🗑").clicked() {
                    self.presets.remove(&name);
                }
                ui.label(&name);
            });
        }
    }
    
    fn apply_value(&self, world: &mut World, path: &str, value: &serde_json::Value) {
        // Parser le path et appliquer la valeur
        // Ex: "Camera2D.position.x" -> world.resource_mut::<Camera2D>().position.x = value
    }
    
    fn save_to_file(&self) {
        let path = "debug_presets.json";
        if let Ok(json) = serde_json::to_string_pretty(&self.presets) {
            let _ = std::fs::write(path, json);
        }
    }
    
    fn load_from_file(&mut self) {
        let path = "debug_presets.json";
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(presets) = serde_json::from_str(&content) {
                self.presets = presets;
            }
        }
    }
}
```

#### 10.6.7 Raccourcis Clavier Live Edit

| Touche | Action |
|--------|--------|
| F9 | Toggle Viewport Editor (click-to-edit) |
| F10 | Toggle Resource Editor panel |
| F11 | Toggle Watch List |
| Ctrl+S | Save current values as preset |
| Ctrl+Z | Undo last edit |
| Ctrl+Y | Redo |
| Delete | Delete selected entity |
| Ctrl+D | Duplicate selected entity |
| Arrow keys | Nudge selected entity (1px) |
| Shift+Arrows | Nudge selected entity (10px) |
| Page Up/Down | Change z-order of selected |
| G | Toggle gizmo type (translate/rotate/scale) |
| Escape | Deselect |

### 10.7 Event Log et Event Debugger

Tracker tous les événements du jeu pour comprendre les chaînes de causalité.

```rust
pub struct EventLog {
    entries: VecDeque<EventEntry>,
    max_entries: usize,
    filters: EventFilters,
    paused: bool,
}

pub struct EventEntry {
    pub timestamp: f64,        // Game time
    pub frame: u64,            // Frame number
    pub category: EventCategory,
    pub message: String,
    pub details: Option<String>,
    pub source_entity: Option<Entity>,
    pub target_entity: Option<Entity>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EventCategory {
    Input,
    Collision,
    StateChange,
    Spawn,
    Despawn,
    Damage,
    Inventory,
    Dialogue,
    Quest,
    Audio,
    Animation,
    Trigger,
    Custom,
}

impl EventCategory {
    pub fn color(&self) -> Color {
        match self {
            Self::Input => Color::new(0.7, 0.7, 0.7, 1.0),
            Self::Collision => Color::new(1.0, 0.5, 0.0, 1.0),
            Self::StateChange => Color::new(0.0, 1.0, 1.0, 1.0),
            Self::Spawn => Color::new(0.0, 1.0, 0.0, 1.0),
            Self::Despawn => Color::new(1.0, 0.0, 0.0, 1.0),
            Self::Damage => Color::new(1.0, 0.0, 0.5, 1.0),
            Self::Inventory => Color::new(1.0, 0.8, 0.0, 1.0),
            Self::Dialogue => Color::new(0.5, 0.5, 1.0, 1.0),
            Self::Quest => Color::new(1.0, 0.0, 1.0, 1.0),
            Self::Audio => Color::new(0.3, 0.7, 0.3, 1.0),
            Self::Animation => Color::new(0.8, 0.6, 0.4, 1.0),
            Self::Trigger => Color::new(1.0, 1.0, 0.0, 1.0),
            Self::Custom => Color::new(0.5, 0.5, 0.5, 1.0),
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Input => "⌨",
            Self::Collision => "💥",
            Self::StateChange => "🔄",
            Self::Spawn => "✨",
            Self::Despawn => "💨",
            Self::Damage => "💔",
            Self::Inventory => "🎒",
            Self::Dialogue => "💬",
            Self::Quest => "📜",
            Self::Audio => "🔊",
            Self::Animation => "🎬",
            Self::Trigger => "⚡",
            Self::Custom => "📌",
        }
    }
}

#[derive(Default)]
pub struct EventFilters {
    pub show_input: bool,
    pub show_collision: bool,
    pub show_state_change: bool,
    pub show_spawn: bool,
    pub show_despawn: bool,
    pub show_damage: bool,
    pub show_inventory: bool,
    pub show_dialogue: bool,
    pub show_quest: bool,
    pub show_audio: bool,
    pub show_animation: bool,
    pub show_trigger: bool,
    pub show_custom: bool,
    pub entity_filter: Option<Entity>,
    pub search: String,
}

impl EventFilters {
    pub fn all_enabled() -> Self {
        Self {
            show_input: true,
            show_collision: true,
            show_state_change: true,
            show_spawn: true,
            show_despawn: true,
            show_damage: true,
            show_inventory: true,
            show_dialogue: true,
            show_quest: true,
            show_audio: true,
            show_animation: true,
            show_trigger: true,
            show_custom: true,
            entity_filter: None,
            search: String::new(),
        }
    }
}

impl EventLog {
    pub fn log(&mut self, category: EventCategory, message: impl Into<String>) {
        if self.paused {
            return;
        }
        
        self.entries.push_back(EventEntry {
            timestamp: get_game_time(),
            frame: get_frame_count(),
            category,
            message: message.into(),
            details: None,
            source_entity: None,
            target_entity: None,
        });
        
        while self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
    }
    
    pub fn log_detailed(&mut self, entry: EventEntry) {
        if self.paused {
            return;
        }
        self.entries.push_back(entry);
        while self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
    }
    
    pub fn render_ui(&mut self, ui: &mut egui::Ui) {
        // Toolbar
        ui.horizontal(|ui| {
            if ui.button(if self.paused { "▶ Resume" } else { "⏸ Pause" }).clicked() {
                self.paused = !self.paused;
            }
            if ui.button("🗑 Clear").clicked() {
                self.entries.clear();
            }
            ui.label(format!("{} events", self.entries.len()));
        });
        
        // Filters
        egui::CollapsingHeader::new("Filters").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.filters.show_input, "Input");
                ui.checkbox(&mut self.filters.show_collision, "Collision");
                ui.checkbox(&mut self.filters.show_state_change, "State");
                ui.checkbox(&mut self.filters.show_spawn, "Spawn");
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.filters.show_damage, "Damage");
                ui.checkbox(&mut self.filters.show_inventory, "Inventory");
                ui.checkbox(&mut self.filters.show_dialogue, "Dialogue");
                ui.checkbox(&mut self.filters.show_trigger, "Trigger");
            });
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut self.filters.search);
            });
        });
        
        ui.separator();
        
        // Event list
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for entry in &self.entries {
                    if !self.should_show(entry) {
                        continue;
                    }
                    
                    ui.horizontal(|ui| {
                        ui.colored_label(
                            entry.category.color(),
                            format!("{} [{}] #{}", 
                                entry.category.icon(),
                                format_time(entry.timestamp),
                                entry.frame
                            ),
                        );
                        ui.label(&entry.message);
                    });
                    
                    if let Some(details) = &entry.details {
                        ui.indent("details", |ui| {
                            ui.label(details);
                        });
                    }
                }
            });
    }
    
    fn should_show(&self, entry: &EventEntry) -> bool {
        let category_match = match entry.category {
            EventCategory::Input => self.filters.show_input,
            EventCategory::Collision => self.filters.show_collision,
            EventCategory::StateChange => self.filters.show_state_change,
            EventCategory::Spawn => self.filters.show_spawn,
            EventCategory::Despawn => self.filters.show_despawn,
            EventCategory::Damage => self.filters.show_damage,
            EventCategory::Inventory => self.filters.show_inventory,
            EventCategory::Dialogue => self.filters.show_dialogue,
            EventCategory::Quest => self.filters.show_quest,
            EventCategory::Audio => self.filters.show_audio,
            EventCategory::Animation => self.filters.show_animation,
            EventCategory::Trigger => self.filters.show_trigger,
            EventCategory::Custom => self.filters.show_custom,
        };
        
        if !category_match {
            return false;
        }
        
        if let Some(filter_entity) = self.filters.entity_filter {
            if entry.source_entity != Some(filter_entity) && entry.target_entity != Some(filter_entity) {
                return false;
            }
        }
        
        if !self.filters.search.is_empty() {
            if !entry.message.to_lowercase().contains(&self.filters.search.to_lowercase()) {
                return false;
            }
        }
        
        true
    }
}

// Macro pour logger facilement depuis n'importe où
#[macro_export]
macro_rules! debug_event {
    ($log:expr, $category:expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $log.log($category, format!($($arg)*));
    };
}

// Usage dans le code
debug_event!(event_log, EventCategory::Collision, 
    "Player collided with NPC {:?}", npc_entity);

debug_event!(event_log, EventCategory::StateChange,
    "NPC {:?} state: {:?} -> {:?}", entity, old_state, new_state);
```

### 10.8 Performance Profiler

Un profiler visuel pour identifier les goulots d'étranglement.

```rust
pub struct FrameProfiler {
    frames: VecDeque<FrameProfile>,
    max_frames: usize,
    current_frame: FrameProfile,
    sections: HashMap<&'static str, SectionTimer>,
}

pub struct FrameProfile {
    pub frame_number: u64,
    pub total_time_ms: f32,
    pub sections: Vec<(&'static str, f32)>,  // (name, time_ms)
    pub draw_calls: u32,
    pub triangles: u32,
    pub entities_rendered: u32,
}

pub struct SectionTimer {
    start: Option<Instant>,
    accumulated: f32,
}

impl FrameProfiler {
    pub fn begin_section(&mut self, name: &'static str) {
        self.sections.entry(name).or_default().start = Some(Instant::now());
    }
    
    pub fn end_section(&mut self, name: &'static str) {
        if let Some(timer) = self.sections.get_mut(name) {
            if let Some(start) = timer.start.take() {
                timer.accumulated += start.elapsed().as_secs_f32() * 1000.0;
            }
        }
    }
    
    pub fn end_frame(&mut self, stats: RenderStats) {
        let mut sections = Vec::new();
        let mut total = 0.0;
        
        for (name, timer) in &mut self.sections {
            sections.push((*name, timer.accumulated));
            total += timer.accumulated;
            timer.accumulated = 0.0;
        }
        
        self.current_frame = FrameProfile {
            frame_number: get_frame_count(),
            total_time_ms: total,
            sections,
            draw_calls: stats.draw_calls,
            triangles: stats.triangles,
            entities_rendered: stats.entities_rendered,
        };
        
        self.frames.push_back(self.current_frame.clone());
        while self.frames.len() > self.max_frames {
            self.frames.pop_front();
        }
    }
    
    pub fn render_ui(&self, ui: &mut egui::Ui) {
        // Frame time graph
        ui.heading("Frame Time");
        
        let frame_times: Vec<f32> = self.frames.iter().map(|f| f.total_time_ms).collect();
        let avg = frame_times.iter().sum::<f32>() / frame_times.len() as f32;
        let max = frame_times.iter().cloned().fold(0.0f32, f32::max);
        let min = frame_times.iter().cloned().fold(f32::MAX, f32::min);
        
        ui.label(format!("Avg: {:.2}ms | Min: {:.2}ms | Max: {:.2}ms", avg, min, max));
        ui.label(format!("FPS: {:.0}", 1000.0 / avg));
        
        // Simple bar chart
        let chart_height = 60.0;
        let bar_width = 2.0;
        let target_ms = 16.67;  // 60 FPS target
        
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (response, painter) = ui.allocate_painter(
                egui::vec2(self.max_frames as f32 * bar_width, chart_height),
                egui::Sense::hover(),
            );
            
            let rect = response.rect;
            
            // 60 FPS line
            let target_y = rect.top() + (chart_height * (1.0 - target_ms / 33.33));
            painter.line_segment(
                [egui::pos2(rect.left(), target_y), egui::pos2(rect.right(), target_y)],
                egui::Stroke::new(1.0, egui::Color32::GREEN),
            );
            
            // Frame bars
            for (i, frame) in self.frames.iter().enumerate() {
                let height = (frame.total_time_ms / 33.33) * chart_height;
                let color = if frame.total_time_ms > target_ms {
                    egui::Color32::RED
                } else {
                    egui::Color32::from_rgb(100, 200, 100)
                };
                
                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(rect.left() + i as f32 * bar_width, rect.bottom() - height),
                        egui::vec2(bar_width - 1.0, height),
                    ),
                    0.0,
                    color,
                );
            }
        });
        
        ui.separator();
        
        // Section breakdown
        ui.heading("Breakdown (last frame)");
        
        if let Some(last) = self.frames.back() {
            for (name, time) in &last.sections {
                let pct = time / last.total_time_ms * 100.0;
                ui.horizontal(|ui| {
                    ui.label(format!("{}: {:.2}ms ({:.1}%)", name, time, pct));
                    ui.add(egui::ProgressBar::new(pct / 100.0));
                });
            }
            
            ui.separator();
            
            // Render stats
            ui.label(format!("Draw calls: {}", last.draw_calls));
            ui.label(format!("Triangles: {}", last.triangles));
            ui.label(format!("Entities rendered: {}", last.entities_rendered));
        }
    }
}

// Macro pour profiler une section
#[macro_export]
macro_rules! profile_section {
    ($profiler:expr, $name:literal, $block:block) => {{
        $profiler.begin_section($name);
        let result = $block;
        $profiler.end_section($name);
        result
    }};
}

// Usage
profile_section!(profiler, "physics", {
    physics_system.update(world, dt);
});

profile_section!(profiler, "render", {
    renderer.render(world, camera);
});
```

### 10.9 Debug Console

Une console in-game pour exécuter des commandes de debug.

```rust
pub struct DebugConsole {
    input: String,
    history: Vec<String>,
    output: Vec<ConsoleMessage>,
    history_index: Option<usize>,
    commands: HashMap<String, Box<dyn DebugCommand>>,
}

pub struct ConsoleMessage {
    pub text: String,
    pub level: MessageLevel,
}

pub enum MessageLevel {
    Info,
    Success,
    Warning,
    Error,
}

pub trait DebugCommand: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, args: &[&str], world: &mut World) -> CommandResult;
}

pub type CommandResult = Result<String, String>;

impl DebugConsole {
    pub fn register_default_commands(&mut self) {
        // Teleport player
        self.register(TeleportCommand);
        // Set time
        self.register(SetTimeCommand);
        // Give item
        self.register(GiveItemCommand);
        // Spawn entity
        self.register(SpawnCommand);
        // Kill entity
        self.register(KillCommand);
        // Set player stat
        self.register(SetStatCommand);
        // List entities
        self.register(ListEntitiesCommand);
        // Toggle god mode
        self.register(GodModeCommand);
        // Reload assets
        self.register(ReloadAssetsCommand);
        // Help
        self.register(HelpCommand);
    }
    
    pub fn execute(&mut self, input: &str, world: &mut World) {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return;
        }
        
        let command_name = parts[0].to_lowercase();
        let args = &parts[1..];
        
        self.history.push(input.to_string());
        self.output.push(ConsoleMessage {
            text: format!("> {}", input),
            level: MessageLevel::Info,
        });
        
        if let Some(command) = self.commands.get(&command_name) {
            match command.execute(args, world) {
                Ok(msg) => {
                    self.output.push(ConsoleMessage {
                        text: msg,
                        level: MessageLevel::Success,
                    });
                }
                Err(msg) => {
                    self.output.push(ConsoleMessage {
                        text: msg,
                        level: MessageLevel::Error,
                    });
                }
            }
        } else {
            self.output.push(ConsoleMessage {
                text: format!("Unknown command: {}", command_name),
                level: MessageLevel::Error,
            });
        }
    }
}

// Example commands implementation
struct TeleportCommand;
impl DebugCommand for TeleportCommand {
    fn name(&self) -> &str { "tp" }
    fn description(&self) -> &str { "tp <x> <y> - Teleport player to position" }
    
    fn execute(&self, args: &[&str], world: &mut World) -> CommandResult {
        if args.len() != 2 {
            return Err("Usage: tp <x> <y>".to_string());
        }
        
        let x: f32 = args[0].parse().map_err(|_| "Invalid x coordinate")?;
        let y: f32 = args[1].parse().map_err(|_| "Invalid y coordinate")?;
        
        if let Some((_, transform)) = world.query::<(&Player, &mut Transform)>().iter().next() {
            transform.position = Vec2::new(x, y);
            Ok(format!("Teleported to ({}, {})", x, y))
        } else {
            Err("No player found".to_string())
        }
    }
}

struct GiveItemCommand;
impl DebugCommand for GiveItemCommand {
    fn name(&self) -> &str { "give" }
    fn description(&self) -> &str { "give <item_id> [quantity] - Give item to player" }
    
    fn execute(&self, args: &[&str], world: &mut World) -> CommandResult {
        if args.is_empty() {
            return Err("Usage: give <item_id> [quantity]".to_string());
        }
        
        let item_id = args[0];
        let quantity: u32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
        
        // Validate item exists
        let item_def = get_item_definition(item_id)
            .ok_or_else(|| format!("Unknown item: {}", item_id))?;
        
        // Find player inventory and add item
        for (_, inventory) in world.query::<(&Player, &mut Inventory)>() {
            let stack = ItemStack {
                item_id: item_def.id,
                quantity,
                quality: Quality::Normal,
            };
            
            if inventory.add_item(stack).is_some() {
                return Err("Inventory full".to_string());
            }
            
            return Ok(format!("Gave {} x{}", item_def.name, quantity));
        }
        
        Err("No player found".to_string())
    }
}

struct SetTimeCommand;
impl DebugCommand for SetTimeCommand {
    fn name(&self) -> &str { "time" }
    fn description(&self) -> &str { "time <hour> [minute] - Set game time" }
    
    fn execute(&self, args: &[&str], world: &mut World) -> CommandResult {
        if args.is_empty() {
            return Err("Usage: time <hour> [minute]".to_string());
        }
        
        let hour: u32 = args[0].parse().map_err(|_| "Invalid hour")?;
        let minute: u32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        
        if hour >= 24 || minute >= 60 {
            return Err("Invalid time (hour: 0-23, minute: 0-59)".to_string());
        }
        
        let game_time = world.resource_mut::<GameTime>();
        let current_day = game_time.day();
        game_time.total_minutes = (current_day - 1) * 24 * 60 + hour * 60 + minute;
        
        Ok(format!("Time set to {:02}:{:02}", hour, minute))
    }
}
```

### 10.10 Raccourcis Clavier Debug (Récapitulatif)

| Touche | Action |
|--------|--------|
| **Panels** | |
| F12 | Toggle debug mode (master switch) |
| F1 | Aide / Légende |
| F2 | Performance profiler |
| F3 | Physics debug (collisions) |
| F4 | Render debug (z-order, layers) |
| F5 | ECS Entity Inspector |
| F6 | Event Log |
| F7 | Console |
| F8 | Pause game (step-by-step avec N) |
| F9 | Viewport Editor (click-to-edit) |
| F10 | Resource Editor panel |
| F11 | Watch List |
| **Overlays** | |
| Ctrl+C | Toggle collision boxes |
| Ctrl+G | Toggle spatial grid |
| Ctrl+Z | Toggle z-order labels |
| Ctrl+T | Toggle trigger zones |
| Ctrl+V | Toggle velocity vectors |
| Ctrl+I | Toggle entity IDs |
| **Frame Control** | |
| N (quand pausé) | Step une frame |
| Shift+N (quand pausé) | Step 10 frames |
| **Live Edit** | |
| Ctrl+S | Save current values as preset |
| Ctrl+Z | Undo last edit |
| Ctrl+Y | Redo |
| Delete | Delete selected entity |
| Ctrl+D | Duplicate selected entity |
| Arrow keys | Nudge selected entity (1px) |
| Shift+Arrows | Nudge selected entity (10px) |
| Page Up/Down | Change z-order of selected |
| G | Toggle gizmo type |
| Escape | Deselect |

### 10.11 Compilation Conditionnelle

Tous les outils de debug doivent pouvoir être exclus en release :

```rust
// Dans Cargo.toml
[features]
default = ["debug-tools"]
debug-tools = ["egui", "egui-wgpu", "egui-winit"]

// Dans le code
#[cfg(feature = "debug-tools")]
mod debug;

#[cfg(feature = "debug-tools")]
use debug::DebugManager;

// Struct conditionnelle
pub struct Game {
    // ...
    #[cfg(feature = "debug-tools")]
    pub debug: DebugManager,
}

// Build release sans debug
// cargo build --release --no-default-features
```

**Impact sur les performances** : En release sans debug-tools, zéro overhead. Les macros de profiling et d'events se compilent en no-op.

---

## 11. Ressources et Références

### 11.1 Documentation

- [Learn wgpu](https://sotrh.github.io/learn-wgpu/) — Tutorial complet wgpu
- [Rust Game Development](https://rust-gamedev.github.io/) — Newsletter et ressources
- [Game Programming Patterns](https://gameprogrammingpatterns.com/) — Patterns essentiels (gratuit en ligne)

### 11.2 Projets de Référence

- **Macroquad** : Moteur 2D minimaliste, excellent pour comprendre les bases
- **Bevy** : Moteur ECS complet, source d'inspiration pour l'architecture
- **ggez** : API simple style Love2D
- **Comfy** : Moteur 2D moderne orienté prototypage

### 11.3 Assets Gratuits

- [OpenGameArt](https://opengameart.org/) — Sprites, tilesets, audio
- [Kenney](https://kenney.nl/) — Assets de qualité, domaine public
- [itch.io](https://itch.io/game-assets/free) — Assets gratuits

---

## 12. Conclusion

Construire un moteur de jeu from scratch est un projet ambitieux mais extrêmement formateur. Les clés du succès :

1. **Itérer petit** : Chaque phase doit produire quelque chose de jouable/testable
2. **Ne pas over-engineer** : Commencer simple, refactorer quand nécessaire
3. **Documenter au fur et à mesure** : Le "toi du futur" te remerciera
4. **Tester sur toutes les plateformes régulièrement** : Pas juste à la fin

Le choix de Rust te force à penser à la structure du code dès le départ, ce qui est bénéfique pour un projet de cette envergure. L'écosystème gamedev Rust est jeune mais dynamique — c'est le bon moment pour s'y investir.

Bon développement ! 🦀🎮
