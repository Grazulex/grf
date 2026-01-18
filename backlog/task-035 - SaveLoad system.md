---
id: 35
title: Save/Load system
status: To Do
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
updated_date: '2026-01-18T20:14:21.696Z'
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
acceptance_criteria:
  - text: Save ecrit un fichier valide
    checked: false
  - text: Load restaure l'etat exact
    checked: false
  - text: Gestion des versions de save
    checked: false
  - text: Multiple save slots supportes
    checked: false
  - text: Pas de crash si fichier corrompu
    checked: false
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
---
Serialisation de l'etat du jeu (world, inventory, progression) avec serde
