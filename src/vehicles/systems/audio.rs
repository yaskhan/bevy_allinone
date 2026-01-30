use bevy::prelude::*;
use crate::vehicles::types::*;

pub fn update_vehicle_audio(
    _time: Res<Time>,
    mut vehicle_query: Query<(&mut Vehicle, &mut VehicleAudio)>,
) {
    for (vehicle, mut audio) in vehicle_query.iter_mut() {
        if vehicle.is_turned_on && vehicle.is_driving {
            // Update engine audio pitch/volume based on RPM
            audio.engine_pitch = 0.5 + (vehicle.current_rpm / 3000.0);
            audio.engine_volume = 0.5 + (vehicle.motor_input.abs() * 0.5);
            audio.is_engine_playing = true;
            
            // Skid audio based on sideways slip
            audio.skid_volume = (vehicle.steer_input.abs() * vehicle.current_speed * 0.02).clamp(0.0, 1.0);
            audio.is_skid_playing = audio.skid_volume > 0.1;
        } else {
            audio.is_engine_playing = false;
            audio.is_skid_playing = false;
            audio.engine_volume = 0.0;
            audio.skid_volume = 0.0;
        }
    }
}
