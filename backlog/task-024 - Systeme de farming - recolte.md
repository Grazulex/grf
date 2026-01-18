---
id: 24
title: Systeme de farming - recolte
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
created_date: '2026-01-18T20:09:55.283Z'
updated_date: '2026-01-18T22:07:18.682Z'
closed_date: '2026-01-18T22:07:18.682Z'
changelog:
  - timestamp: '2026-01-18T20:09:55.283Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T22:06:58.189Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T22:06:58.885Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:06:59.596Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:07:00.331Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:07:01.030Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:07:01.731Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:07:09.718Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:07:10.407Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T22:07:15.838Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:07:16.532Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:07:17.245Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:07:17.945Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T22:07:18.682Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Peut recolter une culture mature
    checked: true
  - text: Item ajoute a l'inventaire
    checked: true
  - text: Crops regrowables continuent
    checked: true
  - text: Crops non-regrowables sont retires
    checked: true
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Recolte des cultures matures et ajout a l'inventaire.

  ### Note
  Deja implemente dans Task #022:
  - Crop.harvest() retourne Option<String> avec crop_id
  - Crops regrowables: reset vers Growing
  - Crops non-regrowables: deviennent Dead
  - GrowthStage::Harvestable.can_harvest() = true

  ### Fichiers concernes
  - game/src/farming.rs (deja fait)
ai_notes: >
  **2026-01-18T22:07:09.717Z** - **22:28** - COMPLETED: Harvest already in
  farming.rs. Crop.harvest() returns crop_id, handles regrowable vs
  non-regrowable crops.
ai_review: |-
  ## Self-Review

  ### Complete (via Task #022)
  - [x] Crop.harvest() method
  - [x] Returns crop_id when harvestable
  - [x] Regrowable: reset to Growing
  - [x] Non-regrowable: becomes Dead
  - [x] can_harvest() check

  ### Tests
  - test_harvest_regrowable: OK
  - test_harvest_non_regrowable: OK
---
Recolte des cultures matures et ajout a l'inventaire
