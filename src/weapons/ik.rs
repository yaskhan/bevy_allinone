use bevy::prelude::*;
use super::types::*;
use crate::input::InputState;
use avian3d::prelude::*;

/// System that handles procedural weapon IK transforms, swaying, and bobbing.
pub fn handle_weapon_ik(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &Weapon,
        &WeaponIkSettings,
        &mut WeaponIkState,
        &mut Transform,
        &InputState,
    )>,
    // Query for the owner (Player) via WeaponManager linkage
    player_query: Query<(&crate::weapons::weapon_manager::WeaponManager, &LinearVelocity)>,
) {
    let dt = time.delta_secs();

    for (entity, weapon, settings, mut state, mut transform, input) in query.iter_mut() {
        // Try to find the player owner to access WeaponManager and Velocity
        let mut weapon_manager_opt = None;
        let mut velocity_opt = None;

        // Traverse all managers to find one that owns this weapon
        // This avoids relying on Parent/Children hierarchy components which might be missing or complex
        for (manager, vel) in player_query.iter() {
            if manager.weapons_list.contains(&entity) {
                weapon_manager_opt = Some(manager);
                velocity_opt = Some(vel);
                break;
            }
        }

        // 1. Determine Target Offset based on state
        let target = if let Some(manager) = weapon_manager_opt {
            if weapon.is_reloading || manager.reloading_with_animation {
                 &settings.walk_offset 
            } else if manager.aiming_in_third_person || manager.aiming_in_first_person { // Use manager state
                &settings.aim_offset
            } else if input.sprint_pressed { // Could also check character state
                &settings.run_offset
            } else {
                &settings.walk_offset
            }
        } else {
            // Fallback to input if no manager found
             if weapon.is_reloading {
                 &settings.walk_offset
            } else if input.aim_pressed {
                &settings.aim_offset
            } else if input.sprint_pressed {
                &settings.run_offset
            } else {
                &settings.walk_offset
            }
        };

        // 2. Smoothly transition to target offset
        state.target_offset = *target;
        
        let lerp_speed = 10.0;
        state.current_offset.translation = state.current_offset.translation.lerp(state.target_offset.translation, lerp_speed * dt);
        state.current_offset.rotation = state.current_offset.rotation.slerp(state.target_offset.rotation, lerp_speed * dt);

        // 3. Procedural Sway (Keep existing mouse delta logic)
        let sway_speed = settings.sway_settings.lerp_speed;
        let mouse_delta = input.look;
        
        // ... (sway logic identical to before)
        let target_sway_x = -mouse_delta.x * settings.sway_settings.horizontal_amount;
        let target_sway_y = -mouse_delta.y * settings.sway_settings.vertical_amount;
        
        let target_sway = Vec3::new(
            target_sway_x.clamp(-settings.sway_settings.max_offset.x, settings.sway_settings.max_offset.x),
            target_sway_y.clamp(-settings.sway_settings.max_offset.y, settings.sway_settings.max_offset.y),
            0.0,
        );
        state.sway_offset = state.sway_offset.lerp(target_sway, sway_speed * dt);

        // 4. Procedural Bobbing
        // Use actual velocity if available
        let speed = if let Some(vel) = velocity_opt {
            vel.0.length() // Horizontal speed mostly
        } else {
            if input.movement.length() > 0.1 { 5.0 } else { 0.0 } // Fallback
        };

        if speed > 0.1 {
            let bob_speed = 10.0; // Could scale with speed
            state.bob_offset.x = (time.elapsed_secs() * bob_speed).sin() * settings.bob_amount.x;
            state.bob_offset.y = (time.elapsed_secs() * bob_speed * 2.0).cos().abs() * settings.bob_amount.y;
        } else {
            state.bob_offset = state.bob_offset.lerp(Vec3::ZERO, 5.0 * dt);
        }

        // 5. Apply recoil recovery
        state.recoil_offset = state.recoil_offset.lerp(Vec3::ZERO, 5.0 * dt);
        state.recoil_rotation = state.recoil_rotation.slerp(Quat::IDENTITY, 5.0 * dt);

        // 6. Combine everything into the final transform with weight
        let weight = state.weight;
        if weight > 0.001 {
            transform.translation = (state.current_offset.translation + state.sway_offset + state.bob_offset + state.recoil_offset) * weight;
            transform.rotation = Quat::IDENTITY.slerp(state.current_offset.rotation * state.recoil_rotation, weight);
        } else {
            transform.translation = Vec3::ZERO;
            transform.rotation = Quat::IDENTITY;
        }
    }
}

/// Helper function to apply procedural recoil to a weapon
pub fn apply_procedural_recoil(state: &mut WeaponIkState, amount: Vec3, rot_amount: Quat) {
    state.recoil_offset += amount;
    state.recoil_rotation *= rot_amount;
}
