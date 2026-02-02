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
                player_action.walk_timer = 0.0;
                player_action.walk_state = WalkToTargetState::Idle;
                
                // Initialize event system
                player_action.events_active = false;
                player_action.event_start_time = 0.0;
                
                // Determine initial state
                if action.use_walk_to_target_before_action {
                    player_action.state = ActionState::WalkingToTargetBefore;
                } else if action.use_position_to_adjust_player && action.match_target_transform.is_some() {
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
                    ActionState::WalkingToTargetBefore => {
                        // Walk-to-target is handled by update_walk_to_target_system
                        // This state just waits for walk completion
                        if player_action.walk_state == WalkToTargetState::ReachedTarget {
                            // Transition to next state
                            if action.use_position_to_adjust_player && action.match_target_transform.is_some() {
                                player_action.state = ActionState::AdjustingTransform;
                            } else {
                                player_action.state = ActionState::PlayingAnimation;
                            }
                            player_action.action_timer = 0.0;
                            info!("Action System: Reached walk target, transitioning to {:?}", player_action.state);
                        } else if player_action.walk_state == WalkToTargetState::TimedOut {
                            // Timeout - cancel action
                            warn!("Action System: Walk to target timed out, cancelling action");
                            action_ended = true;
                        }
                    }
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
                            // Check if we need to walk after action
                            if action.use_walk_to_target_after_action {
                                player_action.state = ActionState::WalkingToTargetAfter;
                                player_action.walk_timer = 0.0;
                                player_action.walk_state = WalkToTargetState::Idle;
                            } else {
                                player_action.state = ActionState::Finished;
                            }
                        }
                    }
                    ActionState::WalkingToTargetAfter => {
                        // Walk-to-target is handled by update_walk_to_target_system
                        if player_action.walk_state == WalkToTargetState::ReachedTarget {
                            player_action.state = ActionState::Finished;
                            info!("Action System: Reached walk target after action");
                        } else if player_action.walk_state == WalkToTargetState::TimedOut {
                            warn!("Action System: Walk after action timed out");
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

/// System to update animator parameters based on player action state
pub fn update_animator_parameters_system(
    mut query: Query<(&PlayerActionSystem, &mut AnimatorParameters)>,
    action_query: Query<&ActionSystem>,
) {
    for (player_action, mut animator_params) in query.iter_mut() {
        if player_action.is_action_active {
            if let Some(action_entity) = player_action.current_action {
                if let Ok(action) = action_query.get(action_entity) {
                    // Set action active flags
                    if action.animation_used_on_upper_body {
                        animator_params.action_active_upper_body = true;
                        if action.disable_regular_action_active_state {
                            animator_params.action_active = false;
                        }
                    } else {
                        animator_params.action_active = true;
                    }
                    
                    // Set action ID
                    if action.use_action_id {
                        animator_params.action_id = action.action_id;
                    }
                }
            }
        } else {
            // Reset animator parameters when no action is active
            animator_params.action_active = false;
            animator_params.action_active_upper_body = false;
            animator_params.action_id = 0;
        }
    }
}

/// System to apply match target during actions
pub fn apply_match_target_system(
    mut query: Query<(&mut PlayerActionSystem, &mut Transform)>,
    mut action_query: Query<&mut ActionSystem>,
    time: Res<Time>,
) {
    for (player_action, mut player_transform) in query.iter_mut() {
        if !player_action.is_action_active {
            continue;
        }
        
        if let Some(action_entity) = player_action.current_action {
            if let Ok(mut action) = action_query.get_mut(action_entity) {
                if !action.use_match_target {
                    continue;
                }
                
                // Clone the match config to avoid borrow issues
                if let Some(match_config) = action.match_target_config.clone() {
                    if !match_config.enabled {
                        continue;
                    }
                    
                    // Update normalized time (assuming duration-based)
                    let normalized_time = (player_action.action_timer / action.duration).clamp(0.0, 1.0);
                    
                    // Update the config's normalized time
                    if let Some(ref mut config) = action.match_target_config {
                        config.current_normalized_time = normalized_time;
                    }
                    
                    // Check if we're in the match target time window
                    if normalized_time >= match_config.start_time && normalized_time <= match_config.end_time {
                        // Calculate interpolation factor within the window
                        let window_duration = match_config.end_time - match_config.start_time;
                        let window_progress = if window_duration > 0.0 {
                            (normalized_time - match_config.start_time) / window_duration
                        } else {
                            1.0
                        };
                        
                        // Apply position matching with weights
                        let target_pos = match_config.target_position;
                        let current_pos = player_transform.translation;
                        let weighted_target = Vec3::new(
                            current_pos.x + (target_pos.x - current_pos.x) * match_config.position_weight.x,
                            current_pos.y + (target_pos.y - current_pos.y) * match_config.position_weight.y,
                            current_pos.z + (target_pos.z - current_pos.z) * match_config.position_weight.z,
                        );
                        player_transform.translation = player_transform.translation.lerp(weighted_target, window_progress);
                        
                        // Apply rotation matching with weight
                        if match_config.rotation_weight > 0.0 {
                            player_transform.rotation = player_transform.rotation.slerp(
                                match_config.target_rotation,
                                window_progress * match_config.rotation_weight
                            );
                        }
                    }
                }
            }
        }
    }
}

/// System to handle custom action activation requests
pub fn handle_custom_action_activation_system(
    mut activate_queue: ResMut<ActivateCustomActionEventQueue>,
    mut start_action_queue: ResMut<StartActionEventQueue>,
    mut interrupted_queue: ResMut<ActionInterruptedEventQueue>,
    custom_action_manager: Res<CustomActionManager>,
    mut player_query: Query<&mut PlayerActionSystem>,
    custom_action_query: Query<&CustomActionInfo>,
    action_query: Query<&ActionSystem>,
) {
    for event in activate_queue.0.drain(..) {
        let action_name_lower = event.action_name.to_lowercase();
        
        // Look up action by name
        if let Some(&custom_action_entity) = custom_action_manager.action_lookup.get(&action_name_lower) {
            if let Ok(custom_action_info) = custom_action_query.get(custom_action_entity) {
                if !custom_action_info.enabled {
                    continue;
                }
                
                // Check conditions
                // TODO: Add locked camera state, aiming state, on ground checks when those systems exist
                
                // Handle probability
                if custom_action_info.use_probability {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    let roll: f32 = rng.gen();
                    if roll > custom_action_info.probability {
                        info!("Action {} failed probability check ({} > {})", custom_action_info.name, roll, custom_action_info.probability);
                        continue;
                    }
                }
                
                // Determine which action entity to use
                let action_entity_to_use = if custom_action_info.use_random_action_list {
                    if custom_action_info.random_action_entities.is_empty() {
                        continue;
                    }
                    
                    if custom_action_info.follow_actions_order {
                        // Use current index (would need to be mutable in real implementation)
                        let index = custom_action_info.current_action_index % custom_action_info.random_action_entities.len();
                        custom_action_info.random_action_entities[index]
                    } else {
                        // Random selection
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        let index = rng.gen_range(0..custom_action_info.random_action_entities.len());
                        custom_action_info.random_action_entities[index]
                    }
                } else {
                    custom_action_info.action_system_entity.unwrap_or(Entity::PLACEHOLDER)
                };
                
                if action_entity_to_use == Entity::PLACEHOLDER {
                    continue;
                }
                
                // Check if we need to interrupt current action
                if let Ok(mut player_action) = player_query.get_mut(event.player_entity) {
                    let mut can_start = true;
                    
                    if player_action.is_action_active {
                        if let Some(current_action_entity) = player_action.current_action {
                            if let Ok(current_action) = action_query.get(current_action_entity) {
                                // Check if current action can be stopped
                                if !current_action.can_stop_previous_action {
                                    // Check if new action can force interrupt
                                    if custom_action_info.can_interrupt_other_actions {
                                        let can_interrupt = if custom_action_info.use_category_to_check_interrupt {
                                            custom_action_info.action_categories_to_interrupt.contains(&current_action.category_name)
                                        } else {
                                            custom_action_info.action_names_to_interrupt.contains(&current_action.action_name)
                                        };
                                        
                                        if can_interrupt || custom_action_info.can_force_interrupt {
                                            // Interrupt the current action
                                            interrupted_queue.0.push(ActionInterruptedEvent {
                                                player_entity: event.player_entity,
                                                interrupted_action_entity: current_action_entity,
                                                new_action_entity: action_entity_to_use,
                                            });
                                            
                                            info!("Action {} interrupted by {}", current_action.action_name, custom_action_info.name);
                                        } else {
                                            // Queue for later
                                            player_action.action_waiting_to_resume = Some(action_entity_to_use);
                                            can_start = false;
                                        }
                                    } else {
                                        // Queue for later
                                        player_action.action_waiting_to_resume = Some(action_entity_to_use);
                                        can_start = false;
                                    }
                                }
                            }
                        }
                    }
                    
                    if can_start {
                        // Set category
                        player_action.current_action_category = Some(custom_action_info.category.name.clone());
                        
                        // Start the action
                        start_action_queue.0.push(StartActionEvent {
                            player_entity: event.player_entity,
                            action_entity: action_entity_to_use,
                        });
                        
                        info!("Starting custom action: {}", custom_action_info.name);
                    }
                }
            }
        } else {
            warn!("Custom action '{}' not found in manager", event.action_name);
        }
    }
}

/// System to update custom action manager lookup tables
pub fn update_custom_action_manager_system(
    mut manager: ResMut<CustomActionManager>,
    query: Query<(Entity, &CustomActionInfo), Changed<CustomActionInfo>>,
) {
    for (entity, custom_action_info) in query.iter() {
        let name_lower = custom_action_info.name.to_lowercase();
        
        // Update name lookup
        manager.action_lookup.insert(name_lower, entity);
        
        // Update category lookup
        let category_name = custom_action_info.category.name.clone();
        manager.category_lookup
            .entry(category_name)
            .or_insert_with(Vec::new)
            .push(entity);
    }
}

/// System to handle walk-to-target for actions
pub fn update_walk_to_target_system(
    mut player_query: Query<(&mut PlayerActionSystem, Entity)>,
    action_query: Query<&ActionSystem>,
    mut navmesh_query: Query<&mut crate::player::navmesh_override::NavMeshOverride>,
    mut enable_queue: ResMut<crate::player::navmesh_override::EnableNavMeshOverrideQueue>,
    mut target_queue: ResMut<crate::player::navmesh_override::SetNavMeshTargetQueue>,
) {
    for (mut player_action, player_entity) in player_query.iter_mut() {
        if !player_action.is_action_active {
            continue;
        }
        
        // Only process if in a walking state
        let is_walking_state = matches!(
            player_action.state,
            ActionState::WalkingToTargetBefore | ActionState::WalkingToTargetAfter
        );
        
        if !is_walking_state {
            continue;
        }
        
        if let Some(action_entity) = player_action.current_action {
            if let Ok(action) = action_query.get(action_entity) {
                // Initialize walk if needed
                if player_action.walk_state == WalkToTargetState::Idle {
                    // Enable NavMesh override
                    enable_queue.0.push(crate::player::navmesh_override::EnableNavMeshOverrideEvent {
                        entity: player_entity,
                    });
                    
                    // Set target
                    target_queue.0.push(crate::player::navmesh_override::SetNavMeshTargetEvent {
                        entity: player_entity,
                        target_position: action.walk_target_position,
                        target_entity: action.walk_target_entity,
                    });
                    
                    // Configure NavMesh settings
                    if let Ok(mut navmesh) = navmesh_query.get_mut(player_entity) {
                        navmesh.walk_speed = action.max_walk_speed;
                        navmesh.min_distance = action.min_distance_to_target;
                        navmesh.timeout = action.walk_timeout;
                        navmesh.elapsed_time = 0.0;
                    }
                    
                    player_action.walk_state = WalkToTargetState::Walking;
                    player_action.walk_timer = 0.0;
                    
                    info!("Walk-to-Target: Started walking for action {}", action.action_name);
                }
                
                // Monitor NavMesh status
                if player_action.walk_state == WalkToTargetState::Walking {
                    if let Ok(navmesh) = navmesh_query.get(player_entity) {
                        match navmesh.path_status.as_str() {
                            "Reached" => {
                                player_action.walk_state = WalkToTargetState::ReachedTarget;
                                info!("Walk-to-Target: Reached target");
                            }
                            "TimedOut" => {
                                player_action.walk_state = WalkToTargetState::TimedOut;
                                warn!("Walk-to-Target: Timed out");
                            }
                            _ => {
                                // Still walking
                                player_action.walk_timer += 0.016; // Approximate delta
                            }
                        }
                    }
                }
            }
        }
    }
}


/// System to process timed events during actions
pub fn process_action_events_system(
    mut player_query: Query<(&mut PlayerActionSystem, Entity)>,
    mut action_query: Query<&mut ActionSystem>,
    mut event_queue: ResMut<ActionEventTriggeredQueue>,
    mut remote_queue: ResMut<RemoteActionEventQueue>,
    time: Res<Time>,
) {
    for (mut player_action, player_entity) in player_query.iter_mut() {
        if !player_action.is_action_active {
            continue;
        }
        
        if let Some(action_entity) = player_action.current_action {
            if let Ok(mut action) = action_query.get_mut(action_entity) {
                if !action.use_event_list || action.event_list.is_empty() {
                    continue;
                }
                
                // Initialize events on first frame
                if !player_action.events_active {
                    player_action.events_active = true;
                    player_action.event_start_time = time.elapsed_secs();
                    
                    // Reset all event triggered flags
                    for event in action.event_list.iter_mut() {
                        event.event_triggered = false;
                    }
                }
                
                let elapsed = time.elapsed_secs() - player_action.event_start_time;
                
                if action.use_accumulative_delay {
                    // Accumulative mode: Sequential events
                    let mut accumulated_time = 0.0;
                    for event in action.event_list.iter_mut() {
                        if !event.event_triggered {
                            accumulated_time += event.delay_to_activate;
                            if elapsed >= accumulated_time {
                                fire_action_event(event, action_entity, player_entity, &mut event_queue, &mut remote_queue);
                                event.event_triggered = true;
                            } else {
                                break; // Stop checking further events
                            }
                        } else {
                            accumulated_time += event.delay_to_activate;
                        }
                    }
                } else {
                    // Parallel mode: All events check independently
                    for event in action.event_list.iter_mut() {
                        if !event.event_triggered && elapsed >= event.delay_to_activate {
                            fire_action_event(event, action_entity, player_entity, &mut event_queue, &mut remote_queue);
                            event.event_triggered = true;
                        }
                    }
                }
            }
        }
    }
}

fn fire_action_event(
    event: &ActionEvent,
    action_entity: Entity,
    player_entity: Entity,
    event_queue: &mut ActionEventTriggeredQueue,
    remote_queue: &mut RemoteActionEventQueue,
) {
    if event.use_bevy_event {
        event_queue.0.push(ActionEventTriggered {
            action_entity,
            player_entity,
            event_name: event.bevy_event_name.clone(),
        });
        info!("Action Event: {} triggered", event.bevy_event_name);
    }
    
    if event.use_remote_event {
        remote_queue.0.push(RemoteActionEvent {
            event_name: event.remote_event_name.clone(),
            player_entity: if event.send_player_entity { Some(player_entity) } else { None },
        });
        info!("Remote Event: {} triggered", event.remote_event_name);
    }
}
