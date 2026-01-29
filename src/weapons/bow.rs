use bevy::prelude::*;
use super::types::{Weapon, BowState, BowSettings};
use super::weapon_manager::WeaponManager;
use crate::input::InputState;

/// System to handle bow pull logic and power scaling
pub fn handle_bow_logic(
    time: Res<Time>,
    input: Res<InputState>,
    mut manager_query: Query<&WeaponManager>,
    mut weapon_query: Query<(&mut Weapon, &mut BowState)>,
) {
    for manager in manager_query.iter() {
        if let Some(&weapon_entity) = manager.weapons_list.get(manager.current_index) {
            if let Ok((mut weapon, mut bow_state)) = weapon_query.get_mut(weapon_entity) {
                if let Some(settings) = &weapon.bow_settings {
                    // Logic for pulling/charging the bow
                    let is_aiming = manager.aiming_in_third_person || manager.aiming_in_first_person;
                    
                    if is_aiming && !weapon.is_reloading {
                        bow_state.is_pulling = true;
                        bow_state.pull_timer += time.delta_secs();
                    } else {
                        // Reset if not aiming or reloading
                        bow_state.is_pulling = false;
                        bow_state.pull_timer = 0.0;
                    }

                    // Power scaling: Apply multipliers if we are about to fire
                    if bow_state.is_pulling && input.fire_just_pressed {
                        if bow_state.pull_timer >= settings.min_time_to_shoot {
                            // Calculate power based on pull time
                            let pull_ratio = (bow_state.pull_timer / settings.pull_force_rate).min(1.0);
                            let damage_mult = 1.0 + pull_ratio * (settings.max_pull_damage_mult - 1.0);
                            
                            // Apply to weapon damage for the next shot
                            // (In a more robust system, we would apply this to the specific projectile spawned)
                            weapon.damage = weapon.base_damage * damage_mult;
                            
                            // Note: handle_weapon_firing will consume the input and trigger the shot
                        }
                    }
                }
            }
        }
    }
}
