---
id: 14
title: Implementation ECS custom
status: Done
priority: critical
milestone: MVP3-ECS
assignees:
  - '@claude'
labels:
  - phase3
  - ecs
  - architecture
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:09:25.631Z'
updated_date: '2026-01-18T21:38:30.149Z'
closed_date: '2026-01-18T21:38:30.149Z'
changelog:
  - timestamp: '2026-01-18T20:09:25.631Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T20:13:32.615Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:13:33.573Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:34.209Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:34.849Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:35.526Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:36.228Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:33:02.087Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T21:33:12.681Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:36:45.774Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:37:13.981Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:37:23.324Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:37:23.988Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:37:24.654Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:37:25.342Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:38:29.496Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:38:30.149Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Entity spawn/despawn fonctionne
    checked: true
  - text: Components peuvent etre ajoutes/retires
    checked: true
  - text: Queries retournent les bonnes entites
    checked: true
  - text: Resources (singletons) accessibles
    checked: true
  - text: Generational index previent les dangling refs
    checked: true
ai_plan: |-
  ## Plan d'implementation

  ### Objectif
  Creer un ECS (Entity-Component-System) custom simple

  ### Etapes
  1. Entity = u32 ID avec generation counter
  2. Component storage avec SparseSet
  3. World struct pour gerer entities et components
  4. Query system pour iterer sur components
  5. Tests unitaires

  ### Fichiers concernes
  - engine_ecs/src/lib.rs (modify)
  - engine_ecs/src/entity.rs (create)
  - engine_ecs/src/component.rs (create)
  - engine_ecs/src/world.rs (create)
  - engine_ecs/src/query.rs (create)

  ### Approche technique
  - Generational indices pour entities
  - SparseSet pour O(1) component access
  - Type-erased storage avec Any
ai_notes: >
  **2026-01-18T21:36:45.773Z** - **22:36** - COMPLETED: ECS implementation with
  all 7 tests passing. SparseSet storage, World struct with
  spawn/despawn/insert/remove/get, QueryIter and QueryIterMut for iteration. No
  warnings.
ai_review: >-
  ## Self-Review


  ### Complete

  - [x] Entity struct with generational indices

  - [x] SparseSet<T> for O(1) component storage

  - [x] ComponentStorage trait for type erasure

  - [x] World struct with spawn/despawn

  - [x] Component insert/remove/get/get_mut/has

  - [x] QueryIter for immutable iteration

  - [x] QueryIterMut for mutable iteration

  - [x] 7 unit tests covering all functionality


  ### Tests effectues

  - test_spawn_despawn: OK

  - test_generation_reuse: OK

  - test_insert_get_component: OK

  - test_remove_component: OK

  - test_query: OK

  - test_query_mut: OK

  - test_despawn_removes_components: OK


  ### Qualite du code

  - Standards respectes: Oui

  - Documentation: Complete (doc comments)

  - No warnings after cleanup


  ### Limitations connues

  - Single-component queries only (no multi-component queries like Query<(&A,
  &B)>)

  - No system scheduler (manual system calls needed)

  - QueryIterMut uses unsafe for split borrows


  ### Recommandations

  - Task #015 will refactor game code to use this ECS

  - Multi-component queries can be added later if needed
---
Implementer engine_ecs avec World, Entity, Component storage et queries
