---
id: 25
title: NPCs basiques
status: Done
priority: high
milestone: MVP4-Gameplay
assignees:
  - '@claude'
labels:
  - phase4
  - gameplay
  - npc
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:09:56.144Z'
updated_date: '2026-01-18T22:12:24.128Z'
closed_date: '2026-01-18T22:12:24.128Z'
changelog:
  - timestamp: '2026-01-18T20:09:56.144Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T22:07:49.459Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T22:07:59.636Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:08:05.866Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:08:06.511Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:08:07.169Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:08:07.849Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:08:08.521Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:12:01.279Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:12:14.069Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:12:20.362Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:12:21.046Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:12:21.749Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:12:22.463Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:12:23.175Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:12:24.128Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: NPC component avec id et nom
    checked: true
  - text: NpcState pour animations
    checked: true
  - text: Schedule avec activites horaires
    checked: true
  - text: Pathfinding vers destination
    checked: true
  - text: NPC definitions depuis TOML
    checked: true
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Creer des NPCs avec state machine, pathfinding basique et schedules.

  ### Etapes
  1. Creer NpcState enum (Idle, Walking, Talking, Working)
  2. Creer NPC component avec id, name, schedule
  3. Creer Schedule struct pour les activites journalieres
  4. Implemente pathfinding basique (A* simplifie ou waypoints)
  5. Creer npc_system pour update des NPCs

  ### Fichiers concernes
  - game/src/npc.rs (create)

  ### Dependances
  - Task #14 (ECS)
  - Task #18 (GameClock - pour schedules)

  ### Approche technique
  - Schedule = liste de (hour, location)
  - NPC suit le schedule selon l'heure
  - Pathfinding simple avec waypoints
  - State machine pour animations
ai_notes: >
  **2026-01-18T22:12:01.278Z** - **22:40** - COMPLETED: Created npc.rs with
  NpcState (Idle, Walking, Talking, Working, Sleeping), Schedule with hourly
  entries, NpcPath for pathfinding, NpcDatabase for TOML loading. Created
  npcs.toml with 4 NPCs (Robin, Pierre, Abigail, Lewis). 9 unit tests.
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] NpcState enum (Idle, Walking, Talking, Working, Sleeping)
  - [x] Direction enum (Up, Down, Left, Right)
  - [x] Schedule with ScheduleEntry
  - [x] NpcPath for waypoint pathfinding
  - [x] Npc component with all properties
  - [x] NpcDatabase with TOML loading
  - [x] Movement update with direction
  - [x] Schedule-based behavior
  - [x] Conversation start/stop
  - [x] npcs.toml with 4 NPCs
  - [x] 9 unit tests

  ### Tests effectues
  - test_npc_creation: OK
  - test_npc_locations: OK
  - test_npc_go_to: OK
  - test_schedule: OK
  - test_npc_movement: OK
  - test_direction_from_velocity: OK
  - test_npc_conversation: OK
  - test_load_npcs: OK
  - test_create_npc_from_definition: OK

  ### Qualite du code
  - Standards respectes: Oui
  - Documentation: Complete
  - Serde pour save/load

  ### API
  - Npc::new(id, name), with_position(x, y)
  - add_location(), get_location()
  - go_to(name), go_to_position(x, y)
  - update_movement(dt), update_schedule(hour)
  - start_talking(), stop_talking()
  - NpcDatabase::load_from_file(), create_npc(id)
---
Entites NPC avec state machine, pathfinding basique et schedules
