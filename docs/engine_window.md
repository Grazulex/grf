# engine_window

Module de fenetrage basé sur winit, fournissant la creation de fenetres et la boucle d'evenements.

## Vue d'ensemble

`engine_window` encapsule winit pour fournir:
- **WindowConfig**: Configuration de la fenetre
- **Window**: Fenetre de l'application
- **App**: Trait pour les applications
- Boucle d'evenements integree

## WindowConfig

Configuration pour creer une fenetre.

### Structure

```rust
pub struct WindowConfig {
    /// Titre de la fenetre
    pub title: String,

    /// Largeur initiale en pixels
    pub width: u32,

    /// Hauteur initiale en pixels
    pub height: u32,

    /// Fenetre redimensionnable?
    pub resizable: bool,

    /// Mode plein ecran?
    pub fullscreen: bool,

    /// VSync active?
    pub vsync: bool,

    /// Decorations de fenetre (bordures, barre de titre)?
    pub decorations: bool,
}
```

### Methodes (Builder Pattern)

```rust
impl WindowConfig {
    /// Configuration par defaut
    pub fn new() -> Self;

    /// Definit le titre
    pub fn with_title(mut self, title: &str) -> Self;

    /// Definit la taille
    pub fn with_size(mut self, width: u32, height: u32) -> Self;

    /// Active/desactive le redimensionnement
    pub fn with_resizable(mut self, resizable: bool) -> Self;

    /// Active/desactive le plein ecran
    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self;

    /// Active/desactive VSync
    pub fn with_vsync(mut self, vsync: bool) -> Self;

    /// Active/desactive les decorations
    pub fn with_decorations(mut self, decorations: bool) -> Self;
}
```

### Valeurs par defaut

```rust
WindowConfig {
    title: "GRF Game".to_string(),
    width: 1280,
    height: 720,
    resizable: true,
    fullscreen: false,
    vsync: true,
    decorations: true,
}
```

### Utilisation

```rust
let config = WindowConfig::new()
    .with_title("Mon Super Jeu")
    .with_size(1920, 1080)
    .with_fullscreen(false)
    .with_vsync(true);
```

---

## Window

Fenetre de l'application encapsulant winit.

### Structure

```rust
pub struct Window {
    // Champs internes prives
    event_loop: EventLoop<()>,
    window: winit::window::Window,
    config: WindowConfig,
}
```

### Methodes

```rust
impl Window {
    /// Cree une nouvelle fenetre avec la configuration donnee
    pub fn new(config: WindowConfig) -> Self;

    /// Retourne une reference au handle winit
    pub fn handle(&self) -> &winit::window::Window;

    /// Retourne la taille actuelle (width, height)
    pub fn size(&self) -> (u32, u32);

    /// Retourne la largeur
    pub fn width(&self) -> u32;

    /// Retourne la hauteur
    pub fn height(&self) -> u32;

    /// Retourne le facteur d'echelle (HiDPI)
    pub fn scale_factor(&self) -> f64;

    /// Definit le titre
    pub fn set_title(&mut self, title: &str);

    /// Bascule le mode plein ecran
    pub fn toggle_fullscreen(&mut self);

    /// Definit le mode plein ecran
    pub fn set_fullscreen(&mut self, fullscreen: bool);

    /// Est en plein ecran?
    pub fn is_fullscreen(&self) -> bool;

    /// Demande un nouveau rendu
    pub fn request_redraw(&self);

    /// Lance la boucle d'evenements (consomme la fenetre)
    pub fn run<A: App + 'static>(self, app: A);
}
```

---

## Trait App

Trait que doit implementer votre application pour s'integrer a la boucle d'evenements.

### Definition

```rust
pub trait App {
    /// Appele a chaque frame pour la mise a jour
    /// `dt` est le delta time en secondes
    fn update(&mut self, dt: f32);

    /// Appele pour le rendu
    fn render(&mut self);

    /// Appele pour chaque evenement de fenetre
    fn handle_event(&mut self, event: &WindowEvent);

    /// Appele quand la fenetre est redimensionnee
    /// Implementation par defaut vide
    fn on_resize(&mut self, width: u32, height: u32) {
        let _ = (width, height);
    }

    /// Appele quand l'application doit se fermer
    /// Retourne true pour confirmer la fermeture
    /// Implementation par defaut: true
    fn on_close_requested(&mut self) -> bool {
        true
    }

    /// Appele a la fermeture de l'application
    /// Implementation par defaut vide
    fn on_shutdown(&mut self) {}
}
```

### WindowEvent

```rust
pub enum WindowEvent {
    /// Touche pressee
    KeyPressed { key: KeyCode, modifiers: Modifiers },

    /// Touche relachee
    KeyReleased { key: KeyCode, modifiers: Modifiers },

    /// Bouton souris presse
    MousePressed { button: MouseButton, position: (f32, f32) },

    /// Bouton souris relache
    MouseReleased { button: MouseButton, position: (f32, f32) },

    /// Souris deplacee
    MouseMoved { position: (f32, f32) },

    /// Molette souris
    MouseWheel { delta: f32 },

    /// Fenetre redimensionnee
    Resized { width: u32, height: u32 },

    /// Focus gagne
    Focused,

    /// Focus perdu
    Unfocused,

    /// Fermeture demandee
    CloseRequested,
}
```

---

## Utilisation complete

### Application minimale

```rust
use engine_window::{App, Window, WindowConfig, WindowEvent};

struct MyGame {
    running: bool,
}

impl MyGame {
    fn new() -> Self {
        Self { running: true }
    }
}

impl App for MyGame {
    fn update(&mut self, dt: f32) {
        // Mise a jour de la logique
    }

    fn render(&mut self) {
        // Rendu du jeu
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyPressed { key, .. } => {
                if *key == KeyCode::Escape {
                    self.running = false;
                }
            }
            _ => {}
        }
    }

    fn on_close_requested(&mut self) -> bool {
        self.running = false;
        true
    }
}

fn main() {
    let config = WindowConfig::new()
        .with_title("Mon Jeu")
        .with_size(1280, 720);

    let window = Window::new(config);
    let game = MyGame::new();

    window.run(game);
}
```

### Integration avec le renderer

```rust
use engine_window::{App, Window, WindowConfig, WindowEvent};
use engine_render::Renderer;
use engine_input::Input;

struct Game {
    renderer: Renderer,
    input: Input,
}

impl Game {
    fn new(window: &Window) -> Self {
        let renderer = pollster::block_on(Renderer::new(
            window.handle(),
            window.width(),
            window.height(),
        ));

        Self {
            renderer,
            input: Input::new(),
        }
    }
}

impl App for Game {
    fn update(&mut self, dt: f32) {
        // Utiliser self.input pour la logique
        if self.input.is_key_pressed(KeyCode::Space) {
            // Action
        }

        // Reset des etats instantanes
        self.input.end_frame();
    }

    fn render(&mut self) {
        self.renderer.begin_frame();
        // ... rendu ...
        self.renderer.end_frame();
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        self.input.process_event(event);
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }
}
```

---

## Boucle d'evenements interne

La methode `Window::run` gere automatiquement:

1. **Polling des evenements** - Collecte tous les evenements winit
2. **Conversion** - Transforme les evenements winit en `WindowEvent`
3. **Distribution** - Appelle `handle_event` pour chaque evenement
4. **Timing** - Calcule le delta time entre les frames
5. **Update** - Appelle `update` avec le delta time
6. **Render** - Appelle `render` apres l'update
7. **Redimensionnement** - Appelle `on_resize` automatiquement

Le code interne ressemble a:

```rust
event_loop.run(move |event, target| {
    match event {
        Event::WindowEvent { event, .. } => {
            let window_event = convert_event(event);
            app.handle_event(&window_event);

            if let WindowEvent::Resized { width, height } = window_event {
                app.on_resize(width, height);
            }
        }
        Event::AboutToWait => {
            let now = Instant::now();
            let dt = (now - last_frame).as_secs_f32();
            last_frame = now;

            app.update(dt);
            app.render();
            window.request_redraw();
        }
        _ => {}
    }
});
```

---

## Gestion HiDPI

La fenetre gere automatiquement les ecrans haute resolution:

```rust
// Taille logique (en points)
let (width, height) = window.size();

// Facteur d'echelle (ex: 2.0 pour Retina)
let scale = window.scale_factor();

// Taille physique (en pixels)
let physical_width = (width as f64 * scale) as u32;
let physical_height = (height as f64 * scale) as u32;
```
