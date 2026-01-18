---
id: 2
title: Creation fenetre avec winit
status: Done
priority: critical
milestone: MVP1-Fondations
assignees:
  - '@claude'
labels:
  - phase1
  - fondations
  - window
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:08:44.991Z'
updated_date: '2026-01-18T20:27:14.013Z'
closed_date: '2026-01-18T20:27:14.013Z'
changelog:
  - timestamp: '2026-01-18T20:08:44.991Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T20:11:29.860Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:11:30.767Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:11:31.405Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:11:32.043Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:11:32.696Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:23:18.449Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T20:23:19.851Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:26:53.446Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:26:54.225Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:27:02.988Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:27:07.697Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:27:08.322Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:27:08.994Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:27:09.659Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:27:14.013Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Une fenetre s'ouvre avec le titre specifie
    checked: true
  - text: La fenetre peut etre redimensionnee
    checked: true
  - text: Fermer la fenetre termine le programme proprement
    checked: true
  - text: Fonctionne sur Linux (test local)
    checked: true
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Implementer engine_window pour creer une fenetre cross-platform avec winit.

  ### Etapes
  1. Ajouter winit comme dependance dans engine_window
  2. Creer struct Window avec EventLoop et winit::Window
  3. Implementer Window::new(title, width, height)
  4. Implementer la boucle d'evenements basique
  5. Gerer les evenements de fermeture et resize
  6. Exposer l'API publique dans lib.rs

  ### Fichiers concernes
  - crates/engine_window/Cargo.toml (modify)
  - crates/engine_window/src/lib.rs (modify)
  - crates/engine_window/src/window.rs (create)

  ### Dependances
  - Task #1 (Setup workspace)

  ### Approche technique
  - Utiliser winit 0.29+ avec event_loop::ControlFlow
  - Window possede EventLoop et WindowHandle
  - Callbacks pour events (resize, close, input)
ai_notes: >
  **2026-01-18T20:23:19.851Z** - **21:25** - PROGRESS: Demarrage. Implementation
  de engine_window avec winit.

  **2026-01-18T20:26:53.446Z** - **20:27** - PROGRESS: Fixed winit API for
  version 0.29 (ApplicationHandler is 0.30+ only)

  **2026-01-18T20:26:54.224Z** - **20:27** - PROGRESS: Window opens successfully
  at 1280x720, events working
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] WindowConfig avec title, size, resizable
  - [x] Window struct avec run() method
  - [x] App trait pour les applications
  - [x] Integration event loop winit 0.29
  - [x] Game struct implementant App

  ### Tests effectues
  - Build: OK
  - Clippy: OK (0 warnings)
  - Fenetre s'ouvre: OK (1280x720)
  - Resize event: OK
  - Close event: OK
  - FPS logging: OK

  ### Notes
  - FPS tres eleve (~130k) car pas de vsync/rendu GPU encore
  - Warning sctk_adwaita ignore (cosmetic)
  - API winit 0.29 vs 0.30 corrigee
---
Implementer engine_window avec winit pour creer une fenetre cross-platform
