//! NPC system with state machines, schedules, and pathfinding
//!
//! Provides NPCs that follow daily schedules and move around the world.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path as FsPath;

/// NPC behavior state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum NpcState {
    /// Standing still
    #[default]
    Idle,
    /// Walking to destination
    Walking,
    /// In conversation with player
    Talking,
    /// Performing work activity
    Working,
    /// Sleeping
    Sleeping,
}

impl NpcState {
    /// Check if NPC can be interacted with
    #[must_use]
    pub fn can_interact(self) -> bool {
        matches!(self, Self::Idle | Self::Walking | Self::Working)
    }

    /// Check if NPC is moving
    #[must_use]
    pub fn is_moving(self) -> bool {
        self == Self::Walking
    }
}

/// Direction the NPC is facing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

impl Direction {
    /// Get unit vector for this direction as (x, y)
    #[must_use]
    pub fn to_tuple(self) -> (f32, f32) {
        match self {
            Self::Up => (0.0, -1.0),
            Self::Down => (0.0, 1.0),
            Self::Left => (-1.0, 0.0),
            Self::Right => (1.0, 0.0),
        }
    }

    /// Get direction from velocity
    #[must_use]
    pub fn from_velocity(vx: f32, vy: f32) -> Self {
        if vx.abs() > vy.abs() {
            if vx > 0.0 {
                Self::Right
            } else {
                Self::Left
            }
        } else if vy > 0.0 {
            Self::Down
        } else {
            Self::Up
        }
    }
}

/// A scheduled activity for an NPC
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleEntry {
    /// Hour to start this activity (0-23)
    pub hour: u32,
    /// Target location name (e.g., "home", "shop", "farm")
    pub location: String,
    /// Activity to perform at location
    pub activity: NpcState,
}

/// NPC schedule for the day
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Schedule {
    /// Entries sorted by hour
    pub entries: Vec<ScheduleEntry>,
}

impl Schedule {
    /// Create an empty schedule
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Add a schedule entry
    pub fn add(&mut self, hour: u32, location: &str, activity: NpcState) {
        self.entries.push(ScheduleEntry {
            hour,
            location: location.to_string(),
            activity,
        });
        // Keep sorted by hour
        self.entries.sort_by_key(|e| e.hour);
    }

    /// Get the current activity based on hour
    #[must_use]
    pub fn get_current(&self, hour: u32) -> Option<&ScheduleEntry> {
        // Find the latest entry that started before or at current hour
        self.entries.iter().rev().find(|e| e.hour <= hour)
    }
}

/// A waypoint for pathfinding
#[derive(Debug, Clone, Copy)]
pub struct Waypoint {
    pub x: f32,
    pub y: f32,
}

/// Simple path as list of waypoints
#[derive(Debug, Clone, Default)]
pub struct NpcPath {
    waypoints: Vec<Waypoint>,
    current_index: usize,
}

impl NpcPath {
    /// Create a new path from waypoints
    #[must_use]
    pub fn new(waypoints: Vec<Waypoint>) -> Self {
        Self {
            waypoints,
            current_index: 0,
        }
    }

    /// Create a direct path to target
    #[must_use]
    pub fn direct(x: f32, y: f32) -> Self {
        Self {
            waypoints: vec![Waypoint { x, y }],
            current_index: 0,
        }
    }

    /// Get current waypoint
    #[must_use]
    pub fn current(&self) -> Option<&Waypoint> {
        self.waypoints.get(self.current_index)
    }

    /// Advance to next waypoint
    pub fn advance(&mut self) {
        if self.current_index < self.waypoints.len() {
            self.current_index += 1;
        }
    }

    /// Check if path is complete
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.current_index >= self.waypoints.len()
    }

    /// Reset path to beginning
    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}

/// Position as simple x, y coordinates (for serialization)
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

impl Position2D {
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn distance_to(&self, other: &Self) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// NPC entity component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Current behavior state
    #[serde(default)]
    pub state: NpcState,
    /// Direction facing
    #[serde(default)]
    pub direction: Direction,
    /// Current position
    #[serde(default)]
    pub position: Position2D,
    /// Movement speed (pixels per second)
    #[serde(default = "default_speed")]
    pub speed: f32,
    /// Daily schedule
    #[serde(default)]
    pub schedule: Schedule,
    /// Current path (if walking)
    #[serde(skip)]
    pub path: Option<NpcPath>,
    /// Known locations for this NPC
    #[serde(default)]
    pub locations: HashMap<String, Position2D>,
    /// Dialogue ID for conversation
    #[serde(default)]
    pub dialogue_id: Option<String>,
}

fn default_speed() -> f32 {
    50.0
}

impl Npc {
    /// Create a new NPC
    #[must_use]
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            state: NpcState::Idle,
            direction: Direction::Down,
            position: Position2D::default(),
            speed: default_speed(),
            schedule: Schedule::new(),
            path: None,
            locations: HashMap::new(),
            dialogue_id: None,
        }
    }

    /// Set position
    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = Position2D::new(x, y);
        self
    }

    /// Add a known location
    pub fn add_location(&mut self, name: &str, x: f32, y: f32) {
        self.locations.insert(name.to_string(), Position2D::new(x, y));
    }

    /// Get position of a named location
    #[must_use]
    pub fn get_location(&self, name: &str) -> Option<Position2D> {
        self.locations.get(name).copied()
    }

    /// Start moving to a location by name
    pub fn go_to(&mut self, location_name: &str) -> bool {
        if let Some(target) = self.locations.get(location_name) {
            self.path = Some(NpcPath::direct(target.x, target.y));
            self.state = NpcState::Walking;
            true
        } else {
            false
        }
    }

    /// Start moving to a position
    pub fn go_to_position(&mut self, x: f32, y: f32) {
        self.path = Some(NpcPath::direct(x, y));
        self.state = NpcState::Walking;
    }

    /// Update NPC movement
    /// Returns true if reached destination
    pub fn update_movement(&mut self, dt: f32) -> bool {
        if self.state != NpcState::Walking {
            return false;
        }

        let path = match &mut self.path {
            Some(p) => p,
            None => {
                self.state = NpcState::Idle;
                return true;
            }
        };

        let waypoint = match path.current() {
            Some(w) => *w,
            None => {
                self.state = NpcState::Idle;
                self.path = None;
                return true;
            }
        };

        let dx = waypoint.x - self.position.x;
        let dy = waypoint.y - self.position.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let arrival_threshold = 2.0;

        if distance < arrival_threshold {
            // Reached waypoint
            path.advance();
            if path.is_complete() {
                self.state = NpcState::Idle;
                self.path = None;
                return true;
            }
        } else {
            // Move towards waypoint
            let dir_x = dx / distance;
            let dir_y = dy / distance;
            let move_dist = self.speed * dt;

            if move_dist > distance {
                self.position.x = waypoint.x;
                self.position.y = waypoint.y;
            } else {
                self.position.x += dir_x * move_dist;
                self.position.y += dir_y * move_dist;
            }

            self.direction = Direction::from_velocity(dx, dy);
        }

        false
    }

    /// Update NPC based on schedule
    pub fn update_schedule(&mut self, hour: u32) {
        // Clone the schedule entry data to avoid borrow issues
        let schedule_data = self.schedule.get_current(hour).map(|e| {
            (e.location.clone(), e.activity)
        });

        if let Some((location, activity)) = schedule_data {
            // If we're not already at the scheduled activity
            if self.state != activity || self.state == NpcState::Idle {
                // Try to go to the location
                if activity != NpcState::Walking && !self.go_to(&location) {
                    // Location not found, just switch state
                    self.state = activity;
                }
            }
        }
    }

    /// Start a conversation
    pub fn start_talking(&mut self) {
        if self.state.can_interact() {
            self.state = NpcState::Talking;
            self.path = None; // Stop walking
        }
    }

    /// End a conversation
    pub fn stop_talking(&mut self) {
        if self.state == NpcState::Talking {
            self.state = NpcState::Idle;
        }
    }
}

/// NPC definition from data file
#[derive(Debug, Clone, Deserialize)]
pub struct NpcDefinition {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub speed: Option<f32>,
    #[serde(default)]
    pub dialogue_id: Option<String>,
    #[serde(default)]
    pub locations: HashMap<String, [f32; 2]>,
    #[serde(default)]
    pub schedule: Vec<ScheduleEntryDef>,
}

/// Schedule entry definition for TOML
#[derive(Debug, Clone, Deserialize)]
pub struct ScheduleEntryDef {
    pub hour: u32,
    pub location: String,
    #[serde(default)]
    pub activity: NpcState,
}

/// TOML file structure
#[derive(Debug, Deserialize)]
struct NpcsFile {
    npcs: Vec<NpcDefinition>,
}

/// NPC database
#[derive(Debug, Default)]
pub struct NpcDatabase {
    definitions: HashMap<String, NpcDefinition>,
}

impl NpcDatabase {
    /// Create empty database
    #[must_use]
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }

    /// Load NPCs from TOML file
    pub fn load_from_file<P: AsRef<FsPath>>(&mut self, path: P) -> Result<usize, NpcLoadError> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| NpcLoadError::Io {
            path: path.as_ref().to_string_lossy().to_string(),
            error: e.to_string(),
        })?;

        self.load_from_str(&content)
    }

    /// Load NPCs from TOML string
    pub fn load_from_str(&mut self, content: &str) -> Result<usize, NpcLoadError> {
        let file: NpcsFile = toml::from_str(content).map_err(|e| NpcLoadError::Parse {
            error: e.to_string(),
        })?;

        let count = file.npcs.len();
        for def in file.npcs {
            self.definitions.insert(def.id.clone(), def);
        }

        Ok(count)
    }

    /// Get NPC definition
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&NpcDefinition> {
        self.definitions.get(id)
    }

    /// Create an NPC instance from definition
    #[must_use]
    pub fn create_npc(&self, id: &str) -> Option<Npc> {
        let def = self.get(id)?;

        let mut npc = Npc::new(&def.id, &def.name);

        if let Some(speed) = def.speed {
            npc.speed = speed;
        }

        npc.dialogue_id = def.dialogue_id.clone();

        // Add locations
        for (name, pos) in &def.locations {
            npc.add_location(name, pos[0], pos[1]);
        }

        // Build schedule
        for entry in &def.schedule {
            npc.schedule.add(entry.hour, &entry.location, entry.activity);
        }

        Some(npc)
    }

    /// Get all NPC IDs
    #[must_use]
    pub fn all_ids(&self) -> Vec<&str> {
        self.definitions.keys().map(|s| s.as_str()).collect()
    }

    /// Get number of NPCs
    #[must_use]
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Check if database is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }
}

/// Errors when loading NPCs
#[derive(Debug)]
pub enum NpcLoadError {
    Io { path: String, error: String },
    Parse { error: String },
}

impl std::fmt::Display for NpcLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, error } => write!(f, "Failed to read {}: {}", path, error),
            Self::Parse { error } => write!(f, "Failed to parse TOML: {}", error),
        }
    }
}

impl std::error::Error for NpcLoadError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npc_creation() {
        let npc = Npc::new("robin", "Robin");
        assert_eq!(npc.id, "robin");
        assert_eq!(npc.name, "Robin");
        assert_eq!(npc.state, NpcState::Idle);
    }

    #[test]
    fn test_npc_locations() {
        let mut npc = Npc::new("robin", "Robin");
        npc.add_location("home", 100.0, 200.0);
        npc.add_location("shop", 300.0, 400.0);

        let home = npc.get_location("home").unwrap();
        assert_eq!(home.x, 100.0);
        assert_eq!(home.y, 200.0);

        assert!(npc.get_location("unknown").is_none());
    }

    #[test]
    fn test_npc_go_to() {
        let mut npc = Npc::new("robin", "Robin");
        npc.add_location("home", 100.0, 200.0);

        assert!(npc.go_to("home"));
        assert_eq!(npc.state, NpcState::Walking);
        assert!(npc.path.is_some());

        assert!(!npc.go_to("unknown"));
    }

    #[test]
    fn test_schedule() {
        let mut schedule = Schedule::new();
        schedule.add(6, "home", NpcState::Sleeping);
        schedule.add(8, "shop", NpcState::Working);
        schedule.add(17, "home", NpcState::Idle);
        schedule.add(22, "home", NpcState::Sleeping);

        assert!(schedule.get_current(5).is_none()); // Before any schedule

        let at_7 = schedule.get_current(7).unwrap();
        assert_eq!(at_7.location, "home");

        let at_12 = schedule.get_current(12).unwrap();
        assert_eq!(at_12.location, "shop");

        let at_23 = schedule.get_current(23).unwrap();
        assert_eq!(at_23.activity, NpcState::Sleeping);
    }

    #[test]
    fn test_npc_movement() {
        let mut npc = Npc::new("robin", "Robin");
        npc.position = Position2D::new(0.0, 0.0);
        npc.speed = 100.0;
        npc.go_to_position(100.0, 0.0);

        // Move for 0.5 seconds
        npc.update_movement(0.5);
        assert_eq!(npc.position.x, 50.0);
        assert_eq!(npc.state, NpcState::Walking);

        // Move another 0.5 seconds (should reach)
        let reached = npc.update_movement(0.5);
        assert!(reached || npc.position.x > 98.0);
    }

    #[test]
    fn test_direction_from_velocity() {
        assert_eq!(Direction::from_velocity(1.0, 0.0), Direction::Right);
        assert_eq!(Direction::from_velocity(-1.0, 0.0), Direction::Left);
        assert_eq!(Direction::from_velocity(0.0, 1.0), Direction::Down);
        assert_eq!(Direction::from_velocity(0.0, -1.0), Direction::Up);
    }

    #[test]
    fn test_npc_conversation() {
        let mut npc = Npc::new("robin", "Robin");
        assert!(npc.state.can_interact());

        npc.start_talking();
        assert_eq!(npc.state, NpcState::Talking);
        assert!(!npc.state.can_interact());

        npc.stop_talking();
        assert_eq!(npc.state, NpcState::Idle);
    }

    const TEST_TOML: &str = r#"
[[npcs]]
id = "robin"
name = "Robin"
speed = 60.0
dialogue_id = "robin_intro"

[npcs.locations]
home = [100.0, 200.0]
shop = [300.0, 400.0]

[[npcs.schedule]]
hour = 6
location = "home"
activity = "sleeping"

[[npcs.schedule]]
hour = 8
location = "shop"
activity = "working"

[[npcs.schedule]]
hour = 17
location = "home"
activity = "idle"

[[npcs]]
id = "pierre"
name = "Pierre"

[npcs.locations]
store = [500.0, 100.0]
"#;

    #[test]
    fn test_load_npcs() {
        let mut db = NpcDatabase::new();
        let count = db.load_from_str(TEST_TOML).unwrap();

        assert_eq!(count, 2);
        assert_eq!(db.len(), 2);
    }

    #[test]
    fn test_create_npc_from_definition() {
        let mut db = NpcDatabase::new();
        db.load_from_str(TEST_TOML).unwrap();

        let npc = db.create_npc("robin").unwrap();
        assert_eq!(npc.name, "Robin");
        assert_eq!(npc.speed, 60.0);
        assert_eq!(npc.dialogue_id, Some("robin_intro".to_string()));

        let home = npc.get_location("home").unwrap();
        assert_eq!(home.x, 100.0);
        assert_eq!(home.y, 200.0);

        assert_eq!(npc.schedule.entries.len(), 3);
    }
}
