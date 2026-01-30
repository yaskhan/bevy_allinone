use bevy::prelude::*;
use crate::ai::types::*;

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct AiCombatSettings {
    pub attack_range: f32,
    pub block_probability: f32,
    pub dodge_probability: f32,
    pub min_time_between_attacks: f32,
    pub fire_rate: f32,
    pub last_fire_time: f32,
    pub strafe_timer: f32,
    pub strafe_dir: f32, // 1.0 or -1.0
}

pub fn update_ai_combat(
    time: Res<Time>,
    mut query: Query<(Entity, &mut AiController, &AiPerception, &mut AiCombatSettings, &mut crate::input::InputState)>,
) {
    let now = time.elapsed_secs();

    for (entity, mut ai, perception, mut settings, mut input) in query.iter_mut() {
        if ai.state != AiBehaviorState::Combat && ai.state != AiBehaviorState::Attack {
            continue;
        }

        let delta = time.delta_secs();

        // Basic targeting: pick first visible target if none
        if ai.target.is_none() {
            ai.target = perception.visible_targets.first().copied();
        }

        if let Some(target_entity) = ai.target {
            // Get target transform if possible (we might need a Query for targets here, but for now we look at input.movement)
            // Strafing logic
            settings.strafe_timer -= delta;
            if settings.strafe_timer <= 0.0 {
                settings.strafe_timer = 2.0; // Strafe for 2 seconds
                settings.strafe_dir *= -1.0; // Flip direction
            }

            // Aiming logic
            input.aim_pressed = true;

            // Firing logic based on rate
            if now - settings.last_fire_time >= settings.fire_rate {
                input.fire_pressed = true;
                settings.last_fire_time = now;
            } else {
                input.fire_pressed = false;
            }

            // Melee check
            if ai.state == AiBehaviorState::Attack {
                input.attack_pressed = true;
            }
        } else {
            input.aim_pressed = false;
            input.fire_pressed = false;
            input.attack_pressed = false;
        }
    }
}
