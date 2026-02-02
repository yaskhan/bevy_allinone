use bevy::prelude::*;
use super::types::*;
use super::ability_info::AbilityInfo;

/// System to update ability timers
pub fn update_abilities(
    time: Res<Time>,
    mut abilities: Query<&mut AbilityInfo>,
) {
    let delta_time = time.delta_secs();
    
    for mut ability in abilities.iter_mut() {
        ability.update(delta_time);
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
