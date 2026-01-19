# engine_physics

Module de collision 2D avec AABB et grille spatiale.

## Vue d'ensemble

`engine_physics` fournit:
- **AABB**: Boites englobantes alignees aux axes
- **SpatialGrid**: Partitionnement spatial pour optimisation
- **CollisionResult**: Resultats de detection

---

## AABB

Boite englobante alignee aux axes (Axis-Aligned Bounding Box).

### Structure

```rust
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    /// Coin minimum (haut-gauche)
    pub min: Vec2,

    /// Coin maximum (bas-droite)
    pub max: Vec2,
}
```

### Methodes

```rust
impl AABB {
    /// Cree depuis min/max
    pub fn new(min: Vec2, max: Vec2) -> Self;

    /// Cree depuis position et taille
    pub fn from_position_size(position: Vec2, size: Vec2) -> Self;

    /// Cree depuis centre et demi-taille
    pub fn from_center(center: Vec2, half_size: Vec2) -> Self;

    /// Retourne le centre
    pub fn center(&self) -> Vec2;

    /// Retourne la taille
    pub fn size(&self) -> Vec2;

    /// Retourne la demi-taille
    pub fn half_size(&self) -> Vec2;

    /// Retourne la largeur
    pub fn width(&self) -> f32;

    /// Retourne la hauteur
    pub fn height(&self) -> f32;

    /// Verifie si un point est contenu
    pub fn contains_point(&self, point: Vec2) -> bool;

    /// Verifie l'intersection avec un autre AABB
    pub fn intersects(&self, other: &AABB) -> bool;

    /// Retourne l'intersection (si elle existe)
    pub fn intersection(&self, other: &AABB) -> Option<AABB>;

    /// Calcule la penetration (vecteur de separation)
    pub fn penetration(&self, other: &AABB) -> Option<Vec2>;

    /// Etend l'AABB pour inclure un point
    pub fn expand_to_include(&mut self, point: Vec2);

    /// Fusionne avec un autre AABB
    pub fn merge(&self, other: &AABB) -> AABB;

    /// Translate l'AABB
    pub fn translate(&self, offset: Vec2) -> AABB;

    /// Agrandit l'AABB (padding)
    pub fn inflate(&self, amount: f32) -> AABB;
}
```

### Utilisation

```rust
// Creer des AABBs
let player_box = AABB::from_position_size(
    Vec2::new(100.0, 100.0),
    Vec2::new(32.0, 48.0),
);

let wall_box = AABB::from_position_size(
    Vec2::new(150.0, 100.0),
    Vec2::new(16.0, 64.0),
);

// Test de collision
if player_box.intersects(&wall_box) {
    // Calculer la separation
    if let Some(penetration) = player_box.penetration(&wall_box) {
        // Deplacer le joueur hors du mur
        player_position -= penetration;
    }
}

// Test de point
let mouse_pos = Vec2::new(110.0, 120.0);
if player_box.contains_point(mouse_pos) {
    println!("Clic sur le joueur!");
}
```

---

## SpatialGrid

Grille spatiale pour optimiser les tests de collision.

### Structure

```rust
pub struct SpatialGrid {
    /// Taille d'une cellule en pixels
    cell_size: f32,

    /// Nombre de cellules en largeur
    width: u32,

    /// Nombre de cellules en hauteur
    height: u32,

    /// Cellules contenant les entites
    cells: Vec<Vec<Entity>>,
}
```

### Methodes

```rust
impl SpatialGrid {
    /// Cree une grille
    pub fn new(world_width: f32, world_height: f32, cell_size: f32) -> Self;

    /// Vide la grille
    pub fn clear(&mut self);

    /// Insere une entite avec son AABB
    pub fn insert(&mut self, entity: Entity, aabb: &AABB);

    /// Retire une entite
    pub fn remove(&mut self, entity: Entity, aabb: &AABB);

    /// Retourne les entites potentiellement en collision avec un AABB
    pub fn query(&self, aabb: &AABB) -> Vec<Entity>;

    /// Retourne les entites dans un rayon
    pub fn query_radius(&self, center: Vec2, radius: f32) -> Vec<Entity>;

    /// Retourne les paires potentiellement en collision
    pub fn broad_phase(&self) -> Vec<(Entity, Entity)>;

    /// Convertit position monde -> cellule
    pub fn world_to_cell(&self, position: Vec2) -> (u32, u32);

    /// Statistiques de remplissage
    pub fn stats(&self) -> GridStats;
}

pub struct GridStats {
    pub total_cells: usize,
    pub occupied_cells: usize,
    pub total_entries: usize,
    pub max_per_cell: usize,
}
```

### Utilisation

```rust
// Creer la grille (map 1000x1000, cellules de 64px)
let mut grid = SpatialGrid::new(1000.0, 1000.0, 64.0);

// Chaque frame: reconstruire la grille
grid.clear();

for (entity, pos, collider) in world.query2::<Position, Collider>() {
    let aabb = collider.to_aabb(*pos);
    grid.insert(entity, &aabb);
}

// Tests de collision optimises
let player_aabb = player_collider.to_aabb(player_pos);
let nearby = grid.query(&player_aabb);

for entity in nearby {
    // Seulement tester les entites proches
    if let Some(collider) = world.get::<Collider>(entity) {
        let other_aabb = collider.to_aabb(*world.get::<Position>(entity).unwrap());

        if player_aabb.intersects(&other_aabb) {
            handle_collision(player_entity, entity);
        }
    }
}
```

---

## Collider

Composant de collision.

### Structure

```rust
#[derive(Debug, Clone, Copy)]
pub struct Collider {
    /// Offset depuis la position
    pub offset: Vec2,

    /// Taille du collider
    pub size: Vec2,

    /// Type de collision
    pub collision_type: CollisionType,

    /// Couches de collision
    pub layer: CollisionLayer,

    /// Masque de collision (avec quoi collisionner)
    pub mask: CollisionLayer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionType {
    /// Collision solide (bloque le mouvement)
    Solid,

    /// Trigger (detecte mais ne bloque pas)
    Trigger,

    /// Pas de collision
    None,
}

bitflags::bitflags! {
    pub struct CollisionLayer: u32 {
        const NONE = 0;
        const PLAYER = 1 << 0;
        const ENEMY = 1 << 1;
        const WALL = 1 << 2;
        const ITEM = 1 << 3;
        const TRIGGER = 1 << 4;
        const ALL = u32::MAX;
    }
}
```

### Methodes

```rust
impl Collider {
    /// Cree un collider solide
    pub fn solid(size: Vec2) -> Self;

    /// Cree un trigger
    pub fn trigger(size: Vec2) -> Self;

    /// Definit l'offset
    pub fn with_offset(mut self, offset: Vec2) -> Self;

    /// Definit la couche
    pub fn with_layer(mut self, layer: CollisionLayer) -> Self;

    /// Definit le masque
    pub fn with_mask(mut self, mask: CollisionLayer) -> Self;

    /// Convertit en AABB a une position
    pub fn to_aabb(&self, position: Vec2) -> AABB;

    /// Verifie si deux colliders peuvent interagir
    pub fn can_collide_with(&self, other: &Collider) -> bool;
}
```

### Utilisation

```rust
// Joueur
let player_collider = Collider::solid(Vec2::new(24.0, 32.0))
    .with_offset(Vec2::new(4.0, 16.0))  // Offset pour pieds
    .with_layer(CollisionLayer::PLAYER)
    .with_mask(CollisionLayer::WALL | CollisionLayer::ENEMY | CollisionLayer::ITEM);

// Mur
let wall_collider = Collider::solid(Vec2::new(16.0, 16.0))
    .with_layer(CollisionLayer::WALL)
    .with_mask(CollisionLayer::PLAYER | CollisionLayer::ENEMY);

// Zone de trigger
let door_trigger = Collider::trigger(Vec2::new(32.0, 48.0))
    .with_layer(CollisionLayer::TRIGGER)
    .with_mask(CollisionLayer::PLAYER);
```

---

## CollisionResult

Resultat d'une detection de collision.

```rust
#[derive(Debug, Clone)]
pub struct CollisionResult {
    /// Entite A
    pub entity_a: Entity,

    /// Entite B
    pub entity_b: Entity,

    /// Vecteur de penetration (pour separer A de B)
    pub penetration: Vec2,

    /// Point de contact
    pub contact_point: Vec2,

    /// Normale de collision
    pub normal: Vec2,
}

impl CollisionResult {
    /// Inverse la collision (A <-> B)
    pub fn inverted(&self) -> Self;
}
```

---

## Systeme de collision complet

```rust
fn collision_system(world: &mut World) {
    // Construire la grille spatiale
    let mut grid = SpatialGrid::new(MAP_WIDTH, MAP_HEIGHT, 64.0);

    for (entity, pos, collider) in world.query2::<Position, Collider>() {
        grid.insert(entity, &collider.to_aabb(*pos));
    }

    // Broad phase: paires potentielles
    let pairs = grid.broad_phase();

    // Narrow phase: verification precise
    let mut collisions = Vec::new();

    for (entity_a, entity_b) in pairs {
        let (pos_a, col_a) = {
            let pos = world.get::<Position>(entity_a).unwrap();
            let col = world.get::<Collider>(entity_a).unwrap();
            (*pos, *col)
        };

        let (pos_b, col_b) = {
            let pos = world.get::<Position>(entity_b).unwrap();
            let col = world.get::<Collider>(entity_b).unwrap();
            (*pos, *col)
        };

        // Verifier les couches
        if !col_a.can_collide_with(&col_b) {
            continue;
        }

        let aabb_a = col_a.to_aabb(pos_a);
        let aabb_b = col_b.to_aabb(pos_b);

        if let Some(penetration) = aabb_a.penetration(&aabb_b) {
            collisions.push(CollisionResult {
                entity_a,
                entity_b,
                penetration,
                contact_point: aabb_a.center(), // Simplifie
                normal: penetration.normalize(),
            });
        }
    }

    // Resoudre les collisions
    for collision in &collisions {
        resolve_collision(world, collision);
    }
}

fn resolve_collision(world: &mut World, collision: &CollisionResult) {
    let col_a = world.get::<Collider>(collision.entity_a).unwrap();
    let col_b = world.get::<Collider>(collision.entity_b).unwrap();

    match (col_a.collision_type, col_b.collision_type) {
        (CollisionType::Solid, CollisionType::Solid) => {
            // Separer les entites
            if let Some(pos) = world.get_mut::<Position>(collision.entity_a) {
                pos.x -= collision.penetration.x;
                pos.y -= collision.penetration.y;
            }
        }
        (CollisionType::Trigger, _) | (_, CollisionType::Trigger) => {
            // Emettre un evenement de trigger
            emit_trigger_event(collision);
        }
        _ => {}
    }
}
```

---

## Collision avec tilemap

```rust
fn check_tilemap_collision(
    tilemap: &Tilemap,
    aabb: &AABB,
) -> Vec<(u32, u32, AABB)> {
    let mut collisions = Vec::new();

    // Determiner les tuiles a verifier
    let (start_x, start_y) = tilemap.pixel_to_tile(aabb.min.x, aabb.min.y);
    let (end_x, end_y) = tilemap.pixel_to_tile(aabb.max.x, aabb.max.y);

    for y in start_y..=end_y {
        for x in start_x..=end_x {
            if tilemap.is_solid(x, y) {
                let tile_aabb = AABB::from_position_size(
                    Vec2::new(
                        x as f32 * tilemap.tile_size() as f32,
                        y as f32 * tilemap.tile_size() as f32,
                    ),
                    Vec2::new(
                        tilemap.tile_size() as f32,
                        tilemap.tile_size() as f32,
                    ),
                );

                if aabb.intersects(&tile_aabb) {
                    collisions.push((x, y, tile_aabb));
                }
            }
        }
    }

    collisions
}
```

---

## Performances

### Complexite

| Operation | Sans grille | Avec grille |
|-----------|-------------|-------------|
| Broad phase | O(n²) | O(n * k) |
| Query | O(n) | O(k) |

Ou `k` est le nombre moyen d'entites par cellule.

### Taille de cellule recommandee

- **Trop petite**: Beaucoup de cellules, overhead
- **Trop grande**: Trop d'entites par cellule
- **Ideal**: 2-4x la taille de l'entite moyenne
