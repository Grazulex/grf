---
id: 7
title: Camera 2D basique
status: Done
priority: high
milestone: MVP1-Fondations
assignees:
  - '@claude'
labels:
  - phase1
  - fondations
  - camera
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:08:49.026Z'
updated_date: '2026-01-18T20:56:24.118Z'
closed_date: '2026-01-18T20:56:24.118Z'
changelog:
  - timestamp: '2026-01-18T20:08:49.026Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T20:12:39.669Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:12:40.628Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:41.266Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:41.909Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:42.553Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:43.192Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:51:22.838Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T20:56:00.995Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:56:11.555Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:56:17.357Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:56:17.985Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:56:18.649Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:56:19.294Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:56:19.947Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:56:24.118Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: world_to_screen transforme correctement
    checked: true
  - text: screen_to_world est l'inverse exact
    checked: true
  - text: Le zoom fonctionne (0.5x a 4x)
    checked: true
  - text: Smooth follow suit une cible avec delai
    checked: true
  - text: visible_bounds retourne les bounds corrects
    checked: true
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Implementer Camera2D avec transformations et smooth follow.

  ### Etapes
  1. Creer struct Camera2D avec position, zoom, rotation, viewport
  2. Implementer world_to_screen transformation
  3. Implementer screen_to_world transformation
  4. Implementer visible_bounds pour culling
  5. Ajouter smooth follow avec lerp
  6. Integrer avec le renderer (view matrix)

  ### Fichiers concernes
  - crates/engine_render/src/camera.rs (create)
  - crates/engine_render/src/lib.rs (modify)

  ### Dependances
  - Task #4 (Sprite rendering)

  ### Approche technique
  - Camera centree (position = center of view)
  - Zoom 1.0 = pixels 1:1
  - Smooth follow avec lerp configurable
ai_notes: >
  **2026-01-18T20:56:00.995Z** - **20:55** - PROGRESS: Camera2D implemented with
  world_to_screen, screen_to_world, visible_bounds, smooth follow with
  exponential lerp. Integrated with Renderer via set_camera() and
  SpriteBatch.set_view_matrix(). Game updated with camera following player in
  2000x2000 world. All 4 unit tests pass.
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] Camera2D struct avec position, zoom, viewport
  - [x] world_to_screen transformation
  - [x] screen_to_world transformation (inverse exact)
  - [x] visible_bounds pour culling
  - [x] is_point_visible et is_rect_visible
  - [x] Smooth follow avec exponential lerp
  - [x] view_matrix() pour GPU
  - [x] Integration avec Renderer et SpriteBatch
  - [x] Gestion du resize viewport

  ### Tests effectues
  - 4 unit tests: OK
  - cargo run: OK - camera suit le joueur
  - clippy: clean

  ### Qualite du code
  - Standards respectes: Oui
  - Documentation: Complete avec doc comments
  - Tests unitaires: 4 tests couvrant les transformations

  ### Limitations connues
  - Rotation camera non implementee (pas necessaire pour l'instant)

  ### Recommandations
  - Phase MVP1-Fondations complete, passer a MVP2-Monde
---
Implementer Camera2D avec world_to_screen, screen_to_world et smooth follow
