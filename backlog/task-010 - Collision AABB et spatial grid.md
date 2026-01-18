---
id: 10
title: Collision AABB et spatial grid
status: Done
priority: critical
milestone: MVP2-Monde
assignees:
  - '@claude'
labels:
  - phase2
  - monde
  - collision
  - physics
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:09:08.526Z'
updated_date: '2026-01-18T21:15:29.077Z'
closed_date: '2026-01-18T21:15:29.077Z'
changelog:
  - timestamp: '2026-01-18T20:09:08.526Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T20:13:18.790Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:13:19.784Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:20.432Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:21.079Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:21.709Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:10:16.466Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T21:15:04.006Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:15:13.386Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:15:23.177Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:15:23.814Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:15:24.446Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:15:25.075Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:15:29.077Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: 'AABB::intersects detecte les collisions'
    checked: true
  - text: SpatialGrid reduit le nombre de tests
    checked: true
  - text: CollisionInfo contient overlap et normal
    checked: true
  - text: Performance O(n) au lieu de O(n^2)
    checked: true
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Implementer collision AABB avec spatial grid pour performance.

  ### Etapes
  1. Creer struct AABB avec min/max Vec2
  2. Implementer intersects, contains_point
  3. Creer struct SpatialGrid avec HashMap<(i32,i32), Vec<Entity>>
  4. Implementer insert, remove, query
  5. Creer CollisionSystem qui utilise la grid
  6. Retourner CollisionInfo avec overlap et normal

  ### Fichiers concernes
  - crates/engine_physics/src/aabb.rs (create)
  - crates/engine_physics/src/spatial_grid.rs (create)
  - crates/engine_physics/src/collision.rs (create)

  ### Dependances
  - Task #14 (ECS pour Entity type)

  ### Approche technique
  - Cell size configurable (64-128 px)
  - Entites dans plusieurs cells si elles chevauchent
  - Query retourne candidats, test AABB ensuite
ai_notes: >
  **2026-01-18T21:15:04.006Z** - **22:15** - PROGRESS: Integrated AABB collision
  with player movement
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] AABB struct with intersects, contains_point
  - [x] CollisionInfo with overlap, normal, mtv
  - [x] SpatialGrid for broad-phase collision
  - [x] from_center and translated helper methods
  - [x] Integration with player movement
  - [x] World bounds collision using AABB

  ### Tests
  - test_aabb_intersects: OK
  - test_aabb_contains_point: OK
  - test_collision_info: OK
  - test_spatial_grid_query: OK
  - test_spatial_grid_update: OK

  ### Quality
  - Standards respectes: Oui
  - Documentation: Complete

  ### Notes
  - SpatialGrid ready for future entity-entity collisions
  - Player.aabb() method prepared for tile collision (Task #11)
---
Implementer engine_physics avec AABB et partitionnement spatial
