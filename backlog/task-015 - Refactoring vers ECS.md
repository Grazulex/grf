---
id: 15
title: Refactoring vers ECS
status: Done
priority: critical
milestone: MVP3-ECS
assignees:
  - '@claude'
labels:
  - phase3
  - ecs
  - refactor
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:09:26.503Z'
updated_date: '2026-01-18T21:42:58.861Z'
closed_date: '2026-01-18T21:42:58.861Z'
changelog:
  - timestamp: '2026-01-18T20:09:26.503Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T21:38:48.176Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T21:39:05.982Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:39:12.389Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:39:13.079Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:39:13.733Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:39:14.405Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:42:25.340Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:42:50.835Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:42:55.922Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:42:56.651Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:42:57.358Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:42:58.080Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:42:58.861Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Joueur est une entite avec components
    checked: true
  - text: Input/Camera/Tilemap sont des Resources
    checked: true
  - text: movement_system gere deplacement et collisions
    checked: true
  - text: Le jeu fonctionne comme avant
    checked: true
ai_plan: >-
  ## Plan d'implementation


  ### Objectif

  Migrer le jeu de la structure monolithique Player/Game vers une architecture
  ECS propre.


  ### Etapes

  1. Creer les components dans game/src/components.rs:
     - Position (x, y, prev_x, prev_y pour interpolation)
     - Velocity (x, y)
     - PlayerControlled (marker pour le joueur)
     - CameraTarget (la camera suit cette entite)
     - SpriteRender (size, texture_id)
     - Collider (aabb size)

  2. Creer les systems dans game/src/systems.rs:
     - input_system: lit Input, modifie Velocity des PlayerControlled
     - movement_system: applique Velocity + collisions
     - camera_system: suit les entites CameraTarget
     - render_system: dessine les SpriteRender

  3. Refactorer Game struct:
     - Remplacer Player par World ECS
     - Stocker Input, Tilemap, Camera comme Resources
     - Appeler les systems dans update/render

  ### Fichiers concernes

  - game/src/main.rs (modify - major refactor)

  - game/src/components.rs (create)

  - game/src/systems.rs (create)

  - crates/engine_ecs/src/lib.rs (may need minor additions)


  ### Approche technique

  - Migrer incrementalement (garder le jeu fonctionnel)

  - Player devient une entite avec components

  - Resources pour les singletons (Input, Camera, Tilemap)


  ### Defis potentiels

  - Gestion des lifetimes avec les borrows du World

  - Acces simultane a plusieurs components dans un system
ai_notes: >
  **2026-01-18T21:42:25.339Z** - **22:42** - PROGRESS: All components and
  systems created. main.rs refactored to use ECS World. Player is now an entity
  with Position, Velocity, PlayerControlled, CameraTarget, SpriteRender,
  Collider. Input/Camera/Tilemap stored as Resources.
ai_review: >-
  ## Self-Review


  ### Complete

  - [x] game/src/components.rs created with Position, Velocity,
  PlayerControlled, CameraTarget, SpriteRender, Collider

  - [x] game/src/systems.rs created with input_system, movement_system,
  camera_system

  - [x] main.rs refactored to use ECS World

  - [x] Player is now an entity with components

  - [x] Input, Camera2D, Tilemap stored as Resources

  - [x] All tests pass (24 tests)

  - [x] Game compiles and runs


  ### Tests effectues

  - cargo test --workspace: OK (24 passed)

  - cargo run: OK (game starts, shows tilemap and player)


  ### Qualite du code

  - Standards respectes: Oui

  - Documentation: Complete (doc comments on components)

  - No warnings after cleanup


  ### Architecture

  - Components: Position, Velocity, PlayerControlled, CameraTarget,
  SpriteRender, Collider

  - Systems: input_system, movement_system, camera_system

  - Resources: Input, Camera2D, Tilemap


  ### Limitations connues

  - Uses unsafe for split borrows in systems and render (necessary for ECS
  pattern)

  - Single-component queries only

  - No Y-sorting for entities yet


  ### Recommandations

  - Task #016 (Resource manager) will improve asset handling

  - Future: add Y-sorting for proper depth ordering
---
Migrer le code existant vers l'architecture ECS (components et systems)
