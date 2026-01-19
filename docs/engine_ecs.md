# engine_ecs

Systeme Entity-Component-System custom pour la gestion des entites de jeu.

## Vue d'ensemble

`engine_ecs` fournit une architecture ECS complete:
- **Entity**: Identifiant unique d'une entite
- **Component**: Trait pour les donnees
- **World**: Conteneur principal
- **SparseSet**: Stockage efficace des composants
- **Resources**: Donnees globales partagees

---

## Entity

Identifiant unique d'une entite compose d'un index et d'une generation.

### Structure

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    /// Index dans le tableau d'entites
    index: u32,

    /// Generation pour detecter les entites recyclees
    generation: u32,
}
```

### Methodes

```rust
impl Entity {
    /// Cree une entite (usage interne)
    pub(crate) fn new(index: u32, generation: u32) -> Self;

    /// Retourne l'index
    pub fn index(self) -> u32;

    /// Retourne la generation
    pub fn generation(self) -> u32;

    /// Entite invalide (sentinelle)
    pub const INVALID: Entity = Entity { index: u32::MAX, generation: 0 };
}
```

### Affichage

```rust
// Format: Entity(index:generation)
println!("{}", entity); // "Entity(42:1)"
```

---

## Component

Trait marqueur pour les composants.

### Definition

```rust
pub trait Component: 'static + Send + Sync {}
```

### Implementation automatique

```rust
// Tout type 'static + Send + Sync implemente automatiquement Component
impl<T: 'static + Send + Sync> Component for T {}
```

### Exemples de composants

```rust
// Composant position
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

// Composant velocite
#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// Composant sprite
#[derive(Debug, Clone)]
pub struct SpriteComponent {
    pub texture_id: String,
    pub uv: Rect,
    pub z_order: i32,
}

// Composant tag (marker)
#[derive(Debug, Clone, Copy)]
pub struct Player;

#[derive(Debug, Clone, Copy)]
pub struct Enemy;

// Composant avec donnees complexes
#[derive(Debug, Clone)]
pub struct Inventory {
    pub items: Vec<ItemStack>,
    pub capacity: usize,
}
```

---

## World

Conteneur principal gerant les entites, composants et ressources.

### Structure

```rust
pub struct World {
    // Entites
    entities: Vec<EntityMeta>,
    free_list: Vec<u32>,
    entity_count: u32,

    // Composants (TypeId -> SparseSet<T>)
    storages: HashMap<TypeId, Box<dyn Any>>,

    // Ressources globales
    resources: HashMap<TypeId, Box<dyn Any>>,
}

struct EntityMeta {
    generation: u32,
    alive: bool,
}
```

### Gestion des entites

```rust
impl World {
    /// Cree un monde vide
    pub fn new() -> Self;

    /// Cree une nouvelle entite
    pub fn spawn(&mut self) -> Entity;

    /// Detruit une entite
    pub fn despawn(&mut self, entity: Entity) -> bool;

    /// Verifie si une entite est vivante
    pub fn is_alive(&self, entity: Entity) -> bool;

    /// Nombre d'entites vivantes
    pub fn entity_count(&self) -> u32;

    /// Iterateur sur toutes les entites vivantes
    pub fn entities(&self) -> impl Iterator<Item = Entity>;
}
```

### Gestion des composants

```rust
impl World {
    /// Ajoute un composant a une entite
    pub fn insert<T: Component>(&mut self, entity: Entity, component: T);

    /// Retire un composant d'une entite
    pub fn remove<T: Component>(&mut self, entity: Entity) -> Option<T>;

    /// Retourne une reference au composant
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T>;

    /// Retourne une reference mutable au composant
    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T>;

    /// Verifie si une entite a un composant
    pub fn has<T: Component>(&self, entity: Entity) -> bool;

    /// Ajoute plusieurs composants (builder pattern)
    pub fn with<T: Component>(mut self, entity: Entity, component: T) -> Self;
}
```

### Builder d'entite

```rust
impl World {
    /// Cree une entite avec des composants
    pub fn spawn_with(&mut self) -> EntityBuilder;
}

pub struct EntityBuilder<'a> {
    world: &'a mut World,
    entity: Entity,
}

impl<'a> EntityBuilder<'a> {
    /// Ajoute un composant
    pub fn with<T: Component>(self, component: T) -> Self;

    /// Termine et retourne l'entite
    pub fn build(self) -> Entity;
}
```

### Utilisation du builder

```rust
let player = world.spawn_with()
    .with(Position { x: 100.0, y: 100.0 })
    .with(Velocity { x: 0.0, y: 0.0 })
    .with(SpriteComponent { ... })
    .with(Player)
    .build();
```

---

## Queries (Requetes)

Systeme de requetes pour iterer sur les composants.

### Methodes de requete

```rust
impl World {
    /// Itere sur les entites avec un composant
    pub fn query<T: Component>(&self) -> impl Iterator<Item = (Entity, &T)>;

    /// Itere sur les entites avec un composant (mutable)
    pub fn query_mut<T: Component>(&mut self) -> impl Iterator<Item = (Entity, &mut T)>;

    /// Itere sur les entites avec deux composants
    pub fn query2<A, B>(&self) -> impl Iterator<Item = (Entity, &A, &B)>
    where
        A: Component,
        B: Component;

    /// Itere sur les entites avec deux composants (mutables)
    pub fn query2_mut<A, B>(&mut self) -> impl Iterator<Item = (Entity, &mut A, &mut B)>
    where
        A: Component,
        B: Component;

    /// Requete avec 3 composants
    pub fn query3<A, B, C>(&self) -> impl Iterator<Item = (Entity, &A, &B, &C)>;
    pub fn query3_mut<A, B, C>(&mut self) -> impl Iterator<Item = (Entity, &mut A, &mut B, &mut C)>;
}
```

### Exemples de requetes

```rust
// Toutes les entites avec Position
for (entity, pos) in world.query::<Position>() {
    println!("{}: ({}, {})", entity, pos.x, pos.y);
}

// Mise a jour des positions avec velocite
for (entity, pos, vel) in world.query2_mut::<Position, Velocity>() {
    pos.x += vel.x * dt;
    pos.y += vel.y * dt;
}

// Rendu des sprites
for (entity, pos, sprite) in world.query2::<Position, SpriteComponent>() {
    renderer.draw_sprite_at(sprite, pos.x, pos.y);
}
```

### Filtrage par tag

```rust
// Trouver le joueur
for (entity, pos, _) in world.query2::<Position, Player>() {
    // 'entity' est le joueur
    player_position = *pos;
}

// Tous les ennemis
for (entity, pos, _) in world.query2::<Position, Enemy>() {
    update_enemy_ai(entity, pos);
}
```

---

## Resources

Donnees globales partagees (singletons).

### Methodes

```rust
impl World {
    /// Ajoute une ressource
    pub fn insert_resource<R: 'static>(&mut self, resource: R);

    /// Retourne une reference a une ressource
    pub fn resource<R: 'static>(&self) -> Option<&R>;

    /// Retourne une reference mutable a une ressource
    pub fn resource_mut<R: 'static>(&mut self) -> Option<&mut R>;

    /// Verifie si une ressource existe
    pub fn has_resource<R: 'static>(&self) -> bool;

    /// Retire une ressource
    pub fn remove_resource<R: 'static>(&mut self) -> Option<R>;
}
```

### Utilisation

```rust
// Definir une ressource
pub struct GameConfig {
    pub difficulty: Difficulty,
    pub volume: f32,
}

pub struct DeltaTime(pub f32);

// Inserer
world.insert_resource(GameConfig {
    difficulty: Difficulty::Normal,
    volume: 0.8,
});

world.insert_resource(DeltaTime(0.0));

// Utiliser
if let Some(config) = world.resource::<GameConfig>() {
    println!("Difficulte: {:?}", config.difficulty);
}

// Modifier
if let Some(dt) = world.resource_mut::<DeltaTime>() {
    dt.0 = raw_dt;
}
```

---

## SparseSet

Stockage efficace des composants utilisant un sparse set.

### Structure

```rust
pub struct SparseSet<T> {
    /// Sparse array: entity_index -> dense_index
    sparse: Vec<Option<u32>>,

    /// Dense array: stockage des composants
    dense: Vec<T>,

    /// Mapping inverse: dense_index -> entity_index
    entities: Vec<u32>,
}
```

### Caracteristiques

- **O(1)** insertion, suppression, acces
- **Memoire dense** pour l'iteration
- **Cache-friendly** pour les requetes

### Methodes (usage interne)

```rust
impl<T> SparseSet<T> {
    pub fn new() -> Self;
    pub fn insert(&mut self, index: u32, value: T);
    pub fn remove(&mut self, index: u32) -> Option<T>;
    pub fn get(&self, index: u32) -> Option<&T>;
    pub fn get_mut(&mut self, index: u32) -> Option<&mut T>;
    pub fn contains(&self, index: u32) -> bool;
    pub fn len(&self) -> usize;
    pub fn iter(&self) -> impl Iterator<Item = (u32, &T)>;
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut T)>;
}
```

---

## Systemes

Les systemes sont des fonctions operant sur le World.

### Pattern recommande

```rust
// Systeme de mouvement
fn movement_system(world: &mut World, dt: f32) {
    for (_, pos, vel) in world.query2_mut::<Position, Velocity>() {
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
    }
}

// Systeme de collision
fn collision_system(world: &World) -> Vec<(Entity, Entity)> {
    let mut collisions = Vec::new();

    let entities: Vec<_> = world.query2::<Position, Collider>()
        .map(|(e, p, c)| (e, *p, c.clone()))
        .collect();

    for i in 0..entities.len() {
        for j in (i + 1)..entities.len() {
            let (e1, p1, c1) = &entities[i];
            let (e2, p2, c2) = &entities[j];

            if aabb_intersects(p1, c1, p2, c2) {
                collisions.push((*e1, *e2));
            }
        }
    }

    collisions
}

// Systeme de rendu
fn render_system(world: &World, renderer: &mut Renderer) {
    // Collecter et trier par Y pour le Y-sorting
    let mut renderables: Vec<_> = world
        .query2::<Position, SpriteComponent>()
        .collect();

    renderables.sort_by(|a, b| {
        a.1.y.partial_cmp(&b.1.y).unwrap()
    });

    for (_, pos, sprite) in renderables {
        renderer.draw_sprite(sprite, pos);
    }
}
```

### Organisation des systemes

```rust
fn update_systems(world: &mut World, dt: f32) {
    // Ordre important!
    input_system(world);
    movement_system(world, dt);
    collision_system(world);
    animation_system(world, dt);
    cleanup_system(world);
}

fn render_systems(world: &World, renderer: &mut Renderer) {
    render_tilemap_system(world, renderer);
    render_entities_system(world, renderer);
    render_ui_system(world, renderer);
}
```

---

## Exemple complet

```rust
use engine_ecs::{World, Entity, Component};

// Composants
#[derive(Debug, Clone, Copy)]
struct Position { x: f32, y: f32 }

#[derive(Debug, Clone, Copy)]
struct Velocity { x: f32, y: f32 }

#[derive(Debug, Clone, Copy)]
struct Health { current: i32, max: i32 }

#[derive(Debug, Clone, Copy)]
struct Player;

#[derive(Debug, Clone, Copy)]
struct Enemy;

// Ressources
struct GameTime { dt: f32, total: f32 }

fn main() {
    let mut world = World::new();

    // Ressources
    world.insert_resource(GameTime { dt: 0.0, total: 0.0 });

    // Creer le joueur
    let player = world.spawn_with()
        .with(Position { x: 100.0, y: 100.0 })
        .with(Velocity { x: 0.0, y: 0.0 })
        .with(Health { current: 100, max: 100 })
        .with(Player)
        .build();

    // Creer des ennemis
    for i in 0..5 {
        world.spawn_with()
            .with(Position { x: i as f32 * 50.0, y: 200.0 })
            .with(Velocity { x: -20.0, y: 0.0 })
            .with(Health { current: 50, max: 50 })
            .with(Enemy)
            .build();
    }

    // Boucle de jeu
    loop {
        let dt = get_delta_time();

        // Mettre a jour la ressource temps
        if let Some(time) = world.resource_mut::<GameTime>() {
            time.dt = dt;
            time.total += dt;
        }

        // Systemes
        movement_system(&mut world, dt);
        collision_system(&mut world);
        cleanup_dead_entities(&mut world);

        // Rendu
        render(&world);
    }
}

fn movement_system(world: &mut World, dt: f32) {
    for (_, pos, vel) in world.query2_mut::<Position, Velocity>() {
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
    }
}

fn cleanup_dead_entities(world: &mut World) {
    let dead: Vec<Entity> = world
        .query::<Health>()
        .filter(|(_, h)| h.current <= 0)
        .map(|(e, _)| e)
        .collect();

    for entity in dead {
        world.despawn(entity);
    }
}
```

---

## Bonnes pratiques

1. **Composants petits**: Garder les composants simples et focus
2. **Pas de logique dans les composants**: Les composants sont des donnees pures
3. **Systemes independants**: Eviter les dependances entre systemes quand possible
4. **Utiliser les ressources**: Pour les donnees globales (temps, config, etc.)
5. **Collecter avant de muter**: Si vous devez modifier pendant l'iteration, collectez d'abord
6. **Tags pour le filtrage**: Utiliser des structs vides comme tags (Player, Enemy)
