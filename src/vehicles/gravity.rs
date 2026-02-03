use bevy::prelude::*;
use avian3d::prelude::*;

use crate::vehicles::types::VehicleGravity;

pub fn update_vehicle_gravity(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut query: Query<(Entity, &mut Transform, &mut LinearVelocity, &VehicleGravity)>,
) {
    let dt = time.delta_secs();

    for (entity, mut transform, mut velocity, settings) in query.iter_mut() {
        let origin = transform.translation();
        let down = -transform.up();
        let filter = SpatialQueryFilter::from_excluded_entities([entity])
            .with_mask(LayerMask::from_bits(settings.surface_mask));

        if let Some(hit) = spatial_query.cast_ray(origin, Dir3::new(*down).unwrap_or(Dir3::NEG_Y), settings.max_ray_distance, true, &filter) {
            let target_up = hit.normal.normalize_or_zero();

            if settings.align_to_surface {
                let current_up = transform.up();
                let rot = Quat::from_rotation_arc(current_up, target_up);
                let blend = (settings.alignment_speed * dt).clamp(0.0, 1.0);
                transform.rotation = transform.rotation.slerp(rot * transform.rotation, blend);
            }

            // Hover spring towards target height
            let height_error = settings.hover_height - hit.distance;
            let spring_force = height_error * settings.hover_strength;
            velocity.0 += target_up * spring_force * dt;

            // Apply custom gravity along surface normal
            velocity.0 += -target_up * settings.gravity_strength * dt;
        } else {
            // No surface hit, fall with local down
            velocity.0 += down * settings.gravity_strength * dt;
        }
    }
}
