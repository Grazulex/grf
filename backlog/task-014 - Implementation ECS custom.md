---
id: 14
title: Implementation ECS custom
status: To Do
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
updated_date: '2026-01-18T20:13:36.228Z'
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
acceptance_criteria:
  - text: Entity spawn/despawn fonctionne
    checked: false
  - text: Components peuvent etre ajoutes/retires
    checked: false
  - text: Queries retournent les bonnes entites
    checked: false
  - text: Resources (singletons) accessibles
    checked: false
  - text: Generational index previent les dangling refs
    checked: false
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Implementer un ECS custom simple et performant.

  ### Etapes
  1. Creer Entity type (generational index u64)
  2. Creer trait Component (marker trait)
  3. Implementer ComponentStorage<T> avec Vec sparse
  4. Creer struct World avec entity allocator et storages
  5. Implementer world.spawn(), world.despawn()
  6. Implementer world.insert(), world.get(), world.get_mut()
  7. Creer Query system avec iterateurs
  8. Ajouter Resources (singletons globaux)

  ### Fichiers concernes
  - crates/engine_ecs/src/entity.rs (create)
  - crates/engine_ecs/src/component.rs (create)
  - crates/engine_ecs/src/world.rs (create)
  - crates/engine_ecs/src/query.rs (create)
  - crates/engine_ecs/src/resource.rs (create)

  ### Approche technique
  - Generational index pour eviter dangling refs
  - Sparse set storage pour components
  - Query retourne iterateur sur tuples
  - TypeId pour identifier les component types
---
Implementer engine_ecs avec World, Entity, Component storage et queries
