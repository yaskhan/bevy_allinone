use bevy::prelude::*;
use crate::vehicles::types::*;

pub fn update_vehicle_gears(
    time: Res<Time>,
    mut vehicle_query: Query<(&mut Vehicle, &Children)>,
    gear_query: Query<&VehicleGear>,
) {
    let delta = time.delta_secs();

    for (mut vehicle, children) in vehicle_query.iter_mut() {
        if vehicle.using_gravity_control {
             // Logic for gravity control if needed
        }

        if vehicle.changing_gear {
            continue;
        }

        // Collect gears from children
        let mut gears: Vec<&VehicleGear> = Vec::new();
        for child in children.iter() {
            if let Ok(gear) = gear_query.get(child.clone()) {
                gears.push(gear);
            }
        }

        if gears.is_empty() {
            continue;
        }

        // Sort gears by index/name if necessary, but here we assume they are in order of creation or we add an index
        // For simplicity, let's assume current_gear is an index into this sorted list
        
        let current_gear_idx = vehicle.current_gear;
        let current_speed = vehicle.current_speed;
        let current_rpm = vehicle.current_rpm;

        // Shift Up
        if current_gear_idx + 1 < gears.len() {
            if current_speed >= gears[current_gear_idx].gear_speed && current_rpm > vehicle.min_rpm {
                vehicle.current_gear += 1;
                // Add a small delay/flag for gear changing
                // In Bevy we might use a timer if we want to pause motor input
                // vehicle.changing_gear = true; 
                info!("Shifted up to gear {}", vehicle.current_gear);
            }
        }

        // Shift Down
        if current_gear_idx > 0 {
            if current_speed < gears[current_gear_idx - 1].gear_speed {
                vehicle.current_gear -= 1;
                info!("Shifted down to gear {}", vehicle.current_gear);
            }
        }

        // Reset gear if speed is low and not on ground
        if !vehicle.is_on_ground && current_speed < 5.0 {
            if vehicle.current_gear > 0 {
                vehicle.current_gear = 0;
            }
        }
    }
}
