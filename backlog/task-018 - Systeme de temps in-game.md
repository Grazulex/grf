---
id: 18
title: Systeme de temps in-game
status: Done
priority: critical
milestone: MVP4-Gameplay
assignees:
  - '@claude'
labels:
  - phase4
  - gameplay
  - time
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:09:50.182Z'
updated_date: '2026-01-18T21:52:32.603Z'
closed_date: '2026-01-18T21:52:32.603Z'
changelog:
  - timestamp: '2026-01-18T20:09:50.182Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T21:49:52.368Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T21:50:10.098Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:50:11.591Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:50:12.421Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:50:13.418Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:50:14.446Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:52:13.432Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:52:27.946Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T21:52:29.423Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:52:30.134Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:52:30.868Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:52:31.634Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T21:52:32.603Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: GameClock avec heures/minutes/jours
    checked: true
  - text: Saisons (4) et annees
    checked: true
  - text: time_scale configurable
    checked: true
  - text: Helpers is_morning/afternoon/evening/night
    checked: true
ai_plan: >-
  ## Plan d'implementation


  ### Objectif

  Creer un systeme de temps in-game avec heures, jours, saisons pour le gameplay
  farming.


  ### Etapes

  1. Creer GameClock struct:
     - Heures (0-23), minutes (0-59)
     - Jour de la semaine (lundi-dimanche)
     - Jour du mois (1-28)
     - Saison (printemps, ete, automne, hiver)
     - Annee

  2. Configuration:
     - time_scale: vitesse du temps (1.0 = normal)
     - Duree d'une minute in-game en secondes reelles

  3. API:
     - advance(dt) - avance le temps
     - hour(), minute(), day(), season(), year()
     - is_morning(), is_afternoon(), is_evening(), is_night()
     - day_of_week() -> String

  4. Integrer comme ECS Resource


  ### Fichiers concernes

  - crates/engine_core/src/lib.rs (modify)

  - crates/engine_core/src/game_clock.rs (create)

  - game/src/main.rs (integrate)


  ### Approche technique

  - 1 minute in-game = 1 seconde reelle (configurable)

  - 28 jours par saison (comme Stardew Valley)

  - 4 saisons par annee


  ### Constantes

  - MINUTES_PER_HOUR = 60

  - HOURS_PER_DAY = 24

  - DAYS_PER_SEASON = 28

  - SEASONS_PER_YEAR = 4
ai_notes: >
  **2026-01-18T21:52:13.432Z** - **22:52** - COMPLETED: GameClock with
  hours/minutes, days (1-28), seasons (4), years. Time helpers for
  morning/afternoon/evening/night. daylight_factor for visual effects. 11 tests
  passing.
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] GameClock struct with hours, minutes, days
  - [x] Season enum (Spring, Summer, Fall, Winter)
  - [x] DayOfWeek enum (Monday-Sunday)
  - [x] TimeOfDay enum (Morning, Afternoon, Evening, Night)
  - [x] time_scale and seconds_per_minute configurable
  - [x] pause support
  - [x] 11 unit tests

  ### Tests effectues
  - test_new_clock: OK
  - test_advance_time: OK
  - test_day_rollover: OK
  - test_season_rollover: OK
  - test_year_rollover: OK
  - test_time_of_day: OK
  - test_time_scale: OK
  - test_pause: OK
  - test_day_of_week: OK
  - test_formatting: OK
  - test_daylight_factor: OK

  ### Qualite du code
  - Standards respectes: Oui
  - Documentation: Complete
  - No warnings

  ### API
  - advance(dt) - advance time
  - hour(), minute(), day(), season(), year()
  - is_morning(), is_afternoon(), is_evening(), is_night()
  - is_daytime(), is_nighttime()
  - daylight_factor() - for visual effects (0.0-1.0)
  - time_string(), time_string_12h(), date_string(), full_string()

  ### Integration
  - Can be stored as ECS Resource
  - Use daylight_factor() for Task #019 (cycle jour/nuit visuel)
---
GameTime avec jour/nuit, heures, saisons et time scale
