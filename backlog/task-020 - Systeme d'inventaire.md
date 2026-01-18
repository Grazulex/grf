---
id: 20
title: Systeme d'inventaire
status: Done
priority: critical
milestone: MVP4-Gameplay
assignees:
  - '@claude'
labels:
  - phase4
  - gameplay
  - inventory
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:09:51.872Z'
updated_date: '2026-01-18T21:59:04.436Z'
closed_date: '2026-01-18T21:59:04.436Z'
changelog:
  - timestamp: '2026-01-18T20:09:51.872Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T20:13:46.647Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:13:47.616Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:48.257Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:48.907Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:49.578Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:50.236Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:55:16.818Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T21:58:39.760Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:58:52.957Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:58:57.235Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:58:57.869Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:58:58.509Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:58:59.178Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:58:59.836Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:59:04.436Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Inventaire avec N slots configurable
    checked: true
  - text: add_item stack automatiquement
    checked: true
  - text: Gestion des slots pleins (retourne overflow)
    checked: true
  - text: Selected slot pour hotbar
    checked: true
  - text: Serialisation serde pour save/load
    checked: true
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Implementer le systeme d'inventaire avec slots et stacking.

  ### Etapes
  1. Creer struct ItemStack (item_id, quantity, quality)
  2. Creer struct Inventory avec Vec<Option<ItemStack>>
  3. Implementer add_item avec stacking automatique
  4. Implementer remove_item, swap_slots
  5. Creer component Inventory pour les entites
  6. Gerer selected_slot pour hotbar

  ### Fichiers concernes
  - game/src/components/inventory.rs (create)
  - game/src/components/item.rs (create)

  ### Dependances
  - Task #14 (ECS)
  - Task #21 (Item definitions)

  ### Approche technique
  - Slots = Option<ItemStack> (None = vide)
  - Stacking jusqu'a max_stack de l'item
  - Quality enum (Normal, Silver, Gold, Iridium)
ai_notes: >
  **2026-01-18T21:58:39.759Z** - **22:00** - COMPLETED: Created inventory.rs
  with Quality enum (Normal/Silver/Gold/Iridium), ItemStack struct, Inventory
  struct with Vec<Option<ItemStack>>. Implemented add_item with auto-stacking,
  remove_item, swap_slots, count_item, has_item. Added hotbar selection (slots
  0-9). Serde serialization. 11 tests passed.
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] Quality enum (Normal, Silver, Gold, Iridium)
  - [x] ItemStack struct with item_id, quantity, quality, max_stack
  - [x] Inventory struct with Vec<Option<ItemStack>>
  - [x] add_item with auto-stacking logic
  - [x] add_item_with_max_stack for custom limits
  - [x] remove_item and remove_item_with_quality
  - [x] swap_slots, clear_slot, clear
  - [x] count_item, has_item
  - [x] selected_slot, select_next, select_prev (hotbar)
  - [x] Serde Serialize/Deserialize
  - [x] 11 unit tests

  ### Tests effectues
  - test_new_inventory: OK
  - test_add_item_simple: OK
  - test_add_item_stacking: OK
  - test_add_item_different_quality: OK
  - test_add_item_overflow: OK
  - test_remove_item: OK
  - test_remove_item_partial: OK
  - test_has_item: OK
  - test_swap_slots: OK
  - test_hotbar_selection: OK
  - test_quality_multiplier: OK

  ### Qualite du code
  - Standards respectes: Oui
  - Documentation: Complete
  - No warnings (dead_code allowed for future use)

  ### API
  - Inventory::new(size) / ::default() (36 slots)
  - add_item(id, qty, quality) -> overflow
  - remove_item(id, qty) -> removed
  - get(slot), get_mut(slot), is_empty(slot)
  - swap_slots(a, b), clear_slot(slot), clear()
  - count_item(id), has_item(id, qty)
  - selected_slot(), select_slot(n), select_next(), select_prev()
  - iter(), hotbar() iterators

  ### Constantes
  - DEFAULT_MAX_STACK = 999
  - DEFAULT_INVENTORY_SIZE = 36
  - HOTBAR_SIZE = 10
---
Inventory component avec slots, stacking, item definitions TOML
