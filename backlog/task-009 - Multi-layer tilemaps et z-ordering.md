---
id: 9
title: Multi-layer tilemaps et z-ordering
status: Done
priority: critical
milestone: MVP2-Monde
assignees:
  - '@claude'
labels:
  - phase2
  - monde
  - tilemap
  - zorder
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:09:07.717Z'
updated_date: '2026-01-18T21:09:17.268Z'
closed_date: '2026-01-18T21:09:17.268Z'
changelog:
  - timestamp: '2026-01-18T20:09:07.717Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T21:05:26.402Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T21:05:27.114Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:05:27.826Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:05:28.509Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:05:29.213Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:08:51.165Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:09:02.934Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:09:09.135Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:09:09.844Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:09:10.563Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:09:11.394Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:09:17.268Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Plusieurs layers se superposent correctement
    checked: true
  - text: Z-order respecte pour le rendu
    checked: true
  - text: Y-sort fonctionne pour les entites
    checked: true
  - text: Player s'affiche entre les layers ground et above
    checked: true
ai_notes: >
  **2026-01-18T21:08:51.165Z** - **21:10** - PROGRESS: Added LayerType enum
  (Below/Above) and z_order to TileLayer. Created below_layers() and
  above_layers() methods. Updated test map with 3 layers (ground, decorations,
  above_player). Game renders: below layers -> player -> above layers. FPS
  stable at 60.
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] LayerType enum (Below/Above)
  - [x] z_order field pour tri intra-type
  - [x] below_layers() retourne indices tries
  - [x] above_layers() retourne indices tries
  - [x] Test map avec 3 layers (ground, decorations below, above_player)
  - [x] Rendu: below -> entities -> above

  ### Tests effectues
  - 6 unit tests: OK
  - cargo run: OK - layers se superposent correctement
  - Player visible entre les layers
  - clippy: clean

  ### Qualite du code
  - Standards respectes: Oui
  - API intuitive: tilemap.below_layers(), tilemap.above_layers()

  ### Notes Y-sort
  - Y-sort basique implemente (player seul pour l'instant)
  - Multi-entity Y-sort a implementer avec ECS (Task #14-15)

  ### Recommandations
  - Task #10 (Collision AABB) next
---
Support des layers multiples avec z-ordering et Y-sort pour effet top-down
