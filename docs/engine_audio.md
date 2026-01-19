# engine_audio

Module audio base sur rodio pour la musique et les effets sonores.

## Vue d'ensemble

`engine_audio` fournit:
- **AudioManager**: Gestionnaire centralise de l'audio
- **SoundEffect**: Effets sonores courts
- **Music**: Musique de fond avec controles

---

## AudioManager

Gestionnaire centralise de l'audio.

### Structure

```rust
pub struct AudioManager {
    // Stream audio rodio
    stream: rodio::OutputStream,
    stream_handle: rodio::OutputStreamHandle,

    // Canaux
    music_sink: Option<rodio::Sink>,
    sfx_sinks: Vec<rodio::Sink>,

    // Volumes
    master_volume: f32,
    music_volume: f32,
    sfx_volume: f32,

    // Etat
    music_paused: bool,
}
```

### Methodes

```rust
impl AudioManager {
    /// Cree un nouveau gestionnaire audio
    pub fn new() -> Result<Self, AudioError>;

    /// Verifie si l'audio est disponible
    pub fn is_available(&self) -> bool;
}
```

### Musique

```rust
impl AudioManager {
    /// Joue une musique (remplace l'actuelle)
    pub fn play_music(&mut self, path: &str) -> Result<(), AudioError>;

    /// Joue une musique en boucle
    pub fn play_music_looped(&mut self, path: &str) -> Result<(), AudioError>;

    /// Arrete la musique
    pub fn stop_music(&mut self);

    /// Pause la musique
    pub fn pause_music(&mut self);

    /// Resume la musique
    pub fn resume_music(&mut self);

    /// Musique en cours?
    pub fn is_music_playing(&self) -> bool;

    /// Musique en pause?
    pub fn is_music_paused(&self) -> bool;

    /// Fondu sortant de la musique
    pub fn fade_out_music(&mut self, duration: f32);

    /// Transition vers une nouvelle musique
    pub fn crossfade_music(&mut self, path: &str, duration: f32) -> Result<(), AudioError>;
}
```

### Effets sonores

```rust
impl AudioManager {
    /// Joue un effet sonore
    pub fn play_sfx(&mut self, path: &str) -> Result<(), AudioError>;

    /// Joue un effet sonore avec volume specifique
    pub fn play_sfx_with_volume(&mut self, path: &str, volume: f32) -> Result<(), AudioError>;

    /// Joue un effet sonore spatial (volume base sur distance)
    pub fn play_sfx_at(
        &mut self,
        path: &str,
        position: Vec2,
        listener: Vec2,
        max_distance: f32,
    ) -> Result<(), AudioError>;

    /// Arrete tous les effets sonores
    pub fn stop_all_sfx(&mut self);
}
```

### Volumes

```rust
impl AudioManager {
    /// Volume general (0.0 - 1.0)
    pub fn set_master_volume(&mut self, volume: f32);
    pub fn master_volume(&self) -> f32;

    /// Volume de la musique (0.0 - 1.0)
    pub fn set_music_volume(&mut self, volume: f32);
    pub fn music_volume(&self) -> f32;

    /// Volume des effets (0.0 - 1.0)
    pub fn set_sfx_volume(&mut self, volume: f32);
    pub fn sfx_volume(&self) -> f32;

    /// Mute/unmute
    pub fn mute(&mut self);
    pub fn unmute(&mut self);
    pub fn is_muted(&self) -> bool;
    pub fn toggle_mute(&mut self);
}
```

### Nettoyage

```rust
impl AudioManager {
    /// Nettoie les sinks termines
    pub fn cleanup(&mut self);

    /// Arrete tout l'audio
    pub fn stop_all(&mut self);
}
```

---

## Formats supportes

| Format | Extension | Usage typique |
|--------|-----------|---------------|
| OGG Vorbis | .ogg | Musique (compresse) |
| WAV | .wav | Effets courts (non compresse) |
| FLAC | .flac | Musique haute qualite |
| MP3 | .mp3 | Musique (compresse) |

---

## AudioError

```rust
#[derive(Debug)]
pub enum AudioError {
    /// Impossible d'initialiser l'audio
    InitError(String),

    /// Fichier non trouve
    FileNotFound(String),

    /// Format non supporte
    UnsupportedFormat(String),

    /// Erreur de decodage
    DecodeError(String),

    /// Erreur de lecture
    PlaybackError(String),
}
```

---

## Utilisation complete

```rust
use engine_audio::AudioManager;

fn main() {
    // Initialiser l'audio
    let mut audio = match AudioManager::new() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Audio indisponible: {:?}", e);
            return;
        }
    };

    // Configurer les volumes
    audio.set_master_volume(1.0);
    audio.set_music_volume(0.7);
    audio.set_sfx_volume(0.8);

    // Jouer la musique du menu
    audio.play_music_looped("assets/audio/music/menu_theme.ogg").ok();

    // Dans le jeu
    fn on_game_start(audio: &mut AudioManager) {
        // Transition vers la musique de jeu
        audio.crossfade_music("assets/audio/music/farm_theme.ogg", 2.0).ok();
    }

    fn on_item_pickup(audio: &mut AudioManager) {
        audio.play_sfx("assets/audio/sfx/pickup.wav").ok();
    }

    fn on_footstep(audio: &mut AudioManager, player_pos: Vec2, listener_pos: Vec2) {
        audio.play_sfx_at(
            "assets/audio/sfx/footstep.wav",
            player_pos,
            listener_pos,
            200.0,  // Distance max
        ).ok();
    }

    fn on_pause(audio: &mut AudioManager) {
        audio.pause_music();
    }

    fn on_resume(audio: &mut AudioManager) {
        audio.resume_music();
    }

    // Nettoyage periodique
    fn update(audio: &mut AudioManager) {
        audio.cleanup();  // Nettoie les sinks termines
    }
}
```

---

## Integration avec les settings

```rust
fn apply_audio_settings(audio: &mut AudioManager, settings: &GameSettings) {
    audio.set_master_volume(settings.master_volume);
    audio.set_music_volume(settings.music_volume);
    audio.set_sfx_volume(settings.sfx_volume);
}

fn on_settings_changed(audio: &mut AudioManager, settings: &GameSettings) {
    apply_audio_settings(audio, settings);

    // Sauvegarder
    settings.save("settings.json").ok();
}
```

---

## Audio spatial simplifie

```rust
/// Calcule le volume base sur la distance
fn calculate_spatial_volume(
    source: Vec2,
    listener: Vec2,
    max_distance: f32,
) -> f32 {
    let distance = source.distance(listener);

    if distance >= max_distance {
        0.0
    } else {
        // Attenuation lineaire
        1.0 - (distance / max_distance)
    }
}

/// Version avec falloff quadratique (plus realiste)
fn calculate_spatial_volume_quadratic(
    source: Vec2,
    listener: Vec2,
    max_distance: f32,
) -> f32 {
    let distance = source.distance(listener);

    if distance >= max_distance {
        0.0
    } else {
        let t = 1.0 - (distance / max_distance);
        t * t  // Quadratique
    }
}
```

---

## Bonnes pratiques

1. **Precharger les SFX frequents**: Les sons courts peuvent etre mis en cache
2. **Utiliser OGG pour la musique**: Bon compromis qualite/taille
3. **Utiliser WAV pour les SFX**: Pas de latence de decodage
4. **Nettoyer regulierement**: Appeler `cleanup()` pour liberer les ressources
5. **Gerer les erreurs**: L'audio peut echouer (pas de peripherique)
6. **Fade transitions**: Utiliser crossfade pour les changements de musique
