use bevy::prelude::*;
use super::skills_system::SkillsSystem;
use super::skill::Skill;
use super::types::*;
use crate::experience::types::PlayerExperience;
use crate::stats::types::{StatModifier, ModifierType, AddModifierEventQueue, AddModifierEvent};
use crate::abilities::types::{SetAbilityEnabledEventQueue, SetAbilityEnabledEvent};

/// Helper to apply effects of a skill to an entity
fn apply_effects_for_skill(
    entity: Entity,
    skill: &Skill,
    stat_events: &mut ResMut<AddModifierEventQueue>,
    ability_events: &mut ResMut<SetAbilityEnabledEventQueue>,
) {
    let skill_name = skill.name.clone();
    
    // Apply effects for current level
    let effects_to_apply = if skill.levels.is_empty() {
        skill.effects.clone()
    } else {
        // Multi-level skills have effects per level
        // For purchase, current_level was just incremented, so we apply the level just reached
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
}

/// Skills system update
pub fn skills_system_update(
    mut query: Query<(Entity, &mut SkillsSystem)>,
    mut event_queue: ResMut<SkillSystemEventQueue>,
    mut experience_query: Query<&mut PlayerExperience>,
    mut stat_events: ResMut<AddModifierEventQueue>,
    mut ability_events: ResMut<SetAbilityEnabledEventQueue>,
) {
    // Initial initialization
    for (entity, mut skills_system) in query.iter_mut() {
        if skills_system.active && !skills_system.effects_initialized {
            // Apply effects for all current skill levels
            let mut effects_to_apply = Vec::new();
            
            for category in &skills_system.skill_tree.categories {
                for skill in &category.skills {
                    if !skill.enabled || (!skill.active && !skill.complete && skill.current_level == 0) {
                        continue;
                    }

                    // Collect effects for current level
                    if skill.levels.is_empty() {
                        if skill.complete || skill.active {
                            for effect in &skill.effects {
                                effects_to_apply.push((skill.name.clone(), effect.clone()));
                            }
                        }
                    } else {
                        // For leveled skills, apply effects for ALL reached levels?
                        // Usually it's either cumulative or just current.
                        // If it's permanent stat boosts, they should be applied once.
                        for i in 0..skill.current_level as usize {
                            if i < skill.levels.len() {
                                for effect in &skill.levels[i].effects {
                                    effects_to_apply.push((skill.name.clone(), effect.clone()));
                                }
                            }
                        }
                    }
                }
            }

            for (skill_name, effect) in effects_to_apply {
                match effect {
                    SkillEffect::StatModifier { stat, amount, is_percentage } => {
                        stat_events.0.push(AddModifierEvent {
                            modifier: StatModifier::new(
                                &format!("Skill Initialize: {}", skill_name),
                                if amount >= 0.0 { ModifierType::Buff } else { ModifierType::Debuff },
                                stat,
                                amount,
                                is_percentage,
                                0.0,
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

            skills_system.effects_initialized = true;
            info!("Skills initialized for entity {:?}", entity);
        }
    }

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
                            apply_effects_for_skill(entity, skill, &mut stat_events, &mut ability_events);
                            info!("Purchased skill: {} (Level {})", skill.name, skill.current_level);
                        }
                    } else {
                        warn!("Failed to purchase skill (not enough points or prerequisite not met)");
                    }
                }
            }
        } else if let SkillSystemEvent::BoolSkillActivated { entity, skill_name, state } = event {
            if let Ok((entity, mut skills_system)) = query.get_mut(*entity) {
                if !skills_system.active { continue; }
                
                if let Some(skill) = skills_system.skill_tree.get_skill_by_name(skill_name) {
                    // For boolean skills, state change might mean enabling/disabling ability or modifier
                    // Currently StatModifier is added permanently. 
                    // If we want to support "Toggles", we need a way to remove modifiers.
                    // For now, let's just apply effects if state is true
                    if *state {
                        apply_effects_for_skill(entity, skill, &mut stat_events, &mut ability_events);
                    } else {
                        // Removal logic would go here
                        // e.g. ability_events.0.push(SetAbilityEnabledEvent { ..., enabled: false });
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
