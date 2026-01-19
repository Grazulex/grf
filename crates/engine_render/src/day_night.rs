//! Day/Night visual cycle
//!
//! Provides ambient colors based on the time of day for visual effects.

use glam::Vec3;

/// Color represented as RGB floats (0.0 to 1.0)
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    /// Create a new color from RGB values (0.0 to 1.0)
    #[must_use]
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Create color from hex value (e.g., 0xFFA07A)
    #[must_use]
    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as f32 / 255.0,
            g: ((hex >> 8) & 0xFF) as f32 / 255.0,
            b: (hex & 0xFF) as f32 / 255.0,
        }
    }

    /// Linear interpolation between two colors
    #[must_use]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
        }
    }

    /// Convert to wgpu Color
    #[must_use]
    pub fn to_wgpu(self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: 1.0,
        }
    }

    /// Convert to Vec3 for shader uniforms
    #[must_use]
    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }

    /// White color
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);
}

/// Day/night cycle configuration
#[derive(Debug, Clone)]
pub struct DayNightCycle {
    /// Color at dawn (6:00)
    pub dawn_color: Color,
    /// Color at noon (12:00)
    pub noon_color: Color,
    /// Color at dusk (18:00)
    pub dusk_color: Color,
    /// Color at midnight (0:00)
    pub midnight_color: Color,
}

impl Default for DayNightCycle {
    fn default() -> Self {
        Self::new()
    }
}

impl DayNightCycle {
    /// Create a new day/night cycle with default colors
    #[must_use]
    pub fn new() -> Self {
        Self {
            // Warm orange-pink dawn
            dawn_color: Color::from_hex(0xFFB07A),
            // Pure white noon
            noon_color: Color::WHITE,
            // Orange-coral dusk
            dusk_color: Color::from_hex(0xFF7F50),
            // Dark blue midnight
            midnight_color: Color::from_hex(0x1A1A3A),
        }
    }

    /// Get the ambient color for a given hour (0-23) and minute (0-59)
    #[must_use]
    pub fn get_ambient_color(&self, hour: u32, minute: u32) -> Color {
        let hour_f = hour as f32 + minute as f32 / 60.0;

        // Define key times
        const DAWN: f32 = 6.0;
        const NOON: f32 = 12.0;
        const DUSK: f32 = 18.0;
        const MIDNIGHT: f32 = 24.0; // Also 0.0

        if hour_f < DAWN {
            // Midnight to Dawn (0:00 - 6:00)
            let t = hour_f / DAWN;
            self.midnight_color.lerp(self.dawn_color, t)
        } else if hour_f < NOON {
            // Dawn to Noon (6:00 - 12:00)
            let t = (hour_f - DAWN) / (NOON - DAWN);
            self.dawn_color.lerp(self.noon_color, t)
        } else if hour_f < DUSK {
            // Noon to Dusk (12:00 - 18:00)
            let t = (hour_f - NOON) / (DUSK - NOON);
            self.noon_color.lerp(self.dusk_color, t)
        } else {
            // Dusk to Midnight (18:00 - 24:00)
            let t = (hour_f - DUSK) / (MIDNIGHT - DUSK);
            self.dusk_color.lerp(self.midnight_color, t)
        }
    }

    /// Get ambient color from a GameClock-compatible interface
    /// Takes hour (0-23) and minute (0-59)
    #[must_use]
    pub fn ambient_from_clock(&self, hour: u32, minute: u32) -> Color {
        self.get_ambient_color(hour, minute)
    }

    /// Get a tinted clear color (darker version of ambient for background)
    #[must_use]
    pub fn get_clear_color(&self, hour: u32, minute: u32) -> Color {
        let ambient = self.get_ambient_color(hour, minute);
        // Darken the ambient color for background
        Color::new(ambient.r * 0.15, ambient.g * 0.15, ambient.b * 0.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_lerp() {
        let black = Color::new(0.0, 0.0, 0.0);
        let white = Color::WHITE;

        let mid = black.lerp(white, 0.5);
        assert!((mid.r - 0.5).abs() < 0.01);
        assert!((mid.g - 0.5).abs() < 0.01);
        assert!((mid.b - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex(0xFF0000);
        assert!((color.r - 1.0).abs() < 0.01);
        assert!(color.g < 0.01);
        assert!(color.b < 0.01);
    }

    #[test]
    fn test_day_night_noon() {
        let cycle = DayNightCycle::new();
        let noon = cycle.get_ambient_color(12, 0);

        // Noon should be white
        assert!((noon.r - 1.0).abs() < 0.01);
        assert!((noon.g - 1.0).abs() < 0.01);
        assert!((noon.b - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_day_night_midnight() {
        let cycle = DayNightCycle::new();
        let midnight = cycle.get_ambient_color(0, 0);

        // Midnight should be dark blue
        assert!(midnight.r < 0.2);
        assert!(midnight.g < 0.2);
        assert!(midnight.b < 0.3);
    }

    #[test]
    fn test_day_night_transitions() {
        let cycle = DayNightCycle::new();

        // Should smoothly transition through the day
        let dawn = cycle.get_ambient_color(6, 0);
        let mid_morning = cycle.get_ambient_color(9, 0);
        let noon = cycle.get_ambient_color(12, 0);

        // Mid-morning should be between dawn and noon
        assert!(mid_morning.r > dawn.r || (dawn.r - mid_morning.r).abs() < 0.2);
        assert!(mid_morning.r < noon.r || (noon.r - mid_morning.r).abs() < 0.2);
    }
}
