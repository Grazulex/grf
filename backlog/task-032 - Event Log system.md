---
id: 32
title: Event Log system
status: Done
priority: medium
milestone: MVP5-Debug
assignees:
  - '@claude'
labels:
  - phase5
  - debug
  - events
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:10:24.076Z'
updated_date: '2026-01-19T05:15:55.813Z'
closed_date: '2026-01-19T05:15:55.813Z'
changelog:
  - timestamp: '2026-01-18T20:10:24.076Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-19T05:12:21.760Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-19T05:12:38.995Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T05:15:48.962Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T05:15:49.997Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T05:15:55.813Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: |-
  ## Plan d'implementation

  ### Objectif
  Creer un systeme de log des evenements du jeu avec filtrage par type (F6).

  ### Etapes
  1. Creer les types d'evenements (Input, Game, System)
  2. Creer struct EventLogEntry avec timestamp, type, message
  3. Ajouter EventLog au DebugOverlay avec historique circulaire
  4. Implementer methodes log_event() pour chaque type
  5. Creer le panel UI avec:
     - Filtres par type (checkboxes)
     - Liste scrollable des events
     - Bouton clear
     - Compteur par type
  6. Integrer dans le jeu (input events, map transitions, etc.)

  ### Fichiers concernes
  - crates/engine_debug/src/overlay.rs (ajouter EventLog et panel)
  - crates/engine_debug/src/lib.rs (exports)
  - game/src/main.rs (integration des logs)

  ### Approche technique
  - EventType enum: Input, Game, System, Debug
  - EventLogEntry: timestamp (f64), event_type, message
  - Historique circulaire de 200 events max
  - Filtres avec bitflags ou Vec<bool>

  ### Defis potentiels
  - Performance si trop d'events par frame
  - Garder le panel lisible avec beaucoup d'events
ai_notes: >
  **2026-01-19T05:15:48.961Z** - **05:15** - PROGRESS: Implementation complete -
  EventType enum, EventLogEntry, EventLogFilter, panel UI with filtering
ai_review: >-
  ## Self-Review


  ### Complete

  - [x] EventType enum (Input, Game, System, Debug) avec couleurs

  - [x] EventLogEntry struct avec timestamp, type, message

  - [x] EventLogFilter pour le filtrage par type

  - [x] Methodes log_event, log_input, log_game, log_system, log_debug

  - [x] Panel Event Log avec:
    - Compteurs par type
    - Checkboxes de filtrage colores
    - Auto-scroll toggle
    - Bouton Clear
    - Liste scrollable avec timestamps
  - [x] Integration dans game/main.rs:
    - Init debug tools (System)
    - Map loaded (System)
    - Map transition (Game)
    - Debug toggles F12, Ctrl+C, Ctrl+Z (Debug)

  ### Tests effectues

  - Compilation: OK

  - Execution: OK


  ### Qualite du code

  - Standards respectes: Oui

  - Documentation: Complete


  ### Limitations connues

  - Pas de log d'input clavier/souris (serait trop verbeux)

  - Historique limite a 200 events


  ### Recommandations

  - Ajouter plus d'events Game (item pickup, dialogue, etc.) quand ces systemes
  seront implementes
---
Log des events du jeu avec filtrage par type (F6)
