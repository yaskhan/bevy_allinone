use bevy::prelude::*;
use avian3d::prelude::*;
use super::types::*;
use crate::input::InputState;

/// System to handle grenade throwing logic
pub fn handle_grenade_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &InputState, &mut GrenadeState, &GlobalTransform)>,
    spatial_query: SpatialQuery,
) {
    let dt = time.delta_secs();

    for (entity, input, mut state, transform) in query.iter_mut() {
        if state.grenade_count <= 0 {
            continue;
        }

        // Use 'G' for grenade (hardcoded for now, should map to InputState)
        // Note: Assuming InputState has a 'grenade_pressed' or similar. 
        // If not, we check for a specific key for now.
        
        if input.fire_pressed { // Placeholder for specific grenade key check
            // If the user wants to cook the grenade or preparing throw
            if !state.is_preparing {
                state.is_preparing = true;
                state.charge_timer = 0.0;
            }
            state.charge_timer += dt;
        } else if state.is_preparing {
            // Confirm throw
            throw_grenade(&mut commands, &state, transform, &spatial_query, entity);
            state.is_preparing = false;
            state.grenade_count -= 1;
            state.charge_timer = 0.0;
        }
    }
}

pub fn throw_grenade(
    commands: &mut Commands,
    state: &GrenadeState,
    thrower_transform: &GlobalTransform,
    spatial_query: &SpatialQuery,
    owner: Entity,
) {
    let origin = thrower_transform.translation() + thrower_transform.forward() * 0.5 + thrower_transform.up() * 0.5;
    let target_dir = thrower_transform.forward();
    
    // Calculate target point via raycast to find where the player is looking
    let mut target_point = origin + target_dir * 20.0; // Default distance
    let filter = SpatialQueryFilter::from_excluded_entities([owner]);
    
    if let Some(hit) = spatial_query.cast_ray(
        origin,
        Dir3::new(target_dir).unwrap_or(Dir3::Y),
        100.0,
        true,
        &filter,
    ) {
        target_point = hit.point;
    }

    let velocity = calculate_parable_velocity(origin, target_point, 1.0); // 1.0s flight time approx

    // Spawn grenade projectile
    // In a real implementation, we'd use a prefab/scene or specific bundle
    commands.spawn((
        Projectile {
            velocity,
            damage: state.settings.explosion_damage,
            lifetime: state.settings.cook_time.max(3.0),
            owner,
            mass: 0.5,
            drag_coeff: 0.47,
            reference_area: 0.01,
            penetration_power: 0.0,
            use_gravity: true,
            rotate_to_velocity: true,
        },
        ExplosionSettings {
            force: state.settings.max_throw_force,
            radius: state.settings.explosion_radius,
            damage: state.settings.explosion_damage,
            push_characters: true,
        },
        Transform::from_translation(origin),
        GlobalTransform::default(),
        VisibilityBundle::default(),
        RigidBody::Dynamic,
        Collider::sphere(0.1),
    ));
}

fn calculate_parable_velocity(origin: Vec3, target: Vec3, time: f32) -> Vec3 {
    let to_target = target - origin;
    let h = to_target.y;
    let horizontal_dist = Vec3::new(to_target.x, 0.0, to_target.z).length();
    
    // Simple kinematics: 
    // x = v_x * t => v_x = x / t
    // y = v_y * t - 0.5 * g * t^2 => v_y = (y + 0.5 * g * t^2) / t
    
    let g = 9.81;
    let v_y = (h + 0.5 * g * time * time) / time;
    let v_xz = horizontal_dist / time;
    
    let mut velocity = Vec3::new(to_target.x, 0.0, to_target.z).normalize_or_zero() * v_xz;
    velocity.y = v_y;
    
    velocity
}
