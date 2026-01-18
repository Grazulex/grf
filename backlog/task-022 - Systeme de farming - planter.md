---
id: 22
title: Systeme de farming - planter
status: To Do
priority: critical
milestone: MVP4-Gameplay
assignees:
  - '@claude'
labels:
  - phase4
  - gameplay
  - farming
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:09:53.589Z'
updated_date: '2026-01-18T20:14:05.586Z'
changelog:
  - timestamp: '2026-01-18T20:09:53.589Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T20:14:02.008Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:14:03.011Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:14:03.640Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:14:04.272Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:14:04.947Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:14:05.586Z'
    action: modified
    details: Task updated
    user: user
acceptance_criteria:
  - text: Peut planter une graine sur tile cultivable
    checked: false
  - text: Graine consommee de l'inventaire
    checked: false
  - text: Entite Crop creee avec bon sprite
    checked: false
  - text: Ne peut pas planter hors saison
    checked: false
  - text: Ne peut pas planter sur tile non-cultivable
    checked: false
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Permettre de planter des graines sur tiles cultivables.

  ### Etapes
  1. Definir CropDefinition dans TOML (growth_stages, seasons)
  2. Creer component Crop (crop_type, growth_stage, watered)
  3. Marquer les tiles cultivables dans la tilemap
  4. Implementer interaction: use Seed item sur tile
  5. Spawner entite Crop avec sprite initial
  6. Verifier la saison appropriee

  ### Fichiers concernes
  - game/src/components/crop.rs (create)
  - game/src/systems/farming.rs (create)
  - assets/data/crops.toml (create)

  ### Dependances
  - Task #14 (ECS)
  - Task #20 (Inventaire - avoir la graine)
  - Task #21 (Item definitions)
  - Task #8 (Tilemap - tiles cultivables)

  ### Approche technique
  - Crop component sur entite separee
  - Tile cultivable = flag dans tilemap data
  - Sprite change selon growth_stage
---
Planter des graines sur tiles cultivables
