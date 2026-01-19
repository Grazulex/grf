//! Dialogue system with branching conversations
//!
//! Provides dialogue trees with choices, conditions, and actions.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// A condition that must be met for a choice to be available
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Condition {
    /// Player has at least N of an item
    HasItem { item_id: String, quantity: u32 },
    /// Player does not have an item
    NotHasItem { item_id: String },
    /// Friendship level is at least N
    FriendshipLevel { npc_id: String, level: u32 },
    /// A flag is set
    FlagSet { flag: String },
    /// A flag is not set
    FlagNotSet { flag: String },
    /// Always true
    #[default]
    Always,
}

/// An action to perform when a choice is selected
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    /// Give an item to the player
    GiveItem { item_id: String, quantity: u32 },
    /// Take an item from the player
    TakeItem { item_id: String, quantity: u32 },
    /// Change friendship level
    ChangeFriendship { npc_id: String, amount: i32 },
    /// Set a flag
    SetFlag { flag: String },
    /// Clear a flag
    ClearFlag { flag: String },
    /// No action
    #[default]
    None,
}

/// A choice the player can make in dialogue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueChoice {
    /// Display text for the choice
    pub text: String,
    /// Node to go to if this choice is selected
    pub next_node: Option<String>,
    /// Condition that must be met (default: Always)
    #[serde(default)]
    pub condition: Condition,
    /// Action to perform when selected
    #[serde(default)]
    pub action: Action,
}

impl DialogueChoice {
    /// Create a simple choice that leads to another node
    #[must_use]
    pub fn new(text: &str, next_node: Option<&str>) -> Self {
        Self {
            text: text.to_string(),
            next_node: next_node.map(|s| s.to_string()),
            condition: Condition::Always,
            action: Action::None,
        }
    }

    /// Add a condition to this choice
    #[must_use]
    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.condition = condition;
        self
    }

    /// Add an action to this choice
    #[must_use]
    pub fn with_action(mut self, action: Action) -> Self {
        self.action = action;
        self
    }
}

/// A single node in a dialogue tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    /// Unique ID for this node within the dialogue
    pub id: String,
    /// Who is speaking (NPC name, "player", or empty for narration)
    #[serde(default)]
    pub speaker: String,
    /// The dialogue text
    pub text: String,
    /// Available choices (empty = end of dialogue or auto-continue)
    #[serde(default)]
    pub choices: Vec<DialogueChoice>,
    /// Node to automatically continue to if no choices
    #[serde(default)]
    pub next: Option<String>,
    /// Action to perform when reaching this node
    #[serde(default)]
    pub action: Action,
}

impl DialogueNode {
    /// Create a new dialogue node
    #[must_use]
    pub fn new(id: &str, speaker: &str, text: &str) -> Self {
        Self {
            id: id.to_string(),
            speaker: speaker.to_string(),
            text: text.to_string(),
            choices: Vec::new(),
            next: None,
            action: Action::None,
        }
    }

    /// Add a choice to this node
    pub fn add_choice(&mut self, choice: DialogueChoice) {
        self.choices.push(choice);
    }

    /// Set the auto-continue node
    pub fn with_next(mut self, next_id: &str) -> Self {
        self.next = Some(next_id.to_string());
        self
    }

    /// Check if this node ends the dialogue
    #[must_use]
    pub fn is_end(&self) -> bool {
        self.choices.is_empty() && self.next.is_none()
    }

    /// Check if this node has choices
    #[must_use]
    pub fn has_choices(&self) -> bool {
        !self.choices.is_empty()
    }
}

/// A complete dialogue tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    /// Unique ID for this dialogue
    pub id: String,
    /// Starting node ID
    pub start_node: String,
    /// All nodes in this dialogue, keyed by ID
    pub nodes: HashMap<String, DialogueNode>,
}

impl Dialogue {
    /// Create a new dialogue
    #[must_use]
    pub fn new(id: &str, start_node: &str) -> Self {
        Self {
            id: id.to_string(),
            start_node: start_node.to_string(),
            nodes: HashMap::new(),
        }
    }

    /// Add a node to this dialogue
    pub fn add_node(&mut self, node: DialogueNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Get the starting node
    #[must_use]
    pub fn get_start(&self) -> Option<&DialogueNode> {
        self.nodes.get(&self.start_node)
    }

    /// Get a node by ID
    #[must_use]
    pub fn get_node(&self, id: &str) -> Option<&DialogueNode> {
        self.nodes.get(id)
    }
}

/// State of an active dialogue
#[derive(Debug, Clone)]
pub struct DialogueState {
    /// The dialogue being played
    pub dialogue_id: String,
    /// Current node ID
    pub current_node: String,
    /// History of visited nodes (for back functionality)
    pub history: Vec<String>,
}

impl DialogueState {
    /// Create a new dialogue state
    #[must_use]
    pub fn new(dialogue_id: &str, start_node: &str) -> Self {
        Self {
            dialogue_id: dialogue_id.to_string(),
            current_node: start_node.to_string(),
            history: vec![start_node.to_string()],
        }
    }

    /// Move to a new node
    pub fn go_to(&mut self, node_id: &str) {
        self.history.push(node_id.to_string());
        self.current_node = node_id.to_string();
    }

    /// Go back to previous node (if possible)
    pub fn go_back(&mut self) -> bool {
        if self.history.len() > 1 {
            self.history.pop();
            if let Some(prev) = self.history.last() {
                self.current_node = prev.clone();
                return true;
            }
        }
        false
    }
}

/// Manager for loading and playing dialogues
#[derive(Debug, Default)]
pub struct DialogueManager {
    /// Loaded dialogues
    dialogues: HashMap<String, Dialogue>,
    /// Current active dialogue state
    active: Option<DialogueState>,
}

impl DialogueManager {
    /// Create a new dialogue manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            dialogues: HashMap::new(),
            active: None,
        }
    }

    /// Load a dialogue from JSON string
    pub fn load_from_str(&mut self, content: &str) -> Result<(), DialogueLoadError> {
        let dialogue: Dialogue =
            serde_json::from_str(content).map_err(|e| DialogueLoadError::Parse {
                error: e.to_string(),
            })?;

        self.dialogues.insert(dialogue.id.clone(), dialogue);
        Ok(())
    }

    /// Load a dialogue from a JSON file
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), DialogueLoadError> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| DialogueLoadError::Io {
            path: path.as_ref().to_string_lossy().to_string(),
            error: e.to_string(),
        })?;

        self.load_from_str(&content)
    }

    /// Get a loaded dialogue by ID
    #[must_use]
    pub fn get_dialogue(&self, id: &str) -> Option<&Dialogue> {
        self.dialogues.get(id)
    }

    /// Start a dialogue
    pub fn start(&mut self, dialogue_id: &str) -> bool {
        if let Some(dialogue) = self.dialogues.get(dialogue_id) {
            self.active = Some(DialogueState::new(dialogue_id, &dialogue.start_node));
            true
        } else {
            false
        }
    }

    /// End the current dialogue
    pub fn end(&mut self) {
        self.active = None;
    }

    /// Check if a dialogue is active
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active.is_some()
    }

    /// Get the current node
    #[must_use]
    pub fn current_node(&self) -> Option<&DialogueNode> {
        let state = self.active.as_ref()?;
        let dialogue = self.dialogues.get(&state.dialogue_id)?;
        dialogue.get_node(&state.current_node)
    }

    /// Get available choices for current node (filtered by conditions)
    /// Note: Condition checking should be done by the game with access to game state
    #[must_use]
    pub fn current_choices(&self) -> Vec<&DialogueChoice> {
        self.current_node()
            .map(|node| node.choices.iter().collect())
            .unwrap_or_default()
    }

    /// Select a choice by index
    /// Returns the action to perform, if any
    pub fn select_choice(&mut self, index: usize) -> Option<Action> {
        let state = self.active.as_ref()?;
        let dialogue = self.dialogues.get(&state.dialogue_id)?;
        let node = dialogue.get_node(&state.current_node)?;

        let choice = node.choices.get(index)?;
        let action = choice.action.clone();

        if let Some(next) = &choice.next_node {
            // Clone to avoid borrow issues
            let next = next.clone();
            if let Some(state) = &mut self.active {
                state.go_to(&next);
            }
        } else {
            // End dialogue
            self.end();
        }

        Some(action)
    }

    /// Continue to next node (for nodes without choices)
    pub fn continue_dialogue(&mut self) -> bool {
        let next_node = {
            let state = match self.active.as_ref() {
                Some(s) => s,
                None => return false,
            };
            let dialogue = match self.dialogues.get(&state.dialogue_id) {
                Some(d) => d,
                None => return false,
            };
            let node = match dialogue.get_node(&state.current_node) {
                Some(n) => n,
                None => return false,
            };

            if node.is_end() {
                self.end();
                return false;
            }

            node.next.clone()
        };

        if let Some(next) = next_node {
            if let Some(state) = &mut self.active {
                state.go_to(&next);
            }
            return true;
        }

        false
    }

    /// Get current dialogue state
    #[must_use]
    pub fn state(&self) -> Option<&DialogueState> {
        self.active.as_ref()
    }
}

/// Errors when loading dialogues
#[derive(Debug)]
pub enum DialogueLoadError {
    Io { path: String, error: String },
    Parse { error: String },
}

impl std::fmt::Display for DialogueLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, error } => write!(f, "Failed to read {}: {}", path, error),
            Self::Parse { error } => write!(f, "Failed to parse JSON: {}", error),
        }
    }
}

impl std::error::Error for DialogueLoadError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialogue_node() {
        let node = DialogueNode::new("start", "Robin", "Hello there!");
        assert_eq!(node.id, "start");
        assert_eq!(node.speaker, "Robin");
        assert_eq!(node.text, "Hello there!");
        assert!(node.is_end());
    }

    #[test]
    fn test_dialogue_with_choices() {
        let mut node = DialogueNode::new("start", "Robin", "How are you?");
        node.add_choice(DialogueChoice::new("I'm fine!", Some("response_good")));
        node.add_choice(DialogueChoice::new("Not great...", Some("response_bad")));

        assert!(node.has_choices());
        assert!(!node.is_end());
        assert_eq!(node.choices.len(), 2);
    }

    #[test]
    fn test_dialogue_tree() {
        let mut dialogue = Dialogue::new("robin_intro", "start");

        let mut start = DialogueNode::new("start", "Robin", "Hey, welcome to town!");
        start.add_choice(DialogueChoice::new("Thanks!", Some("thanks")));
        start.add_choice(DialogueChoice::new("Who are you?", Some("who")));
        dialogue.add_node(start);

        let thanks = DialogueNode::new("thanks", "Robin", "No problem!");
        dialogue.add_node(thanks);

        let who = DialogueNode::new("who", "Robin", "I'm Robin, the carpenter.");
        dialogue.add_node(who);

        assert_eq!(dialogue.nodes.len(), 3);
        assert!(dialogue.get_start().is_some());
        assert!(dialogue.get_node("thanks").is_some());
    }

    #[test]
    fn test_dialogue_manager() {
        let mut manager = DialogueManager::new();

        let json = r#"{
            "id": "test",
            "start_node": "start",
            "nodes": {
                "start": {
                    "id": "start",
                    "speaker": "NPC",
                    "text": "Hello!",
                    "choices": [
                        {"text": "Hi!", "next_node": "end"}
                    ]
                },
                "end": {
                    "id": "end",
                    "speaker": "NPC",
                    "text": "Goodbye!"
                }
            }
        }"#;

        manager.load_from_str(json).unwrap();
        assert!(manager.get_dialogue("test").is_some());
    }

    #[test]
    fn test_dialogue_navigation() {
        let mut manager = DialogueManager::new();

        let json = r#"{
            "id": "test",
            "start_node": "start",
            "nodes": {
                "start": {
                    "id": "start",
                    "speaker": "NPC",
                    "text": "Hello!",
                    "choices": [
                        {"text": "Hi!", "next_node": "middle"}
                    ]
                },
                "middle": {
                    "id": "middle",
                    "speaker": "NPC",
                    "text": "How are you?",
                    "next": "end"
                },
                "end": {
                    "id": "end",
                    "speaker": "NPC",
                    "text": "Goodbye!"
                }
            }
        }"#;

        manager.load_from_str(json).unwrap();
        assert!(manager.start("test"));
        assert!(manager.is_active());

        // Check current node
        let node = manager.current_node().unwrap();
        assert_eq!(node.id, "start");
        assert_eq!(node.text, "Hello!");

        // Select choice
        manager.select_choice(0);
        let node = manager.current_node().unwrap();
        assert_eq!(node.id, "middle");

        // Continue
        manager.continue_dialogue();
        let node = manager.current_node().unwrap();
        assert_eq!(node.id, "end");

        // Continue at end should end dialogue
        manager.continue_dialogue();
        assert!(!manager.is_active());
    }

    #[test]
    fn test_choice_with_condition() {
        let choice = DialogueChoice::new("Give gift", Some("give"))
            .with_condition(Condition::HasItem {
                item_id: "diamond".to_string(),
                quantity: 1,
            })
            .with_action(Action::TakeItem {
                item_id: "diamond".to_string(),
                quantity: 1,
            });

        assert!(matches!(choice.condition, Condition::HasItem { .. }));
        assert!(matches!(choice.action, Action::TakeItem { .. }));
    }

    #[test]
    fn test_dialogue_actions() {
        let mut manager = DialogueManager::new();

        let json = r#"{
            "id": "test",
            "start_node": "start",
            "nodes": {
                "start": {
                    "id": "start",
                    "speaker": "NPC",
                    "text": "Have a gift!",
                    "choices": [
                        {
                            "text": "Thanks!",
                            "action": {
                                "type": "give_item",
                                "item_id": "apple",
                                "quantity": 5
                            }
                        }
                    ]
                }
            }
        }"#;

        manager.load_from_str(json).unwrap();
        manager.start("test");

        let action = manager.select_choice(0);
        assert!(matches!(action, Some(Action::GiveItem { .. })));
    }
}
