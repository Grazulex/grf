# GRF - Game Rust Framework

Documentation technique complete du framework de jeu 2D en Rust.

## Vue d'ensemble

GRF est un moteur de jeu 2D modulaire ecrit en Rust, oriente vers les jeux RPG/Farming top-down (style Stardew Valley). Il est construit autour d'une architecture en crates separees pour une meilleure modularite et reutilisabilite.

## Architecture

```
grf/
├── crates/
│   ├── engine_core/      # Boucle de jeu, temps, cycle de vie
│   ├── engine_window/    # Fenetrage (winit)
│   ├── engine_render/    # Pipeline de rendu 2D (wgpu)
│   ├── engine_input/     # Abstraction des inputs
│   ├── engine_ecs/       # Entity-Component-System
│   ├── engine_ui/        # Interface utilisateur in-game
│   ├── engine_assets/    # Gestion des ressources
│   ├── engine_physics/   # Collisions 2D
│   ├── engine_audio/     # Systeme audio
│   └── engine_debug/     # Outils de debug (feature-gated)
└── game/                 # Application de test
```

## Modules de documentation

| Module | Description |
|--------|-------------|
| [engine_core](./engine_core.md) | Boucle de jeu, temps, horloge in-game, parametres |
| [engine_window](./engine_window.md) | Creation de fenetre, evenements, trait App |
| [engine_render](./engine_render.md) | Rendu 2D, sprites, tilemaps, camera, animations |
| [engine_input](./engine_input.md) | Gestion des inputs clavier/souris |
| [engine_ecs](./engine_ecs.md) | Systeme Entity-Component-System |
| [engine_ui](./engine_ui.md) | Composants UI (HUD, menus) |
| [engine_assets](./engine_assets.md) | Chargement et cache des ressources |
| [engine_physics](./engine_physics.md) | Collisions AABB et grille spatiale |
| [engine_audio](./engine_audio.md) | Systeme audio |
| [engine_debug](./engine_debug.md) | Outils de debug egui |

## Stack technique

| Crate | Usage |
|-------|-------|
| `winit` | Fenetrage cross-platform |
| `wgpu` | Abstraction GPU (Vulkan/Metal/DX12/WebGPU) |
| `glam` | Mathematiques vectorielles avec SIMD |
| `serde` | Serialisation (JSON, TOML) |
| `image` | Chargement d'images |
| `rodio` | Audio cross-platform |
| `egui` | UI de debug |

## Boucle de jeu

Le framework utilise un **fixed timestep** pour la logique de jeu (60 UPS) avec un rendu a frequence variable:

```
1. POLL INPUT      -> Collecte des evenements
2. FIXED UPDATE    -> Mise a jour logique (60 fois/sec)
3. RENDER          -> Rendu avec interpolation
4. PRESENT         -> Affichage
```

## Demarrage rapide

```rust
use engine_window::{App, Window, WindowConfig};
use engine_render::Renderer;
use engine_input::Input;
use engine_core::GameTime;

struct MyGame {
    renderer: Renderer,
    input: Input,
}

impl App for MyGame {
    fn update(&mut self, dt: f32) {
        // Logique de jeu
    }

    fn render(&mut self) {
        // Rendu
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        self.input.process_event(event);
    }
}

fn main() {
    let config = WindowConfig::default()
        .with_title("Mon Jeu")
        .with_size(1280, 720);

    let window = Window::new(config);
    let game = MyGame::new(&window);
    window.run(game);
}
```

## Compilation

```bash
# Debug
cargo build

# Release
cargo build --release

# Avec outils de debug
cargo run --features debug-tools

# Tests
cargo test --workspace

# Linting
cargo clippy --workspace
```

## Licence

MIT
