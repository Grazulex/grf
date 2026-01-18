---
id: 6
title: Input system clavier
status: Done
priority: high
milestone: MVP1-Fondations
assignees:
  - '@claude'
labels:
  - phase1
  - fondations
  - input
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:08:48.234Z'
updated_date: '2026-01-18T20:49:17.161Z'
closed_date: '2026-01-18T20:49:17.161Z'
changelog:
  - timestamp: '2026-01-18T20:08:48.234Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T20:12:26.972Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:12:27.962Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:28.589Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:29.246Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:29.891Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:44:25.883Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T20:48:50.138Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:48:59.237Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:49:10.368Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:49:11.029Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:49:11.671Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:49:12.319Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:49:17.161Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Les touches clavier sont detectees
    checked: true
  - text: is_pressed retourne true pendant l'appui
    checked: true
  - text: is_just_pressed retourne true une seule frame
    checked: true
  - text: Actions abstraites mappees aux touches
    checked: true
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Implementer l'abstraction input pour clavier avec etats des touches.

  ### Etapes
  1. Creer enum Action pour les actions abstraites
  2. Creer enum ButtonState (Released, JustPressed, Held, JustReleased)
  3. Creer struct InputState avec HashMap<Action, ButtonState>
  4. Mapper les evenements winit vers InputState
  5. Implementer is_pressed, is_just_pressed, etc
  6. Preparer le support du remapping (config TOML)

  ### Fichiers concernes
  - crates/engine_input/src/lib.rs (modify)
  - crates/engine_input/src/input_state.rs (create)
  - crates/engine_input/src/keyboard.rs (create)

  ### Dependances
  - Task #2 (Window events)

  ### Approche technique
  - Actions abstraites (MoveUp, Interact, etc)
  - Transition d'etats par frame
  - Double buffering pour detecter JustPressed/JustReleased
ai_notes: >
  **2026-01-18T20:48:50.137Z** - **20:48** - PROGRESS: Build succeeded and game
  tested successfully. WASD/Arrow keys control sprite movement, position updates
  visible in logs. FPS/UPS stable at 60. Input state transitions
  (JustPressed→Held→JustReleased) working correctly.
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] KeyCode enum with mapping from winit
  - [x] KeyboardState with is_pressed/is_just_pressed/is_just_released
  - [x] Input struct aggregating all input devices
  - [x] get_movement_direction() for WASD/arrows
  - [x] Integration with window events (on_keyboard_event)
  - [x] end_frame() for state transitions (JustPressed→Held)
  - [x] Player control in game with keyboard input

  ### Tests effectues
  - cargo build: OK
  - cargo run: OK - sprite controlled with WASD/arrows
  - Position logging confirms movement
  - FPS/UPS stable at 60

  ### Qualite du code
  - Standards respectes: Oui
  - Documentation: Complete avec doc comments

  ### Limitations connues
  - Aucune pour cette iteration

  ### Recommandations
  - Task #7 (Camera 2D) ready to proceed
---
Implementer engine_input avec abstraction clavier et etats des touches
