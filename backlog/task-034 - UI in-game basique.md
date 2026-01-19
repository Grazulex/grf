---
id: 34
title: UI in-game basique
status: Done
priority: high
milestone: MVP5-Debug
assignees:
  - '@claude'
labels:
  - phase5
  - ui
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:10:25.884Z'
updated_date: '2026-01-19T05:27:44.968Z'
closed_date: '2026-01-19T05:27:44.968Z'
changelog:
  - timestamp: '2026-01-18T20:10:25.884Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-19T05:23:58.409Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-19T05:24:15.732Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T05:27:29.255Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T05:27:40.668Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T05:27:44.968Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: >-
  ## Plan d'implementation


  ### Objectif

  Implementer engine_ui avec un HUD basique (health, stamina, time, hotbar)


  ### Etapes

  1. Creer structs pour les donnees HUD (HealthBar, StaminaBar, TimeDisplay,
  Hotbar)

  2. Ajouter methode reset_to_screen_coords() au Renderer pour UI

  3. Implementer widget de barre avec fond/remplissage

  4. Implementer affichage du temps in-game (heure/jour)

  5. Implementer hotbar avec slots (9 emplacements)

  6. Integrer le rendu HUD dans la game loop (apres world, avant egui)


  ### Fichiers concernes

  - crates/engine_ui/src/lib.rs (extend)

  - crates/engine_ui/src/hud.rs (create)

  - crates/engine_ui/src/widgets.rs (create)

  - crates/engine_render/src/renderer.rs (add UI method)

  - game/src/main.rs (integrate HUD rendering)


  ### Approche technique

  - UI utilise le meme SpriteBatch mais avec matrice orthographique screen-space

  - Les widgets sont des primitives simples (rectangles colores)

  - Pas de texture pour MVP - juste des rectangles avec couleurs


  ### Defis potentiels

  - Z-order UI par rapport au debug overlay egui

  - Performance si trop de draw calls
ai_notes: >
  **2026-01-19T05:27:29.255Z** - **05:27** - PROGRESS: Implemented HUD module
  with ProgressBar, Hotbar, TimeDisplay, Hud structs

  **05:27** - PROGRESS: Added screen-space rendering method to Renderer

  **05:27** - PROGRESS: Integrated HUD rendering in game loop (after world
  sprites, before egui)

  **05:27** - PROGRESS: Added hotbar selection via number keys 1-9

  **05:27** - PROGRESS: HUD resizes with window
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] ProgressBar widget (health, stamina) with background and fill
  - [x] Hotbar with 9 slots and selection highlight
  - [x] TimeDisplay background (placeholder - no text rendering yet)
  - [x] Screen-space rendering method in Renderer
  - [x] HUD integration in game loop
  - [x] Hotbar selection via 1-9 keys
  - [x] HUD resize on window resize

  ### Tests effectues
  - Build avec debug-tools: OK
  - Game startup: OK
  - HUD visibility: OK (rectangles rendered)

  ### Qualite du code
  - Standards respectes: Oui
  - Documentation: Complete (doc comments)
  - Separation des concerns: Oui (hud.rs module)

  ### Limitations connues
  - Pas de rendu de texte (time string non affichee)
  - Health/stamina sont des placeholder values (pas de composants player stats)
  - Hotbar slots vides (inventaire pas implemente)

  ### Recommendations futures
  - Ajouter text rendering pour afficher l'heure
  - Creer composants Health et Stamina pour le player
  - Connecter l'inventaire aux slots du hotbar
  - Ajouter icones pour les items
---
Implementer engine_ui avec HUD (hotbar, health, stamina, time)
