---
id: 4
title: Sprite rendering basique
status: Done
priority: critical
milestone: MVP1-Fondations
assignees:
  - '@claude'
labels:
  - phase1
  - fondations
  - render
  - sprite
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:08:46.612Z'
updated_date: '2026-01-18T20:39:58.761Z'
closed_date: '2026-01-18T20:39:58.761Z'
changelog:
  - timestamp: '2026-01-18T20:08:46.612Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-18T20:11:58.522Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:11:59.428Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:00.080Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:00.712Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:01.352Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:12:01.990Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:33:44.657Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-18T20:38:35.257Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:38:36.546Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:38:37.781Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:39:28.305Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:39:42.351Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-18T20:39:48.952Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:39:49.664Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:39:50.738Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:39:51.772Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:39:52.780Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-01-18T20:39:58.761Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Chargement d'une texture PNG fonctionne
    checked: true
  - text: Un sprite s'affiche a l'ecran
    checked: true
  - text: 'Le sprite peut etre positionne (x, y)'
    checked: true
  - text: Support des sprite regions (atlas)
    checked: true
  - text: Transparence alpha fonctionne
    checked: true
ai_plan: |-
  ## Plan d'Implementation

  ### Objectif
  Implementer le rendu de sprites avec chargement de textures PNG.

  ### Etapes
  1. Ajouter image comme dependance pour chargement PNG
  2. Creer struct Texture avec wgpu::Texture et TextureView
  3. Implementer Texture::from_path() avec image crate
  4. Creer struct Sprite avec texture_id, region, position
  5. Creer shader sprite.wgsl avec texture sampling
  6. Implementer SpriteRenderer avec vertex/index buffers
  7. Gerer les UVs pour sprite sheets

  ### Fichiers concernes
  - crates/engine_render/Cargo.toml (modify)
  - crates/engine_render/src/texture.rs (create)
  - crates/engine_render/src/sprite.rs (create)
  - crates/engine_render/src/shaders/sprite.wgsl (create)

  ### Dependances
  - Task #3 (wgpu init)

  ### Approche technique
  - Textures RGBA8 avec mipmaps optionnels
  - Sprite utilise un quad (4 vertices, 6 indices)
  - Support des sprite regions pour atlas
ai_notes: >
  **2026-01-18T20:38:35.257Z** - **20:38** - PROGRESS: Created Texture, Sprite,
  SpriteBatch modules

  **2026-01-18T20:38:36.546Z** - **20:38** - PROGRESS: Added bytemuck for GPU
  data casting

  **2026-01-18T20:38:37.780Z** - **20:38** - PROGRESS: Test sprite (smiley)
  renders correctly with batching

  **2026-01-18T20:39:28.305Z** - **20:39** - FIX: Corrigé l'orientation des
  sprites (UV mapping)
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] Texture loading depuis PNG (via image crate)
  - [x] Sprite struct avec position, size, region, color
  - [x] SpriteRegion pour sprite sheets/atlas
  - [x] SpriteBatch avec batching (max 10k sprites)
  - [x] Shader WGSL avec texture sampling et alpha
  - [x] Integration dans Renderer
  - [x] Test sprite cree et affiche correctement

  ### Tests effectues
  - Build: OK
  - Texture loading: OK (32x32 PNG)
  - Sprite rendering: OK (5 sprites batches)
  - Orientation: OK (apres fix UV)
  - Alpha transparency: OK (fond transparent)
  - FPS: ~60

  ### Fichiers crees
  - engine_render/src/texture.rs
  - engine_render/src/sprite.rs
  - engine_render/src/shaders/sprite.wgsl
  - assets/textures/test_sprite.png

  ### Notes
  - Ajout bytemuck pour serialisation GPU
  - Coordonnees UV corrigees pour systeme Y-down
---
Implementer le rendu de sprites avec chargement de textures
