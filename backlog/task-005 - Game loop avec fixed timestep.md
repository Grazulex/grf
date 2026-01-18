---
id: 5
title: Game loop avec fixed timestep
status: Done
priority: critical
milestone: MVP1-Fondations
assignees:
  - '@claude'
labels:
  - phase1
  - fondations
  - core
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:08:47.433Z'
updated_date: '2026-01-18T20:43:44.069Z'
closed_date: '2026-01-18T20:43:44.069Z'
changelog:
  - timestamp: '2026-01-18T20:08:47.433Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T20:12:12.824Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:12:13.767Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:14.380Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:15.018Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:15.674Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:16.307Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:40:42.538Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T20:43:11.665Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:43:13.685Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:43:25.920Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:43:33.130Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:43:34.269Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:43:35.335Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:43:36.394Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:43:37.446Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:43:44.069Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Game loop tourne a 60 updates/sec fixes
    checked: true
  - text: Render tourne aussi vite que possible
    checked: true
  - text: Delta time est calcule correctement
    checked: true
  - text: Alpha d'interpolation disponible pour le render
    checked: true
  - text: FPS counter affiche les stats
    checked: true
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Implementer la game loop avec fixed timestep pour physique deterministe.

  ### Etapes
  1. Creer struct GameTime avec delta, total, frame_count
  2. Implementer fixed timestep accumulator pattern
  3. Separer update (fixed 60Hz) et render (variable)
  4. Calculer alpha pour interpolation
  5. Integrer avec Window event loop
  6. Ajouter FPS counter basique

  ### Fichiers concernes
  - crates/engine_core/src/time.rs (create)
  - crates/engine_core/src/game_loop.rs (create)
  - crates/engine_core/src/lib.rs (modify)

  ### Dependances
  - Task #2 (Window)

  ### Approche technique
  - FIXED_TIMESTEP = 1.0/60.0 (60 UPS)
  - Accumulator pattern pour fixed update
  - Alpha = accumulator / FIXED_TIMESTEP pour interpolation
ai_notes: >
  **2026-01-18T20:43:11.664Z** - **20:42** - PROGRESS: Enhanced GameTime with
  UPS tracking and MAX_DELTA protection

  **2026-01-18T20:43:13.684Z** - **20:42** - PROGRESS: Demo with bouncing sprite
  using fixed_update + interpolation
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] GameTime avec fixed timestep (60 UPS)
  - [x] Accumulator pattern implementé
  - [x] Delta time calculé correctement
  - [x] MAX_DELTA pour éviter spiral of death
  - [x] Alpha d'interpolation (0.0-1.0)
  - [x] FPS counter (smoothed)
  - [x] UPS counter (smoothed)
  - [x] Demo sprite rebondissant avec interpolation

  ### Tests effectues
  - Build: OK
  - Clippy: OK
  - FPS: ~60 (vsync)
  - UPS: ~60 (fixed timestep)
  - Interpolation: fluide
  - Bouncing: correct

  ### Fichiers modifies
  - engine_core/src/time.rs (enhanced)

  ### Notes
  - update_count tracke les fixed updates
  - fps_timer accumule sur 1 seconde pour smoothing
  - Sprite rebondit avec prev_position.lerp(position, alpha)
---
Implementer la game loop avec fixed timestep (60 UPS) et interpolation pour le rendu
