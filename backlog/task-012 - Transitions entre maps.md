---
id: 12
title: Transitions entre maps
status: Done
priority: high
milestone: MVP2-Monde
assignees:
  - '@claude'
labels:
  - phase2
  - monde
  - transition
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:09:10.170Z'
updated_date: '2026-01-18T21:24:33.011Z'
closed_date: '2026-01-18T21:24:33.011Z'
changelog:
  - timestamp: '2026-01-18T20:09:10.170Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T21:20:14.243Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T21:20:25.581Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:24:07.888Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:24:17.267Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:24:24.284Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:24:24.957Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:24:25.630Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:24:26.339Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:24:27.036Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:24:27.825Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:24:33.011Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Triggers et SpawnPoints dans Tilemap
    checked: true
  - text: 2 maps de test avec transitions
    checked: true
  - text: Player teleporte au spawn correct
    checked: true
ai_plan: |-
  ## Plan d'implementation

  ### Objectif
  Permettre les transitions entre maps via des zones de trigger

  ### Etapes
  1. Ajouter struct Trigger (zone AABB + target_map + spawn_id)
  2. Ajouter struct SpawnPoint (position + id)
  3. Ajouter triggers et spawns a la Tilemap
  4. Creer une 2eme map de test (house.json)
  5. Implementer detection de trigger et chargement de map

  ### Fichiers concernes
  - engine_render/src/tilemap.rs (modify) - triggers et spawns
  - game/src/main.rs (modify) - logique de transition
  - assets/maps/house.json (create) - 2eme map de test
  - assets/maps/test.json (modify) - ajouter triggers

  ### Approche technique
  - Triggers stockes dans la Tilemap (JSON)
  - Spawns identifies par ID string
  - Game detecte collision avec trigger et charge la map cible
ai_notes: >
  **2026-01-18T21:24:07.888Z** - **22:24** - PROGRESS: Transitions entre maps
  implementees - test.json <-> house.json
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] SpawnPoint struct avec id, x, y
  - [x] Trigger struct avec bounds et target
  - [x] get_spawn, default_spawn, check_trigger methods
  - [x] house.json map (interieur maison)
  - [x] test.json avec spawns et triggers
  - [x] Game.load_map method
  - [x] Transition detection dans update loop

  ### Tests
  - All 12 tests pass

  ### Quality
  - Standards respectes: Oui
  - Documentation: Complete

  ### Notes
  - Player peut entrer dans la maison (top-left of outdoor map)
  - Player peut sortir de la maison (bottom door)
  - Camera snap au changement de map
---
Systeme de triggers pour transitions entre maps avec spawn points
