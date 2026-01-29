use bevy::prelude::*;
use super::types::*;
use crate::input::InputState;

/// System that handles procedural weapon IK transforms, swaying, and bobbing.
pub fn handle_weapon_ik(
    time: Res<Time>,
    mut query: Query<(
        &Weapon,
        &WeaponIkSettings,
        &mut WeaponIkState,
        &mut Transform,
        &InputState,
    )>,
) {
    let dt = time.delta_secs();

    for (weapon, settings, mut state, mut transform, input) in query.iter_mut() {
        // 1. Determine Target Offset based on state
        // This is a simplified version; real logic would check aiming/running states from Weapon component.
        let target = if weapon.is_reloading {
             &settings.walk_offset // Use walk offset during reload for now
        } else if input.aim_pressed {
            &settings.aim_offset
        } else if input.sprint_pressed {
            &settings.run_offset
        } else {
            &settings.walk_offset
        };

        // 2. Smoothly transition to target offset
        state.target_offset = *target;
        
        let lerp_speed = 10.0; // Hardcoded lerp speed for smooth transitions
        state.current_offset.translation = state.current_offset.translation.lerp(state.target_offset.translation, lerp_speed * dt);
        state.current_offset.rotation = state.current_offset.rotation.slerp(state.target_offset.rotation, lerp_speed * dt);

        // 3. Procedural Sway
        // Based on mouse movement from InputState
        let sway_speed = settings.sway_settings.lerp_speed;
        let mouse_delta = input.look;
        
        let target_sway_x = -mouse_delta.x * settings.sway_settings.horizontal_amount;
        let target_sway_y = -mouse_delta.y * settings.sway_settings.vertical_amount;
        
        let target_sway = Vec3::new(
            target_sway_x.clamp(-settings.sway_settings.max_offset.x, settings.sway_settings.max_offset.x),
            target_sway_y.clamp(-settings.sway_settings.max_offset.y, settings.sway_settings.max_offset.y),
            0.0,
        );
        
        state.sway_offset = state.sway_offset.lerp(target_sway, sway_speed * dt);

        // 4. Procedural Bobbing
        // Based on player movement (simplified: use sine wave if moving)
        if input.movement.length() > 0.1 {
            let bob_speed = 10.0;
            state.bob_offset.x = (time.elapsed_secs() * bob_speed).sin() * settings.bob_amount.x;
            state.bob_offset.y = (time.elapsed_secs() * bob_speed * 2.0).cos().abs() * settings.bob_amount.y;
        } else {
            state.bob_offset = state.bob_offset.lerp(Vec3::ZERO, 5.0 * dt);
        }

        // 5. Apply recoil recovery
        state.recoil_offset = state.recoil_offset.lerp(Vec3::ZERO, 5.0 * dt);
        state.recoil_rotation = state.recoil_rotation.slerp(Quat::IDENTITY, 5.0 * dt);

        // 6. Combine everything into the final transform
        // Note: We modify the local transform of the weapon entity.
        // It's assumed the weapon is a child of the camera or a hand bone.
        transform.translation = state.current_offset.translation + state.sway_offset + state.bob_offset + state.recoil_offset;
        transform.rotation = state.current_offset.rotation * state.recoil_rotation;
    }
}

/// Helper function to apply procedural recoil to a weapon
pub fn apply_procedural_recoil(state: &mut WeaponIkState, amount: Vec3, rot_amount: Quat) {
    state.recoil_offset += amount;
    state.recoil_rotation *= rot_amount;
}
