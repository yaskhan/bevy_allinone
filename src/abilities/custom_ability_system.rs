use bevy::prelude::*;
use super::ability_info::AbilityInfo;

#[derive(Debug, Clone)]
pub struct CustomAbilityEvent {
    pub entity: Entity,
    pub ability_name: String,
}

#[derive(Resource, Default)]
pub struct CustomAbilityPressDownEventQueue(pub Vec<CustomAbilityEvent>);
#[derive(Resource, Default)]
pub struct CustomAbilityPressHoldEventQueue(pub Vec<CustomAbilityEvent>);
#[derive(Resource, Default)]
pub struct CustomAbilityPressUpEventQueue(pub Vec<CustomAbilityEvent>);
#[derive(Resource, Default)]
pub struct CustomAbilityPressUpBeforeEventQueue(pub Vec<CustomAbilityEvent>);
#[derive(Resource, Default)]
pub struct CustomAbilityPressUpAfterEventQueue(pub Vec<CustomAbilityEvent>);
#[derive(Resource, Default)]
pub struct CustomAbilityUpdateEventQueue(pub Vec<CustomAbilityEvent>);
#[derive(Resource, Default)]
pub struct CustomAbilityEnableEventQueue(pub Vec<CustomAbilityEvent>);
#[derive(Resource, Default)]
pub struct CustomAbilityDisableEventQueue(pub Vec<CustomAbilityEvent>);
#[derive(Resource, Default)]
pub struct CustomAbilityDeactivateEventQueue(pub Vec<CustomAbilityEvent>);

/// Generic custom ability behavior.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CustomAbilitySystem {
    pub use_event_on_press_down: bool,
    pub use_event_on_press_hold: bool,
    pub use_event_on_press_up: bool,

    pub use_delay_time_to_use_event_on_press_up: bool,
    pub delay_time_to_use_event_on_press_up: f32,

    pub use_event_on_press_up_before_and_after: bool,
    pub delay_time_to_use_event_on_press_up_before: f32,
    pub delay_time_to_use_event_on_press_up_after: f32,

    pub use_delay_time_to_use_event_on_press_hold: bool,
    pub delay_time_to_use_event_on_press_hold: f32,
    pub use_event_on_press_hold_just_once: bool,

    pub use_event_on_update_ability_state: bool,
    pub use_event_on_enable_ability: bool,
    pub use_event_on_disable_ability: bool,
    pub use_event_on_deactivate_ability: bool,

    pub last_time_press_down_used: f32,
    pub event_triggered_on_press_hold: bool,

    pub prev_press_down_active: bool,
    pub prev_press_up_active: bool,
    pub prev_enabled: bool,
    pub prev_active: bool,
}

impl Default for CustomAbilitySystem {
    fn default() -> Self {
        Self {
            use_event_on_press_down: false,
            use_event_on_press_hold: false,
            use_event_on_press_up: false,
            use_delay_time_to_use_event_on_press_up: false,
            delay_time_to_use_event_on_press_up: 0.0,
            use_event_on_press_up_before_and_after: false,
            delay_time_to_use_event_on_press_up_before: 0.0,
            delay_time_to_use_event_on_press_up_after: 0.0,
            use_delay_time_to_use_event_on_press_hold: false,
            delay_time_to_use_event_on_press_hold: 0.0,
            use_event_on_press_hold_just_once: false,
            use_event_on_update_ability_state: false,
            use_event_on_enable_ability: false,
            use_event_on_disable_ability: false,
            use_event_on_deactivate_ability: false,
            last_time_press_down_used: 0.0,
            event_triggered_on_press_hold: false,
            prev_press_down_active: false,
            prev_press_up_active: false,
            prev_enabled: false,
            prev_active: false,
        }
    }
}

pub fn update_custom_ability_system(
    time: Res<Time>,
    mut query: Query<(Entity, &AbilityInfo, &mut CustomAbilitySystem)>,
    mut press_down_events: ResMut<CustomAbilityPressDownEventQueue>,
    mut press_hold_events: ResMut<CustomAbilityPressHoldEventQueue>,
    mut press_up_events: ResMut<CustomAbilityPressUpEventQueue>,
    mut press_up_before_events: ResMut<CustomAbilityPressUpBeforeEventQueue>,
    mut press_up_after_events: ResMut<CustomAbilityPressUpAfterEventQueue>,
    mut update_events: ResMut<CustomAbilityUpdateEventQueue>,
    mut enable_events: ResMut<CustomAbilityEnableEventQueue>,
    mut disable_events: ResMut<CustomAbilityDisableEventQueue>,
    mut deactivate_events: ResMut<CustomAbilityDeactivateEventQueue>,
) {
    for (entity, ability, mut system) in query.iter_mut() {
        if system.use_event_on_update_ability_state {
            update_events.0.push(CustomAbilityEvent {
                entity,
                ability_name: ability.name.clone(),
            });
        }

        if ability.enabled && !system.prev_enabled && system.use_event_on_enable_ability {
            enable_events.0.push(CustomAbilityEvent {
                entity,
                ability_name: ability.name.clone(),
            });
        }

        if !ability.enabled && system.prev_enabled && system.use_event_on_disable_ability {
            disable_events.0.push(CustomAbilityEvent {
                entity,
                ability_name: ability.name.clone(),
            });
        }

        if system.prev_active && !ability.active && system.use_event_on_deactivate_ability {
            deactivate_events.0.push(CustomAbilityEvent {
                entity,
                ability_name: ability.name.clone(),
            });
        }

        if ability.active_from_press_down && !system.prev_press_down_active {
            system.last_time_press_down_used = time.elapsed_secs();
            system.event_triggered_on_press_hold = false;
            if system.use_event_on_press_down {
                press_down_events.0.push(CustomAbilityEvent {
                    entity,
                    ability_name: ability.name.clone(),
                });
            }
        }

        if ability.active && system.use_event_on_press_hold {
            if system.use_delay_time_to_use_event_on_press_hold {
                if (!system.use_event_on_press_hold_just_once || !system.event_triggered_on_press_hold) {
                    if time.elapsed_secs() - system.last_time_press_down_used >= system.delay_time_to_use_event_on_press_hold {
                        press_hold_events.0.push(CustomAbilityEvent {
                            entity,
                            ability_name: ability.name.clone(),
                        });
                        if system.use_event_on_press_hold_just_once {
                            system.event_triggered_on_press_hold = true;
                        }
                    }
                }
            } else {
                press_hold_events.0.push(CustomAbilityEvent {
                    entity,
                    ability_name: ability.name.clone(),
                });
            }
        }

        if ability.active_from_press_up && !system.prev_press_up_active {
            if system.use_event_on_press_up {
                if system.use_event_on_press_up_before_and_after {
                    let elapsed = time.elapsed_secs() - system.last_time_press_down_used;
                    if elapsed < system.delay_time_to_use_event_on_press_up_before {
                        press_up_before_events.0.push(CustomAbilityEvent {
                            entity,
                            ability_name: ability.name.clone(),
                        });
                    } else if elapsed > system.delay_time_to_use_event_on_press_up_after {
                        press_up_after_events.0.push(CustomAbilityEvent {
                            entity,
                            ability_name: ability.name.clone(),
                        });
                    }
                } else if system.use_delay_time_to_use_event_on_press_up {
                    if time.elapsed_secs() - system.last_time_press_down_used >= system.delay_time_to_use_event_on_press_up {
                        press_up_events.0.push(CustomAbilityEvent {
                            entity,
                            ability_name: ability.name.clone(),
                        });
                    }
                } else {
                    press_up_events.0.push(CustomAbilityEvent {
                        entity,
                        ability_name: ability.name.clone(),
                    });
                }
            }
        }

        system.prev_press_down_active = ability.active_from_press_down;
        system.prev_press_up_active = ability.active_from_press_up;
        system.prev_enabled = ability.enabled;
        system.prev_active = ability.active;
    }
}
