---
id: 27
title: Integration egui et debug overlay
status: Done
priority: high
milestone: MVP5-Debug
assignees:
  - '@claude'
labels:
  - phase5
  - debug
  - egui
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:10:19.735Z'
updated_date: '2026-01-18T22:32:41.671Z'
closed_date: '2026-01-18T22:32:41.671Z'
changelog:
  - timestamp: '2026-01-18T20:10:19.735Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T22:17:57.025Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T22:17:57.693Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:18:04.669Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:18:05.337Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:18:06.023Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:18:06.713Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:22:50.632Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:32:20.034Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:32:33.764Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:32:38.980Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:32:39.639Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:32:40.289Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:32:40.974Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:32:41.671Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: EguiRenderer integration wgpu
    checked: true
  - text: DebugOverlay avec panels
    checked: true
  - text: Toggle F12 fonctionnel
    checked: true
  - text: Feature-gated derriere debug-tools
    checked: true
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Integrer egui avec wgpu pour les debug overlays, toggle avec F12.

  ### Etapes
  1. Ajouter wgpu et winit comme deps de engine_debug
  2. Creer EguiRenderer pour l'integration wgpu
  3. Creer DebugOverlay struct avec panels
  4. Implementer le toggle F12
  5. Creer les stubs pour les panels futurs

  ### Fichiers concernes
  - crates/engine_debug/src/lib.rs (modify)
  - crates/engine_debug/src/egui_renderer.rs (create)
  - crates/engine_debug/src/overlay.rs (create)
  - crates/engine_debug/Cargo.toml (modify)

  ### Approche technique
  - egui-wgpu pour le rendu
  - egui-winit pour les events
  - DebugOverlay orchestre les panels
  - Feature-gated derriere debug-tools
ai_notes: >
  **2026-01-18T22:22:50.631Z** - **23:22** - PROGRESS: EguiRenderer and
  DebugOverlay modules created and compiled successfully. Fixed API differences
  with egui 0.27 (State::new takes 5 args, Renderer::new takes 4 args). Fixed
  f64/f32 conversion for FPS history.

  **2026-01-18T22:32:20.034Z** - **23:32** - PROGRESS: Full debug overlay
  integration complete. Added flush_sprites method for proper render order. All
  99 tests pass.
ai_review: >-
  ## Self-Review


  ### Complete

  - [x] Added egui_plot to workspace dependencies

  - [x] Created EguiRenderer in engine_debug

  - [x] Created DebugOverlay with panels (Performance, ECS, Collision, Events,
  Console)

  - [x] Added F12 toggle functionality

  - [x] Integrated window event handling for egui input

  - [x] Added flush_sprites method for proper render ordering

  - [x] All 99 tests pass


  ### Files Modified

  - Cargo.toml (workspace) - added egui_plot

  - crates/engine_debug/Cargo.toml - added dependencies

  - crates/engine_debug/src/lib.rs - module exports

  - crates/engine_debug/src/egui_renderer.rs - new file

  - crates/engine_debug/src/overlay.rs - new file

  - crates/engine_render/src/renderer.rs - added flush_sprites, Frame public
  fields

  - crates/engine_render/src/sprite.rs - added is_empty

  - crates/engine_window/src/window.rs - added on_window_event

  - game/src/main.rs - integrated debug overlay


  ### Technical Decisions

  - Made Frame.view and Frame.encoder public to avoid borrow conflicts

  - Added flush_sprites to render sprites before egui overlay

  - Feature-gated all debug code with #[cfg(feature = "debug-tools")]


  ### Quality

  - All code compiles cleanly

  - No new warnings introduced

  - Feature properly gated for release builds
---
Setup egui-wgpu avec toggle F12 et structure DebugManager
