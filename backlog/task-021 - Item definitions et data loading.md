---
id: 21
title: Item definitions et data loading
status: Done
priority: high
milestone: MVP4-Gameplay
assignees:
  - '@claude'
labels:
  - phase4
  - gameplay
  - items
  - data
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:09:52.754Z'
updated_date: '2026-01-18T22:02:28.970Z'
closed_date: '2026-01-18T22:02:28.970Z'
changelog:
  - timestamp: '2026-01-18T20:09:52.754Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T21:59:27.036Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T21:59:38.284Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:59:43.672Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:59:44.305Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:59:44.968Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:59:45.601Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:59:46.290Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:02:04.324Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:02:16.976Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:02:21.902Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:02:22.551Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:02:23.205Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:02:23.897Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:02:24.566Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:02:28.970Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: ItemType enum avec categories
    checked: true
  - text: ItemDefinition struct complete
    checked: true
  - text: Chargement depuis TOML
    checked: true
  - text: ItemDatabase avec lookup par id
    checked: true
  - text: Fichier items.toml avec exemples
    checked: true
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Charger les definitions d'items depuis TOML avec types categorises.

  ### Etapes
  1. Creer ItemType enum (Tool, Seed, Crop, Resource, Consumable)
  2. Creer ItemDefinition struct avec id, name, type, max_stack, sell_price
  3. Creer ItemDatabase pour stocker toutes les definitions
  4. Implementer le chargement depuis TOML
  5. Creer le fichier assets/data/items.toml avec exemples
  6. Ajouter les criteres d'acceptation

  ### Fichiers concernes
  - game/src/items.rs (create)
  - assets/data/items.toml (create)

  ### Dependances
  - Task #20 (Inventory) - utilise item_id pour referencer les items

  ### Approche technique
  - ItemDefinition = metadonnees statiques (nom, prix, stack max)
  - ItemStack dans Inventory = instance runtime (quantite, qualite)
  - TOML format lisible pour les game designers
ai_notes: >
  **2026-01-18T22:02:04.324Z** - **22:10** - COMPLETED: Created items.rs with
  ItemType enum (Tool, Seed, Crop, Resource, Consumable, Furniture, Special),
  ItemDefinition struct, ItemDatabase with TOML loading. Created items.toml with
  40 items (6 tools, 9 seeds, 9 crops, 11 resources, 5 consumables). 7 unit
  tests.
ai_review: >-
  ## Self-Review


  ### Complete

  - [x] ItemType enum: Tool, Seed, Crop, Resource, Consumable, Furniture,
  Special

  - [x] ToolType enum: Hoe, WateringCan, Axe, Pickaxe, Scythe, FishingRod

  - [x] ItemDefinition struct with all properties

  - [x] ItemDatabase with HashMap storage

  - [x] load_from_file() and load_from_str() methods

  - [x] get(), get_by_type(), contains(), iter()

  - [x] ItemLoadError with Display/Error impl

  - [x] items.toml with 40 items

  - [x] 7 unit tests


  ### Tests effectues

  - test_load_items: OK

  - test_get_item: OK

  - test_get_by_type: OK

  - test_item_properties: OK

  - test_consumable: OK

  - test_default_max_stack: OK

  - test_contains: OK


  ### Qualite du code

  - Standards respectes: Oui

  - Documentation: Complete

  - Serde derive for serialization

  - Error handling with custom type


  ### API

  - ItemDatabase::new()

  - load_from_file(path) / load_from_str(content)

  - get(id), contains(id), get_by_type(type)

  - all_ids(), len(), is_empty(), iter()

  - ItemDefinition: is_tool(), is_seed(), is_stackable(), is_edible()


  ### Fichier items.toml

  - 6 outils

  - 9 graines (spring, summer, fall)

  - 9 crops correspondants

  - 11 ressources (bois, minerais, barres)

  - 5 consommables
---
Chargement des definitions d'items depuis TOML avec types (Tool, Seed, Crop, etc)
