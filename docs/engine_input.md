# engine_input

Module de gestion des inputs clavier et souris.

## Vue d'ensemble

`engine_input` fournit une abstraction des inputs:
- **Input**: Gestionnaire d'etat des inputs
- **KeyCode**: Codes des touches clavier
- **MouseButton**: Boutons de souris
- **ButtonState**: Etats des boutons (pressed, held, released)

---

## Input

Gestionnaire centralise des inputs.

### Structure

```rust
pub struct Input {
    // Etat des touches
    keys_current: HashSet<KeyCode>,
    keys_previous: HashSet<KeyCode>,

    // Etat de la souris
    mouse_buttons_current: HashSet<MouseButton>,
    mouse_buttons_previous: HashSet<MouseButton>,
    mouse_position: Vec2,
    mouse_delta: Vec2,
    scroll_delta: f32,

    // Modificateurs
    modifiers: Modifiers,
}
```

### Methodes principales

```rust
impl Input {
    /// Cree un nouveau gestionnaire
    pub fn new() -> Self;

    /// Traite un evenement de fenetre
    pub fn process_event(&mut self, event: &WindowEvent);

    /// Appeler en fin de frame pour reset les etats "just"
    pub fn end_frame(&mut self);
}
```

### Clavier

```rust
impl Input {
    /// Touche actuellement enfoncee
    pub fn is_key_held(&self, key: KeyCode) -> bool;

    /// Touche vient d'etre pressee ce frame
    pub fn is_key_pressed(&self, key: KeyCode) -> bool;

    /// Touche vient d'etre relachee ce frame
    pub fn is_key_released(&self, key: KeyCode) -> bool;

    /// Retourne l'etat complet d'une touche
    pub fn key_state(&self, key: KeyCode) -> ButtonState;

    /// Une des touches est enfoncee
    pub fn any_key_held(&self, keys: &[KeyCode]) -> bool;

    /// Une des touches vient d'etre pressee
    pub fn any_key_pressed(&self, keys: &[KeyCode]) -> bool;
}
```

### Souris

```rust
impl Input {
    /// Bouton actuellement enfonce
    pub fn is_mouse_held(&self, button: MouseButton) -> bool;

    /// Bouton vient d'etre presse ce frame
    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool;

    /// Bouton vient d'etre relache ce frame
    pub fn is_mouse_released(&self, button: MouseButton) -> bool;

    /// Position actuelle de la souris (ecran)
    pub fn mouse_position(&self) -> Vec2;

    /// Deplacement de la souris ce frame
    pub fn mouse_delta(&self) -> Vec2;

    /// Delta de la molette ce frame
    pub fn scroll_delta(&self) -> f32;
}
```

### Modificateurs

```rust
impl Input {
    /// Shift enfonce
    pub fn is_shift_held(&self) -> bool;

    /// Ctrl enfonce
    pub fn is_ctrl_held(&self) -> bool;

    /// Alt enfonce
    pub fn is_alt_held(&self) -> bool;

    /// Super/Windows/Cmd enfonce
    pub fn is_super_held(&self) -> bool;

    /// Retourne les modificateurs actifs
    pub fn modifiers(&self) -> Modifiers;
}
```

### Utilitaires

```rust
impl Input {
    /// Vecteur de direction WASD/Fleches normalise
    pub fn movement_vector(&self) -> Vec2;

    /// Vecteur de direction WASD uniquement
    pub fn wasd_vector(&self) -> Vec2;

    /// Vecteur de direction fleches uniquement
    pub fn arrow_vector(&self) -> Vec2;

    /// Reset complet de l'etat
    pub fn reset(&mut self);
}
```

---

## KeyCode

Enumeration des codes de touches.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // Lettres
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // Chiffres
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,

    // Pave numerique
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4,
    Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd, NumpadSubtract, NumpadMultiply, NumpadDivide,
    NumpadEnter, NumpadDecimal,

    // Fonctions
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    // Fleches
    Up, Down, Left, Right,

    // Modificateurs
    LeftShift, RightShift,
    LeftControl, RightControl,
    LeftAlt, RightAlt,
    LeftSuper, RightSuper,

    // Speciales
    Space, Enter, Tab, Backspace, Delete, Insert,
    Home, End, PageUp, PageDown,
    Escape, CapsLock, NumLock, ScrollLock,
    PrintScreen, Pause,

    // Ponctuation
    Comma, Period, Slash, Backslash,
    Semicolon, Apostrophe, BracketLeft, BracketRight,
    Minus, Equal, Grave,

    // Inconnu
    Unknown,
}
```

---

## MouseButton

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,    // Bouton 4
    Forward, // Bouton 5
}
```

---

## ButtonState

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    /// Pas presse
    Up,

    /// Vient d'etre presse ce frame
    JustPressed,

    /// Maintenu enfonce
    Held,

    /// Vient d'etre relache ce frame
    JustReleased,
}

impl ButtonState {
    /// Est presse (JustPressed ou Held)
    pub fn is_pressed(self) -> bool;

    /// Est relache (Up ou JustReleased)
    pub fn is_released(self) -> bool;
}
```

---

## Modifiers

```rust
#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}

impl Modifiers {
    pub fn none() -> Self;
    pub fn shift() -> Self;
    pub fn ctrl() -> Self;
    pub fn alt() -> Self;
}
```

---

## Utilisation complete

### Integration avec la boucle de jeu

```rust
use engine_input::{Input, KeyCode, MouseButton};
use engine_window::{App, WindowEvent};

struct Game {
    input: Input,
    player_pos: Vec2,
}

impl App for Game {
    fn update(&mut self, dt: f32) {
        // Mouvement avec WASD
        let move_dir = self.input.movement_vector();
        self.player_pos += move_dir * PLAYER_SPEED * dt;

        // Actions
        if self.input.is_key_pressed(KeyCode::Space) {
            self.player_jump();
        }

        if self.input.is_key_pressed(KeyCode::E) {
            self.interact();
        }

        if self.input.is_key_pressed(KeyCode::Escape) {
            self.toggle_pause();
        }

        // Hotbar (touches 1-9)
        for i in 0..9 {
            let key = match i {
                0 => KeyCode::Key1,
                1 => KeyCode::Key2,
                // ...
                _ => continue,
            };
            if self.input.is_key_pressed(key) {
                self.select_hotbar_slot(i);
            }
        }

        // Souris
        if self.input.is_mouse_pressed(MouseButton::Left) {
            self.use_tool();
        }

        if self.input.is_mouse_pressed(MouseButton::Right) {
            self.use_item();
        }

        // Zoom avec molette
        let scroll = self.input.scroll_delta();
        if scroll != 0.0 {
            self.camera.zoom_by(1.0 + scroll * 0.1);
        }

        // IMPORTANT: Reset en fin de frame
        self.input.end_frame();
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        self.input.process_event(event);
    }
}
```

### Raccourcis avec modificateurs

```rust
// Ctrl+S pour sauvegarder
if self.input.is_ctrl_held() && self.input.is_key_pressed(KeyCode::S) {
    self.save_game();
}

// Shift+Click pour action alternative
if self.input.is_shift_held() && self.input.is_mouse_pressed(MouseButton::Left) {
    self.quick_move_item();
}

// Debug keys (F1-F12)
if self.input.is_key_pressed(KeyCode::F3) {
    self.toggle_debug_overlay();
}
```

### Selection de slot hotbar

```rust
fn get_hotbar_key_pressed(&self) -> Option<usize> {
    const HOTBAR_KEYS: [KeyCode; 10] = [
        KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4, KeyCode::Key5,
        KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9, KeyCode::Key0,
    ];

    for (i, key) in HOTBAR_KEYS.iter().enumerate() {
        if self.input.is_key_pressed(*key) {
            return Some(i);
        }
    }
    None
}
```

### Mouvement normalise

```rust
// Le vecteur est automatiquement normalise pour eviter
// le mouvement diagonal plus rapide
let movement = self.input.movement_vector();

// Equivalent a:
let mut dir = Vec2::ZERO;
if self.input.is_key_held(KeyCode::W) || self.input.is_key_held(KeyCode::Up) {
    dir.y -= 1.0;
}
if self.input.is_key_held(KeyCode::S) || self.input.is_key_held(KeyCode::Down) {
    dir.y += 1.0;
}
if self.input.is_key_held(KeyCode::A) || self.input.is_key_held(KeyCode::Left) {
    dir.x -= 1.0;
}
if self.input.is_key_held(KeyCode::D) || self.input.is_key_held(KeyCode::Right) {
    dir.x += 1.0;
}
if dir != Vec2::ZERO {
    dir = dir.normalize();
}
```

---

## Notes importantes

1. **Appeler `end_frame()`**: Toujours appeler `input.end_frame()` a la fin de l'update pour reset les etats "just pressed/released".

2. **Ordre des operations**: Process event -> Update -> End frame

3. **Pas de doublons**: `is_key_pressed` ne retourne `true` qu'une seule fois par pression physique.

4. **Mouse position**: Les coordonnees sont en pixels ecran. Utiliser `camera.screen_to_world()` pour convertir.
