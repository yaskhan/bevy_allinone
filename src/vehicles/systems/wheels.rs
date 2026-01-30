use bevy::prelude::*;
use crate::vehicles::types::*;
use avian3d::prelude::*;

pub fn update_vehicle_wheels(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut vehicle_query: Query<(&mut Vehicle, &Children, &GlobalTransform)>,
    mut wheel_query: Query<(&mut VehicleWheel, &GlobalTransform, &Children)>,
    mut transform_query: Query<&mut Transform>,
) {
    let delta = time.delta_secs();

    for (mut vehicle, children, v_gt) in vehicle_query.iter_mut() {
        let mut total_rpm = 0.0;
        let mut powered_wheels = 0;
        let vehicle_up = v_gt.up();
        let _vehicle_scale = v_gt.compute_transform().scale.y;

        for child in children.iter() {
            if let Ok((mut wheel, wheel_gt, wheel_children)) = wheel_query.get_mut(child.clone()) {
                // Wheel center in world space
                let wheel_pos = wheel_gt.translation();
                let ray_direction = Dir3::new(-*vehicle_up).unwrap_or(Dir3::NEG_Y);
                let ray_distance = wheel.suspension_distance + wheel.radius;

                // Raycast for suspension
                let filter = SpatialQueryFilter::default().with_excluded_entities(vec![child.clone()]); // Should exclude vehicle too
                let hit = spatial_query.cast_ray(wheel_pos, ray_direction, ray_distance, false, &filter);

                if let Some(hit_data) = hit {
                    // Update wheel suspension position
                    wheel.suspension_spring_pos = -(hit_data.distance - wheel.radius);
                    
                    // Update mesh position (visual)
                    for mesh_child in wheel_children.iter() {
                        if let Ok(mut mesh_trans) = transform_query.get_mut(mesh_child.clone()) {
                            // Local offset relative to wheel entity
                            let local_hit_pos = hit_data.distance - wheel.radius;
                            mesh_trans.translation.y = -local_hit_pos;
                        }
                    }
                } else {
                    // Wheel in air
                    wheel.suspension_spring_pos = -wheel.suspension_distance;
                    for mesh_child in wheel_children.iter() {
                        if let Ok(mut mesh_trans) = transform_query.get_mut(mesh_child.clone()) {
                            mesh_trans.translation.y = -wheel.suspension_distance;
                        }
                    }
                }

                // Update wheel rotation based on vehicle speed
                let wheel_rpm = if wheel.is_powered {
                    vehicle.current_speed * 10.0
                } else {
                    vehicle.current_speed * 5.0 // Free rolling
                };

                wheel.current_rpm = wheel_rpm;
                wheel.rotation_value += wheel_rpm * delta * 6.0;

                if wheel.is_powered {
                    total_rpm += wheel_rpm;
                    powered_wheels += 1;
                }

                // Update visual rotation and steering
                for mesh_child in wheel_children.iter() {
                    if let Ok(mut mesh_trans) = transform_query.get_mut(mesh_child.clone()) {
                        let steer_angle = if wheel.is_steerable {
                            let angle = vehicle.steer_input * vehicle.steering_angle.to_radians();
                            if wheel.reverse_steer { -angle } else { angle }
                        } else {
                            0.0
                        };

                        mesh_trans.rotation = Quat::from_euler(
                            EulerRot::XYZ,
                            wheel.rotation_value.to_radians(),
                            steer_angle,
                            0.0,
                        );
                    }
                }

                // Calculate slip
                let forward_speed = vehicle.current_speed;
                wheel.slip_amount_forward = (forward_speed * 0.1).clamp(0.0, 1.0);
                wheel.slip_amount_sideways = (vehicle.steer_input.abs() * forward_speed * 0.05).clamp(0.0, 1.0);
            }
        }

        // Average RPM for powered wheels
        if powered_wheels > 0 {
            vehicle.current_rpm = total_rpm / powered_wheels as f32;
        }
    }
}
