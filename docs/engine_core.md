# engine_core

Module central du framework gerant le temps, l'horloge in-game et les parametres du jeu.

## Vue d'ensemble

`engine_core` fournit les fondations temporelles du jeu:
- **GameTime**: Gestion du temps reel (delta time, temps total)
- **GameClock**: Horloge in-game (heures, jours, saisons)
- **GameSettings**: Parametres persistants du jeu

## GameTime

Gere le temps reel de l'application avec support pour le fixed timestep.

### Constantes

```rust
/// Timestep fixe (60 updates par seconde)
pub const FIXED_TIMESTEP: f32 = 1.0 / 60.0;

/// Delta time maximum pour eviter les sauts
pub const MAX_DELTA_TIME: f32 = 0.25;
```

### Structure

```rust
pub struct GameTime {
    /// Temps total depuis le demarrage (secondes)
    pub total_time: f32,

    /// Delta time du frame actuel
    pub delta_time: f32,

    /// Accumulateur pour le fixed timestep
    pub accumulator: f32,

    /// Nombre de fixed updates ce frame
    pub fixed_updates_this_frame: u32,

    /// Alpha d'interpolation pour le rendu (0.0 - 1.0)
    pub alpha: f32,
}
```

### Methodes

```rust
impl GameTime {
    /// Cree une nouvelle instance
    pub fn new() -> Self;

    /// Met a jour avec le delta time brut
    /// Retourne le nombre de fixed updates a effectuer
    pub fn update(&mut self, raw_dt: f32) -> u32;

    /// Consomme un fixed timestep de l'accumulateur
    /// Retourne true si un update doit etre effectue
    pub fn consume_fixed_timestep(&mut self) -> bool;

    /// Calcule l'alpha d'interpolation pour le rendu
    pub fn calculate_alpha(&mut self);
}
```

### Utilisation

```rust
let mut game_time = GameTime::new();

// Dans la boucle principale
let raw_dt = /* temps depuis le dernier frame */;
let updates = game_time.update(raw_dt);

// Fixed updates
while game_time.consume_fixed_timestep() {
    // Logique a 60 UPS
    physics_update(FIXED_TIMESTEP);
}

// Interpolation pour le rendu
game_time.calculate_alpha();
render_with_interpolation(game_time.alpha);
```

---

## GameClock

Horloge in-game simulant le passage du temps (heures, jours, saisons).

### Constantes

```rust
/// Duree d'une heure in-game en secondes reelles
pub const SECONDS_PER_GAME_HOUR: f32 = 60.0;

/// Heures dans une journee
pub const HOURS_PER_DAY: u32 = 24;

/// Jours dans une saison
pub const DAYS_PER_SEASON: u32 = 28;

/// Nombre de saisons
pub const SEASONS_COUNT: u32 = 4;
```

### Enumerations

```rust
/// Saisons du jeu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Season {
    Spring = 0,
    Summer = 1,
    Fall = 2,
    Winter = 3,
}

impl Season {
    /// Saison suivante (cyclique)
    pub fn next(self) -> Self;

    /// Nom de la saison
    pub fn name(self) -> &'static str;
}
```

### Structure

```rust
pub struct GameClock {
    /// Heure actuelle (0-23)
    hour: u32,

    /// Minutes actuelles (0-59)
    minute: u32,

    /// Jour actuel (1-28)
    day: u32,

    /// Saison actuelle
    season: Season,

    /// Annee actuelle (commence a 1)
    year: u32,

    /// Accumulateur de temps (secondes)
    time_accumulator: f32,

    /// Vitesse de l'horloge (multiplicateur)
    time_scale: f32,

    /// Horloge en pause?
    paused: bool,
}
```

### Methodes

```rust
impl GameClock {
    /// Cree une nouvelle horloge (Jour 1, Printemps, Annee 1, 6h00)
    pub fn new() -> Self;

    /// Met a jour l'horloge avec le delta time
    pub fn update(&mut self, dt: f32);

    // Getters
    pub fn hour(&self) -> u32;
    pub fn minute(&self) -> u32;
    pub fn day(&self) -> u32;
    pub fn season(&self) -> Season;
    pub fn year(&self) -> u32;

    /// Heure decimale (ex: 14.5 pour 14h30)
    pub fn hour_decimal(&self) -> f32;

    /// Progression dans la journee (0.0 = minuit, 1.0 = 23h59)
    pub fn day_progress(&self) -> f32;

    /// Progression dans la saison (0.0 - 1.0)
    pub fn season_progress(&self) -> f32;

    /// Est-ce le jour? (6h - 20h)
    pub fn is_daytime(&self) -> bool;

    /// Est-ce la nuit? (20h - 6h)
    pub fn is_nighttime(&self) -> bool;

    /// Temps formate "HH:MM"
    pub fn time_string(&self) -> String;

    /// Date formatee "Jour X, Saison, Annee Y"
    pub fn date_string(&self) -> String;

    // Controles
    pub fn set_time_scale(&mut self, scale: f32);
    pub fn time_scale(&self) -> f32;
    pub fn pause(&mut self);
    pub fn resume(&mut self);
    pub fn is_paused(&self) -> bool;
    pub fn toggle_pause(&mut self);

    /// Avance au jour suivant
    pub fn skip_to_next_day(&mut self);

    /// Definit l'heure directement
    pub fn set_time(&mut self, hour: u32, minute: u32);
}
```

### Utilisation

```rust
let mut clock = GameClock::new();

// Dans la boucle de jeu
clock.update(dt);

// Affichage
println!("{} - {}", clock.time_string(), clock.date_string());
// "14:30 - Day 5, Spring, Year 1"

// Verifications
if clock.is_nighttime() {
    // Faire apparaitre des monstres
}

if clock.hour() >= 2 && clock.hour() < 6 {
    // Forcer le joueur a dormir
}

// Controle du temps
clock.set_time_scale(2.0);  // Temps 2x plus rapide
clock.pause();              // Pause pendant un menu
```

---

## GameSettings

Parametres persistants du jeu avec sauvegarde/chargement JSON.

### Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    /// Volume de la musique (0.0 - 1.0)
    pub music_volume: f32,

    /// Volume des effets sonores (0.0 - 1.0)
    pub sfx_volume: f32,

    /// Volume general (0.0 - 1.0)
    pub master_volume: f32,

    /// Plein ecran?
    pub fullscreen: bool,

    /// VSync active?
    pub vsync: bool,

    /// Afficher les FPS?
    pub show_fps: bool,
}
```

### Methodes

```rust
impl GameSettings {
    /// Charge les parametres depuis un fichier
    /// Retourne les valeurs par defaut si le fichier n'existe pas
    pub fn load(path: &str) -> Self;

    /// Sauvegarde les parametres dans un fichier
    pub fn save(&self, path: &str) -> Result<(), std::io::Error>;

    /// Valeurs par defaut
    pub fn default() -> Self;
}
```

### Valeurs par defaut

```rust
GameSettings {
    music_volume: 0.7,
    sfx_volume: 0.8,
    master_volume: 1.0,
    fullscreen: false,
    vsync: true,
    show_fps: false,
}
```

### Utilisation

```rust
// Chargement au demarrage
let settings = GameSettings::load("settings.json");

// Modification
settings.music_volume = 0.5;
settings.fullscreen = true;

// Sauvegarde
settings.save("settings.json")?;
```

---

## Re-exports

```rust
pub use game_time::{GameTime, FIXED_TIMESTEP, MAX_DELTA_TIME};
pub use game_clock::{GameClock, Season, SECONDS_PER_GAME_HOUR, HOURS_PER_DAY, DAYS_PER_SEASON};
pub use settings::GameSettings;
```

---

## Exemples

### Boucle de jeu complete

```rust
use engine_core::{GameTime, GameClock, FIXED_TIMESTEP};

fn main() {
    let mut game_time = GameTime::new();
    let mut game_clock = GameClock::new();

    loop {
        let raw_dt = get_frame_delta();
        game_time.update(raw_dt);

        // Fixed updates (logique)
        while game_time.consume_fixed_timestep() {
            game_clock.update(FIXED_TIMESTEP);
            update_game_logic(FIXED_TIMESTEP);
        }

        // Rendu avec interpolation
        game_time.calculate_alpha();
        render(game_time.alpha);
    }
}
```

### Systeme jour/nuit

```rust
use engine_core::GameClock;

fn update_lighting(clock: &GameClock) {
    let progress = clock.day_progress();

    // Calculer la luminosite basee sur l'heure
    let brightness = if clock.is_daytime() {
        1.0
    } else {
        0.3 + 0.2 * (progress * std::f32::consts::PI).sin()
    };

    set_ambient_light(brightness);
}
```
