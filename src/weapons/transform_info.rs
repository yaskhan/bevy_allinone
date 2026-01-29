use bevy::prelude::*;
use super::types::Weapon;
use super::weapon_manager::WeaponManager;

/// System to handle weapon transform offsets based on camera mode and携带 status
pub fn update_weapon_transforms(
    time: Res<Time>,
    manager_query: Query<&WeaponManager>,
    mut weapon_query: Query<(&Weapon, &mut Transform)>,
) {
    for manager in manager_query.iter() {
        if let Some(&weapon_entity) = manager.weapons_list.get(manager.current_index) {
            if let Ok((weapon, mut transform)) = weapon_query.get_mut(weapon_entity) {
                let info = &weapon.transform_info;
                
                let target_transform = if manager.carrying_weapon_in_first_person {
                    info.hand_offset_1p
                } else if manager.carrying_weapon_in_third_person {
                    info.hand_offset_3p
                } else {
                    info.holster_offset
                };

                // Smoothly lerp to target transform
                transform.translation = transform.translation.lerp(target_transform.translation, info.lerp_speed * time.delta_secs());
                transform.rotation = transform.rotation.slerp(target_transform.rotation, info.lerp_speed * time.delta_secs());
                transform.scale = transform.scale.lerp(target_transform.scale, info.lerp_speed * time.delta_secs());
            }
        }
        
        // Also handle holster positions for unequipped weapons if needed
        // For now, we only focus on the current weapon active state transitions
    }
}
