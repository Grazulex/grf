# engine_render

Module de rendu 2D base sur wgpu, gerant les sprites, tilemaps, camera et animations.

## Vue d'ensemble

`engine_render` fournit un pipeline de rendu 2D complet:
- **Renderer**: Pipeline wgpu principal
- **Camera2D**: Camera 2D avec transformations
- **Sprite**: Sprites individuels
- **SpriteBatch**: Rendu groupe de sprites
- **Tilemap**: Cartes de tuiles multi-couches
- **Animation**: Systeme d'animation sprite
- **DayNightCycle**: Cycle jour/nuit visuel

---

## Renderer

Pipeline de rendu wgpu principal.

### Structure

```rust
pub struct Renderer {
    // Configuration wgpu interne
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,

    // Pipelines
    sprite_pipeline: wgpu::RenderPipeline,

    // Ressources
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    // Buffers de rendu
    world_buffer: FrameBuffer,  // Rendu du monde
    ui_buffer: FrameBuffer,     // Rendu de l'UI

    // Etat
    width: u32,
    height: u32,
}
```

### Methodes

```rust
impl Renderer {
    /// Cree un nouveau renderer (async)
    pub async fn new(
        window: &winit::window::Window,
        width: u32,
        height: u32,
    ) -> Self;

    /// Redimensionne les buffers
    pub fn resize(&mut self, width: u32, height: u32);

    /// Retourne les dimensions
    pub fn size(&self) -> (u32, u32);

    /// Met a jour la camera pour le rendu du monde
    pub fn update_camera(&mut self, camera: &Camera2D);

    /// Met a jour la camera pour le rendu UI (identite)
    pub fn update_camera_ui(&mut self);

    /// Cree une texture depuis des bytes
    pub fn create_texture(&self, bytes: &[u8]) -> Texture;

    /// Cree une texture depuis un chemin
    pub fn load_texture(&self, path: &str) -> Result<Texture, TextureError>;

    /// Commence un frame de rendu
    pub fn begin_frame(&mut self) -> FrameContext;

    /// Termine le frame et presente
    pub fn end_frame(&mut self, frame: FrameContext);
}
```

### FrameContext

```rust
pub struct FrameContext<'a> {
    renderer: &'a mut Renderer,
    encoder: wgpu::CommandEncoder,
    frame: wgpu::SurfaceTexture,
}

impl<'a> FrameContext<'a> {
    /// Dessine un sprite
    pub fn draw_sprite(&mut self, sprite: &Sprite, texture: &Texture);

    /// Dessine un batch de sprites
    pub fn draw_batch(&mut self, batch: &SpriteBatch, texture: &Texture);

    /// Dessine une tilemap
    pub fn draw_tilemap(&mut self, tilemap: &Tilemap, tileset: &Texture);

    /// Dessine un rectangle colore
    pub fn draw_rect(&mut self, rect: Rect, color: Color);

    /// Efface avec une couleur
    pub fn clear(&mut self, color: Color);
}
```

---

## Camera2D

Camera 2D avec position, zoom et bornes.

### Structure

```rust
pub struct Camera2D {
    /// Position de la camera (centre)
    pub position: Vec2,

    /// Niveau de zoom (1.0 = normal)
    pub zoom: f32,

    /// Rotation en radians
    pub rotation: f32,

    /// Dimensions du viewport
    viewport_width: f32,
    viewport_height: f32,

    /// Bornes optionnelles (min_x, min_y, max_x, max_y)
    bounds: Option<Rect>,
}
```

### Methodes

```rust
impl Camera2D {
    /// Cree une camera centree a l'origine
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self;

    /// Definit la position
    pub fn set_position(&mut self, x: f32, y: f32);

    /// Translate la camera
    pub fn translate(&mut self, dx: f32, dy: f32);

    /// Definit le zoom (clamp entre 0.1 et 10.0)
    pub fn set_zoom(&mut self, zoom: f32);

    /// Zoom relatif (multiplie le zoom actuel)
    pub fn zoom_by(&mut self, factor: f32);

    /// Definit les bornes de la camera
    pub fn set_bounds(&mut self, bounds: Rect);

    /// Supprime les bornes
    pub fn clear_bounds(&mut self);

    /// Met a jour les dimensions du viewport
    pub fn set_viewport(&mut self, width: f32, height: f32);

    /// Applique les bornes a la position
    pub fn clamp_to_bounds(&mut self);

    /// Retourne la matrice de vue
    pub fn view_matrix(&self) -> Mat4;

    /// Retourne la matrice de projection
    pub fn projection_matrix(&self) -> Mat4;

    /// Retourne la matrice combinee (projection * view)
    pub fn view_projection_matrix(&self) -> Mat4;

    /// Convertit des coordonnees ecran en monde
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2;

    /// Convertit des coordonnees monde en ecran
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2;

    /// Retourne le rectangle visible dans le monde
    pub fn visible_bounds(&self) -> Rect;

    /// Suit une cible avec lissage
    pub fn follow(&mut self, target: Vec2, smoothing: f32, dt: f32);
}
```

### Utilisation

```rust
let mut camera = Camera2D::new(1280.0, 720.0);

// Positionner
camera.set_position(player_x, player_y);

// Bornes de la map
camera.set_bounds(Rect::new(0.0, 0.0, map_width, map_height));

// Suivi du joueur avec lissage
camera.follow(player_pos, 0.1, dt);

// Dans le rendu
renderer.update_camera(&camera);
```

---

## Sprite

Representation d'un sprite 2D.

### Structure

```rust
pub struct Sprite {
    /// Position dans le monde
    pub position: Vec2,

    /// Taille en pixels
    pub size: Vec2,

    /// Point d'origine (0.0-1.0, defaut: centre)
    pub origin: Vec2,

    /// Rotation en radians
    pub rotation: f32,

    /// Echelle
    pub scale: Vec2,

    /// Couleur de teinte (multiplicative)
    pub color: Color,

    /// Region de texture (UV)
    pub uv: Rect,

    /// Flip horizontal
    pub flip_x: bool,

    /// Flip vertical
    pub flip_y: bool,

    /// Z-order pour le tri
    pub z_order: i32,
}
```

### Methodes

```rust
impl Sprite {
    /// Cree un sprite basique
    pub fn new(position: Vec2, size: Vec2) -> Self;

    /// Definit la region UV (pour sprite sheets)
    pub fn with_uv(mut self, uv: Rect) -> Self;

    /// Definit la couleur de teinte
    pub fn with_color(mut self, color: Color) -> Self;

    /// Definit l'origine
    pub fn with_origin(mut self, origin: Vec2) -> Self;

    /// Definit la rotation
    pub fn with_rotation(mut self, rotation: f32) -> Self;

    /// Definit l'echelle
    pub fn with_scale(mut self, scale: Vec2) -> Self;

    /// Definit le z-order
    pub fn with_z_order(mut self, z: i32) -> Self;

    /// Flip horizontal
    pub fn with_flip_x(mut self, flip: bool) -> Self;

    /// Flip vertical
    pub fn with_flip_y(mut self, flip: bool) -> Self;

    /// Retourne le rectangle de collision
    pub fn bounds(&self) -> Rect;

    /// Cree les UV pour une sprite sheet
    pub fn uv_from_sheet(
        col: u32,
        row: u32,
        sprite_width: u32,
        sprite_height: u32,
        sheet_width: u32,
        sheet_height: u32,
    ) -> Rect;
}
```

### Utilisation

```rust
// Sprite simple
let sprite = Sprite::new(
    Vec2::new(100.0, 100.0),
    Vec2::new(32.0, 32.0),
);

// Sprite depuis une sprite sheet
let uv = Sprite::uv_from_sheet(2, 1, 16, 16, 256, 256);
let sprite = Sprite::new(pos, Vec2::new(16.0, 16.0))
    .with_uv(uv)
    .with_z_order(3);

// Sprite anime avec flip
let sprite = Sprite::new(pos, size)
    .with_flip_x(facing_left)
    .with_color(Color::rgba(1.0, 1.0, 1.0, 0.8));
```

---

## SpriteBatch

Groupe de sprites pour le rendu optimise (meme texture).

### Structure

```rust
pub struct SpriteBatch {
    sprites: Vec<Sprite>,
    dirty: bool,
}
```

### Methodes

```rust
impl SpriteBatch {
    /// Cree un batch vide
    pub fn new() -> Self;

    /// Cree un batch avec capacite
    pub fn with_capacity(capacity: usize) -> Self;

    /// Ajoute un sprite
    pub fn add(&mut self, sprite: Sprite);

    /// Vide le batch
    pub fn clear(&mut self);

    /// Nombre de sprites
    pub fn len(&self) -> usize;

    /// Trie par z-order
    pub fn sort_by_z(&mut self);

    /// Trie par position Y (pour le Y-sorting)
    pub fn sort_by_y(&mut self);

    /// Itere sur les sprites
    pub fn iter(&self) -> impl Iterator<Item = &Sprite>;
}
```

---

## Tilemap

Carte de tuiles multi-couches.

### Structure

```rust
pub struct Tilemap {
    /// Largeur en tuiles
    width: u32,

    /// Hauteur en tuiles
    height: u32,

    /// Taille d'une tuile en pixels
    tile_size: u32,

    /// Couches de tuiles
    layers: Vec<TileLayer>,

    /// Donnees de collision
    collision: Vec<bool>,
}

pub struct TileLayer {
    /// Nom de la couche
    pub name: String,

    /// Donnees des tuiles (0 = vide)
    pub tiles: Vec<u32>,

    /// Visible?
    pub visible: bool,

    /// Z-order
    pub z_order: i32,
}
```

### Methodes

```rust
impl Tilemap {
    /// Cree une tilemap vide
    pub fn new(width: u32, height: u32, tile_size: u32) -> Self;

    /// Charge depuis un fichier JSON
    pub fn load_from_file(path: &str) -> Result<Self, TilemapError>;

    /// Charge depuis une chaine JSON
    pub fn load_from_str(json: &str) -> Result<Self, TilemapError>;

    // Getters
    pub fn width(&self) -> u32;
    pub fn height(&self) -> u32;
    pub fn tile_size(&self) -> u32;
    pub fn pixel_width(&self) -> u32;
    pub fn pixel_height(&self) -> u32;

    /// Ajoute une couche
    pub fn add_layer(&mut self, layer: TileLayer);

    /// Retourne une couche par nom
    pub fn get_layer(&self, name: &str) -> Option<&TileLayer>;

    /// Retourne une couche mutable
    pub fn get_layer_mut(&mut self, name: &str) -> Option<&mut TileLayer>;

    /// Definit une tuile dans une couche
    pub fn set_tile(&mut self, layer: &str, x: u32, y: u32, tile: u32);

    /// Retourne une tuile
    pub fn get_tile(&self, layer: &str, x: u32, y: u32) -> Option<u32>;

    /// Verifie si une position est solide
    pub fn is_solid(&self, x: u32, y: u32) -> bool;

    /// Definit la collision
    pub fn set_collision(&mut self, x: u32, y: u32, solid: bool);

    /// Convertit pixel -> tile
    pub fn pixel_to_tile(&self, x: f32, y: f32) -> (u32, u32);

    /// Convertit tile -> pixel (coin superieur gauche)
    pub fn tile_to_pixel(&self, x: u32, y: u32) -> (f32, f32);

    /// Itere sur les couches visibles
    pub fn visible_layers(&self) -> impl Iterator<Item = &TileLayer>;
}
```

### Format JSON

```json
{
    "width": 20,
    "height": 15,
    "tile_size": 16,
    "layers": [
        {
            "name": "ground",
            "z_order": 0,
            "visible": true,
            "tiles": [1, 1, 1, 2, 2, ...]
        },
        {
            "name": "objects",
            "z_order": 2,
            "visible": true,
            "tiles": [0, 0, 5, 0, 6, ...]
        }
    ],
    "collision": [false, false, true, true, ...]
}
```

---

## Animation

Systeme d'animation par frames.

### Structure

```rust
pub struct Animation {
    /// Frames de l'animation
    frames: Vec<AnimationFrame>,

    /// Duree totale
    duration: f32,

    /// Animation en boucle?
    looping: bool,
}

pub struct AnimationFrame {
    /// Region UV de la frame
    pub uv: Rect,

    /// Duree de cette frame
    pub duration: f32,
}

pub struct AnimationController {
    /// Animations disponibles
    animations: HashMap<String, Animation>,

    /// Animation courante
    current: Option<String>,

    /// Temps ecoule
    time: f32,

    /// En pause?
    paused: bool,
}
```

### Methodes

```rust
impl Animation {
    /// Cree une animation depuis des frames uniformes
    pub fn from_strip(
        start_col: u32,
        row: u32,
        frame_count: u32,
        frame_width: u32,
        frame_height: u32,
        sheet_width: u32,
        sheet_height: u32,
        frame_duration: f32,
        looping: bool,
    ) -> Self;

    /// Cree une animation personnalisee
    pub fn new(frames: Vec<AnimationFrame>, looping: bool) -> Self;

    /// Retourne la frame a un temps donne
    pub fn frame_at(&self, time: f32) -> &AnimationFrame;

    /// Animation terminee?
    pub fn is_finished(&self, time: f32) -> bool;
}

impl AnimationController {
    /// Cree un controleur vide
    pub fn new() -> Self;

    /// Ajoute une animation
    pub fn add(&mut self, name: &str, animation: Animation);

    /// Joue une animation
    pub fn play(&mut self, name: &str);

    /// Met a jour le controleur
    pub fn update(&mut self, dt: f32);

    /// Retourne l'UV courante
    pub fn current_uv(&self) -> Option<Rect>;

    /// Animation courante terminee?
    pub fn is_finished(&self) -> bool;

    /// Pause/Resume
    pub fn pause(&mut self);
    pub fn resume(&mut self);

    /// Nom de l'animation courante
    pub fn current_animation(&self) -> Option<&str>;
}
```

### Utilisation

```rust
let mut controller = AnimationController::new();

// Animation de marche (4 frames)
controller.add("walk_down", Animation::from_strip(
    0, 0,          // col, row de depart
    4,             // nombre de frames
    16, 24,        // taille d'une frame
    64, 96,        // taille de la sheet
    0.15,          // duree par frame
    true,          // boucle
));

controller.add("walk_up", Animation::from_strip(0, 1, 4, 16, 24, 64, 96, 0.15, true));
controller.add("idle", Animation::from_strip(0, 2, 2, 16, 24, 64, 96, 0.5, true));

// Utilisation
controller.play("walk_down");
controller.update(dt);

if let Some(uv) = controller.current_uv() {
    sprite.uv = uv;
}
```

---

## DayNightCycle

Cycle jour/nuit visuel.

### Structure

```rust
pub struct DayNightCycle {
    /// Couleurs pour chaque periode
    colors: DayNightColors,

    /// Couleur ambiante actuelle
    current_color: Color,
}

pub struct DayNightColors {
    pub dawn: Color,      // 5h-7h
    pub morning: Color,   // 7h-10h
    pub noon: Color,      // 10h-16h
    pub evening: Color,   // 16h-19h
    pub dusk: Color,      // 19h-21h
    pub night: Color,     // 21h-5h
}
```

### Methodes

```rust
impl DayNightCycle {
    /// Cree avec couleurs par defaut
    pub fn new() -> Self;

    /// Cree avec couleurs personnalisees
    pub fn with_colors(colors: DayNightColors) -> Self;

    /// Met a jour selon l'heure (0.0 - 24.0)
    pub fn update(&mut self, hour: f32);

    /// Retourne la couleur ambiante
    pub fn ambient_color(&self) -> Color;

    /// Retourne le multiplicateur de luminosite (0.0 - 1.0)
    pub fn brightness(&self) -> f32;
}
```

### Utilisation

```rust
let mut day_night = DayNightCycle::new();

// Dans la boucle
day_night.update(game_clock.hour_decimal());

// Appliquer au rendu
renderer.set_ambient_color(day_night.ambient_color());
```

---

## Types utilitaires

### Color

```rust
#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    // ...

    pub fn rgb(r: f32, g: f32, b: f32) -> Self;
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self;
    pub fn from_hex(hex: u32) -> Self;
    pub fn lerp(self, other: Color, t: f32) -> Self;
}
```

### Rect

```rust
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self;
    pub fn from_corners(min: Vec2, max: Vec2) -> Self;
    pub fn contains(&self, point: Vec2) -> bool;
    pub fn intersects(&self, other: &Rect) -> bool;
    pub fn center(&self) -> Vec2;
    pub fn min(&self) -> Vec2;
    pub fn max(&self) -> Vec2;
}
```

---

## Z-Order Layers

| Z | Layer | Description |
|---|-------|-------------|
| 0 | Ground | Tuiles de sol |
| 1 | Ground Decor | Decorations au sol |
| 2 | Shadows | Ombres des entites |
| 3 | Entities | Joueur, NPCs (Y-sorted) |
| 4 | Above Entities | Effets, bulles |
| 5 | Weather/UI | Meteo, interface |
