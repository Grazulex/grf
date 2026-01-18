---
id: 11
title: Tile-based collision map
status: Done
priority: high
milestone: MVP2-Monde
assignees:
  - '@claude'
labels:
  - phase2
  - monde
  - collision
  - tilemap
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:09:09.370Z'
updated_date: '2026-01-18T21:19:58.050Z'
closed_date: '2026-01-18T21:19:58.050Z'
changelog:
  - timestamp: '2026-01-18T20:09:09.370Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T21:15:50.273Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T21:15:59.878Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:19:22.956Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:19:31.574Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:19:44.972Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:19:45.793Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:19:46.600Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:19:51.243Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:19:51.964Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:19:52.662Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:19:58.050Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Tilemap a un champ collision
    checked: true
  - text: is_tile_solid et get_solid_tiles_in_rect implementes
    checked: true
  - text: Player ne peut pas traverser les tiles solides
    checked: true
ai_plan: |-
  ## Plan d'implementation

  ### Objectif
  Permettre au joueur de collisionner avec des tiles solides

  ### Etapes
  1. Ajouter un champ collision layer a la Tilemap (Vec<bool>)
  2. Creer une methode get_collision_tiles qui retourne les AABBs solides
  3. Mettre a jour le test.json avec des tiles solides (eau = solide)
  4. Integrer la collision avec le Player.fixed_update

  ### Fichiers concernes
  - engine_render/src/tilemap.rs (modify) - ajouter collision layer
  - game/src/main.rs (modify) - utiliser collision tiles
  - assets/maps/test.json (modify) - ajouter collision data

  ### Approche technique
  - Layer de collision separee des layers visuelles
  - Query des tiles dans le bounds du joueur
  - Resolution de collision avec MTV
ai_notes: >
  **2026-01-18T21:19:22.955Z** - **22:19** - PROGRESS: Tile collision integrated
  - player cannot walk through water tiles
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] Added collision field to Tilemap struct
  - [x] is_tile_solid method
  - [x] get_solid_tiles_in_rect method
  - [x] has_collision method
  - [x] Updated test.json with water collision data
  - [x] Integrated collision in Player.fixed_update

  ### Tests
  - test_tile_collision: OK (new test)
  - All 12 workspace tests pass

  ### Quality
  - Standards respectes: Oui
  - Documentation: Complete

  ### Notes
  - Water tiles (center pond) are now solid
  - MTV used for collision resolution
  - Out-of-bounds tiles treated as solid
---
Collision basee sur la tilemap avec layer de collision dediee
