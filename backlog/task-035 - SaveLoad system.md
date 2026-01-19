---
id: 35
title: Save/Load system
status: Done
priority: critical
milestone: MVP6-Release
assignees:
  - '@claude'
labels:
  - phase6
  - finition
  - save
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:10:44.640Z'
updated_date: '2026-01-19T09:14:59.161Z'
closed_date: '2026-01-19T09:14:59.161Z'
changelog:
  - timestamp: '2026-01-18T20:10:44.640Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T20:14:17.997Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:14:19.052Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:14:19.674Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:14:20.332Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:14:21.022Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:14:21.696Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:06:37.168Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-19T09:06:43.313Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T09:14:23.608Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T09:14:49.847Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:14:50.880Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:14:51.936Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:14:52.974Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:14:54.006Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:14:59.161Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Save ecrit un fichier valide
    checked: true
  - text: Load restaure l'etat exact
    checked: true
  - text: Gestion des versions de save
    checked: true
  - text: Multiple save slots supportes
    checked: true
  - text: Pas de crash si fichier corrompu
    checked: true
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Implementer la serialisation complete de l'etat du jeu.

  ### Etapes
  1. Definir SaveData struct avec toutes les donnees
  2. Implementer Serialize/Deserialize pour components
  3. Creer SaveManager avec save() et load()
  4. Sauvegarder: World state, GameTime, Player data
  5. Gerer les versions de save pour compatibilite
  6. Ecrire en JSON (dev) ou binaire (release)

  ### Fichiers concernes
  - game/src/save/mod.rs (create)
  - game/src/save/save_data.rs (create)
  - game/src/save/save_manager.rs (create)

  ### Dependances
  - Task #14 (ECS - world serialisation)
  - Task #18 (GameTime)
  - Task #20 (Inventory)
  - Toutes les taches gameplay

  ### Approche technique
  - serde pour serialisation
  - Version number dans SaveData
  - Slot-based saves (save1.json, save2.json, etc)
  - Backup avant overwrite
ai_notes: >
  **2026-01-19T09:06:43.313Z** - **09:05** - Starting implementation. Creating
  save module structure.

  **2026-01-19T09:14:23.607Z** - **09:15** - Save/Load system implementation
  complete:

  - Added serde serialization to all game components and GameClock

  - Created save module with SaveData, PlayerData, GameClockData structs

  - Implemented SaveManager with save/load/backup functionality

  - Save versioning (SAVE_VERSION = 1) for future compatibility

  - Multiple save slots support (MAX_SAVE_SLOTS = 5)

  - JSON format with pretty-print for human readability

  - Automatic backup before overwriting saves

  - F5 = Save to slot 0, F9 = Load from slot 0

  - Saves player position, speed, sprite/collider sizes, inventory, game clock,
  current map
---
Serialisation de l'etat du jeu (world, inventory, progression) avec serde
