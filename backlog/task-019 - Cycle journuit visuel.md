---
id: 19
title: Cycle jour/nuit visuel
status: Done
priority: high
milestone: MVP4-Gameplay
assignees:
  - '@claude'
labels:
  - phase4
  - gameplay
  - time
  - render
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:09:51.032Z'
updated_date: '2026-01-18T21:54:59.067Z'
closed_date: '2026-01-18T21:54:59.067Z'
changelog:
  - timestamp: '2026-01-18T20:09:51.032Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T21:52:51.116Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T21:53:11.135Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:53:12.972Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:53:14.008Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:53:15.024Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:53:15.998Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:54:36.650Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:54:52.975Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:54:55.051Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:54:56.091Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:54:57.104Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:54:58.057Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:54:59.067Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: DayNightCycle avec couleurs par periode
    checked: true
  - text: Interpolation lisse entre periodes
    checked: true
  - text: Integration avec GameClock
    checked: true
  - text: Effet visible dans le jeu
    checked: true
ai_plan: |-
  ## Plan d'implementation

  ### Objectif
  Creer un effet visuel jour/nuit qui teinte l'ecran selon l'heure in-game.

  ### Etapes
  1. Creer DayNightCycle struct:
     - Couleurs pour chaque periode (matin, midi, soir, nuit)
     - Interpolation lisse entre les couleurs
     - get_ambient_color(clock) -> Color

  2. Modifier le renderer:
     - Ajouter uniform pour ambient_color
     - Multiplier les sprites par ambient_color dans le shader
     - OU appliquer un overlay apres le rendu

  3. Integrer avec GameClock:
     - Lire daylight_factor() et time_of_day()
     - Mettre a jour l'ambient color chaque frame

  ### Fichiers concernes
  - crates/engine_render/src/day_night.rs (create)
  - crates/engine_render/src/lib.rs (export)
  - game/src/main.rs (integrate)

  ### Approche technique
  - Option 1: Modifier le clear color selon l'heure
  - Option 2: Overlay fullscreen avec blend
  - Option 3: Uniform dans le sprite shader (plus complexe)

  Je choisis Option 1 (clear color) + teinte des sprites pour simplicite.

  ### Couleurs cibles
  - Matin (6h): orange chaud #FFA07A
  - Midi (12h): blanc pur #FFFFFF
  - Soir (18h): orange/rose #FF7F50
  - Nuit (0h): bleu fonce #1A1A3A
ai_notes: >
  **2026-01-18T21:54:36.649Z** - **22:54** - COMPLETED: DayNightCycle with Color
  struct, smooth interpolation between dawn/noon/dusk/midnight.
  get_ambient_color and get_clear_color methods. 5 tests for day_night module.
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] Color struct with lerp and hex conversion
  - [x] DayNightCycle with 4 key colors
  - [x] Smooth interpolation between periods
  - [x] get_ambient_color(hour, minute)
  - [x] get_clear_color for background
  - [x] 5 unit tests

  ### Tests effectues
  - test_color_lerp: OK
  - test_color_from_hex: OK
  - test_day_night_noon: OK
  - test_day_night_midnight: OK
  - test_day_night_transitions: OK

  ### Qualite du code
  - Standards respectes: Oui
  - Documentation: Complete
  - No warnings

  ### API
  - DayNightCycle::new() - default colors
  - get_ambient_color(hour, minute) -> Color
  - get_clear_color(hour, minute) -> Color (darker for background)
  - Color::lerp, to_wgpu, to_vec3

  ### Colors par defaut
  - Dawn (6h): #FFB07A (warm orange)
  - Noon (12h): #FFFFFF (white)
  - Dusk (18h): #FF7F50 (coral)
  - Midnight (0h): #1A1A3A (dark blue)

  ### Integration
  - Use with GameClock: cycle.get_ambient_color(clock.hour(), clock.minute())
  - Apply to clear color or as sprite tint
---
Effet visuel jour/nuit avec ambient color progressif
