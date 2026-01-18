---
id: 8
title: Tilemap loading et rendering
status: To Do
priority: critical
milestone: MVP2-Monde
assignees:
  - '@claude'
labels:
  - phase2
  - monde
  - tilemap
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:09:06.909Z'
updated_date: '2026-01-18T20:13:05.546Z'
changelog:
  - timestamp: '2026-01-18T20:09:06.909Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T20:13:02.706Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:13:03.666Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:04.292Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:04.917Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:13:05.546Z'
    action: modified
    details: Task updated
    user: user
acceptance_criteria:
  - text: Chargement JSON de tilemap fonctionne
    checked: false
  - text: Tiles s'affichent correctement
    checked: false
  - text: Culling actif (seuls tiles visibles rendus)
    checked: false
  - text: Performance acceptable (60 FPS avec 100x100 tiles)
    checked: false
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Implementer le chargement et rendu de tilemaps depuis JSON.

  ### Etapes
  1. Definir struct Tilemap, TileLayer, Tileset
  2. Implementer le parsing JSON avec serde
  3. Creer TilemapRenderer avec batching par tileset
  4. Implementer le culling (ne rendre que les tiles visibles)
  5. Gerer les tiles transparents (Option<TileId>)
  6. Optimiser avec un seul draw call par layer

  ### Fichiers concernes
  - crates/engine_render/src/tilemap.rs (create)
  - assets/maps/test.json (create - exemple)

  ### Dependances
  - Task #4 (Sprite/Texture)
  - Task #7 (Camera pour culling)

  ### Approche technique
  - Tiles indexes dans un Vec (row-major)
  - Tileset avec columns pour calculer UV
  - Batch tous les tiles du meme tileset
---
Implementer le chargement et rendu de tilemaps depuis JSON
