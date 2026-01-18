//! Engine Audio - Audio playback system
//!
//! This crate provides audio management for sound effects and music
//! using rodio for cross-platform audio.
//!
//! # Example
//! ```ignore
//! let mut audio = AudioManager::new()?;
//! audio.load_sfx("assets/audio/jump.wav", "jump")?;
//! audio.play_sfx("jump");
//! audio.play_music("assets/audio/music.ogg")?;
//! ```

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

use log::{error, info, warn};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

/// Default master volume (0.0 to 1.0)
pub const DEFAULT_MASTER_VOLUME: f32 = 1.0;
/// Default music volume (0.0 to 1.0)
pub const DEFAULT_MUSIC_VOLUME: f32 = 0.7;
/// Default SFX volume (0.0 to 1.0)
pub const DEFAULT_SFX_VOLUME: f32 = 1.0;

/// Audio system error
#[derive(Debug)]
pub enum AudioError {
    /// Failed to initialize audio output
    OutputError(String),
    /// Failed to load audio file
    LoadError(String),
    /// Audio file not found
    NotFound(String),
    /// Decoder error
    DecodeError(String),
}

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutputError(msg) => write!(f, "Audio output error: {}", msg),
            Self::LoadError(msg) => write!(f, "Failed to load audio: {}", msg),
            Self::NotFound(msg) => write!(f, "Audio file not found: {}", msg),
            Self::DecodeError(msg) => write!(f, "Decoder error: {}", msg),
        }
    }
}

impl std::error::Error for AudioError {}

/// Cached sound effect data
struct SfxData {
    /// Raw audio data
    data: Arc<Vec<u8>>,
}

/// Audio manager for playing sound effects and music
pub struct AudioManager {
    /// Audio output stream (must be kept alive)
    _stream: OutputStream,
    /// Stream handle for creating sinks
    stream_handle: OutputStreamHandle,
    /// Music sink (controllable playback)
    music_sink: Option<Sink>,
    /// Cached sound effects
    sfx_cache: HashMap<String, SfxData>,
    /// Master volume
    master_volume: f32,
    /// Music volume
    music_volume: f32,
    /// SFX volume
    sfx_volume: f32,
}

impl AudioManager {
    /// Create a new audio manager
    ///
    /// # Errors
    /// Returns an error if the audio output cannot be initialized
    pub fn new() -> Result<Self, AudioError> {
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| AudioError::OutputError(e.to_string()))?;

        info!("Audio system initialized");

        Ok(Self {
            _stream: stream,
            stream_handle,
            music_sink: None,
            sfx_cache: HashMap::new(),
            master_volume: DEFAULT_MASTER_VOLUME,
            music_volume: DEFAULT_MUSIC_VOLUME,
            sfx_volume: DEFAULT_SFX_VOLUME,
        })
    }

    // ========== Volume Controls ==========

    /// Set the master volume (affects all audio)
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
        self.update_music_volume();
    }

    /// Get the current master volume
    #[must_use]
    pub fn master_volume(&self) -> f32 {
        self.master_volume
    }

    /// Set the music volume
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
        self.update_music_volume();
    }

    /// Get the current music volume
    #[must_use]
    pub fn music_volume(&self) -> f32 {
        self.music_volume
    }

    /// Set the SFX volume
    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }

    /// Get the current SFX volume
    #[must_use]
    pub fn sfx_volume(&self) -> f32 {
        self.sfx_volume
    }

    /// Calculate effective music volume (master * music)
    fn effective_music_volume(&self) -> f32 {
        self.master_volume * self.music_volume
    }

    /// Calculate effective SFX volume (master * sfx)
    fn effective_sfx_volume(&self) -> f32 {
        self.master_volume * self.sfx_volume
    }

    /// Update music sink volume
    fn update_music_volume(&self) {
        if let Some(sink) = &self.music_sink {
            sink.set_volume(self.effective_music_volume());
        }
    }

    // ========== Music Controls ==========

    /// Play music from a file path (replaces current music)
    ///
    /// # Errors
    /// Returns an error if the file cannot be loaded or decoded
    pub fn play_music(&mut self, path: impl AsRef<Path>) -> Result<(), AudioError> {
        let path = path.as_ref();

        // Stop current music
        self.stop_music();

        // Open and decode the file
        let file = File::open(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AudioError::NotFound(path.display().to_string())
            } else {
                AudioError::LoadError(e.to_string())
            }
        })?;

        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| AudioError::DecodeError(e.to_string()))?;

        // Create sink and play
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| AudioError::OutputError(e.to_string()))?;

        sink.set_volume(self.effective_music_volume());
        sink.append(source.repeat_infinite());

        self.music_sink = Some(sink);

        info!("Playing music: {}", path.display());
        Ok(())
    }

    /// Pause the current music
    pub fn pause_music(&self) {
        if let Some(sink) = &self.music_sink {
            sink.pause();
        }
    }

    /// Resume the current music
    pub fn resume_music(&self) {
        if let Some(sink) = &self.music_sink {
            sink.play();
        }
    }

    /// Stop the current music
    pub fn stop_music(&mut self) {
        if let Some(sink) = self.music_sink.take() {
            sink.stop();
        }
    }

    /// Check if music is currently playing
    #[must_use]
    pub fn is_music_playing(&self) -> bool {
        self.music_sink
            .as_ref()
            .map(|s| !s.is_paused() && !s.empty())
            .unwrap_or(false)
    }

    /// Check if music is paused
    #[must_use]
    pub fn is_music_paused(&self) -> bool {
        self.music_sink
            .as_ref()
            .map(|s| s.is_paused())
            .unwrap_or(false)
    }

    // ========== SFX Controls ==========

    /// Load a sound effect into the cache
    ///
    /// # Errors
    /// Returns an error if the file cannot be loaded
    pub fn load_sfx(&mut self, path: impl AsRef<Path>, name: &str) -> Result<(), AudioError> {
        let path = path.as_ref();

        // Read file data
        let data = std::fs::read(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AudioError::NotFound(path.display().to_string())
            } else {
                AudioError::LoadError(e.to_string())
            }
        })?;

        self.sfx_cache.insert(
            name.to_string(),
            SfxData {
                data: Arc::new(data),
            },
        );

        info!("Loaded SFX '{}' from {}", name, path.display());
        Ok(())
    }

    /// Play a cached sound effect
    pub fn play_sfx(&self, name: &str) {
        let Some(sfx) = self.sfx_cache.get(name) else {
            warn!("SFX '{}' not loaded", name);
            return;
        };

        // Create cursor from the cached data
        let cursor = std::io::Cursor::new(sfx.data.as_ref().clone());

        // Decode the audio
        let source = match Decoder::new(cursor) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to decode SFX '{}': {}", name, e);
                return;
            }
        };

        // Play with volume adjustment
        let volume = self.effective_sfx_volume();

        if let Err(e) = self
            .stream_handle
            .play_raw(source.convert_samples().amplify(volume))
        {
            error!("Failed to play SFX '{}': {}", name, e);
        }
    }

    /// Play a one-shot sound effect directly from a file
    ///
    /// Note: For frequently played sounds, use `load_sfx` and `play_sfx` instead
    pub fn play_sfx_file(&self, path: impl AsRef<Path>) {
        let path = path.as_ref();

        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                error!("Failed to open SFX file {}: {}", path.display(), e);
                return;
            }
        };

        let source = match Decoder::new(BufReader::new(file)) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to decode SFX {}: {}", path.display(), e);
                return;
            }
        };

        let volume = self.effective_sfx_volume();

        if let Err(e) = self
            .stream_handle
            .play_raw(source.convert_samples().amplify(volume))
        {
            error!("Failed to play SFX {}: {}", path.display(), e);
        }
    }

    /// Check if a sound effect is loaded
    #[must_use]
    pub fn has_sfx(&self, name: &str) -> bool {
        self.sfx_cache.contains_key(name)
    }

    /// Unload a sound effect from the cache
    pub fn unload_sfx(&mut self, name: &str) {
        self.sfx_cache.remove(name);
    }

    /// Clear all cached sound effects
    pub fn clear_sfx_cache(&mut self) {
        self.sfx_cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Audio tests are limited since they require audio hardware
    // These tests only verify the non-audio logic

    #[test]
    fn test_volume_clamping() {
        // We can't create AudioManager in tests without audio hardware
        // but we can test the volume clamping logic
        let volume = 1.5f32.clamp(0.0, 1.0);
        assert_eq!(volume, 1.0);

        let volume = (-0.5f32).clamp(0.0, 1.0);
        assert_eq!(volume, 0.0);
    }

    #[test]
    fn test_default_volumes() {
        assert_eq!(DEFAULT_MASTER_VOLUME, 1.0);
        assert_eq!(DEFAULT_MUSIC_VOLUME, 0.7);
        assert_eq!(DEFAULT_SFX_VOLUME, 1.0);
    }
}
