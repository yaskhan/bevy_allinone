use bevy::prelude::*;
use super::ability_info::AbilityInfo;
use super::types::*;

/// Magic spell ability settings.
///
/// GKC reference: `magicSpellActionSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MagicSpellAbility {
    pub ability_name: String,
    pub cast_time: f32,
    pub cast_timer: f32,
    pub active: bool,
}

impl Default for MagicSpellAbility {
    fn default() -> Self {
        Self {
            ability_name: "MagicSpell".to_string(),
            cast_time: 0.5,
            cast_timer: 0.0,
            active: false,
        }
    }
}

/// Start casting a spell when the matching ability is active.
pub fn start_magic_spell_cast(
    mut query: Query<(Entity, &AbilityInfo, &mut MagicSpellAbility)>,
) {
    for (entity, ability, mut spell) in query.iter_mut() {
        if ability.name != spell.ability_name {
            continue;
        }

        if ability.active && !spell.active {
            spell.active = true;
            spell.cast_timer = spell.cast_time;
        }
    }
}

/// Finish casts and emit events.
pub fn update_magic_spell_cast(
    time: Res<Time>,
    mut events: ResMut<MagicSpellCastEventQueue>,
    mut query: Query<(Entity, &mut MagicSpellAbility)>,
) {
    for (entity, mut spell) in query.iter_mut() {
        if !spell.active {
            continue;
        }

        spell.cast_timer -= time.delta_secs();
        if spell.cast_timer <= 0.0 {
            spell.active = false;
            events.0.push(MagicSpellCastEvent {
                caster: entity,
                ability_name: spell.ability_name.clone(),
            });
        }
    }
}
