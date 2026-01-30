use bevy::prelude::*;
use crate::vehicles::types::*;

pub fn update_vehicle_weapon_aiming(
    time: Res<Time>,
    mut weapon_system_query: Query<(&VehicleWeaponSystem, &GlobalTransform)>,
    mut transform_query: Query<&mut Transform>,
    camera_query: Query<&GlobalTransform, With<Camera>>, // Simplified: follow main camera
) {
    let delta = time.delta_secs();
    let camera_gt = camera_query.iter().next();
    let camera_forward = camera_gt.map(|gt| gt.forward()).unwrap_or(Dir3::NEG_Z);

    for (weapon_sys, v_gt) in weapon_system_query.iter() {
        if !weapon_sys.aiming_enabled { continue; }

        // Horizontal rotation (Base Y)
        if let Some(base_x) = weapon_sys.base_x_entity {
            if let Ok(mut transform) = transform_query.get_mut(base_x) {
                let target_dir = camera_forward.as_vec3();
                let local_target = v_gt.affine().inverse().transform_vector3(target_dir);
                let target_yaw = local_target.x.atan2(local_target.z);
                
                let (current_yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
                let new_yaw = current_yaw + (target_yaw - current_yaw) * delta * weapon_sys.rotation_speed;
                transform.rotation = Quat::from_rotation_y(new_yaw);
            }
        }

        // Vertical rotation (Base Y)
        if let Some(base_y) = weapon_sys.base_y_entity {
            if let Ok(mut transform) = transform_query.get_mut(base_y) {
                let target_dir = camera_forward.as_vec3();
                let local_target = v_gt.affine().inverse().transform_vector3(target_dir);
                let target_pitch = (-local_target.y).atan2((local_target.x.powi(2) + local_target.z.powi(2)).sqrt());
                
                let (_, current_pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                let new_pitch = current_pitch + (target_pitch - current_pitch) * delta * weapon_sys.rotation_speed;
                transform.rotation = Quat::from_rotation_x(new_pitch);
            }
        }
    }
}

pub fn handle_vehicle_weapon_firing(
    time: Res<Time>,
    mut query: Query<&mut VehicleWeaponSystem>,
    _mouse_button: Res<ButtonInput<MouseButton>>, // Integration with input system
) {
    let current_time = time.elapsed_secs();

    for mut weapon_sys in query.iter_mut() {
        if !weapon_sys.weapons_activated { continue; }
        
        let idx = weapon_sys.current_weapon_index;
        if idx >= weapon_sys.weapons.len() { continue; }
        
        // Handle reload
        if weapon_sys.weapons[idx].is_reloading {
            weapon_sys.weapons[idx].reload_timer -= time.delta_secs();
            if weapon_sys.weapons[idx].reload_timer <= 0.0 {
                weapon_sys.weapons[idx].is_reloading = false;
                let needed = weapon_sys.weapons[idx].clip_size - weapon_sys.weapons[idx].ammo_in_clip;
                let to_add = needed.min(weapon_sys.weapons[idx].total_ammo);
                weapon_sys.weapons[idx].ammo_in_clip += to_add;
                weapon_sys.weapons[idx].total_ammo -= to_add;
            }
            continue;
        }

        // Firing logic would go here, checking for input from a shared vehicle input component
    }
}
