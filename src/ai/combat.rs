use bevy::prelude::*;
use crate::ai::types::*;
use rand::Rng;

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
    mut query: Query<(
        &GlobalTransform,
        &mut AiController,
        &AiPerception,
        &mut AiCombatSettings,
        &mut crate::input::InputState,
        Option<&AiCombatBrain>,
        Option<&mut AiRangedCombatSettings>,
        Option<&mut AiMeleeCombatSettings>,
        Option<&mut AiCloseCombatSettings>,
        Option<&mut AiPowersCombatSettings>,
    )>,
    target_query: Query<&GlobalTransform>,
) {
    let now = time.elapsed_secs();

    for (transform, mut ai, perception, mut settings, mut input, brain_opt, ranged, melee, close, powers) in query.iter_mut() {
        if ai.state != AiBehaviorState::Combat && ai.state != AiBehaviorState::Attack {
            continue;
        }

        let delta = time.delta_secs();

        // Basic targeting: pick first visible target if none
        if ai.target.is_none() {
            ai.target = perception.visible_targets.first().copied();
        }

        if let Some(target_entity) = ai.target {
            let target_pos = match target_query.get(target_entity) {
                Ok(xf) => xf.translation(),
                Err(_) => {
                    input.aim_pressed = false;
                    input.fire_pressed = false;
                    input.attack_pressed = false;
                    continue;
                }
            };
            let dist = target_pos.distance(transform.translation());

            if let Some(brain) = brain_opt {
                match brain.strategy {
                    AiCombatStrategy::Ranged => {
                        if let Some(mut ranged_settings) = ranged {
                            update_ranged_combat(delta, dist, ai.attack_range, &mut ranged_settings, &mut input);
                        }
                        continue;
                    }
                    AiCombatStrategy::MeleeAdvanced => {
                        if let Some(mut melee_settings) = melee {
                            update_melee_combat(
                                now,
                                dist,
                                ai.attack_range,
                                &mut melee_settings,
                                &mut input,
                            );
                        }
                        continue;
                    }
                    AiCombatStrategy::CloseCombat => {
                        if let Some(mut close_settings) = close {
                            update_close_combat(
                                now,
                                dist,
                                ai.attack_range,
                                &mut close_settings,
                                &mut input,
                            );
                        }
                        continue;
                    }
                    AiCombatStrategy::Powers => {
                        if let Some(mut powers_settings) = powers {
                            update_powers_combat(
                                now,
                                dist,
                                &mut powers_settings,
                                &mut input,
                            );
                        }
                        continue;
                    }
                }
            }

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

fn update_ranged_combat(
    delta: f32,
    dist: f32,
    attack_range: f32,
    settings: &mut AiRangedCombatSettings,
    input: &mut crate::input::InputState,
) {
    input.aim_pressed = true;
    settings.aim_timer = (settings.aim_timer + delta).min(settings.aim_time);

    if dist > attack_range * 1.1 {
        input.fire_pressed = false;
        return;
    }

    if settings.reload_timer > 0.0 {
        settings.reload_timer = (settings.reload_timer - delta).max(0.0);
        input.fire_pressed = false;
        if settings.reload_timer <= 0.0 {
            settings.ammo_in_clip = settings.clip_size;
        }
        return;
    }

    if settings.ammo_in_clip <= 0 {
        settings.reload_timer = settings.reload_time;
        input.reload_pressed = true;
        input.fire_pressed = false;
        return;
    }

    settings.fire_timer = (settings.fire_timer - delta).max(0.0);
    if settings.burst_remaining == 0 {
        if settings.fire_timer <= 0.0 {
            settings.burst_remaining = settings.burst_size;
            settings.fire_timer = settings.burst_cooldown;
        }
    }

    if settings.burst_remaining > 0 {
        if settings.aim_timer >= settings.aim_time && settings.fire_timer <= 0.0 {
            input.fire_pressed = true;
            settings.burst_remaining -= 1;
            settings.fire_timer = settings.burst_interval;
            settings.ammo_in_clip -= 1;
        } else {
            input.fire_pressed = false;
        }
    } else {
        input.fire_pressed = false;
    }
}

fn update_melee_combat(
    now: f32,
    dist: f32,
    attack_range: f32,
    settings: &mut AiMeleeCombatSettings,
    input: &mut crate::input::InputState,
) {
    if dist > attack_range {
        input.attack_pressed = false;
        input.block_pressed = false;
        return;
    }

    let can_attack = now - settings.last_attack_time >= settings.min_time_between_attacks;
    if can_attack {
        input.attack_pressed = true;
        settings.last_attack_time = now;
    } else {
        input.attack_pressed = false;
    }

    let mut rng = rand::rng();
    if rng.random::<f32>() < settings.block_probability {
        input.block_pressed = true;
    }
    if rng.random::<f32>() < settings.parry_probability {
        input.block_pressed = true;
    }

    if rng.random::<f32>() < settings.combo_probability {
        input.attack_pressed = true;
    }
}

fn update_close_combat(
    now: f32,
    dist: f32,
    attack_range: f32,
    settings: &mut AiCloseCombatSettings,
    input: &mut crate::input::InputState,
) {
    if dist > attack_range {
        input.attack_pressed = false;
        return;
    }

    if now - settings.last_attack_time >= settings.min_time_between_attacks {
        input.attack_pressed = true;
        settings.last_attack_time = now;
    } else {
        input.attack_pressed = false;
    }
}

fn update_powers_combat(
    now: f32,
    dist: f32,
    settings: &mut AiPowersCombatSettings,
    input: &mut crate::input::InputState,
) {
    if dist < settings.min_range || dist > settings.max_range {
        input.ability_use_pressed = false;
        return;
    }

    if now - settings.last_cast_time >= settings.cooldown {
        input.ability_use_pressed = true;
        settings.last_cast_time = now;
    } else {
        input.ability_use_pressed = false;
    }
}
