//! Ballistics system for projectile physics

use bevy::prelude::*;
use avian3d::prelude::*;
use crate::combat::{DamageEventQueue, DamageEvent, DamageType};
use super::types::{BallisticsEnvironment, Projectile};

/// Update projectile physics and collision
pub fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    spatial_query: SpatialQuery,
    ballistics_env: Res<BallisticsEnvironment>,
    mut damage_events: ResMut<DamageEventQueue>,
    mut query: Query<(Entity, &mut Transform, &mut Projectile)>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 { return; }

    for (entity, mut transform, mut projectile) in query.iter_mut() {
        projectile.lifetime -= dt;
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // --- PHYSICS INTEGRATION (RK4) ---
        // Current state
        let pos = transform.translation;
        let vel = projectile.velocity;

        // Helper closure for acceleration calculation
        // a = g + (F_drag / m)
        // F_drag = 0.5 * density * speed^2 * Cd * Area * direction
        let calc_acceleration = |p: Vec3, v: Vec3| -> Vec3 {
            let relative_velocity = v - ballistics_env.wind;
            let speed_sq = relative_velocity.length_squared();

            if speed_sq < 0.0001 {
                return ballistics_env.gravity;
            }

            let speed = speed_sq.sqrt();
            let direction = relative_velocity / speed; // Normalize

            let drag_magnitude = 0.5 * ballistics_env.air_density * speed_sq * projectile.drag_coeff * projectile.reference_area;
            let drag_force = direction * -drag_magnitude;

            ballistics_env.gravity + (drag_force / projectile.mass)
        };

        // RK4 Steps
        // k1
        let a1 = calc_acceleration(pos, vel);
        let v1 = vel;

        // k2
        let v2 = vel + a1 * (dt * 0.5);
        let a2 = calc_acceleration(pos + v1 * (dt * 0.5), v2);

        // k3
        let v3 = vel + a2 * (dt * 0.5);
        let a3 = calc_acceleration(pos + v2 * (dt * 0.5), v3);

        // k4
        let v4 = vel + a3 * dt;
        let a4 = calc_acceleration(pos + v3 * dt, v4);

        // Final integration
        // velocity += (a1 + 2*a2 + 2*a3 + a4) * (dt / 6.0)
        // position += (v1 + 2*v2 + 2*v3 + v4) * (dt / 6.0)

        let dv = (a1 + 2.0 * a2 + 2.0 * a3 + a4) * (dt / 6.0);
        let dp = (v1 + 2.0 * v2 + 2.0 * v3 + v4) * (dt / 6.0);

        projectile.velocity += dv;
        let new_pos = pos + dp;

        // --- COLLISION DETECTION ---
        let ray_dir = (new_pos - pos).normalize_or_zero();
        let ray_dist = (new_pos - pos).length();

        if ray_dist > 0.0001 {
            let filter = SpatialQueryFilter::from_excluded_entities([projectile.owner]);

            // Raycast from old position to new position
            if let Some(hit) = spatial_query.cast_ray(
                pos,
                Dir3::new(ray_dir).unwrap_or(Dir3::NEG_Z),
                ray_dist,
                true,
                &filter
            ) {
                // --- PENETRATION LOGIC ---
                let hit_point = pos + ray_dir * hit.distance;

                // Check for Surface Properties (Mock implementation)
                // In a real scenario, we would query the entity's components for a `SurfaceMaterial` struct.
                // Here we assume a default "Hard" surface resistance.
                let surface_resistance = 100.0;
                let remaining_energy = projectile.penetration_power - (hit.distance * 0.1); // Simple energy loss model

                if remaining_energy > surface_resistance {
                    // Penetration successful
                    info!("Projectile penetrated surface at {:?}!", hit_point);
                    projectile.penetration_power = remaining_energy - surface_resistance;

                    // Visual effect for penetration
                    spawn_impact_effect(&mut commands, hit_point, "Penetration".to_string());

                    // Continue flight from hit point with reduced velocity (simulating drag inside material)
                    projectile.velocity *= 0.8;
                    transform.translation = hit_point + ray_dir * 0.01; // Push slightly forward to avoid re-hitting same surface
                } else {
                    // Stop or Ricochet
                    info!("Projectile stopped at {:?}!", hit_point);
                    damage_events.0.push(DamageEvent {
                        amount: projectile.damage,
                        damage_type: DamageType::Ranged,
                        source: Some(projectile.owner),
                        target: hit.entity,
                    });

                    spawn_impact_effect(&mut commands, hit_point, "Impact".to_string());
                    commands.entity(entity).despawn();
                }
                continue; // Skip position update if we handled collision
            }
        }

        // Update visual transform
        transform.translation = new_pos;

        // Spawn Tracer Visual (Visual/Simulation Separation)
        spawn_tracer(&mut commands, pos, new_pos);
    }
}

// Helper function to spawn impact effects (Visual Pooling placeholder)
fn spawn_impact_effect(commands: &mut Commands, position: Vec3, effect_type: String) {
    // In a full implementation, this would use the VisualEffectPool
    // For now, we spawn a simple marker
    commands.spawn((
        Transform::from_translation(position),
        Name::new(format!("{}_Effect", effect_type)),
        // Marker component for cleanup system
    ));
}

// Helper function to spawn tracers (Visual/Simulation Separation)
fn spawn_tracer(commands: &mut Commands, start: Vec3, end: Vec3) {
    // In a full implementation, this spawns a BulletTracer entity that interpolates
    // For now, we can spawn a temporary line mesh or particle
    // This function is called every frame for every projectile, so it must be cheap.
    // Ideally, we spawn a component that is handled by a separate visual system.
}
