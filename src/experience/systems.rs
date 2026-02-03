use bevy::prelude::*;
use super::types::*;
use rand::Rng;
use crate::stats::{stats_system::StatsSystem, types::{CoreAttribute, DerivedStat, StatValue}};

pub fn initialize_experience_settings(
    mut settings: ResMut<ExperienceSettings>,
) {
    if !settings.levels.is_empty() {
        return;
    }

    // Default curve: 20 levels, quadratic growth.
    let mut levels = Vec::new();
    for level in 1..=20 {
        let xp_required = 100 * level * level;
        levels.push(ExperienceLevel {
            level_number: level as u32,
            xp_required: xp_required as u32,
            skill_points_reward: 1,
            stat_rewards: Vec::new(),
        });
    }

    settings.levels = levels;
    settings.max_level = Some(20);
    settings.xp_multiplier_enabled = true;
}

pub fn handle_experience_gain(
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
    mut level_up_queue: ResMut<LevelUpQueue>,
    mut query: Query<&mut PlayerExperience>,
    settings: Res<ExperienceSettings>,
) {
    // Drain the queue
    for event in xp_queue.0.drain(..) {
        if let Ok(mut player_xp) = query.get_mut(event.entity) {
            let mut gain = event.amount as f32;
            if player_xp.xp_multiplier_timer > 0.0 {
                gain *= player_xp.xp_multiplier;
            }
            let final_gain = gain as u32;

            player_xp.current_xp += final_gain;
            player_xp.total_xp += final_gain;

            // Check for level up
            loop {
                let current_level_idx = (player_xp.current_level as usize).saturating_sub(1);
                if let Some(level_info) = settings.levels.get(current_level_idx) {
                    if player_xp.current_xp >= level_info.xp_required {
                        player_xp.current_xp -= level_info.xp_required;
                        player_xp.current_level += 1;
                        player_xp.skill_points += level_info.skill_points_reward;

                        level_up_queue.0.push(LevelUpEvent {
                            entity: event.entity,
                            new_level: player_xp.current_level,
                        });

                        // Check if we hit max level
                        if let Some(max) = settings.max_level {
                            if player_xp.current_level >= max {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
}

pub fn handle_level_up_rewards(
    mut level_up_queue: ResMut<LevelUpQueue>,
    settings: Res<ExperienceSettings>,
    mut stats_query: Query<&mut StatsSystem>,
) {
    for event in level_up_queue.0.drain(..) {
        let level_idx = (event.new_level as usize).saturating_sub(1);
        let Some(level_info) = settings.levels.get(level_idx) else { continue };

        let Ok(mut stats) = stats_query.get_mut(event.entity) else { continue };

        for reward in &level_info.stat_rewards {
            if reward.is_bool {
                stats.set_custom_stat(&reward.stat_name, StatValue::Bool(reward.bool_value));
                continue;
            }

            if let Some(core) = parse_core_attribute(&reward.stat_name) {
                stats.increase_core_attribute(core, reward.amount);
                continue;
            }

            if let Some(derived) = parse_derived_stat(&reward.stat_name) {
                stats.increase_derived_stat(derived, reward.amount);
                continue;
            }

            stats.increase_custom_stat(&reward.stat_name, reward.amount);
        }

        // Restore resources to max after level-up.
        if let Some(max_health) = stats.get_derived_stat(DerivedStat::MaxHealth).copied() {
            stats.set_derived_stat(DerivedStat::CurrentHealth, max_health);
        }
        if let Some(max_stamina) = stats.get_derived_stat(DerivedStat::MaxStamina).copied() {
            stats.set_derived_stat(DerivedStat::CurrentStamina, max_stamina);
        }
        if let Some(max_mana) = stats.get_derived_stat(DerivedStat::MaxMana).copied() {
            stats.set_derived_stat(DerivedStat::CurrentMana, max_mana);
        }
    }
}

pub fn sync_experience_to_stats(
    query: Query<(&PlayerExperience, &mut StatsSystem)>,
) {
    for (xp, mut stats) in query.iter() {
        stats.set_derived_stat(DerivedStat::Experience, xp.total_xp as f32);
        stats.set_custom_stat("level", StatValue::Amount(xp.current_level as f32));
    }
}

fn parse_core_attribute(name: &str) -> Option<CoreAttribute> {
    match name.to_lowercase().as_str() {
        "strength" => Some(CoreAttribute::Strength),
        "agility" => Some(CoreAttribute::Agility),
        "intelligence" => Some(CoreAttribute::Intelligence),
        "constitution" => Some(CoreAttribute::Constitution),
        "charisma" => Some(CoreAttribute::Charisma),
        _ => None,
    }
}

fn parse_derived_stat(name: &str) -> Option<DerivedStat> {
    match name.to_lowercase().as_str() {
        "maxhealth" | "max_health" => Some(DerivedStat::MaxHealth),
        "currenthealth" | "current_health" => Some(DerivedStat::CurrentHealth),
        "maxstamina" | "max_stamina" => Some(DerivedStat::MaxStamina),
        "currentstamina" | "current_stamina" => Some(DerivedStat::CurrentStamina),
        "maxmana" | "max_mana" => Some(DerivedStat::MaxMana),
        "currentmana" | "current_mana" => Some(DerivedStat::CurrentMana),
        "attackpower" | "attack_power" => Some(DerivedStat::AttackPower),
        "defense" => Some(DerivedStat::Defense),
        "criticalchance" | "critical_chance" => Some(DerivedStat::CriticalChance),
        "movementspeed" | "movement_speed" => Some(DerivedStat::MovementSpeed),
        "attackspeed" | "attack_speed" => Some(DerivedStat::AttackSpeed),
        "magicresistance" | "magic_resistance" => Some(DerivedStat::MagicResistance),
        "stealth" => Some(DerivedStat::Stealth),
        "persuasion" => Some(DerivedStat::Persuasion),
        "fire_resistance" | "fireresistance" => Some(DerivedStat::FireResistance),
        "poison_resistance" | "poisonresistance" => Some(DerivedStat::PoisonResistance),
        "electric_resistance" | "electricresistance" => Some(DerivedStat::ElectricResistance),
        "explosion_resistance" | "explosionresistance" => Some(DerivedStat::ExplosionResistance),
        _ => None,
    }
}

pub fn update_xp_multiplier(
    time: Res<Time>,
    mut query: Query<&mut PlayerExperience>,
) {
    for mut player_xp in query.iter_mut() {
        if player_xp.xp_multiplier_timer > 0.0 {
            player_xp.xp_multiplier_timer -= time.delta_secs();
            if player_xp.xp_multiplier_timer <= 0.0 {
                player_xp.xp_multiplier_timer = 0.0;
                player_xp.xp_multiplier = 1.0;
            }
        }
    }
}

/// Helper system to grant XP from an object (can be used by other systems like combat)
pub fn grant_xp_from_object(
    object_experience: &ObjectExperience,
    player_entity: Entity,
    source_position: Option<Vec3>,
    xp_queue: &mut ExperienceObtainedQueue,
) {
    let mut rng = rand::rng();
    let amount = if let Some((min, max)) = object_experience.xp_range {
        rng.random_range(min..=max)
    } else {
        object_experience.xp_amount
    };

    xp_queue.0.push(ExperienceObtainedEvent {
        entity: player_entity,
        amount,
        source_position,
    });
}
