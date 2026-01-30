use bevy::prelude::*;
use crate::vehicles::types::*;

pub fn update_vehicle_wheels(
    time: Res<Time>,
    mut vehicle_query: Query<(&mut Vehicle, &Children)>,
    mut wheel_query: Query<&mut VehicleWheel>,
    transform_query: Query<&Transform>,
) {
    let delta = time.delta_secs();

    for (mut vehicle, children) in vehicle_query.iter_mut() {
        let mut total_rpm = 0.0;
        let mut powered_wheels = 0;

        for child in children.iter() {
            if let Ok(mut wheel) = wheel_query.get_mut(child) {
                if let Ok(_) = transform_query.get(child) {
                    // Update wheel rotation based on vehicle speed
                    if wheel.is_powered {
                        let wheel_rpm = vehicle.current_speed * 10.0;
                        wheel.current_rpm = wheel_rpm;
                        wheel.rotation_value += wheel_rpm * delta * 6.0;
                        total_rpm += wheel_rpm;
                        powered_wheels += 1;
                    }

                    // Calculate slip
                    let forward_speed = vehicle.current_speed;
                    wheel.slip_amount_forward = (forward_speed * 0.1).clamp(0.0, 1.0);
                    wheel.slip_amount_sideways = (vehicle.steer_input.abs() * forward_speed * 0.05).clamp(0.0, 1.0);
                }
            }
        }

        // Average RPM for powered wheels
        if powered_wheels > 0 {
            vehicle.current_rpm = total_rpm / powered_wheels as f32;
        }
    }
}
