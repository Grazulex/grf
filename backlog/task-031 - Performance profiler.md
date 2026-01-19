---
id: 31
title: Performance profiler
status: Done
priority: medium
milestone: MVP5-Debug
assignees:
  - '@claude'
labels:
  - phase5
  - debug
  - perf
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:10:23.217Z'
updated_date: '2026-01-19T05:10:51.303Z'
closed_date: '2026-01-19T05:10:51.303Z'
changelog:
  - timestamp: '2026-01-18T20:10:23.217Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-19T05:05:37.020Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-19T05:06:18.108Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T05:10:34.366Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T05:10:46.280Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T05:10:51.303Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: >-
  ## Plan d'implementation


  ### Objectif

  Creer un profiler de performance complet avec graphes FPS, frame time, draw
  calls et memory.


  ### Etapes

  1. Creer struct RenderStats dans engine_render pour collecter les stats de
  rendu

  2. Exposer sprite_count et draw calls depuis SpriteBatch

  3. Ajouter tracking memory approximatif (allocations Rust)

  4. Ameliorer le panel Performance existant avec:
     - Section Frame Stats (FPS, frame time, total time)
     - Graphe FPS ameliore avec min/max/avg
     - Nouveau graphe Frame Time
     - Section Render Stats (sprites, vertices, draw calls)
     - Section Memory (heap approximatif)
  5. Ajouter historique pour draw calls et memory


  ### Fichiers concernes

  - crates/engine_render/src/lib.rs (add RenderStats export)

  - crates/engine_render/src/renderer.rs (add stats collection)

  - crates/engine_render/src/sprite.rs (expose sprite_count getter)

  - crates/engine_debug/src/overlay.rs (improve performance panel)


  ### Approche technique

  - RenderStats: struct avec frame_sprites, frame_vertices, draw_calls

  - Memory via std::alloc tracking ou sys-info crate pour heap

  - Panel avec egui_plot pour graphes multiples

  - Historique circulaire de 120 frames pour tous les metrics


  ### Defis potentiels

  - Memory tracking precis sans overhead excessif

  - Garder le panel compact mais informatif
ai_notes: >
  **2026-01-19T05:10:34.366Z** - **05:10** - PROGRESS: Implementation terminee -
  RenderStats, panel Performance ameliore avec graphes FPS/frame time/sprites
ai_review: >-
  ## Self-Review


  ### Complete

  - [x] RenderStats struct ajoutee dans engine_render/src/stats.rs

  - [x] Getter sprite_count() ajoute dans SpriteBatch

  - [x] Stats collection dans Renderer (begin_frame reset,
  flush_sprites/end_frame record)

  - [x] Panel Performance ameliore avec:
    - Section Frame Stats (FPS, frame time, min/max/avg, total time)
    - Graphe FPS avec ligne cible 60 FPS
    - Graphe Frame Time avec ligne cible 16.67ms
    - Section Render Stats (sprites, vertices, draw calls, texture binds)
    - Graphe sprites over time
  - [x] Historiques circulaires de 120 frames pour tous les metrics

  - [x] Integration dans game/main.rs


  ### Tests effectues

  - Compilation: OK (cargo build --features debug-tools)

  - Execution: OK (le jeu demarre)


  ### Qualite du code

  - Standards respectes: Oui

  - Documentation: Complete (docstrings sur structs et methodes)


  ### Limitations connues

  - Memory tracking non implemente (placeholder) - necessite custom allocator

  - Le panel memory a ete retire car non fonctionnel sans tracking reel


  ### Recommandations

  - Pour le tracking memoire, considerer l'integration de memory_stats crate ou
  un custom GlobalAlloc
---
Graphes FPS, frame time, draw calls et memory (F2)
