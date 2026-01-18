---
id: 20
title: Systeme d'inventaire
status: To Do
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
updated_date: '2026-01-18T20:13:50.236Z'
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
acceptance_criteria:
  - text: Inventaire avec N slots configurable
    checked: false
  - text: add_item stack automatiquement
    checked: false
  - text: Gestion des slots pleins (retourne overflow)
    checked: false
  - text: Selected slot pour hotbar
    checked: false
  - text: Serialisation serde pour save/load
    checked: false
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
---
Inventory component avec slots, stacking, item definitions TOML
