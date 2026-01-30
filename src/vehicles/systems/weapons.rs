use bevy::prelude::*;
use crate::vehicles::types::*;
use crate::input::InputState;
use avian3d::prelude::*;

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


pub fn update_vehicle_weapon_firing(
    time: Res<Time>,
    mut query: Query<(&mut VehicleWeaponSystem, &InputState, &GlobalTransform)>,
    spatial_query: SpatialQuery,
) {
    let current_time = time.elapsed_secs();
    let delta = time.delta_secs();

    for (mut weapon_sys, input, v_gt) in query.iter_mut() {
        if !weapon_sys.weapons_activated { continue; }
        
        let idx = weapon_sys.current_weapon_index;
        if idx >= weapon_sys.weapons.len() { continue; }

        // Switch weapon
        if input.next_weapon_pressed {
            weapon_sys.current_weapon_index = (weapon_sys.current_weapon_index + 1) % weapon_sys.weapons.len();
        } else if input.prev_weapon_pressed {
            if weapon_sys.current_weapon_index == 0 {
                weapon_sys.current_weapon_index = weapon_sys.weapons.len() - 1;
            } else {
                weapon_sys.current_weapon_index -= 1;
            }
        } else if let Some(sel) = input.select_weapon {
            if sel < weapon_sys.weapons.len() {
                weapon_sys.current_weapon_index = sel;
            }
        }

        let mut weapon = &mut weapon_sys.weapons[idx];

        if input.fire_pressed && !weapon.is_reloading {
            match weapon.weapon_type {
                VehicleWeaponType::Laser => {
                    // Continuous damage
                    if current_time - weapon.last_fire_time > weapon.fire_rate {
                        // Cast laser ray (simplified: forward from vehicle/turret)
                        let fire_dir = v_gt.forward();
                        let fire_pos = v_gt.translation();
                        
                        if let Some(hit) = spatial_query.cast_ray(fire_pos, fire_dir, 1000.0, true, &SpatialQueryFilter::from_mask(0xFFFF_FFFF)) {
                            info!("Laser hit entity: {:?}", hit.entity);
                            // Apply damage logic here
                        }
                        
                        weapon.last_fire_time = current_time;
                    }
                }
                VehicleWeaponType::MachineGun | VehicleWeaponType::Cannon => {
                    if current_time - weapon.last_fire_time > weapon.fire_rate {
                        if weapon.ammo_in_clip > 0 {
                            weapon.ammo_in_clip -= 1;
                            weapon.last_fire_time = current_time;
                            info!("Firing {}: Ammo left {}", weapon.name, weapon.ammo_in_clip);
                        } else {
                            // Auto reload if out of ammo
                            if !weapon.is_reloading && weapon.total_ammo > 0 {
                                weapon.is_reloading = true;
                                weapon.reload_timer = weapon.reload_time;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        
        // Manual reload
        if input.reload_pressed && !weapon.is_reloading && weapon.ammo_in_clip < weapon.clip_size && weapon.total_ammo > 0 {
            weapon.is_reloading = true;
            weapon.reload_timer = weapon.reload_time;
        }

        // Handle active reloading timer
        if weapon.is_reloading {
            weapon.reload_timer -= delta;
            if weapon.reload_timer <= 0.0 {
                weapon.is_reloading = false;
                let needed = weapon.clip_size - weapon.ammo_in_clip;
                let to_add = needed.min(weapon.total_ammo);
                weapon.ammo_in_clip += to_add;
                weapon.total_ammo -= to_add;
                info!("Reloaded {}: Clip {}", weapon.name, weapon.ammo_in_clip);
            }
        }
    }
}
