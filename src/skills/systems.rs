use bevy::prelude::*;
use super::skills_system::SkillsSystem;
use super::types::*;
use crate::experience::types::PlayerExperience;
use crate::stats::types::{StatModifier, ModifierType, AddModifierEventQueue, AddModifierEvent};
use crate::abilities::types::{SetAbilityEnabledEventQueue, SetAbilityEnabledEvent};

/// Skills system update
pub fn skills_system_update(
    mut query: Query<(Entity, &mut SkillsSystem)>,
    mut event_queue: ResMut<SkillSystemEventQueue>,
    mut experience_query: Query<&mut PlayerExperience>,
    mut stat_events: ResMut<AddModifierEventQueue>,
    mut ability_events: ResMut<SetAbilityEnabledEventQueue>,
) {
    // Process purchase requests
    for event in event_queue.0.drain(..) {
        if let SkillSystemEvent::PurchaseSkillRequest { player_entity, category_index, skill_index } = event {
            if let Ok((entity, mut skills_system)) = query.get_mut(player_entity) {
                if !skills_system.active { continue; }

                // Check experience
                if let Ok(mut experience) = experience_query.get_mut(entity) {
                    let available_points = experience.skill_points;
                    
                    // Check prerequisites and points in SkillTree
                    if let Some(required_points) = skills_system.skill_tree.use_skill_points(category_index, skill_index, available_points, false) {
                        // Success!
                        experience.skill_points -= required_points;
                        
                        // Get the skill to apply effects
                        if let Some(skill) = skills_system.skill_tree.get_skill_by_index(category_index, skill_index) {
                            let skill_name = skill.name.clone();
                            
                            // Apply effects for current level
                            let effects_to_apply = if skill.levels.is_empty() {
                                skill.effects.clone()
                            } else {
                                // Multi-level skills have effects per level
                                // Subtract 1 because current_level was just incremented in use_skill_points
                                let level_idx = (skill.current_level as usize).saturating_sub(1);
                                if level_idx < skill.levels.len() {
                                    skill.levels[level_idx].effects.clone()
                                } else {
                                    Vec::new()
                                }
                            };

                            for effect in effects_to_apply {
                                match effect {
                                    SkillEffect::StatModifier { stat, amount, is_percentage } => {
                                        stat_events.0.push(AddModifierEvent {
                                            modifier: StatModifier::new(
                                                &format!("Skill: {}", skill_name),
                                                if amount >= 0.0 { ModifierType::Buff } else { ModifierType::Debuff },
                                                stat,
                                                amount,
                                                is_percentage,
                                                0.0, // Permanent
                                            ),
                                        });
                                    }
                                    SkillEffect::UnlockAbility(ability_name) => {
                                        ability_events.0.push(SetAbilityEnabledEvent {
                                            player_entity: entity,
                                            ability_name,
                                            enabled: true,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            
                            info!("Purchased skill: {} (Level {})", skill_name, skill.current_level);
                        }
                    } else {
                        warn!("Failed to purchase skill (not enough points or prerequisite not met)");
                    }
                }
            }
        }
    }

    // Additional update logic for each skills system
    for (_entity, mut skills_system) in query.iter_mut() {
        if !skills_system.active {
            continue;
        }
        // ...
    }
}
