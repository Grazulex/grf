# GRF - Game Rust Framework

Moteur de jeu 2D from scratch en Rust, oriente RPG/Farming top-down (style Stardew Valley).

---

## Configuration Claude Code

**Ne PAS utiliser container-use pour ce projet.** Travailler directement sur le filesystem local.

---

## Architecture du Projet

### Workspace Cargo (Modulaire)

```
grf/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── engine_core/        # Event loop, temps, lifecycle
│   ├── engine_window/      # Fenetrage (winit wrapper)
│   ├── engine_render/      # Pipeline de rendu 2D
│   ├── engine_input/       # Abstraction input
│   ├── engine_audio/       # Systeme audio
│   ├── engine_ecs/         # Entity-Component-System
│   ├── engine_physics/     # Collision 2D
│   ├── engine_assets/      # Chargement et cache des ressources
│   ├── engine_ui/          # UI in-game basique
│   └── engine_debug/       # Outils de debug (feature-gated)
├── game/                   # Le jeu lui-meme
└── assets/                 # Ressources (textures, audio, maps, data)
```

### Stack Technique

| Crate | Usage |
|-------|-------|
| `winit` | Fenetrage et evenements cross-platform |
| `wgpu` | Abstraction GPU (Vulkan/Metal/DX12/WebGPU) |
| `glam` | Mathematiques (vecteurs, matrices) avec SIMD |
| `serde` + `serde_json` + `toml` | Serialisation (configs, saves) |
| `rodio` | Audio cross-platform |
| `image` | Chargement d'images |
| `egui` | Debug UI (feature-gated) |

---

## Conventions de Code Rust

### Structure des Crates

Chaque crate engine_* suit cette structure:
```
engine_xxx/
├── Cargo.toml
└── src/
    ├── lib.rs          # Re-exports publics
    └── xxx.rs          # Implementation
```

### Naming Conventions

- **Types**: PascalCase (`SpriteRenderer`, `GameTime`)
- **Fonctions/Methodes**: snake_case (`update_position`, `get_sprite`)
- **Constantes**: SCREAMING_SNAKE_CASE (`FIXED_TIMESTEP`, `MAX_ENTITIES`)
- **Modules**: snake_case (`sprite_batch`, `collision_system`)

### Patterns ECS

Components = Donnees pures (structs simples):
```rust
#[derive(Component)]
pub struct Position { pub x: f32, pub y: f32 }
```

Systems = Logique operant sur components:
```rust
fn movement_system(world: &mut World, dt: f32) {
    for (pos, vel) in world.query::<(&mut Position, &Velocity)>() {
        pos.x += vel.x * dt;
    }
}
```

### Error Handling

- Utiliser `Result<T, E>` pour les erreurs recuperables
- `thiserror` pour definir des types d'erreur custom
- `anyhow` pour la propagation rapide dans le code applicatif

### Documentation

```rust
/// Description breve.
///
/// # Arguments
/// * `param` - Description du parametre
///
/// # Returns
/// Description du retour
///
/// # Examples
/// ```
/// let result = ma_fonction(42);
/// ```
pub fn ma_fonction(param: i32) -> i32 { ... }
```

---

## Game Loop Architecture

```
Fixed Timestep (60 UPS) + Variable Render:

1. POLL INPUT       -> InputState
2. FIXED UPDATE     -> GameState (while accumulator >= dt)
3. RENDER           -> FrameBuffer (avec interpolation alpha)
4. PRESENT          -> Display
```

Constante: `FIXED_TIMESTEP = 1.0 / 60.0`

---

## Render Pipeline

### Z-Order Layers (Top-Down)

| Z | Layer | Description |
|---|-------|-------------|
| 0 | Ground | Tiles de sol |
| 1 | Ground Decor | Fleurs, chemins |
| 2 | Shadows | Ombres des entites |
| 3 | Entities | Joueur, NPCs, objets (Y-sorted) |
| 4 | Above Entities | Bulles de dialogue |
| 5 | Weather/UI | Effets meteo, UI |

### Sprite Batching

Grouper les sprites par texture atlas pour minimiser les draw calls.

---

## Collision System

### AABB (Axis-Aligned Bounding Box)

```rust
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}
```

### Spatial Grid

Partitionnement spatial pour eviter O(n²) tests:
- Cell size configurable (typiquement 64-128 pixels)
- Query par bounds pour collisions potentielles

---

## Data-Driven Design

### Formats de Donnees

| Type | Format | Exemple |
|------|--------|---------|
| Maps | JSON | `assets/maps/farm.json` |
| Items | TOML | `assets/data/items.toml` |
| Dialogues | JSON | `assets/dialogues/npc_intro.json` |
| Input Mapping | TOML | `assets/config/input.toml` |

### Structure Assets

```
assets/
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

## Debug Tools (F12 toggle)

| Touche | Panel |
|--------|-------|
| F1 | Help |
| F2 | Performance |
| F3 | Physics (collisions) |
| F4 | Render (z-order) |
| F5 | ECS Inspector |
| F6 | Event Log |
| F7 | Console |
| F8 | Pause |

### Overlays (Ctrl+touche)

- `Ctrl+C` : Collision boxes
- `Ctrl+Z` : Z-order labels
- `Ctrl+G` : Tile grid

---

## Phases de Developpement

1. **Fondations** - Fenetre, rendu sprite, game loop, input, camera
2. **Monde** - Tilemaps, collisions, transitions, animations
3. **ECS & Systems** - Architecture ECS, resource manager, audio
4. **Gameplay Core** - Temps in-game, inventaire, farming, dialogues
5. **Debug Tools** - egui overlay, inspectors, console
6. **Finition** - Save/load, polish, documentation

---

## Commandes Utiles

```bash
# Build debug
cargo build

# Build release
cargo build --release

# Run avec debug tools
cargo run --features debug-tools

# Run release (sans debug)
cargo run --release

# Tests
cargo test --workspace

# Clippy (linting)
cargo clippy --workspace -- -W clippy::all

# Format
cargo fmt --all

# Documentation
cargo doc --workspace --open
```

---

## Cross-Compilation

```bash
# Windows (depuis Linux)
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# macOS (necessite osxcross)
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
```

---

## Ressources

- **Document d'architecture**: `2d-game-engine-plan.md`
- **winit**: https://docs.rs/winit
- **wgpu**: https://docs.rs/wgpu
- **glam**: https://docs.rs/glam
- **egui**: https://docs.rs/egui
- **rodio**: https://docs.rs/rodio
