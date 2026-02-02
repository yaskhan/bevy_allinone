use bevy::prelude::*;
use super::types::*;
use super::ability_info::AbilityInfo;
use super::player_abilities::PlayerAbilitiesSystem;
use crate::camera::{CameraController, CameraMode};
use crate::input::InputState;
use crate::physics::GroundDetection;
use crate::stats::StatsSystem;

/// System to update ability timers
pub fn update_abilities(
    time: Res<Time>,
    mut abilities: Query<&mut AbilityInfo>,
    mut cooldown_events: EventWriter<AbilityCooldownEvent>,
    mut time_limit_events: EventWriter<AbilityTimeLimitEvent>,
) {
    let delta_time = time.delta_secs();
    
    for mut ability in abilities.iter_mut() {
        let prev_cooldown = ability.cooldown_in_process;
        let prev_time_limit = ability.time_limit_in_process;

        ability.update(delta_time);

        if ability.cooldown_in_process != prev_cooldown {
            cooldown_events.send(AbilityCooldownEvent {
                ability_name: ability.name.clone(),
                started: ability.cooldown_in_process,
            });
            ability.was_cooldown_in_process = ability.cooldown_in_process;
        }

        if ability.time_limit_in_process != prev_time_limit {
            time_limit_events.send(AbilityTimeLimitEvent {
                ability_name: ability.name.clone(),
                started: ability.time_limit_in_process,
            });
            ability.was_time_limit_in_process = ability.time_limit_in_process;
        }
    }
}

/// System to handle ability activation events
pub fn handle_ability_activation(
    mut events: EventReader<ActivateAbilityEvent>,
    mut abilities: Query<&mut AbilityInfo>,
) {
    for event in events.read() {
        if let Some(mut ability) = abilities.iter_mut().find(|a| a.name == event.ability_name) {
            if !ability.enabled {
                continue;
            }

            ability.active = true;
            ability.status = AbilityStatus::Active;

            match event.input_type {
                AbilityInputType::PressDown => {
                    ability.use_press_down();
                }
                AbilityInputType::PressHold => {
                    ability.use_press_hold();
                }
                AbilityInputType::PressUp => {
                    ability.use_press_up();
                }
            }
        }
    }
}

/// Update cached grounded/first-person state for abilities
pub fn update_player_abilities_context(
    mut query: Query<(Entity, &mut PlayerAbilitiesSystem, Option<&GroundDetection>, Option<&StatsSystem>)>,
    camera_query: Query<&CameraController>,
) {
    for (entity, mut system, ground, stats) in query.iter_mut() {
        if let Some(ground_detection) = ground {
            system.is_on_ground = ground_detection.is_grounded;
        }

        if let Some(stats_system) = stats {
            system.sync_energy_from_stats(stats_system);
        }

        let mut is_first_person = false;
        for camera in camera_query.iter() {
            if camera.follow_target == Some(entity) {
                is_first_person = camera.mode == CameraMode::FirstPerson;
                break;
            }
        }

        system.is_first_person_mode = is_first_person;
    }
}

/// Handle ability input from the input system (selection + use).
pub fn handle_ability_input(
    mut player_query: Query<(&InputState, &mut PlayerAbilitiesSystem, Option<&mut StatsSystem>)>,
    mut abilities: Query<&mut AbilityInfo>,
) {
    for (input, mut system, mut stats) in player_query.iter_mut() {
        if !system.enabled || !system.abilities_mode_active || !input.enabled {
            continue;
        }

        if let Some(index) = input.select_ability {
            let mut ability_names: Vec<String> = abilities
                .iter()
                .filter(|a| a.enabled)
                .map(|a| a.name.clone())
                .collect();
            ability_names.sort();

            if let Some(name) = ability_names.get(index) {
                system.set_current_ability_by_name(name, &mut abilities);
            }
        }

        if input.ability_use_pressed {
            if let Some(mut ability) = abilities.iter_mut().find(|a| a.is_current) {
                system.input_press_down_use_current_ability(
                    &mut ability,
                    system.is_on_ground,
                    stats.as_deref_mut(),
                );
            }
        }

        if input.ability_use_held {
            if let Some(mut ability) = abilities.iter_mut().find(|a| a.is_current) {
                system.input_press_hold_use_current_ability(
                    &mut ability,
                    system.is_on_ground,
                    stats.as_deref_mut(),
                );
            }
        }

        if input.ability_use_released {
            if let Some(mut ability) = abilities.iter_mut().find(|a| a.is_current) {
                system.input_press_up_use_current_ability(
                    &mut ability,
                    system.is_on_ground,
                    stats.as_deref_mut(),
                );
            }
        }
    }
}

/// System to handle ability deactivation events
pub fn handle_ability_deactivation(
    mut events: EventReader<DeactivateAbilityEvent>,
    mut abilities: Query<&mut AbilityInfo>,
) {
    for event in events.read() {
        if let Some(mut ability) = abilities.iter_mut().find(|a| a.name == event.ability_name) {
            ability.deactivate();
            ability.status = if ability.enabled {
                AbilityStatus::Enabled
            } else {
                AbilityStatus::Disabled
            };
        }
    }
}

/// System to handle ability enable/disable events
pub fn handle_ability_enabled_events(
    mut events: EventReader<SetAbilityEnabledEvent>,
    mut abilities: Query<&mut AbilityInfo>,
) {
    for event in events.read() {
        if let Some(mut ability) = abilities.iter_mut().find(|a| a.name == event.ability_name) {
            if event.enabled {
                ability.enable();
            } else {
                ability.disable();
            }
        }
    }
}
