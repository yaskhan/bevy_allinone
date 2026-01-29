use bevy::prelude::*;
use super::types::{Weapon, SniperSight, SniperSightSettings};
use super::weapon_manager::WeaponManager;
use crate::camera::CameraState;

/// System to handle sniper sight zoom and overlays
pub fn handle_sniper_sight(
    mut manager_query: Query<&WeaponManager>,
    weapon_query: Query<&Weapon>,
    mut camera_query: Query<&mut CameraState>,
    // mut ui_query: Query<&mut Visibility, With<SniperSightOverlay>>, // Placeholder for actual UI overlay
) {
    for manager in manager_query.iter_mut() {
        let Ok(mut camera_state) = camera_query.get_single_mut() else { continue };

        let mut sight_active = false;
        let mut target_fov = None;
        let mut fov_speed = None;

        if manager.aiming_in_third_person || manager.aiming_in_first_person {
            if let Some(&weapon_entity) = manager.weapons_list.get(manager.current_index) {
                if let Ok(weapon) = weapon_query.get(weapon_entity) {
                    if let Some(settings) = &weapon.sniper_sight_settings {
                        let is_first_person = manager.carrying_weapon_in_first_person; // Assuming logic for 1P/3P
                        
                        let enabled = if is_first_person {
                            settings.enabled_first_person
                        } else {
                            settings.enabled_third_person
                        };

                        if enabled {
                            sight_active = true;
                            target_fov = Some(settings.fov_value);
                            if settings.smooth_fov {
                                fov_speed = Some(settings.fov_speed);
                            }
                        }
                    }
                }
            }
        }

        // Apply FOV override
        camera_state.fov_override = target_fov;
        camera_state.fov_override_speed = fov_speed;

        // Toggle UI overlay (logic would be expanded with actual UI components)
        if sight_active {
            // Logic to show scope overlay
        } else {
            // Logic to hide scope overlay
        }
    }
}
