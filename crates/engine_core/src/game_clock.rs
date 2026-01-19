//! In-game time system for farming/RPG games
//!
//! Provides a game clock with hours, days, seasons, and years.
//! Time flows at a configurable rate relative to real time.

use serde::{Deserialize, Serialize};

/// Minutes per hour
pub const MINUTES_PER_HOUR: u32 = 60;
/// Hours per day
pub const HOURS_PER_DAY: u32 = 24;
/// Days per season (like Stardew Valley)
pub const DAYS_PER_SEASON: u32 = 28;
/// Seasons per year
pub const SEASONS_PER_YEAR: u32 = 4;

/// Default real seconds per in-game minute
pub const DEFAULT_SECONDS_PER_MINUTE: f32 = 1.0;

/// Season of the year
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

impl Season {
    /// Get the next season
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Spring => Self::Summer,
            Self::Summer => Self::Fall,
            Self::Fall => Self::Winter,
            Self::Winter => Self::Spring,
        }
    }

    /// Get the season name
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Spring => "Spring",
            Self::Summer => "Summer",
            Self::Fall => "Fall",
            Self::Winter => "Winter",
        }
    }

    /// Get the season index (0-3)
    #[must_use]
    pub fn index(self) -> u32 {
        match self {
            Self::Spring => 0,
            Self::Summer => 1,
            Self::Fall => 2,
            Self::Winter => 3,
        }
    }

    /// Create season from index (wraps around)
    #[must_use]
    pub fn from_index(index: u32) -> Self {
        match index % 4 {
            0 => Self::Spring,
            1 => Self::Summer,
            2 => Self::Fall,
            _ => Self::Winter,
        }
    }
}

/// Day of the week
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayOfWeek {
    /// Get the next day
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Monday => Self::Tuesday,
            Self::Tuesday => Self::Wednesday,
            Self::Wednesday => Self::Thursday,
            Self::Thursday => Self::Friday,
            Self::Friday => Self::Saturday,
            Self::Saturday => Self::Sunday,
            Self::Sunday => Self::Monday,
        }
    }

    /// Get the day name
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
            Self::Sunday => "Sunday",
        }
    }

    /// Get short name (3 letters)
    #[must_use]
    pub fn short_name(self) -> &'static str {
        match self {
            Self::Monday => "Mon",
            Self::Tuesday => "Tue",
            Self::Wednesday => "Wed",
            Self::Thursday => "Thu",
            Self::Friday => "Fri",
            Self::Saturday => "Sat",
            Self::Sunday => "Sun",
        }
    }

    /// Create from day index (0 = Monday, wraps around)
    #[must_use]
    pub fn from_index(index: u32) -> Self {
        match index % 7 {
            0 => Self::Monday,
            1 => Self::Tuesday,
            2 => Self::Wednesday,
            3 => Self::Thursday,
            4 => Self::Friday,
            5 => Self::Saturday,
            _ => Self::Sunday,
        }
    }
}

/// Time of day period
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeOfDay {
    /// 6:00 - 11:59
    Morning,
    /// 12:00 - 17:59
    Afternoon,
    /// 18:00 - 21:59
    Evening,
    /// 22:00 - 5:59
    Night,
}

impl TimeOfDay {
    /// Get the time period name
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Morning => "Morning",
            Self::Afternoon => "Afternoon",
            Self::Evening => "Evening",
            Self::Night => "Night",
        }
    }
}

/// In-game clock for farming/RPG games
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameClock {
    /// Current minute (0-59)
    minute: u32,
    /// Current hour (0-23)
    hour: u32,
    /// Current day of the month (1-28)
    day: u32,
    /// Current season
    season: Season,
    /// Current year (starts at 1)
    year: u32,
    /// Accumulated real time for minute tracking
    accumulated_time: f32,
    /// Real seconds per in-game minute
    pub seconds_per_minute: f32,
    /// Time scale multiplier (1.0 = normal, 2.0 = double speed)
    pub time_scale: f32,
    /// Whether time is paused
    pub paused: bool,
}

impl Default for GameClock {
    fn default() -> Self {
        Self::new()
    }
}

impl GameClock {
    /// Create a new game clock starting at 6:00 AM, Day 1, Spring, Year 1
    #[must_use]
    pub fn new() -> Self {
        Self {
            minute: 0,
            hour: 6, // Start at 6 AM
            day: 1,
            season: Season::Spring,
            year: 1,
            accumulated_time: 0.0,
            seconds_per_minute: DEFAULT_SECONDS_PER_MINUTE,
            time_scale: 1.0,
            paused: false,
        }
    }

    /// Create a game clock with custom starting time
    #[must_use]
    pub fn with_time(hour: u32, minute: u32) -> Self {
        let mut clock = Self::new();
        clock.hour = hour % HOURS_PER_DAY;
        clock.minute = minute % MINUTES_PER_HOUR;
        clock
    }

    /// Advance time by real delta seconds
    pub fn advance(&mut self, dt: f32) {
        if self.paused {
            return;
        }

        self.accumulated_time += dt * self.time_scale;

        // Convert accumulated real time to in-game minutes
        while self.accumulated_time >= self.seconds_per_minute {
            self.accumulated_time -= self.seconds_per_minute;
            self.advance_minute();
        }
    }

    /// Advance by one in-game minute
    fn advance_minute(&mut self) {
        self.minute += 1;

        if self.minute >= MINUTES_PER_HOUR {
            self.minute = 0;
            self.advance_hour();
        }
    }

    /// Advance by one in-game hour
    fn advance_hour(&mut self) {
        self.hour += 1;

        if self.hour >= HOURS_PER_DAY {
            self.hour = 0;
            self.advance_day();
        }
    }

    /// Advance by one in-game day
    fn advance_day(&mut self) {
        self.day += 1;

        if self.day > DAYS_PER_SEASON {
            self.day = 1;
            self.advance_season();
        }
    }

    /// Advance by one season
    fn advance_season(&mut self) {
        self.season = self.season.next();

        if self.season == Season::Spring {
            self.year += 1;
        }
    }

    // ========== Getters ==========

    /// Get current minute (0-59)
    #[must_use]
    pub fn minute(&self) -> u32 {
        self.minute
    }

    /// Get current hour (0-23)
    #[must_use]
    pub fn hour(&self) -> u32 {
        self.hour
    }

    /// Get current day of the month (1-28)
    #[must_use]
    pub fn day(&self) -> u32 {
        self.day
    }

    /// Get current season
    #[must_use]
    pub fn season(&self) -> Season {
        self.season
    }

    /// Get current year
    #[must_use]
    pub fn year(&self) -> u32 {
        self.year
    }

    /// Get day of the week
    #[must_use]
    pub fn day_of_week(&self) -> DayOfWeek {
        // Calculate total days since start
        let total_days = (self.year - 1) * SEASONS_PER_YEAR * DAYS_PER_SEASON
            + self.season.index() * DAYS_PER_SEASON
            + (self.day - 1);
        DayOfWeek::from_index(total_days)
    }

    /// Get the current time of day
    #[must_use]
    pub fn time_of_day(&self) -> TimeOfDay {
        match self.hour {
            6..=11 => TimeOfDay::Morning,
            12..=17 => TimeOfDay::Afternoon,
            18..=21 => TimeOfDay::Evening,
            _ => TimeOfDay::Night,
        }
    }

    // ========== Time of Day Helpers ==========

    /// Check if it's morning (6:00 - 11:59)
    #[must_use]
    pub fn is_morning(&self) -> bool {
        self.time_of_day() == TimeOfDay::Morning
    }

    /// Check if it's afternoon (12:00 - 17:59)
    #[must_use]
    pub fn is_afternoon(&self) -> bool {
        self.time_of_day() == TimeOfDay::Afternoon
    }

    /// Check if it's evening (18:00 - 21:59)
    #[must_use]
    pub fn is_evening(&self) -> bool {
        self.time_of_day() == TimeOfDay::Evening
    }

    /// Check if it's night (22:00 - 5:59)
    #[must_use]
    pub fn is_night(&self) -> bool {
        self.time_of_day() == TimeOfDay::Night
    }

    /// Check if it's daytime (6:00 - 21:59)
    #[must_use]
    pub fn is_daytime(&self) -> bool {
        (6..=21).contains(&self.hour)
    }

    /// Check if it's nighttime (22:00 - 5:59)
    #[must_use]
    pub fn is_nighttime(&self) -> bool {
        !self.is_daytime()
    }

    // ========== Formatting ==========

    /// Get time as "HH:MM" string
    #[must_use]
    pub fn time_string(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    /// Get time as "H:MM AM/PM" string
    #[must_use]
    pub fn time_string_12h(&self) -> String {
        let (hour_12, period) = if self.hour == 0 {
            (12, "AM")
        } else if self.hour < 12 {
            (self.hour, "AM")
        } else if self.hour == 12 {
            (12, "PM")
        } else {
            (self.hour - 12, "PM")
        };
        format!("{}:{:02} {}", hour_12, self.minute, period)
    }

    /// Get date as "Season Day Year" string
    #[must_use]
    pub fn date_string(&self) -> String {
        format!("{} {} Year {}", self.season.name(), self.day, self.year)
    }

    /// Get full date and time string
    #[must_use]
    pub fn full_string(&self) -> String {
        format!(
            "{}, {} {} - {}",
            self.day_of_week().name(),
            self.season.name(),
            self.day,
            self.time_string_12h()
        )
    }

    // ========== Time Manipulation ==========

    /// Set the time of day
    pub fn set_time(&mut self, hour: u32, minute: u32) {
        self.hour = hour % HOURS_PER_DAY;
        self.minute = minute % MINUTES_PER_HOUR;
        self.accumulated_time = 0.0;
    }

    /// Skip to the next day at 6:00 AM
    pub fn skip_to_next_day(&mut self) {
        self.advance_day();
        self.hour = 6;
        self.minute = 0;
        self.accumulated_time = 0.0;
    }

    /// Get normalized time of day (0.0 = midnight, 0.5 = noon, 1.0 = midnight)
    #[must_use]
    pub fn normalized_time(&self) -> f32 {
        (self.hour as f32 + self.minute as f32 / 60.0) / 24.0
    }

    /// Get daylight factor (0.0 = darkest, 1.0 = brightest)
    /// Useful for day/night visual effects
    #[must_use]
    pub fn daylight_factor(&self) -> f32 {
        let hour_f = self.hour as f32 + self.minute as f32 / 60.0;

        // Peak daylight at noon (12:00), darkest at midnight (0:00)
        // Use cosine curve for smooth transition
        let angle = (hour_f - 12.0) / 12.0 * std::f32::consts::PI;
        (angle.cos() + 1.0) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_clock() {
        let clock = GameClock::new();
        assert_eq!(clock.hour(), 6);
        assert_eq!(clock.minute(), 0);
        assert_eq!(clock.day(), 1);
        assert_eq!(clock.season(), Season::Spring);
        assert_eq!(clock.year(), 1);
    }

    #[test]
    fn test_advance_time() {
        let mut clock = GameClock::new();
        clock.seconds_per_minute = 1.0;

        // Advance 1 second = 1 minute
        clock.advance(1.0);
        assert_eq!(clock.minute(), 1);

        // Advance 59 more minutes = 1 hour
        clock.advance(59.0);
        assert_eq!(clock.hour(), 7);
        assert_eq!(clock.minute(), 0);
    }

    #[test]
    fn test_day_rollover() {
        let mut clock = GameClock::new();
        clock.set_time(23, 59);

        clock.seconds_per_minute = 1.0;
        clock.advance(1.0);

        assert_eq!(clock.hour(), 0);
        assert_eq!(clock.minute(), 0);
        assert_eq!(clock.day(), 2);
    }

    #[test]
    fn test_season_rollover() {
        let mut clock = GameClock::new();
        clock.day = DAYS_PER_SEASON;
        clock.set_time(23, 59);

        clock.seconds_per_minute = 1.0;
        clock.advance(1.0);

        assert_eq!(clock.day(), 1);
        assert_eq!(clock.season(), Season::Summer);
    }

    #[test]
    fn test_year_rollover() {
        let mut clock = GameClock::new();
        clock.season = Season::Winter;
        clock.day = DAYS_PER_SEASON;
        clock.set_time(23, 59);

        clock.seconds_per_minute = 1.0;
        clock.advance(1.0);

        assert_eq!(clock.season(), Season::Spring);
        assert_eq!(clock.year(), 2);
    }

    #[test]
    fn test_time_of_day() {
        let mut clock = GameClock::new();

        clock.set_time(6, 0);
        assert!(clock.is_morning());

        clock.set_time(12, 0);
        assert!(clock.is_afternoon());

        clock.set_time(18, 0);
        assert!(clock.is_evening());

        clock.set_time(22, 0);
        assert!(clock.is_night());

        clock.set_time(3, 0);
        assert!(clock.is_night());
    }

    #[test]
    fn test_time_scale() {
        let mut clock = GameClock::new();
        clock.seconds_per_minute = 1.0;
        clock.time_scale = 2.0;

        // At 2x speed, 0.5 seconds = 1 minute
        clock.advance(0.5);
        assert_eq!(clock.minute(), 1);
    }

    #[test]
    fn test_pause() {
        let mut clock = GameClock::new();
        clock.seconds_per_minute = 1.0;
        clock.paused = true;

        clock.advance(100.0);
        assert_eq!(clock.minute(), 0);
    }

    #[test]
    fn test_day_of_week() {
        let clock = GameClock::new();
        // Day 1 = Monday
        assert_eq!(clock.day_of_week(), DayOfWeek::Monday);
    }

    #[test]
    fn test_formatting() {
        let mut clock = GameClock::new();
        clock.set_time(14, 30);

        assert_eq!(clock.time_string(), "14:30");
        assert_eq!(clock.time_string_12h(), "2:30 PM");
    }

    #[test]
    fn test_daylight_factor() {
        let mut clock = GameClock::new();

        // Noon should be brightest
        clock.set_time(12, 0);
        assert!((clock.daylight_factor() - 1.0).abs() < 0.01);

        // Midnight should be darkest
        clock.set_time(0, 0);
        assert!(clock.daylight_factor() < 0.01);
    }
}
