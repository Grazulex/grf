---
id: 36
title: Menu principal
status: Done
priority: high
milestone: MVP6-Release
assignees:
  - '@claude'
labels:
  - phase6
  - finition
  - ui
  - menu
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:10:45.584Z'
updated_date: '2026-01-19T09:32:42.069Z'
closed_date: '2026-01-19T09:32:42.069Z'
changelog:
  - timestamp: '2026-01-18T20:10:45.584Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-19T09:23:11.737Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-19T09:32:06.606Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T09:32:21.026Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:32:22.057Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:32:23.092Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:32:24.140Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:32:31.759Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:32:32.820Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:32:33.849Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:32:34.907Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-19T09:32:42.069Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Menu s'affiche au lancement
    checked: true
  - text: Navigation avec fleches/WASD
    checked: true
  - text: New Game demarre le jeu
    checked: true
  - text: Quit ferme le jeu
    checked: true
ai_notes: |
  **2026-01-19T09:32:06.606Z** - **09:32** - Main menu implementation complete:
  - Added Sprite::colored() to engine_render for solid color rectangles
  - Created Menu component in engine_ui with:
    - MenuItem struct for menu items with id/label/enabled
    - MenuStyle for customizable appearance
    - Menu widget with navigation (up/down), selection indicator
    - Presets for main menu and pause menu
  - Integrated menu in game:
    - GameState enum (MainMenu, Playing, Paused)
    - Menu renders on game start
    - Navigate with W/S or Arrow keys
    - Select with Enter/Space
    - New Game starts the game
    - Load Game loads save
    - Quit exits the game
    - Settings shows 'not implemented' message
---
Menu principal avec New Game, Load, Settings, Quit
