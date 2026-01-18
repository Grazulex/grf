//! ECS Systems for the game
//!
//! Systems contain the game logic that operates on components.

use engine_core::FIXED_TIMESTEP;
use engine_ecs::World;
use engine_input::Input;
use engine_physics::AABB;
use engine_render::{Camera2D, Tilemap};

use crate::components::{CameraTarget, Collider, PlayerControlled, Position, Velocity};

/// Input system: reads input and sets velocity for player-controlled entities
pub fn input_system(world: &mut World) {
    // Get input resource
    let input = match world.get_resource::<Input>() {
        Some(i) => i,
        None => return,
    };

    // Get movement direction from input
    let direction = input.get_movement_direction();

    // We need to collect entity data first, then modify
    // This avoids borrowing issues
    let entities_to_update: Vec<_> = world
        .query::<PlayerControlled>()
        .map(|(entity, pc)| (entity, pc.speed))
        .collect();

    // Update velocities
    for (entity, speed) in entities_to_update {
        if let Some(vel) = world.get_mut::<Velocity>(entity) {
            vel.x = direction.x * speed;
            vel.y = direction.y * speed;
        }
    }
}

/// Movement system: applies velocity and handles collisions
pub fn movement_system(world: &mut World) {
    let dt = FIXED_TIMESTEP as f32;

    // Get world bounds from tilemap resource
    let world_bounds = world
        .get_resource::<Tilemap>()
        .map(|tm| {
            let (w, h) = tm.pixel_size();
            AABB::new(0.0, 0.0, w as f32, h as f32)
        })
        .unwrap_or_else(|| AABB::new(0.0, 0.0, 320.0, 240.0));

    // Collect entities with position, velocity, and collider
    let entities_to_move: Vec<_> = world
        .query::<Position>()
        .filter_map(|(entity, _)| {
            let vel = world.get::<Velocity>(entity)?;
            let col = world.get::<Collider>(entity)?;
            Some((entity, vel.as_vec2(), col.half_size()))
        })
        .collect();

    // Get tilemap for collision checking
    let tilemap_ptr = world.get_resource::<Tilemap>().map(|t| t as *const Tilemap);

    // Move each entity
    for (entity, velocity, half_size) in entities_to_move {
        if let Some(pos) = world.get_mut::<Position>(entity) {
            // Save previous position for interpolation
            pos.save_previous();

            // Calculate new position
            let new_position = pos.current + velocity * dt;
            let mut final_pos = new_position;

            // 1. Check collision with world bounds
            let new_aabb = AABB::from_center(final_pos, half_size);
            if new_aabb.min.x < world_bounds.min.x {
                final_pos.x = world_bounds.min.x + half_size.x;
            }
            if new_aabb.max.x > world_bounds.max.x {
                final_pos.x = world_bounds.max.x - half_size.x;
            }
            if new_aabb.min.y < world_bounds.min.y {
                final_pos.y = world_bounds.min.y + half_size.y;
            }
            if new_aabb.max.y > world_bounds.max.y {
                final_pos.y = world_bounds.max.y - half_size.y;
            }

            // 2. Check collision with solid tiles
            if let Some(tm_ptr) = tilemap_ptr {
                // Safety: tilemap is borrowed from world resource, still valid
                let tm = unsafe { &*tm_ptr };
                if tm.has_collision() {
                    let player_aabb = AABB::from_center(final_pos, half_size);
                    let solid_tiles = tm.get_solid_tiles_in_rect(player_aabb.min, player_aabb.max);

                    for (_, _, tile_min, tile_max) in solid_tiles {
                        let tile_aabb = AABB {
                            min: tile_min,
                            max: tile_max,
                        };
                        let player_aabb = AABB::from_center(final_pos, half_size);

                        if let Some(collision) = player_aabb.get_collision(&tile_aabb) {
                            final_pos += collision.mtv;
                        }
                    }
                }
            }

            pos.current = final_pos;
        }
    }
}

/// Camera system: makes camera follow entities with CameraTarget component
pub fn camera_system(world: &mut World, dt: f32) {
    // Find the first entity with CameraTarget and Position
    let target_pos = world
        .query::<CameraTarget>()
        .filter_map(|(entity, _)| world.get::<Position>(entity).map(|p| p.current))
        .next();

    // Update camera to follow target
    if let (Some(camera), Some(target)) =
        (world.get_resource_mut::<Camera2D>(), target_pos)
    {
        camera.follow(target, 5.0);
        camera.update(dt);
    }
}
