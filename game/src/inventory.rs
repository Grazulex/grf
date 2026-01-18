//! Inventory system for items and stacking
//!
//! Provides a slot-based inventory with automatic stacking.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Maximum default stack size
pub const DEFAULT_MAX_STACK: u32 = 999;
/// Default inventory size
pub const DEFAULT_INVENTORY_SIZE: usize = 36;
/// Hotbar size (first N slots)
pub const HOTBAR_SIZE: usize = 10;

/// Item quality levels (like Stardew Valley)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Quality {
    Normal,
    Silver,
    Gold,
    Iridium,
}

impl Default for Quality {
    fn default() -> Self {
        Self::Normal
    }
}

impl Quality {
    /// Get quality multiplier for selling price
    #[must_use]
    pub fn price_multiplier(self) -> f32 {
        match self {
            Self::Normal => 1.0,
            Self::Silver => 1.25,
            Self::Gold => 1.5,
            Self::Iridium => 2.0,
        }
    }

    /// Get quality star count for display
    #[must_use]
    pub fn stars(self) -> u32 {
        match self {
            Self::Normal => 0,
            Self::Silver => 1,
            Self::Gold => 2,
            Self::Iridium => 3,
        }
    }
}

/// A stack of items in a slot
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemStack {
    /// Item type identifier
    pub item_id: String,
    /// Quantity in this stack
    pub quantity: u32,
    /// Quality level
    pub quality: Quality,
    /// Maximum stack size for this item type
    pub max_stack: u32,
}

impl ItemStack {
    /// Create a new item stack
    #[must_use]
    pub fn new(item_id: impl Into<String>, quantity: u32) -> Self {
        Self {
            item_id: item_id.into(),
            quantity,
            quality: Quality::Normal,
            max_stack: DEFAULT_MAX_STACK,
        }
    }

    /// Create with quality
    #[must_use]
    pub fn with_quality(mut self, quality: Quality) -> Self {
        self.quality = quality;
        self
    }

    /// Create with custom max stack
    #[must_use]
    pub fn with_max_stack(mut self, max_stack: u32) -> Self {
        self.max_stack = max_stack;
        self
    }

    /// Check if stack is full
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.quantity >= self.max_stack
    }

    /// Get remaining space in this stack
    #[must_use]
    pub fn remaining_space(&self) -> u32 {
        self.max_stack.saturating_sub(self.quantity)
    }

    /// Check if this stack can accept items of the given type and quality
    #[must_use]
    pub fn can_stack_with(&self, item_id: &str, quality: Quality) -> bool {
        self.item_id == item_id && self.quality == quality && !self.is_full()
    }

    /// Add items to this stack, returns overflow
    pub fn add(&mut self, amount: u32) -> u32 {
        let can_add = self.remaining_space().min(amount);
        self.quantity += can_add;
        amount - can_add
    }

    /// Remove items from this stack, returns actually removed amount
    pub fn remove(&mut self, amount: u32) -> u32 {
        let can_remove = self.quantity.min(amount);
        self.quantity -= can_remove;
        can_remove
    }
}

/// Slot-based inventory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    /// Inventory slots (None = empty)
    slots: Vec<Option<ItemStack>>,
    /// Currently selected hotbar slot (0-9)
    selected_slot: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new(DEFAULT_INVENTORY_SIZE)
    }
}

impl Inventory {
    /// Create a new inventory with given size
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![None; size],
            selected_slot: 0,
        }
    }

    /// Get inventory size
    #[must_use]
    pub fn size(&self) -> usize {
        self.slots.len()
    }

    /// Get a slot reference
    #[must_use]
    pub fn get(&self, slot: usize) -> Option<&ItemStack> {
        self.slots.get(slot).and_then(|s| s.as_ref())
    }

    /// Get a mutable slot reference
    pub fn get_mut(&mut self, slot: usize) -> Option<&mut ItemStack> {
        self.slots.get_mut(slot).and_then(|s| s.as_mut())
    }

    /// Check if a slot is empty
    #[must_use]
    pub fn is_empty(&self, slot: usize) -> bool {
        self.slots.get(slot).map_or(true, |s| s.is_none())
    }

    /// Get currently selected slot index
    #[must_use]
    pub fn selected_slot(&self) -> usize {
        self.selected_slot
    }

    /// Get the item in the selected slot
    #[must_use]
    pub fn selected_item(&self) -> Option<&ItemStack> {
        self.get(self.selected_slot)
    }

    /// Select a slot (clamps to valid range)
    pub fn select_slot(&mut self, slot: usize) {
        self.selected_slot = slot.min(HOTBAR_SIZE - 1);
    }

    /// Select next slot (wraps around)
    pub fn select_next(&mut self) {
        self.selected_slot = (self.selected_slot + 1) % HOTBAR_SIZE;
    }

    /// Select previous slot (wraps around)
    pub fn select_prev(&mut self) {
        self.selected_slot = if self.selected_slot == 0 {
            HOTBAR_SIZE - 1
        } else {
            self.selected_slot - 1
        };
    }

    /// Add an item to the inventory, returns overflow quantity
    pub fn add_item(&mut self, item_id: &str, quantity: u32, quality: Quality) -> u32 {
        let mut remaining = quantity;

        // First, try to stack with existing items
        for slot in &mut self.slots {
            if remaining == 0 {
                break;
            }

            if let Some(stack) = slot {
                if stack.can_stack_with(item_id, quality) {
                    remaining = stack.add(remaining);
                }
            }
        }

        // Then, fill empty slots
        for slot in &mut self.slots {
            if remaining == 0 {
                break;
            }

            if slot.is_none() {
                let can_add = remaining.min(DEFAULT_MAX_STACK);
                *slot = Some(
                    ItemStack::new(item_id, can_add)
                        .with_quality(quality),
                );
                remaining -= can_add;
            }
        }

        remaining
    }

    /// Add an item with custom max stack
    pub fn add_item_with_max_stack(
        &mut self,
        item_id: &str,
        quantity: u32,
        quality: Quality,
        max_stack: u32,
    ) -> u32 {
        let mut remaining = quantity;

        // First, try to stack with existing items
        for slot in &mut self.slots {
            if remaining == 0 {
                break;
            }

            if let Some(stack) = slot {
                if stack.can_stack_with(item_id, quality) {
                    remaining = stack.add(remaining);
                }
            }
        }

        // Then, fill empty slots
        for slot in &mut self.slots {
            if remaining == 0 {
                break;
            }

            if slot.is_none() {
                let can_add = remaining.min(max_stack);
                *slot = Some(
                    ItemStack::new(item_id, can_add)
                        .with_quality(quality)
                        .with_max_stack(max_stack),
                );
                remaining -= can_add;
            }
        }

        remaining
    }

    /// Remove items from inventory, returns actually removed amount
    pub fn remove_item(&mut self, item_id: &str, quantity: u32) -> u32 {
        let mut remaining = quantity;

        for slot in &mut self.slots {
            if remaining == 0 {
                break;
            }

            if let Some(stack) = slot {
                if stack.item_id == item_id {
                    remaining -= stack.remove(remaining);

                    // Remove empty stacks
                    if stack.quantity == 0 {
                        *slot = None;
                    }
                }
            }
        }

        quantity - remaining
    }

    /// Remove items with specific quality
    pub fn remove_item_with_quality(
        &mut self,
        item_id: &str,
        quantity: u32,
        quality: Quality,
    ) -> u32 {
        let mut remaining = quantity;

        for slot in &mut self.slots {
            if remaining == 0 {
                break;
            }

            if let Some(stack) = slot {
                if stack.item_id == item_id && stack.quality == quality {
                    remaining -= stack.remove(remaining);

                    if stack.quantity == 0 {
                        *slot = None;
                    }
                }
            }
        }

        quantity - remaining
    }

    /// Count total quantity of an item
    #[must_use]
    pub fn count_item(&self, item_id: &str) -> u32 {
        self.slots
            .iter()
            .filter_map(|s| s.as_ref())
            .filter(|s| s.item_id == item_id)
            .map(|s| s.quantity)
            .sum()
    }

    /// Check if inventory has at least N of an item
    #[must_use]
    pub fn has_item(&self, item_id: &str, quantity: u32) -> bool {
        self.count_item(item_id) >= quantity
    }

    /// Swap two slots
    pub fn swap_slots(&mut self, slot_a: usize, slot_b: usize) {
        if slot_a < self.slots.len() && slot_b < self.slots.len() {
            self.slots.swap(slot_a, slot_b);
        }
    }

    /// Clear a slot
    pub fn clear_slot(&mut self, slot: usize) -> Option<ItemStack> {
        if slot < self.slots.len() {
            self.slots[slot].take()
        } else {
            None
        }
    }

    /// Clear entire inventory
    pub fn clear(&mut self) {
        for slot in &mut self.slots {
            *slot = None;
        }
    }

    /// Get first empty slot index
    #[must_use]
    pub fn first_empty_slot(&self) -> Option<usize> {
        self.slots.iter().position(|s| s.is_none())
    }

    /// Count empty slots
    #[must_use]
    pub fn empty_slots(&self) -> usize {
        self.slots.iter().filter(|s| s.is_none()).count()
    }

    /// Check if inventory is full
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.empty_slots() == 0
    }

    /// Iterate over all slots
    pub fn iter(&self) -> impl Iterator<Item = (usize, Option<&ItemStack>)> {
        self.slots.iter().enumerate().map(|(i, s)| (i, s.as_ref()))
    }

    /// Iterate over hotbar slots only
    pub fn hotbar(&self) -> impl Iterator<Item = (usize, Option<&ItemStack>)> {
        self.slots
            .iter()
            .take(HOTBAR_SIZE)
            .enumerate()
            .map(|(i, s)| (i, s.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_inventory() {
        let inv = Inventory::new(10);
        assert_eq!(inv.size(), 10);
        assert_eq!(inv.empty_slots(), 10);
    }

    #[test]
    fn test_add_item_simple() {
        let mut inv = Inventory::new(10);
        let overflow = inv.add_item("wood", 50, Quality::Normal);

        assert_eq!(overflow, 0);
        assert_eq!(inv.count_item("wood"), 50);
    }

    #[test]
    fn test_add_item_stacking() {
        let mut inv = Inventory::new(10);
        inv.add_item("wood", 50, Quality::Normal);
        inv.add_item("wood", 30, Quality::Normal);

        // Should stack in same slot
        assert_eq!(inv.count_item("wood"), 80);
        assert_eq!(inv.empty_slots(), 9);
    }

    #[test]
    fn test_add_item_different_quality() {
        let mut inv = Inventory::new(10);
        inv.add_item("crop", 10, Quality::Normal);
        inv.add_item("crop", 10, Quality::Gold);

        // Different quality = different stacks
        assert_eq!(inv.count_item("crop"), 20);
        assert_eq!(inv.empty_slots(), 8);
    }

    #[test]
    fn test_add_item_overflow() {
        let mut inv = Inventory::new(2);
        let overflow = inv.add_item_with_max_stack("stone", 500, Quality::Normal, 100);

        // 2 slots * 100 = 200 max, 300 overflow
        assert_eq!(overflow, 300);
        assert_eq!(inv.count_item("stone"), 200);
    }

    #[test]
    fn test_remove_item() {
        let mut inv = Inventory::new(10);
        inv.add_item("wood", 100, Quality::Normal);

        let removed = inv.remove_item("wood", 30);
        assert_eq!(removed, 30);
        assert_eq!(inv.count_item("wood"), 70);
    }

    #[test]
    fn test_remove_item_partial() {
        let mut inv = Inventory::new(10);
        inv.add_item("wood", 50, Quality::Normal);

        let removed = inv.remove_item("wood", 100);
        assert_eq!(removed, 50);
        assert_eq!(inv.count_item("wood"), 0);
    }

    #[test]
    fn test_has_item() {
        let mut inv = Inventory::new(10);
        inv.add_item("wood", 50, Quality::Normal);

        assert!(inv.has_item("wood", 50));
        assert!(inv.has_item("wood", 30));
        assert!(!inv.has_item("wood", 100));
        assert!(!inv.has_item("stone", 1));
    }

    #[test]
    fn test_swap_slots() {
        let mut inv = Inventory::new(10);
        inv.add_item("wood", 50, Quality::Normal);
        inv.add_item("stone", 30, Quality::Normal);

        inv.swap_slots(0, 1);

        assert_eq!(inv.get(0).map(|s| s.item_id.as_str()), Some("stone"));
        assert_eq!(inv.get(1).map(|s| s.item_id.as_str()), Some("wood"));
    }

    #[test]
    fn test_hotbar_selection() {
        let mut inv = Inventory::new(36);

        assert_eq!(inv.selected_slot(), 0);

        inv.select_slot(5);
        assert_eq!(inv.selected_slot(), 5);

        inv.select_next();
        assert_eq!(inv.selected_slot(), 6);

        inv.select_slot(9);
        inv.select_next();
        assert_eq!(inv.selected_slot(), 0); // Wraps around
    }

    #[test]
    fn test_quality_multiplier() {
        assert!((Quality::Normal.price_multiplier() - 1.0).abs() < 0.01);
        assert!((Quality::Silver.price_multiplier() - 1.25).abs() < 0.01);
        assert!((Quality::Gold.price_multiplier() - 1.5).abs() < 0.01);
        assert!((Quality::Iridium.price_multiplier() - 2.0).abs() < 0.01);
    }
}
