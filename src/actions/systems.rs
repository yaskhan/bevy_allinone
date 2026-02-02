use bevy::prelude::*;
use crate::input::InputState;
use super::types::*;

pub fn update_action_system(
    mut start_action_queue: ResMut<StartActionEventQueue>,
    mut end_action_queue: ResMut<EndActionEventQueue>,
    mut player_query: Query<(Entity, &mut PlayerActionSystem, &mut Transform)>,
    mut action_query: Query<(Entity, &mut ActionSystem, &GlobalTransform)>,
    mut commands: Commands,
    time: Res<Time>,
    input: Res<InputState>,
) {
    // 1. Handle Start Action
    for event in start_action_queue.0.drain(..) {
        if let Ok((_player_entity, mut player_action, _)) = player_query.get_mut(event.player_entity) {
             // Only start if not already active
            if player_action.is_action_active {
                 continue;
            }

            if let Ok((action_entity, action, _action_transform)) = action_query.get_mut(event.action_entity) {
                // Set Player State
                player_action.current_action = Some(action_entity);
                player_action.is_action_active = true;
                player_action.action_timer = 0.0;
                
                // Determine initial state
                if action.use_position_to_adjust_player && action.match_target_transform.is_some() {
                    player_action.state = ActionState::AdjustingTransform;
                } else {
                    player_action.state = ActionState::PlayingAnimation;
                }
                
                // TODO: Backup physics/gravity state here
                // In a real implementation we might read from the character controller
                
                info!("Started action: {} in state {:?}", action.action_name, player_action.state);
            }
        }
    }
    
    // 2. Update Active Actions
    for (player_entity, mut player_action, mut player_transform) in player_query.iter_mut() {
        if !player_action.is_action_active { continue; }
        
        let mut action_ended = false;

        if let Some(action_entity) = player_action.current_action {
             if let Ok((_, action, action_glob_transform)) = action_query.get(action_entity) {
                 
                 match player_action.state {
                    ActionState::AdjustingTransform => {
                         if let Some(target_local) = action.match_target_transform {
                            // Calculate target world transform
                            // Assuming match_target_transform is local to the action entity
                            // If it's absolute, we use it directly. Assuming relative for now as per comment.
                            // But for simplicity in this port, let's treat match_target_transform as an offset from action root
                            
                            let action_rotation = action_glob_transform.compute_transform().rotation;
                            let action_translation = action_glob_transform.translation();
                            
                            let target_rotation = action_rotation * target_local.rotation;
                            let target_translation = action_translation + action_rotation * target_local.translation;
                            
                            // Interpolate
                            let speed = action.adjust_player_position_speed * time.delta_secs();
                            
                            let new_pos = player_transform.translation.lerp(target_translation, speed);
                            let new_rot = player_transform.rotation.slerp(target_rotation, speed);
                            
                            player_transform.translation = new_pos;
                            player_transform.rotation = new_rot;
                            
                            let dist = player_transform.translation.distance(target_translation);
                            let angle = player_transform.rotation.angle_between(target_rotation);
                            
                            // Check if reached
                            if dist < 0.05 && angle < 0.1 {
                                player_transform.translation = target_translation;
                                player_transform.rotation = target_rotation;
                                player_action.state = ActionState::PlayingAnimation;
                                player_action.action_timer = 0.0;
                                info!("Action System: Finished adjustment, starting animation");
                            }
                         } else {
                             // Fallback if data missing
                             player_action.state = ActionState::PlayingAnimation;
                         }
                    }
                    ActionState::PlayingAnimation => {
                        player_action.action_timer += time.delta_secs();
                        
                        if player_action.action_timer >= action.duration {
                            player_action.state = ActionState::Finished;
                        }
                    }
                    ActionState::Finished => {
                        action_ended = true;
                    }
                    ActionState::Idle => {
                        // Should not happen if active
                        action_ended = true;
                    }
                 }

             } else {
                 // Action entity missing?
                 action_ended = true;
             }
        } else {
             action_ended = true;
        }
        
        if action_ended {
            end_action_queue.0.push(EndActionEvent {
                player_entity,
                action_entity: player_action.current_action.unwrap_or(Entity::PLACEHOLDER),
            });
        }
    }
    
    // 3. Handle End Action
    for event in end_action_queue.0.drain(..) {
        if let Ok((_, mut player_action, _)) = player_query.get_mut(event.player_entity) {
            // Validate it's the correct action ending
            let is_matching_action = player_action.current_action == Some(event.action_entity) || event.action_entity == Entity::PLACEHOLDER;
            
            if is_matching_action && player_action.is_action_active {
                player_action.is_action_active = false;
                player_action.current_action = None;
                player_action.state = ActionState::Idle;
                
                // TODO: Restore physics/gravity state
                info!("Ended action");
            }
        }
    }
    
    // 4. Input Detection (Simple Proximity Check for now)
    // In a real implementation this would be more optimized or use the existing spatial query system
    if input.interact_pressed {
         for (player_entity, player_action, player_transform) in player_query.iter() {
             if player_action.is_action_active { continue; }
             
             for (action_entity, action, action_transform) in action_query.iter() {
                 if !action.is_active { continue; }
                 
                 let dist = player_transform.translation.distance(action_transform.translation());
                 if dist <= action.min_distance {
                     // Check Angle if needed
                     // Simple forward check
                     let to_action = (action_transform.translation() - player_transform.translation).normalize_or_zero();
                     let player_forward = player_transform.forward();
                     
                     let dot = player_forward.dot(to_action);
                     
                     // ~45 degrees is dot > 0.707
                     let angle_threshold = if action.use_min_angle { action.min_angle.to_radians().cos() } else { -1.0 };
                     
                     if dot >= angle_threshold {
                         start_action_queue.0.push(StartActionEvent {
                             player_entity,
                             action_entity,
                         });
                         break; // Only start one action
                     }
                 }
             }
         }
    }
}
