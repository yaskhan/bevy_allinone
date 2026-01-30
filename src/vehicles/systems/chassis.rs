use bevy::prelude::*;
use crate::vehicles::types::*;

pub fn update_vehicle_chassis(
    time: Res<Time>,
    mut vehicle_query: Query<(&mut Vehicle, &mut Transform)>,
) {
    let delta = time.delta_secs();

    for (mut vehicle, mut transform) in vehicle_query.iter_mut() {
        // Apply chassis lean
        let lean_x = vehicle.chassis_lean_x;
        let lean_y = vehicle.chassis_lean_y;

        // Decay lean over time
        vehicle.chassis_lean_x *= 0.95;
        vehicle.chassis_lean_y *= 0.95;

        // Apply rotation to chassis (visual only)
        let current_euler = transform.rotation.to_euler(EulerRot::XYZ);
        let new_euler = (
            current_euler.0 + lean_x.to_radians() * delta,
            current_euler.1,
            current_euler.2 + lean_y.to_radians() * delta,
        );
        transform.rotation = Quat::from_euler(EulerRot::XYZ, new_euler.0, new_euler.1, new_euler.2);
    }
}
