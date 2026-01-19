# engine_debug

Module d'outils de debug avec egui (feature-gated).

## Vue d'ensemble

`engine_debug` fournit des outils de debug actives par la feature `debug-tools`:
- **DebugOverlay**: Affichage FPS et statistiques
- **CollisionVisualizer**: Visualisation des collisions
- **EcsInspector**: Inspection des entites et composants
- **PerformanceProfiler**: Mesures de performance
- **EventLog**: Journal des evenements
- **DebugConsole**: Console de commandes

> **Note**: Ce module est compile uniquement avec `--features debug-tools`

---

## Activation

### Cargo.toml

```toml
[features]
debug-tools = ["egui", "egui-wgpu"]

[dependencies]
egui = { version = "0.24", optional = true }
egui-wgpu = { version = "0.24", optional = true }
```

### Compilation

```bash
# Avec outils de debug
cargo run --features debug-tools

# Sans (release)
cargo run --release
```

---

## DebugOverlay

Affichage d'informations de debug.

### Structure

```rust
#[cfg(feature = "debug-tools")]
pub struct DebugOverlay {
    /// Visible?
    visible: bool,

    /// Panneau actif
    active_panel: DebugPanel,

    /// FPS tracker
    fps_counter: FpsCounter,

    /// Statistiques
    stats: DebugStats,
}

pub enum DebugPanel {
    None,
    Performance,
    Physics,
    Render,
    Ecs,
    EventLog,
    Console,
}
```

### Raccourcis clavier

| Touche | Action |
|--------|--------|
| F1 | Aide |
| F2 | Performance |
| F3 | Physics (collisions) |
| F4 | Render (z-order) |
| F5 | ECS Inspector |
| F6 | Event Log |
| F7 | Console |
| F8 | Pause |
| F12 | Toggle overlay |

### Overlays (Ctrl+touche)

| Raccourci | Overlay |
|-----------|---------|
| Ctrl+C | Collision boxes |
| Ctrl+Z | Z-order labels |
| Ctrl+G | Tile grid |

### Methodes

```rust
impl DebugOverlay {
    /// Cree un overlay
    pub fn new() -> Self;

    /// Traite un evenement
    pub fn handle_event(&mut self, event: &WindowEvent) -> bool;

    /// Met a jour
    pub fn update(&mut self, dt: f32);

    /// Dessine l'overlay egui
    pub fn render(&mut self, ctx: &egui::Context);

    /// Toggle visibilite
    pub fn toggle(&mut self);

    /// Est visible?
    pub fn is_visible(&self) -> bool;

    /// Change de panneau
    pub fn set_panel(&mut self, panel: DebugPanel);

    /// Ajoute une statistique
    pub fn set_stat(&mut self, name: &str, value: impl ToString);
}
```

---

## FpsCounter

Compteur de FPS lisse.

```rust
pub struct FpsCounter {
    frame_times: VecDeque<f32>,
    fps: f32,
    frame_time: f32,
    sample_count: usize,
}

impl FpsCounter {
    pub fn new(sample_count: usize) -> Self;
    pub fn update(&mut self, dt: f32);
    pub fn fps(&self) -> f32;
    pub fn frame_time_ms(&self) -> f32;
    pub fn min_fps(&self) -> f32;
    pub fn max_fps(&self) -> f32;
}
```

---

## CollisionVisualizer

Visualisation des boites de collision.

### Structure

```rust
pub struct CollisionVisualizer {
    /// Afficher les AABBs?
    show_aabb: bool,

    /// Afficher la grille spatiale?
    show_grid: bool,

    /// Afficher les collisions actives?
    show_active: bool,

    /// Couleurs
    aabb_color: Color,
    grid_color: Color,
    active_color: Color,
}
```

### Methodes

```rust
impl CollisionVisualizer {
    pub fn new() -> Self;

    /// Toggle affichage des AABBs
    pub fn toggle_aabb(&mut self);

    /// Toggle affichage de la grille
    pub fn toggle_grid(&mut self);

    /// Dessine les AABBs des entites
    pub fn draw_aabbs(&self, world: &World, ctx: &mut FrameContext);

    /// Dessine la grille spatiale
    pub fn draw_grid(&self, grid: &SpatialGrid, ctx: &mut FrameContext);

    /// Dessine les collisions actives
    pub fn draw_active(&self, collisions: &[CollisionResult], ctx: &mut FrameContext);
}
```

---

## EcsInspector

Inspection des entites et composants.

### Structure

```rust
pub struct EcsInspector {
    /// Entite selectionnee
    selected: Option<Entity>,

    /// Filtre de recherche
    filter: String,

    /// Composants deplies
    expanded: HashSet<TypeId>,
}
```

### Methodes

```rust
impl EcsInspector {
    pub fn new() -> Self;

    /// Dessine l'inspecteur egui
    pub fn render(&mut self, ctx: &egui::Context, world: &World);

    /// Selectionne une entite
    pub fn select(&mut self, entity: Entity);

    /// Deselectionne
    pub fn deselect(&mut self);

    /// Entite selectionnee
    pub fn selected(&self) -> Option<Entity>;
}
```

### Affichage

```
+---------------------------+
| ECS Inspector             |
+---------------------------+
| Filter: [________]        |
| Entities: 42              |
+---------------------------+
| > Entity(0:1) - Player    |
|   - Position: (100, 100)  |
|   - Velocity: (0, 0)      |
|   - Health: 100/100       |
| > Entity(1:1) - Enemy     |
| > Entity(2:1) - Crop      |
+---------------------------+
```

---

## PerformanceProfiler

Mesures de performance detaillees.

### Structure

```rust
pub struct PerformanceProfiler {
    /// Sections mesurees
    sections: HashMap<String, ProfileSection>,

    /// Section en cours
    current: Option<String>,

    /// Timestamp de debut
    start_time: Instant,
}

pub struct ProfileSection {
    pub name: String,
    pub total_time: f32,
    pub call_count: u32,
    pub min_time: f32,
    pub max_time: f32,
}
```

### Methodes

```rust
impl PerformanceProfiler {
    pub fn new() -> Self;

    /// Commence une section
    pub fn begin(&mut self, name: &str);

    /// Termine une section
    pub fn end(&mut self);

    /// Reset les mesures
    pub fn reset(&mut self);

    /// Retourne les statistiques
    pub fn stats(&self) -> &HashMap<String, ProfileSection>;

    /// Dessine le profiler egui
    pub fn render(&self, ctx: &egui::Context);
}
```

### Utilisation

```rust
profiler.begin("physics");
physics_system(world);
profiler.end();

profiler.begin("render");
render_system(world, renderer);
profiler.end();
```

### Affichage

```
+---------------------------+
| Performance Profiler      |
+---------------------------+
| Section     | Time   | %  |
+---------------------------+
| physics     | 0.5ms  | 8% |
| render      | 4.2ms  | 67%|
| input       | 0.1ms  | 2% |
| update      | 1.4ms  | 23%|
+---------------------------+
| Total: 6.2ms (161 FPS)    |
+---------------------------+
```

---

## EventLog

Journal des evenements.

### Structure

```rust
pub struct EventLog {
    /// Evenements
    events: VecDeque<LogEntry>,

    /// Capacite max
    capacity: usize,

    /// Filtre de niveau
    level_filter: LogLevel,
}

pub struct LogEntry {
    pub timestamp: f32,
    pub level: LogLevel,
    pub category: String,
    pub message: String,
}

pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}
```

### Methodes

```rust
impl EventLog {
    pub fn new(capacity: usize) -> Self;

    /// Log un message
    pub fn log(&mut self, level: LogLevel, category: &str, message: &str);

    /// Raccourcis
    pub fn debug(&mut self, category: &str, message: &str);
    pub fn info(&mut self, category: &str, message: &str);
    pub fn warn(&mut self, category: &str, message: &str);
    pub fn error(&mut self, category: &str, message: &str);

    /// Vide le log
    pub fn clear(&mut self);

    /// Filtre par niveau
    pub fn set_filter(&mut self, level: LogLevel);

    /// Dessine le log egui
    pub fn render(&self, ctx: &egui::Context);
}
```

---

## DebugConsole

Console de commandes interactives.

### Structure

```rust
pub struct DebugConsole {
    /// Visible?
    visible: bool,

    /// Ligne de saisie
    input: String,

    /// Historique des commandes
    history: Vec<String>,

    /// Index dans l'historique
    history_index: Option<usize>,

    /// Sortie
    output: Vec<String>,

    /// Commandes enregistrees
    commands: HashMap<String, Box<dyn Command>>,
}
```

### Commandes integrees

| Commande | Description |
|----------|-------------|
| `help` | Liste des commandes |
| `clear` | Efface la sortie |
| `spawn <type>` | Spawn une entite |
| `teleport <x> <y>` | Teleporte le joueur |
| `god` | Mode invincible |
| `time <hour>` | Change l'heure |
| `give <item> [qty]` | Donne un item |
| `stats` | Statistiques |

### Methodes

```rust
impl DebugConsole {
    pub fn new() -> Self;

    /// Enregistre une commande
    pub fn register<C: Command + 'static>(&mut self, name: &str, command: C);

    /// Execute une commande
    pub fn execute(&mut self, input: &str, world: &mut World);

    /// Toggle visibilite
    pub fn toggle(&mut self);

    /// Traite les evenements clavier
    pub fn handle_event(&mut self, event: &WindowEvent) -> bool;

    /// Dessine la console egui
    pub fn render(&mut self, ctx: &egui::Context, world: &mut World);
}
```

### Trait Command

```rust
pub trait Command {
    /// Execute la commande
    fn execute(&self, args: &[&str], world: &mut World) -> Result<String, String>;

    /// Description pour l'aide
    fn description(&self) -> &str;

    /// Usage
    fn usage(&self) -> &str;
}
```

---

## Utilisation complete

```rust
#[cfg(feature = "debug-tools")]
use engine_debug::{DebugOverlay, CollisionVisualizer, EcsInspector, EventLog};

struct Game {
    // ...

    #[cfg(feature = "debug-tools")]
    debug_overlay: DebugOverlay,

    #[cfg(feature = "debug-tools")]
    collision_viz: CollisionVisualizer,

    #[cfg(feature = "debug-tools")]
    event_log: EventLog,
}

impl Game {
    fn new() -> Self {
        Self {
            // ...

            #[cfg(feature = "debug-tools")]
            debug_overlay: DebugOverlay::new(),

            #[cfg(feature = "debug-tools")]
            collision_viz: CollisionVisualizer::new(),

            #[cfg(feature = "debug-tools")]
            event_log: EventLog::new(1000),
        }
    }

    fn update(&mut self, dt: f32) {
        #[cfg(feature = "debug-tools")]
        self.debug_overlay.update(dt);

        #[cfg(feature = "debug-tools")]
        self.event_log.info("game", &format!("Update: {:.2}ms", dt * 1000.0));
    }

    fn render(&mut self) {
        // Rendu normal...

        #[cfg(feature = "debug-tools")]
        {
            if self.collision_viz.show_aabb {
                self.collision_viz.draw_aabbs(&self.world, &mut ctx);
            }

            self.debug_overlay.render(&egui_ctx);
        }
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        #[cfg(feature = "debug-tools")]
        {
            if self.debug_overlay.handle_event(event) {
                return; // Evenement consomme par le debug
            }
        }

        // Gestion normale des evenements...
    }
}
```

---

## Bonnes pratiques

1. **Feature-gate tout**: Utiliser `#[cfg(feature = "debug-tools")]`
2. **Pas en release**: Ne jamais compiler les outils en release
3. **Overhead minimal**: Les outils ne doivent pas impacter les perfs
4. **Log utile**: Logger les evenements importants pour le debug
5. **Commandes pratiques**: Creer des commandes pour accelerer le test
