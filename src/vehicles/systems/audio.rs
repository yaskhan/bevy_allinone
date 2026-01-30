use bevy::prelude::*;
use crate::vehicles::types::*;

pub fn update_vehicle_audio(
    _time: Res<Time>,
    mut vehicle_query: Query<(&mut Vehicle, &mut VehicleAudio, &Children)>,
    wheel_query: Query<&VehicleWheel>,
) {
    for (vehicle, mut audio, children) in vehicle_query.iter_mut() {
        if vehicle.is_turned_on && vehicle.is_driving {
            // Update engine audio pitch/volume based on RPM
            audio.engine_pitch = 0.5 + (vehicle.current_rpm / 3000.0);
            audio.engine_volume = 0.5 + (vehicle.motor_input.abs() * 0.5);
            audio.is_engine_playing = true;
            
            // Calculate average slip from wheels
            let mut total_slip = 0.0;
            let mut wheel_count = 0;
            for child in children.iter() {
                if let Ok(wheel) = wheel_query.get(child.clone()) {
                    total_slip += wheel.slip_amount_sideways + wheel.slip_amount_forward * 0.5;
                    wheel_count += 1;
                }
            }

            // Skid audio based on sideways slip
            if wheel_count > 0 {
                audio.skid_volume = (total_slip / wheel_count as f32).clamp(0.0, 1.0);
                audio.is_skid_playing = audio.skid_volume > 0.2;
            } else {
                audio.is_skid_playing = false;
                audio.skid_volume = 0.0;
            }
        } else {
            audio.is_engine_playing = false;
            audio.is_skid_playing = false;
            audio.engine_volume = 0.0;
            audio.skid_volume = 0.0;
        }
    }
}
