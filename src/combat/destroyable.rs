use bevy::prelude::*;
use avian3d::prelude::*;
use super::types::*;

/// System to handle death of destroyable objects.
pub fn handle_destroyable_death(
    mut commands: Commands,
    mut death_queue: ResMut<DeathEventQueue>,
    query: Query<(Entity, &GlobalTransform, &DestroyableObject)>,
    spatial_query: SpatialQuery,
    mut damage_queue: ResMut<DamageEventQueue>,
    mut velocity_query: Query<(Entity, &mut LinearVelocity, &GlobalTransform)>,
) {
    let death_events = death_queue.0.drain(..).collect::<Vec<_>>();
    
    for event in death_events {
        if let Ok((entity, transform, destroyable)) = query.get(event.entity) {
            info!("Destroyable object {:?} destroyed!", entity);

            if destroyable.explosion_enabled {
                trigger_explosion(
                    &mut commands,
                    transform.translation(),
                    &destroyable.explosion_settings,
                    &spatial_query,
                    &mut damage_queue,
                    &mut velocity_query,
                    entity,
                );
            }

            // Despawn the object
            commands.entity(entity).despawn();
        }
    }
}

pub fn trigger_explosion(
    _commands: &mut Commands,
    origin: Vec3,
    settings: &ExplosionSettings,
    _spatial_query: &SpatialQuery,
    damage_queue: &mut DamageEventQueue,
    velocity_query: &mut Query<(Entity, &mut LinearVelocity, &GlobalTransform)>,
    source_entity: Entity,
) {
    info!("Triggering explosion at {:?} with radius {}", origin, settings.radius);

    // 1. Collect affected entities
    let mut affected: Vec<(Entity, Vec3, f32)> = Vec::new();

    for (entity, _, transform) in velocity_query.iter() {
        if entity == source_entity {
            continue;
        }

        let hit_pos = transform.translation();
        let distance = hit_pos.distance(origin);

        if distance <= settings.radius {
            affected.push((entity, hit_pos, distance));
        }
    }

    // 2. Apply effects
    for (entity, hit_pos, distance) in affected {
        // Apply Damage
        damage_queue.0.push(DamageEvent {
            amount: settings.damage,
            damage_type: settings.damage_type,
            source: Some(source_entity),
            target: entity,
            position: Some(origin),
            direction: None,
            ignore_shield: settings.ignore_shield,
        });

        // Apply Force (Directly to Velocity as impulse fallback)
        if let Ok((_, mut velocity, _)) = velocity_query.get_mut(entity) {
            let dir = (hit_pos - origin).normalize_or_zero();
            let falloff = (1.0f32 - (distance / settings.radius)).clamp(0.0f32, 1.0f32);
            let impulse_magnitude = settings.force * falloff;
            
            // v += p / m. Assuming mass=1 for simple environment push.
            velocity.0 += dir * impulse_magnitude;
        }
    }
}
