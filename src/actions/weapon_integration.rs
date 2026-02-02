use bevy::prelude::*;
use crate::weapons::{WeaponManager, Weapon, GrenadeState, throw_grenade};
use crate::devices::types::Power;
use avian3d::prelude::SpatialQuery;
use super::types::*;

/// Process weapon events triggered by actions
pub fn process_weapon_events(
    mut commands: Commands,
    mut weapon_queue: ResMut<WeaponEventQueue>,
    mut weapon_manager_query: Query<&mut WeaponManager>,
    mut weapon_query: Query<(&mut Weapon, &mut Visibility)>,
    mut grenade_query: Query<&mut GrenadeState>,
    transform_query: Query<&GlobalTransform>,
    spatial_query: SpatialQuery,
    time: Res<Time>,
) {
    for event in weapon_queue.0.drain(..) {
        if let Ok(mut manager) = weapon_manager_query.get_mut(event.player_entity) {
            match event.event_type {
                WeaponEventType::Fire { burst_count, delay_between_shots } => {
                    // Trigger weapon fire
                    manager.shooting_single_weapon = true;
                    manager.last_time_fired = time.elapsed_secs();
                    
                    if manager.show_debug_log {
                        info!("Action triggered weapon fire: burst={}, delay={}", burst_count, delay_between_shots);
                    }
                    
                    // Note: Actual projectile spawning or damage is handled by weapon systems
                    // based on shooting_single_weapon flag.
                }
                WeaponEventType::Reload => {
                    if !manager.reloading_with_animation_active {
                        manager.reloading_with_animation_active = true;
                        manager.last_time_reload = time.elapsed_secs();
                        info!("Action triggered weapon reload");
                    }
                }
                WeaponEventType::Aim { enable } => {
                    manager.aiming_in_third_person = enable;
                    manager.aiming_in_first_person = enable;
                    info!("Action triggered aim: {}", enable);
                }
                WeaponEventType::SwitchWeapon { weapon_index } => {
                    if weapon_index >= 0 && (weapon_index as usize) < manager.weapons_list.len() {
                        manager.current_index = weapon_index as usize;
                        manager.changing_weapon = true;
                        info!("Action triggered weapon switch to index {}", weapon_index);
                    }
                }
                WeaponEventType::ThrowGrenade { force: _, direction: _ } => {
                    // Get grenade state and transform
                    if let Ok(mut grenade_state) = grenade_query.get_mut(event.player_entity) {
                        if grenade_state.grenade_count > 0 {
                            if let Ok(transform) = transform_query.get(event.player_entity) {
                                // Trigger throw
                                throw_grenade(&mut commands, &grenade_state, transform, &spatial_query, event.player_entity);
                                
                                grenade_state.grenade_count -= 1;
                                grenade_state.is_preparing = false;
                                grenade_state.charge_timer = 0.0;
                                
                                info!("Action triggered grenade throw. Remaining: {}", grenade_state.grenade_count);
                            }
                        } else {
                            warn!("Action triggered grenade throw but player has no grenades!");
                        }
                    }
                }
                WeaponEventType::None => {}
            }
        }
    }
}

/// Process power events triggered by actions
pub fn process_power_events(
    mut power_queue: ResMut<PowerEventQueue>,
    mut power_query: Query<&mut Power>,
) {
    for event in power_queue.0.drain(..) {
        if let Ok(mut power) = power_query.get_mut(event.player_entity) {
            match event.event_type {
                PowerEventType::ConsumePower { amount } => {
                    let previous = power.current;
                    power.current = (power.current - amount).max(0.0);
                    info!("Action consumed power: {} -> {}", previous, power.current);
                }
                PowerEventType::RestorePower { amount } => {
                    let previous = power.current;
                    power.current = (power.current + amount).min(power.max);
                    info!("Action restored power: {} -> {}", previous, power.current);
                }
                PowerEventType::DrainOverTime { amount_per_second, duration } => {
                    // This would typically be handled by a separate component or a timer
                    // For now, we'll just consume the first tick or log it
                    info!("Action started power drain: {}/s for {}s", amount_per_second, duration);
                }
                PowerEventType::RequirePower { minimum_amount } => {
                    if power.current < minimum_amount {
                        info!("Action power requirement not met: {} < {}", power.current, minimum_amount);
                        // The action itself is already running, so this might need to 
                        // trigger a "power failure" state or cancel the action.
                    }
                }
                PowerEventType::None => {}
            }
        }
    }
}
