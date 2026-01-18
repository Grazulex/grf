---
id: 10
title: Collision AABB et spatial grid
status: To Do
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
updated_date: '2026-01-18T20:13:21.709Z'
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
acceptance_criteria:
  - text: 'AABB::intersects detecte les collisions'
    checked: false
  - text: SpatialGrid reduit le nombre de tests
    checked: false
  - text: CollisionInfo contient overlap et normal
    checked: false
  - text: Performance O(n) au lieu de O(n^2)
    checked: false
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
---
Implementer engine_physics avec AABB et partitionnement spatial
