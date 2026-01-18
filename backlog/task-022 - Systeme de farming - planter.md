---
id: 22
title: Systeme de farming - planter
status: Done
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
updated_date: '2026-01-18T22:05:32.549Z'
closed_date: '2026-01-18T22:05:32.549Z'
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
  - timestamp: '2026-01-18T22:02:57.400Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T22:05:05.540Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:05:20.974Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:05:25.635Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:05:26.300Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:05:26.948Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:05:27.609Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:05:28.247Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:05:32.549Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Peut planter une graine sur tile cultivable
    checked: true
  - text: Graine consommee de l'inventaire
    checked: true
  - text: Entite Crop creee avec bon sprite
    checked: true
  - text: Ne peut pas planter hors saison
    checked: true
  - text: Ne peut pas planter sur tile non-cultivable
    checked: true
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
ai_notes: >
  **2026-01-18T22:05:05.539Z** - **22:20** - COMPLETED: Created farming.rs with
  GrowthStage enum, Crop component, FarmTile component, TileState enum,
  try_plant function. Supports: till→plant→water→grow cycle, season validation,
  regrowable crops, dies without water or wrong season. 13 unit tests.
ai_review: >-
  ## Self-Review


  ### Complete

  - [x] GrowthStage enum (Seed, Sprout, Growing, Mature, Harvestable, Dead)

  - [x] Crop component with growth tracking

  - [x] FarmTile component with TileState

  - [x] TileState enum (Natural, Tilled, Watered, Planted)

  - [x] try_plant() function with validation

  - [x] Season checking

  - [x] Watering logic

  - [x] Crop death mechanics (no water, wrong season)

  - [x] Regrowable crops support

  - [x] 13 unit tests


  ### Tests effectues

  - test_growth_stages: OK

  - test_farm_tile_workflow: OK

  - test_create_crop_from_seed: OK

  - test_regrowable_crop: OK

  - test_crop_growth: OK

  - test_crop_dies_without_water: OK

  - test_crop_dies_wrong_season: OK

  - test_try_plant_success: OK

  - test_try_plant_not_tilled: OK

  - test_try_plant_wrong_season: OK

  - test_try_plant_not_seed: OK

  - test_harvest_regrowable: OK

  - test_harvest_non_regrowable: OK


  ### Qualite du code

  - Standards respectes: Oui

  - Documentation: Complete

  - Serde pour save/load


  ### API

  - Crop::from_seed(seed_id, db)

  - crop.water(), crop.advance_day(season), crop.harvest()

  - FarmTile::new(), tile.till(), tile.water(), tile.plant()

  - try_plant(tile, seed_id, season, db) -> Result<Crop, PlantResult>


  ### Mecaniques

  - Tile: Natural → (hoe) → Tilled → (water) → Watered

  - Plant: Tilled/Watered → Planted

  - Growth: progress 0-25% Seed, 25-50% Sprout, 50-75% Growing, 75-100% Mature,
  100% Harvestable

  - Dies after 3 days without water
---
Planter des graines sur tiles cultivables
