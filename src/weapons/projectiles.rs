use bevy::prelude::*;
use avian3d::prelude::*;
use super::types::*;
use crate::combat::{DamageEventQueue, DamageEvent, DamageType};

/// System to handle advanced projectile behaviors
pub fn handle_advanced_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut damage_events: ResMut<DamageEventQueue>,
    spatial_query: SpatialQuery,
    mut projectile_query: Query<(Entity, &mut Projectile, &mut Transform, &GlobalTransform, Option<&mut Homing>, Option<&mut StickToSurface>)>,
    target_query: Query<&GlobalTransform, (With<Health>, Without<Projectile>)>,
) {
    let dt = time.delta_secs();

    for (entity, mut projectile, mut transform, global_transform, mut homing, mut stuck) in projectile_query.iter_mut() {
        // 1. Handle Sticking
        if let Some(mut stuck_data) = stuck {
            if stuck_data.is_stuck {
                if let Some(parent) = stuck_data.parent_entity {
                    if let Ok(parent_transform) = target_query.get(parent) {
                        // Keep relative position
                        let world_pos = parent_transform.transform_point(stuck_data.relative_transform.translation);
                        let world_rot = parent_transform.compute_transform().rotation * stuck_data.relative_transform.rotation;
                        transform.translation = world_pos;
                        transform.rotation = world_rot;
                    } else {
                        // Parent is gone, destroy projectile
                        commands.entity(entity).despawn_recursive();
                    }
                }
                continue; // Skip movement logic if stuck
            }
        }

        // 2. Handle Homing
        if let Some(mut homing_data) = homing {
            if homing_data.initial_delay > 0.0 {
                homing_data.initial_delay -= dt;
            } else {
                if let Some(target) = homing_data.target {
                    if let Ok(target_transform) = target_query.get(target) {
                        let target_pos = target_transform.translation();
                        let current_pos = global_transform.translation();
                        let target_dir = (target_pos - current_pos).normalize();
                        let current_dir = projectile.velocity.normalize();
                        
                        // Slerp velocity towards target
                        let new_dir = Vec3::slerp(current_dir, target_dir, homing_data.turn_speed * dt);
                        projectile.velocity = new_dir * projectile.velocity.length();
                    } else {
                        homing_data.target = None; // Target lost
                    }
                } else {
                    // Search for nearest target
                    let mut nearest = None;
                    let mut min_dist = homing_data.search_radius;
                    for (target_ent, target_transform) in target_query.iter_entities() {
                        let dist = global_transform.translation().distance(target_transform.translation());
                        if dist < min_dist {
                            min_dist = dist;
                            nearest = Some(target_ent);
                        }
                    }
                    homing_data.target = nearest;
                }
            }
        }

        // 3. Handle Gravity & Trajectory Rotation
        if projectile.use_gravity {
            projectile.velocity += Vec3::new(0.0, -9.81, 0.0) * dt;
        }

        if projectile.rotate_to_velocity && projectile.velocity.length_squared() > 0.001 {
            transform.look_to(projectile.velocity.normalize(), Vec3::Y);
        }

        // 4. Update Position & Check Collision (Movement is usually handled by a separate system, 
        // but for advanced logic we might need to intercept or augment it)
        // Note: The base firing.rs handles standard projectile movement. 
        // We should ensure they don't fight. 
    }
}

/// System to handle projectile impact effects (Stick, Explode, etc.)
pub fn handle_projectile_impacts(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Projectile, &GlobalTransform, Option<&mut StickToSurface>, Option<&ExplosionSettings>)>,
    spatial_query: SpatialQuery,
    mut damage_events: ResMut<DamageEventQueue>,
) {
    // This would be called when a projectile hits a surface.
    // In avian3d, we usually use Collision events or Raycasts.
    // For now, these are stubs to show where the logic goes.
}
