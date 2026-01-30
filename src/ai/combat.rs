use bevy::prelude::*;
use crate::ai::types::*;

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct AiCombatSettings {
    pub attack_range: f32,
    pub block_probability: f32,
    pub dodge_probability: f32,
    pub min_time_between_attacks: f32,
}

pub fn update_ai_combat(
    time: Res<Time>,
    mut query: Query<(Entity, &mut AiController, &AiPerception, &AiCombatSettings)>,
) {
    let delta = time.delta_secs();

    for (_entity, mut ai, perception, _settings) in query.iter_mut() {
        if ai.state != AiBehaviorState::Combat {
            continue;
        }

        // Basic targeting: pick first visible target
        if ai.target.is_none() {
            ai.target = perception.visible_targets.first().copied();
        }

        if let Some(_target) = ai.target {
            // Logic to move and attack...
            // This will interface with the weapon/action systems
        }
    }
}
