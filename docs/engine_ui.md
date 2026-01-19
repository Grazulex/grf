# engine_ui

Module d'interface utilisateur in-game pour le HUD et les menus.

## Vue d'ensemble

`engine_ui` fournit des composants UI basiques:
- **Hud**: Affichage tete haute (barre de vie, energie, hotbar)
- **ProgressBar**: Barre de progression generique
- **Menu**: Systeme de menus interactifs
- **SettingsMenu**: Menu de parametres

> **Note**: Le rendu de texte n'est pas encore implemente. Les labels utilisent des rectangles colores comme placeholders.

---

## Hud

Affichage tete haute avec barres de vie, energie et hotbar.

### Structure

```rust
pub struct Hud {
    /// Barre de vie
    health_bar: ProgressBar,

    /// Barre d'energie
    energy_bar: ProgressBar,

    /// Slots de la hotbar
    hotbar: Hotbar,

    /// Affichage de l'heure
    time_display: TimeDisplay,

    /// Position et dimensions
    layout: HudLayout,
}
```

### HudLayout

```rust
pub struct HudLayout {
    /// Padding depuis les bords
    pub padding: f32,

    /// Taille des barres de progression
    pub bar_width: f32,
    pub bar_height: f32,

    /// Taille des slots hotbar
    pub slot_size: f32,
    pub slot_spacing: f32,
}

impl Default for HudLayout {
    fn default() -> Self {
        Self {
            padding: 16.0,
            bar_width: 200.0,
            bar_height: 20.0,
            slot_size: 48.0,
            slot_spacing: 4.0,
        }
    }
}
```

### Methodes

```rust
impl Hud {
    /// Cree un nouveau HUD
    pub fn new(screen_width: f32, screen_height: f32) -> Self;

    /// Cree avec layout personnalise
    pub fn with_layout(screen_width: f32, screen_height: f32, layout: HudLayout) -> Self;

    /// Met a jour les valeurs
    pub fn update(&mut self, health: f32, max_health: f32, energy: f32, max_energy: f32);

    /// Met a jour la hotbar
    pub fn update_hotbar(&mut self, items: &[Option<HotbarItem>], selected: usize);

    /// Met a jour l'affichage du temps
    pub fn update_time(&mut self, time_string: &str, date_string: &str);

    /// Redimensionne selon la nouvelle taille d'ecran
    pub fn resize(&mut self, screen_width: f32, screen_height: f32);

    /// Dessine le HUD
    pub fn render(&self, ctx: &mut FrameContext);
}
```

### HotbarItem

```rust
pub struct HotbarItem {
    /// ID de l'item
    pub item_id: String,

    /// Quantite
    pub quantity: u32,

    /// Region de texture
    pub uv: Option<Rect>,
}
```

### Utilisation

```rust
let mut hud = Hud::new(1280.0, 720.0);

// Mise a jour chaque frame
hud.update(
    player.health,
    player.max_health,
    player.energy,
    player.max_energy,
);

hud.update_hotbar(&inventory.hotbar_items(), inventory.selected_slot());
hud.update_time(&clock.time_string(), &clock.date_string());

// Rendu
hud.render(&mut frame_ctx);
```

---

## ProgressBar

Barre de progression generique.

### Structure

```rust
pub struct ProgressBar {
    /// Position (coin superieur gauche)
    pub position: Vec2,

    /// Dimensions
    pub width: f32,
    pub height: f32,

    /// Valeur actuelle (0.0 - 1.0)
    pub value: f32,

    /// Couleur de fond
    pub background_color: Color,

    /// Couleur de remplissage
    pub fill_color: Color,

    /// Couleur de bordure
    pub border_color: Color,

    /// Epaisseur de bordure
    pub border_width: f32,
}
```

### Methodes

```rust
impl ProgressBar {
    /// Cree une nouvelle barre
    pub fn new(position: Vec2, width: f32, height: f32) -> Self;

    /// Definit la valeur (clamp 0-1)
    pub fn set_value(&mut self, value: f32);

    /// Definit depuis current/max
    pub fn set_from_values(&mut self, current: f32, max: f32);

    /// Definit les couleurs
    pub fn with_colors(mut self, background: Color, fill: Color) -> Self;

    /// Definit la bordure
    pub fn with_border(mut self, color: Color, width: f32) -> Self;

    /// Dessine la barre
    pub fn render(&self, ctx: &mut FrameContext);
}
```

### Couleurs predefinies

```rust
impl ProgressBar {
    /// Barre de vie rouge
    pub fn health_bar(position: Vec2, width: f32, height: f32) -> Self;

    /// Barre d'energie jaune/orange
    pub fn energy_bar(position: Vec2, width: f32, height: f32) -> Self;

    /// Barre d'experience bleue
    pub fn xp_bar(position: Vec2, width: f32, height: f32) -> Self;
}
```

### Utilisation

```rust
// Barre de vie personnalisee
let mut health_bar = ProgressBar::new(Vec2::new(16.0, 16.0), 200.0, 20.0)
    .with_colors(Color::from_hex(0x333333), Color::from_hex(0xFF3333))
    .with_border(Color::WHITE, 2.0);

health_bar.set_from_values(player.health, player.max_health);
health_bar.render(&mut ctx);
```

---

## Menu

Systeme de menus avec items selectionnables.

### Structure

```rust
pub struct Menu {
    /// Titre du menu
    title: String,

    /// Items du menu
    items: Vec<MenuItem>,

    /// Index selectionne
    selected: usize,

    /// Style du menu
    style: MenuStyle,

    /// Position
    position: Vec2,

    /// Dimensions
    size: Vec2,
}

pub struct MenuItem {
    /// Label de l'item
    pub label: String,

    /// Item active?
    pub enabled: bool,

    /// Donnees associees
    pub data: Option<String>,
}
```

### MenuStyle

```rust
pub struct MenuStyle {
    /// Couleur de fond
    pub background_color: Color,

    /// Couleur de bordure
    pub border_color: Color,

    /// Epaisseur de bordure
    pub border_width: f32,

    /// Couleur d'item normal
    pub item_color: Color,

    /// Couleur d'item selectionne
    pub selected_color: Color,

    /// Couleur d'item desactive
    pub disabled_color: Color,

    /// Hauteur d'un item
    pub item_height: f32,

    /// Padding interne
    pub padding: f32,
}

impl Default for MenuStyle {
    fn default() -> Self {
        Self {
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.9),
            border_color: Color::WHITE,
            border_width: 2.0,
            item_color: Color::WHITE,
            selected_color: Color::from_hex(0xFFD700), // Gold
            disabled_color: Color::rgba(0.5, 0.5, 0.5, 1.0),
            item_height: 40.0,
            padding: 16.0,
        }
    }
}
```

### Methodes

```rust
impl Menu {
    /// Cree un nouveau menu
    pub fn new(title: &str) -> Self;

    /// Cree centre sur l'ecran
    pub fn centered(title: &str, screen_width: f32, screen_height: f32) -> Self;

    /// Ajoute un item
    pub fn add_item(&mut self, label: &str);

    /// Ajoute un item avec donnees
    pub fn add_item_with_data(&mut self, label: &str, data: &str);

    /// Ajoute un item desactive
    pub fn add_disabled_item(&mut self, label: &str);

    /// Definit le style
    pub fn with_style(mut self, style: MenuStyle) -> Self;

    /// Selection precedente
    pub fn select_previous(&mut self);

    /// Selection suivante
    pub fn select_next(&mut self);

    /// Retourne l'index selectionne
    pub fn selected_index(&self) -> usize;

    /// Retourne l'item selectionne
    pub fn selected_item(&self) -> Option<&MenuItem>;

    /// Definit la selection
    pub fn set_selected(&mut self, index: usize);

    /// Nombre d'items
    pub fn item_count(&self) -> usize;

    /// Dessine le menu
    pub fn render(&self, ctx: &mut FrameContext);
}
```

### MenuAction

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuAction {
    /// Item selectionne (index)
    Select(usize),

    /// Menu ferme/annule
    Cancel,

    /// Aucune action
    None,
}
```

### Utilisation

```rust
// Menu principal
let mut main_menu = Menu::centered("MAIN MENU", screen_width, screen_height);
main_menu.add_item("New Game");
main_menu.add_item("Load Game");
main_menu.add_item("Settings");
main_menu.add_item("Quit");

// Dans la boucle
if input.is_key_pressed(KeyCode::Up) {
    main_menu.select_previous();
}
if input.is_key_pressed(KeyCode::Down) {
    main_menu.select_next();
}
if input.is_key_pressed(KeyCode::Enter) {
    match main_menu.selected_index() {
        0 => start_new_game(),
        1 => load_game(),
        2 => show_settings(),
        3 => quit(),
        _ => {}
    }
}

// Rendu
main_menu.render(&mut ctx);
```

---

## SettingsMenu

Menu de parametres avec sliders et toggles.

### Structure

```rust
pub struct SettingsMenu {
    /// Items de configuration
    items: Vec<SettingsItem>,

    /// Index selectionne
    selected: usize,

    /// Style
    style: MenuStyle,

    /// Position et taille
    position: Vec2,
    size: Vec2,

    /// Etat precedent pour retour
    previous_state: PreviousState,
}

pub enum SettingsItem {
    /// Slider (0.0 - 1.0)
    Slider {
        label: String,
        value: f32,
        step: f32,
    },

    /// Toggle on/off
    Toggle {
        label: String,
        value: bool,
    },

    /// Action (bouton)
    Action {
        label: String,
    },
}

pub enum PreviousState {
    MainMenu,
    Paused,
}
```

### Methodes

```rust
impl SettingsMenu {
    /// Cree un menu de settings
    pub fn new(screen_width: f32, screen_height: f32, previous: PreviousState) -> Self;

    /// Cree depuis GameSettings
    pub fn from_settings(
        settings: &GameSettings,
        screen_width: f32,
        screen_height: f32,
        previous: PreviousState,
    ) -> Self;

    /// Navigation
    pub fn select_previous(&mut self);
    pub fn select_next(&mut self);

    /// Ajustement des valeurs
    pub fn adjust_left(&mut self);
    pub fn adjust_right(&mut self);

    /// Active le toggle selectionne
    pub fn toggle_selected(&mut self);

    /// Retourne l'etat precedent
    pub fn previous_state(&self) -> PreviousState;

    /// Applique les changements a GameSettings
    pub fn apply_to_settings(&self, settings: &mut GameSettings);

    /// Dessine le menu
    pub fn render(&self, ctx: &mut FrameContext);
}
```

### Utilisation

```rust
// Creer depuis les settings actuels
let mut settings_menu = SettingsMenu::from_settings(
    &settings,
    screen_width,
    screen_height,
    PreviousState::MainMenu,
);

// Gestion des inputs
if input.is_key_pressed(KeyCode::Up) {
    settings_menu.select_previous();
}
if input.is_key_pressed(KeyCode::Down) {
    settings_menu.select_next();
}
if input.is_key_pressed(KeyCode::Left) {
    settings_menu.adjust_left();
}
if input.is_key_pressed(KeyCode::Right) {
    settings_menu.adjust_right();
}
if input.is_key_pressed(KeyCode::Enter) {
    settings_menu.toggle_selected();
}

// Appliquer et retourner
if input.is_key_pressed(KeyCode::Escape) {
    settings_menu.apply_to_settings(&mut settings);
    settings.save("settings.json");

    game_state = match settings_menu.previous_state() {
        PreviousState::MainMenu => GameState::MainMenu,
        PreviousState::Paused => GameState::Paused,
    };
}

// Rendu
settings_menu.render(&mut ctx);
```

---

## Hotbar

Barre d'outils rapide.

### Structure

```rust
pub struct Hotbar {
    /// Slots
    slots: Vec<HotbarSlot>,

    /// Index selectionne
    selected: usize,

    /// Position
    position: Vec2,

    /// Taille d'un slot
    slot_size: f32,

    /// Espacement entre slots
    spacing: f32,
}

pub struct HotbarSlot {
    /// Item dans le slot
    pub item: Option<HotbarItem>,

    /// Position du slot
    pub position: Vec2,
}
```

### Methodes

```rust
impl Hotbar {
    /// Cree une hotbar
    pub fn new(slot_count: usize, slot_size: f32) -> Self;

    /// Position centree en bas de l'ecran
    pub fn centered_bottom(slot_count: usize, slot_size: f32, screen_width: f32, screen_height: f32) -> Self;

    /// Met a jour un slot
    pub fn set_slot(&mut self, index: usize, item: Option<HotbarItem>);

    /// Selectionne un slot
    pub fn select(&mut self, index: usize);

    /// Selection suivante (avec wrap)
    pub fn select_next(&mut self);

    /// Selection precedente (avec wrap)
    pub fn select_previous(&mut self);

    /// Retourne l'index selectionne
    pub fn selected(&self) -> usize;

    /// Retourne l'item selectionne
    pub fn selected_item(&self) -> Option<&HotbarItem>;

    /// Dessine la hotbar
    pub fn render(&self, ctx: &mut FrameContext, item_texture: Option<&Texture>);
}
```

---

## TimeDisplay

Affichage de l'heure et de la date.

### Structure

```rust
pub struct TimeDisplay {
    /// Position
    position: Vec2,

    /// Heure formatee
    time_text: String,

    /// Date formatee
    date_text: String,

    /// Style
    style: TimeDisplayStyle,
}

pub struct TimeDisplayStyle {
    pub background_color: Color,
    pub text_color: Color,
    pub padding: f32,
    pub width: f32,
    pub height: f32,
}
```

### Methodes

```rust
impl TimeDisplay {
    /// Cree un affichage de temps
    pub fn new(position: Vec2) -> Self;

    /// Position en haut a droite
    pub fn top_right(screen_width: f32, padding: f32) -> Self;

    /// Met a jour les textes
    pub fn update(&mut self, time: &str, date: &str);

    /// Dessine l'affichage
    pub fn render(&self, ctx: &mut FrameContext);
}
```

---

## Notes de rendu

Actuellement, tous les elements UI sont rendus comme des rectangles colores car le rendu de texte n'est pas encore implemente.

### Couleurs utilisees

| Element | Couleur |
|---------|---------|
| Fond de barre | Gris fonce (#333333) |
| Vie | Rouge (#FF3333) |
| Energie | Orange (#FFAA33) |
| Selection | Or (#FFD700) |
| Fond de menu | Noir transparent (0.1, 0.1, 0.1, 0.9) |
| Bordure | Blanc |
| Item desactive | Gris (0.5, 0.5, 0.5) |

### Future amelioration

Le module `engine_ui` sera enrichi avec:
- Rendu de texte (bitmap fonts ou SDF)
- Systeme de themes
- Widgets supplementaires (boutons, checkboxes, text input)
- Layout automatique
