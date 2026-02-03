use bevy::prelude::*;
use super::ability_info::AbilityInfo;
use super::player_abilities::PlayerAbilitiesSystem;

/// UI marker for ability wheel text output.
///
///
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct AbilityWheelUI {
    /// Whether to show disabled abilities
    pub show_disabled: bool,
}

/// UI marker for a single ability slot element.
///
///
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct AbilitySlotElement {
    /// Slot index in the UI
    pub slot_index: usize,
    /// Ability name displayed in this slot
    pub ability_name: String,
}

/// Update the ability wheel UI text (simple list version).
pub fn update_ability_wheel_ui(
    abilities: Query<&AbilityInfo>,
    player_query: Query<&PlayerAbilitiesSystem>,
    mut text_query: Query<(&AbilityWheelUI, &mut Text)>,
) {
    let Some((ui, mut text)) = text_query.iter_mut().next() else { return };
    if text.0.is_empty() {
        return;
    }

    let current_name = player_query
        .iter()
        .next()
        .and_then(|_| abilities.iter().find(|a| a.is_current))
        .map(|a| a.name.clone());

    let mut output = String::from("Abilities:\n");

    for ability in abilities.iter() {
        if !ability.enabled && !ui.show_disabled {
            continue;
        }

        let marker = if Some(&ability.name) == current_name.as_ref() { "*" } else { "-" };
        let mut status = format!("{:?}", ability.status);
        if ability.cooldown_in_process {
            status.push_str(&format!(" cd:{:.1}", ability.cooldown_timer.max(0.0)));
        }
        if ability.time_limit_in_process {
            status.push_str(&format!(" tl:{:.1}", ability.time_limit_timer.max(0.0)));
        }
        output.push_str(&format!("{} {} ({})\n", marker, ability.name, status));
    }

    text.0[0].value = output;
}

/// Update individual ability slot elements.
pub fn update_ability_slot_elements(
    abilities: Query<&AbilityInfo>,
    mut slots: Query<(&AbilitySlotElement, &mut Text)>,
) {
    for (slot, mut text) in slots.iter_mut() {
        let ability = abilities.iter().find(|a| a.name == slot.ability_name);
        let label = match ability {
            Some(info) => {
                let mut status = format!("{:?}", info.status);
                if info.cooldown_in_process {
                    status.push_str(&format!(" cd:{:.1}", info.cooldown_timer.max(0.0)));
                }
                if info.time_limit_in_process {
                    status.push_str(&format!(" tl:{:.1}", info.time_limit_timer.max(0.0)));
                }
                format!("{}: {} ({})", slot.slot_index, info.name, status)
            }
            None => format!("{}: (empty)", slot.slot_index),
        };

        if text.0.is_empty() {
            continue;
        }
        text.0[0].value = label;
    }
}
