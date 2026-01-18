---
id: 29
title: Z-order debug et layer visualization
status: Done
priority: medium
milestone: MVP5-Debug
assignees:
  - '@claude'
labels:
  - phase5
  - debug
  - render
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:10:21.497Z'
updated_date: '2026-01-18T22:52:21.864Z'
closed_date: '2026-01-18T22:52:21.864Z'
changelog:
  - timestamp: '2026-01-18T20:10:21.497Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T22:44:47.302Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T22:44:59.632Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:45:04.657Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:45:05.291Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:45:05.948Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:45:06.588Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:47:13.968Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:52:01.362Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:52:13.711Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:52:19.222Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:52:19.865Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:52:20.520Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:52:21.201Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:52:21.864Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Labels Z-order sur entites
    checked: true
  - text: Legende des layers visible
    checked: true
  - text: Toggle Ctrl+Z fonctionnel
    checked: true
  - text: Integration avec le rendu existant
    checked: true
ai_plan: >-
  ## Plan d'implementation


  ### Objectif

  Ajouter une visualisation du z-order avec labels sur les entites et une
  legende des layers.


  ### Etapes

  1. Ajouter ZOrderDebugData struct dans overlay.rs pour collecter les z-orders

  2. Ajouter methodes pour collecter z-order: set_zorder_data(),
  add_entity_zorder()

  3. Implementer render_zorder_labels() pour afficher les labels sur les entites

  4. Ajouter une legende des layers (window egui) montrant l'ordre

  5. Ajouter Ctrl+Z toggle dans game/src/main.rs

  6. Collecter les z-orders des entites et tiles dans render()


  ### Fichiers concernes

  - crates/engine_debug/src/overlay.rs (modify) - ZOrderDebugData, render
  methods

  - crates/engine_debug/src/lib.rs (modify) - export ZOrderDebugData si
  necessaire

  - game/src/main.rs (modify) - Ctrl+Z toggle, z-order collection


  ### Approche technique

  - Utiliser egui painter pour labels (comme collision boxes)

  - Z-order = position Y pour entities (y-sorting)

  - Layers = tilemap layers avec leur index

  - Legende affiche: Ground (0), Decor (1), Entities (y-sorted), Above (3+)
ai_notes: >
  **2026-01-18T22:47:13.968Z** - **23:47** - PROGRESS: Analyzed overlay.rs
  structure. Will add ZOrderLabel struct and render methods alongside existing
  collision visualization.

  **2026-01-18T22:52:01.362Z** - **23:52** - PROGRESS: Z-order visualization
  fully implemented. Features:

  - ZOrderLabel and LayerInfo structs for data collection

  - render_zorder_labels() displays labels on entities with black background for
  readability

  - render_layer_legend() shows standard layer order and dynamic frame info

  - Ctrl+Z toggle in game update()

  - Z-order data collection in render() with entity position.y as z-order

  - Menu bar has Z-Order button

  - Build successful
ai_review: >-
  ## Self-Review


  ### Complete

  - [x] ZOrderLabel struct with position, z_order, label, color

  - [x] LayerInfo struct with index, name, color, count

  - [x] ZOrderDebugData struct for collection

  - [x] Methods: should_show_zorder(), set_zorder_data(), add_zorder_label(),
  add_layer_info()

  - [x] render_zorder_labels() with black background for readability

  - [x] render_layer_legend() with standard layers and dynamic frame info

  - [x] Z-Order button in menu bar

  - [x] Ctrl+Z toggle in game update()

  - [x] Z-order data collection for entities in render()

  - [x] Layer info collection from tilemap


  ### Tests effectues

  - Build: OK (cargo build --features debug-tools)

  - Game launch: OK

  - No runtime errors


  ### Qualite du code

  - Standards respectes: Oui

  - Documentation: Complete (doc comments on public items)

  - Feature-gated: Oui (debug-tools)


  ### Limitations connues

  - Z-order labels only show on entities with Position component

  - Layer count is approximate (below_layers, entities, above_layers)


  ### Recommandations

  - Could add NPC name to z-order labels in future

  - Could show tilemap layer names from JSON
---
Visualisation z-order avec labels et legende (Ctrl+Z)
