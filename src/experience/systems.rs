use bevy::prelude::*;
use super::types::*;
use rand::Rng;

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
